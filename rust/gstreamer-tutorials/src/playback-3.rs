mod error;

use gstreamer::glib::{MainLoop, SourceId};
use gstreamer::prelude::*;
use gstreamer::*;
use gstreamer_audio::{AudioFormat, AudioInfo};
use std::slice;
use std::sync::{Arc, Mutex};

const CHUNK_SIZE: usize = 1024; /* Amount of bytes we are sending in each buffer */
const SAMPLE_RATE: u32 = 44100; /* Samples per second we are sending */

/* Structure to contain all our information, so we can pass it to callbacks */
struct CustomData {
    /* For waveform generation */
    a: f64,
    b: f64,
    c: f64,
    d: f64,

    num_samples: usize,

    source_id: Option<SourceId>,
}

/* This method is called by the idle GSource in the mainloop, to feed CHUNK_SIZE bytes into appsrc.
 * The idle handler is added to the mainloop when appsrc requests us to start sending data (need-data signal)
 * and is removed when appsrc has enough data (enough-data signal).
 */
fn push_data(data: Arc<Mutex<CustomData>>, app_source: &Element) -> Continue {
    let num_samples = CHUNK_SIZE / 2; /* Because each sample is 16 bits */

    /* Create a new empty buffer */
    let mut buffer = Buffer::with_size(CHUNK_SIZE).unwrap();

    {
        let mut data = data.lock().unwrap();
        /* Set its timestamp and duration */
        let buffer_inner = buffer.get_mut().unwrap();
        buffer_inner.set_pts(Some(ClockTime::from_nseconds(
            (data.num_samples * 1_000_000_000 / SAMPLE_RATE as usize) as _,
        )));
        buffer_inner.set_duration(Some(ClockTime::from_nseconds(
            (num_samples * 1_000_000_000 / SAMPLE_RATE as usize) as _,
        )));

        /* Generate some psychodelic waveforms */
        data.c += data.d;
        data.d -= data.c / 1000.0;
        let freq = 1100.0 + 1000.0 * data.d;

        let mut map = buffer_inner.map_writable().unwrap();
        unsafe {
            let map = slice::from_raw_parts_mut(
                map.as_mut_slice().as_mut_ptr() as *mut i16,
                map.len() / 2,
            );
            for raw in map {
                data.a += data.b;
                data.b -= data.a / freq;
                let value = (500.0 * data.a).floor() as i64;
                *raw = i16::try_from(value).unwrap();
            }
        }
        data.num_samples += num_samples;
    }

    /* Push the buffer into the appsrc */
    let ret: FlowReturn = app_source.emit_by_name("push-buffer", &[&buffer]);

    /* Free the buffer now that we are done with it */

    if ret != FlowReturn::Ok {
        /* We got some error, stop sending data */
        return Continue(false);
    }

    Continue(true)
}

/* This signal callback triggers when appsrc needs data. Here, we add an idle handler
 * to the mainloop to start pushing data into the appsrc */
fn start_feed(data: Arc<Mutex<CustomData>>, app_source: &Element) {
    let data_for_idle = Arc::clone(&data);
    let mut data = data.lock().unwrap();
    if data.source_id.is_some() {
        return;
    }
    println!("Start feeding");
    let app_source = app_source.clone();
    data.source_id = Some(glib::idle_add(move || {
        let data = Arc::clone(&data_for_idle);
        push_data(data, &app_source)
    }));
}

/* This callback triggers when appsrc has enough data and we can stop sending.
 * We remove the idle handler from the mainloop */
fn stop_feed(source_id: &mut Option<SourceId>) {
    if source_id.is_none() {
        return;
    }
    println!("Stop feeding");
    std::mem::replace(source_id, None).unwrap().remove();
}

/* This function is called when an error message is posted on the bus */
fn error_cb(err: &message::Error, main_loop: &MainLoop) {
    /* Print error details on the screen */
    let name_src = err.src().unwrap().name();
    let debug_info = err.debug().unwrap_or_else(|| "none".into());
    eprintln!("Error received from element {}: {}", name_src, err.error());
    eprintln!("Debugging information: {}", debug_info);

    main_loop.quit();
}

/* This function is called when playbin has created the appsrc element, so we have
 * a chance to configure it. */
fn source_setup(source: &Element, data: &Arc<Mutex<CustomData>>) {
    println!("Source has been created. Configuring.");
    // todo!();

    /* Configure appsrc */
    let info = AudioInfo::builder(AudioFormat::S16le, SAMPLE_RATE, 1)
        .build()
        .unwrap();
    let audio_caps = info.to_caps().unwrap();
    source.set_properties(&[("caps", &audio_caps), ("format", &Format::Time)]);

    let source_for_need_data = source.clone();
    let data_for_need_data = Arc::clone(&data);
    source.connect("need-data", false, move |_| {
        start_feed(Arc::clone(&data_for_need_data), &source_for_need_data);
        None
    });
    let data_for_enough_data = Arc::clone(&data);
    source.connect("enough-data", false, move |_| {
        let mut data = data_for_enough_data.lock().unwrap();
        stop_feed(&mut data.source_id);
        None
    });
}

fn main() -> Result<(), error::Error> {
    let data = Arc::new(Mutex::new(CustomData {
        a: 0.0,
        b: 1.0, /* For waveform generation */
        c: 0.0,
        d: 1.0,
        num_samples: 0,
        source_id: None,
    }));

    /* Initialize GStreamer */
    init()?;

    /* Create the playbin element */
    let pipeline = parse_launch("playbin uri=appsrc://")?;
    let data_for_source_setup = Arc::clone(&data);
    pipeline.connect("source-setup", false, move |args| {
        let source = args[1].get::<Element>().unwrap();
        source_setup(&source, &data_for_source_setup);
        None
    });

    /* Create a GLib Main Loop and set it to run */
    let main_loop = glib::MainLoop::new(None, false);

    /* Instruct the bus to emit signals for each received message, and connect to the interesting signals */
    let bus = pipeline.bus().unwrap();
    bus.add_signal_watch();
    let main_loop_for_error_cb = main_loop.clone();
    bus.connect_message(Some("error"), move |_, message| match message.view() {
        MessageView::Error(err) => {
            error_cb(err, &main_loop_for_error_cb);
        }
        _ => {
            panic!();
        }
    });
    drop(bus);

    /* Start playing the pipeline */
    let _ = pipeline.set_state(State::Playing);

    main_loop.run();

    /* Free resources */
    let _ = pipeline.set_state(State::Null);
    drop(pipeline);

    Ok(())
}
