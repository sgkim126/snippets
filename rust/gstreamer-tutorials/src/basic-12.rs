mod error;

use gstreamer::prelude::*;
use gstreamer::*;
use std::io::Write;

fn cb_message(msg: &Message, pipeline: &Element, main_loop: &glib::MainLoop, is_live: &bool) {
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
            if *is_live {
                return;
            }

            let percent = msg.percent();
            print!("Buffering ({:>3}%)\r", percent);
            let _ = std::io::stdout().flush();
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

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    gstreamer::init().unwrap();

    /* Build the pipeline */
    let pipeline = parse_launch("playbin uri=https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm").unwrap();
    let bus = pipeline.bus().unwrap();

    /* Start playing */
    let is_live = match pipeline.set_state(State::Playing) {
        Err(_) => {
            panic!("Unable to set the pipeline to the playing state.");
        }
        Ok(StateChangeSuccess::NoPreroll) => true,
        _ => false,
    };

    let main_loop = glib::MainLoop::new(None, false);

    bus.add_signal_watch();
    let main_loop_for_message_cb = main_loop.clone();
    let pipeline_for_message_cb = pipeline.clone();
    bus.connect_message(None, move |_, msg| {
        cb_message(
            msg,
            &pipeline_for_message_cb,
            &main_loop_for_message_cb,
            &is_live,
        );
    });

    main_loop.run();

    /* Free resources */
    let _ = pipeline.set_state(State::Null);

    Ok(())
}
