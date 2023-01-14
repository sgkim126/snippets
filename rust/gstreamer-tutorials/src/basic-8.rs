mod error;

use gstreamer::glib::{MainLoop, SourceId};
use gstreamer::prelude::*;
use gstreamer::*;
use gstreamer_audio::{AudioFormat, AudioInfo};
use std::io::Write;
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
        let mut data = Arc::clone(&data_for_idle);
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

/* The appsink has received a buffer */
fn new_sample(sink: &Element) -> FlowReturn {
    /* Retrieve the buffer */
    let _sample: Sample = sink.emit_by_name("pull-sample", &[]);
    print!("*");
    let _ = std::io::stdout().flush();
    return FlowReturn::Ok;
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

fn main() -> Result<(), error::Error> {
    let data = Arc::new(Mutex::new(CustomData {
        a: 0.0,
        b: 1.0,
        c: 0.0,
        d: 1.0,
        num_samples: 0,
        source_id: None,
    }));

    /* Initialize GStreamer */
    init()?;

    /* Create the elements */
    let app_source = ElementFactory::make("appsrc")
        .name("audio_source")
        .build()?;
    let tee = ElementFactory::make("tee").name("tee").build()?;
    let audio_queue = ElementFactory::make("queue").name("audio_queue").build()?;
    let audio_convert1 = ElementFactory::make("audioconvert")
        .name("audio_convert1")
        .build()?;
    let audio_resample = ElementFactory::make("audioresample")
        .name("audio_resample")
        .build()?;
    let audio_sink = ElementFactory::make("autoaudiosink")
        .name("audio_sink")
        .build()?;
    let video_queue = ElementFactory::make("queue").name("video_queue").build()?;
    let audio_convert2 = ElementFactory::make("audioconvert")
        .name("audio_convert2")
        .build()?;
    let visual = ElementFactory::make("wavescope").name("visual").build()?;
    let video_convert = ElementFactory::make("videoconvert")
        .name("video_convert")
        .build()?;
    let video_sink = ElementFactory::make("autovideosink")
        .name("video_sink")
        .build()?;
    let app_queue = ElementFactory::make("queue").name("app_queue").build()?;
    let app_sink = ElementFactory::make("appsink").name("app_sink").build()?;

    /* Create the empty pipeline */
    let pipeline = Pipeline::new(Some("test-pipeline"));

    /* Configure wavescope */
    visual.set_property_from_str("shader", "none");
    visual.set_property_from_str("style", "dots");

    /* Configure appsrc */
    let info = AudioInfo::builder(AudioFormat::S16le, SAMPLE_RATE, 1).build()?;
    let audio_caps = info.to_caps()?;
    app_source.set_property("caps", &audio_caps);
    app_source.set_property_from_str("format", "time");

    let app_source_for_need_data = app_source.clone();
    let data_for_need_data = Arc::clone(&data);
    app_source.connect("need-data", false, move |_| {
        start_feed(Arc::clone(&data_for_need_data), &app_source_for_need_data);
        None
    });
    let data_for_enough_data = Arc::clone(&data);
    app_source.connect("enough-data", false, move |_| {
        let mut data = data_for_enough_data.lock().unwrap();
        stop_feed(&mut data.source_id);
        None
    });

    /* Configure appsink */
    app_sink.set_properties(&[("emit-signals", &true), ("caps", &audio_caps)]);
    let app_sink_for_new_sample = app_sink.clone();
    app_sink.connect("new-sample", false, move |_| {
        Some(new_sample(&app_sink_for_new_sample).to_value())
    });
    drop(audio_caps);

    /* Link all elements that can be automatically linked because they have "Always" pads */
    pipeline.add_many(&[
        &app_source,
        &tee,
        &audio_queue,
        &audio_convert1,
        &audio_resample,
        &audio_sink,
        &video_queue,
        &audio_convert2,
        &visual,
        &video_convert,
        &video_sink,
        &app_queue,
        &app_sink,
    ])?;
    if Element::link_many(&[&app_source, &tee]).is_err()
        || Element::link_many(&[&audio_queue, &audio_convert1, &audio_resample, &audio_sink])
            .is_err()
        || Element::link_many(&[
            &video_queue,
            &audio_convert2,
            &visual,
            &video_convert,
            &video_sink,
        ])
        .is_err()
        || Element::link_many(&[&app_queue, &app_sink]).is_err()
    {
        panic!("Elements could not be linked.");
    }

    /* Manually link the Tee, which has "Request" pads */
    let tee_audio_pad = tee.request_pad_simple("src_%u").unwrap();
    println!(
        "Obtained request pad {} for audio branch.",
        tee_audio_pad.name()
    );
    let queue_audio_pad = audio_queue.static_pad("sink").unwrap();
    let tee_video_pad = tee.request_pad_simple("src_%u").unwrap();
    println!(
        "Obtained request pad {} for video branch.",
        tee_video_pad.name()
    );
    let queue_video_pad = video_queue.static_pad("sink").unwrap();
    let tee_app_pad = tee.request_pad_simple("src_%u").unwrap();
    println!(
        "Obtained request pad {} for app branch.",
        tee_app_pad.name()
    );
    let queue_app_pad = app_queue.static_pad("sink").unwrap();
    if tee_audio_pad.link(&queue_audio_pad).is_err()
        || tee_video_pad.link(&queue_video_pad).is_err()
        || tee_app_pad.link(&queue_app_pad).is_err()
    {
        panic!("Tee could not be linked");
    }
    drop(queue_audio_pad);
    drop(queue_video_pad);
    drop(queue_app_pad);

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

    /* Release the request pads from the Tee, and unref them */
    tee.release_request_pad(&tee_audio_pad);
    tee.release_request_pad(&tee_video_pad);
    tee.release_request_pad(&tee_app_pad);
    drop(tee_audio_pad);
    drop(tee_video_pad);
    drop(tee_app_pad);

    /* Free resources */
    let _ = pipeline.set_state(State::Null);
    drop(pipeline);

    Ok(())
}
