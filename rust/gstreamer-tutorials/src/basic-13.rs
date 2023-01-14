mod error;

use gstreamer::event::Step;
use gstreamer::format::Buffers;
use gstreamer::prelude::*;
use gstreamer::*;
use std::iter::Iterator;

/* Send seek event to change rate */
fn send_seek_event(pipeline: &Element, video_sink: &mut Option<Element>, rate: f64) {
    /* Obtain the current position, needed for the seek event */
    if let Some(position) = pipeline.query_position::<ClockTime>() {
        /* Create the seek event */
        let seek_event = if rate > 0.0 {
            event::Seek::new(
                rate,
                SeekFlags::FLUSH | SeekFlags::ACCURATE,
                SeekType::Set,
                position,
                SeekType::End,
                ClockTime::ZERO,
            )
        } else {
            event::Seek::new(
                rate,
                SeekFlags::FLUSH | SeekFlags::ACCURATE,
                SeekType::Set,
                ClockTime::ZERO,
                SeekType::Set,
                position,
            )
        };

        if video_sink.is_none() {
            /* If we have not done so, obtain the sink through which we will send the seek events */
            *video_sink = pipeline.property("video-sink");
        }

        /* Send the event */
        video_sink.as_ref().unwrap().send_event(seek_event);

        println!("Current rate: {}", rate);
    } else {
        eprintln!("Unable to retrieve current position.");
    }
}

/* Process keyboard input */
fn handle_keyboard(
    main_loop: &glib::MainLoop,
    playing: &mut bool,
    rate: &mut f64,
    pipeline: &Element,
    video_sink: &mut Option<Element>,
) {
    loop {
        let mut str = String::new();
        match std::io::stdin().read_line(&mut str) {
            Ok(size) if 0 < size => {
                let c = str.chars().next().unwrap();
                match c.to_lowercase().to_string().as_str() {
                    "p" => {
                        *playing = !*playing;
                        let state = if *playing {
                            let _ = pipeline.set_state(State::Playing);
                            "PLAYING"
                        } else {
                            let _ = pipeline.set_state(State::Paused);
                            "PAUSED"
                        };
                        println!("Setting state to {}", state);
                    }
                    "s" => {
                        if c.is_uppercase() {
                            *rate *= 2.0;
                        } else {
                            *rate /= 2.0;
                        }
                        send_seek_event(pipeline, video_sink, *rate);
                    }
                    "d" => {
                        *rate *= -1.0;
                        send_seek_event(pipeline, video_sink, *rate);
                    }
                    "n" => {
                        if video_sink.is_none() {
                            /* If we have not done so, obtain the sink through which we will send the step events */
                            *video_sink = pipeline.property("video-sink");
                        }
                        video_sink.as_ref().unwrap().send_event(Step::new(
                            Buffers::ONE,
                            rate.abs(),
                            true,
                            false,
                        ));
                        println!("Stepping one frame");
                    }
                    "q" => {
                        main_loop.quit();
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    gstreamer::init().unwrap();

    /* Print usage map */
    println!("USAGE: Choose one of the following options, then press enter:");
    println!(" 'P' to toggle between PAUSE and PLAY");
    println!(" 'S' to increase playback speed, 's' to decrease playback speed");
    println!(" 'D' to toggle playback direction");
    println!(" 'N' to move to next frame (in the current direction, better in PAUSE)");
    println!(" 'Q' to quit");

    /* Build the pipeline */
    let pipeline = parse_launch("playbin uri=https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm").unwrap();

    /* Start playing */
    if pipeline.set_state(State::Playing).is_err() {
        eprintln!("Unable to set the pipeline to the playing state.");
        panic!();
    }

    let main_loop = glib::MainLoop::new(None, false);

    /* Add a keyboard watch so we get notified of keystrokes */
    let main_loop_for_keyboard = main_loop.clone();
    let pipeline_for_keyboard = pipeline.clone();
    std::thread::spawn(move || {
        let mut playing = true; /* Playing or Paused */
        let mut rate = 1.0; /* Current playback rate (can be negative) */
        let mut video_sink = None;
        handle_keyboard(
            &main_loop_for_keyboard,
            &mut playing,
            &mut rate,
            &pipeline_for_keyboard,
            &mut video_sink,
        );
    });

    main_loop.run();

    /* Free resources */
    let _ = pipeline.set_state(State::Null);

    Ok(())
}
