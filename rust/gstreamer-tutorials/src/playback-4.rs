mod error;
mod play_flag;

use gstreamer::prelude::*;
use gstreamer::*;
use std::io::Write;
use std::iter::Iterator;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

const GRAPH_LENGTH: usize = 78;

fn got_location(prop_object: &Object) {
    let location: String = prop_object.property("temp-location");
    println!("Temporary file: {}", location);
    /* Uncomment this line to keep the temporary file after the program exits */
    // prop_object.set_property("temp-remove", false);
}

fn cb_message(
    pipeline: &Element,
    main_loop: &glib::MainLoop,
    msg: &Message,
    is_live: bool,
    buffering_level: &AtomicI32,
) {
    match msg.view() {
        MessageView::Error(err) => {
            println!("Error: {}", err.error());
            let _ = pipeline.set_state(State::Ready);
            main_loop.quit();
        }
        MessageView::Eos(_) => {
            /* end-of-stream */
            let _ = pipeline.set_state(State::Ready);
            main_loop.quit();
        }
        MessageView::Buffering(msg) => {
            /* If the stream is live, we do not care about buffering. */
            if is_live {
                return;
            }
            let percent = msg.percent();
            buffering_level.store(percent, Ordering::SeqCst);

            /* Wait until buffering is complete before start/resume playing */
            if percent < 100 {
                let _ = pipeline.set_state(State::Paused);
            } else {
                let _ = pipeline.set_state(State::Playing);
            }
        }
        MessageView::ClockLost(_) => {
            /* Get a new clock */
            let _ = pipeline.set_state(State::Paused);
            let _ = pipeline.set_state(State::Playing);
        }
        _ => { /* Unhandled message */ }
    }
}

fn refresh_ui(pipeline: &Element, buffering_level: i32) {
    let mut query = query::Buffering::new(Format::Percent);
    let result = pipeline.query(&mut query);
    if result {
        let mut graph = [' '; GRAPH_LENGTH];

        for (start, stop) in query.ranges() {
            let start = start.value() as usize;
            let stop = stop.value() as usize;
            let denominator = stop - start;
            let start = start * GRAPH_LENGTH / denominator;
            let stop = stop * GRAPH_LENGTH / denominator;
            for char in graph.iter_mut().take(stop).skip(start) {
                *char = '-';
            }
        }

        if let Some(position) = pipeline.query_position::<ClockTime>() {
            if let Some(duration) = pipeline.query_duration::<ClockTime>() {
                let i = (GRAPH_LENGTH as f64 * position.nseconds() as f64
                    / (duration.nseconds() + 1) as f64) as usize;
                graph[i] = if buffering_level < 100 { 'X' } else { '>' };
            }
        }
        print!("[{}]", graph.iter().collect::<String>());
        if buffering_level < 100 {
            print!(" Buffering: {:>3}%", buffering_level);
        } else {
            print!("                ");
        }
        print!("\r");
        let _ = std::io::stdout().flush();
    }
}

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    gstreamer::init().unwrap();

    /* Initialize our data structure */
    let buffering_level = Arc::new(AtomicI32::new(100));

    /* Build the pipeline */
    let pipeline = parse_launch("playbin uri=https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm").unwrap();
    let bus = pipeline.bus().unwrap();

    /* Start playing */
    let is_live = match pipeline.set_state(State::Playing) {
        Err(_) => {
            eprintln!("Unable to set the pipeline to the playing state.");
            panic!();
        }
        Ok(StateChangeSuccess::NoPreroll) => true,
        _ => false,
    };

    let main_loop = glib::MainLoop::new(None, false);

    bus.add_signal_watch();
    let pipeline_for_msg = pipeline.clone();
    let main_loop_for_msg = main_loop.clone();
    let buffering_level_for_msg = Arc::clone(&buffering_level);

    bus.connect_message(None, move |_, msg| {
        cb_message(
            &pipeline_for_msg,
            &main_loop_for_msg,
            msg,
            is_live,
            &buffering_level_for_msg,
        );
    });
    pipeline.connect("deep-notify::temp-location", false, |args| {
        let prop_object = args[1].get::<Object>().unwrap();
        got_location(&prop_object);
        None
    });

    /* Register a function that GLib will call every second */
    let pipeline_for_ui = pipeline.clone();
    glib::timeout_add_seconds(1, move || {
        refresh_ui(&pipeline_for_ui, buffering_level.load(Ordering::SeqCst));
        Continue(true)
    });

    main_loop.run();

    /* Free resources */
    let _ = pipeline.set_state(State::Null);
    println!();

    Ok(())
}
