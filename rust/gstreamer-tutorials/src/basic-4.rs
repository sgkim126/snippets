mod error;

use gstreamer::glib::{g_print, g_printerr};
use gstreamer::prelude::*;
use gstreamer::query::Seeking;
use gstreamer::*;

#[derive(Default)]
struct CustomData {
    playing: bool,               /* Are we in the PLAYING state? */
    terminate: bool,             /* Should we terminate execution? */
    seek_enabled: bool,          /* Is seeking enabled for this media? */
    seek_done: bool,             /* Have we performed the seek already? */
    duration: Option<ClockTime>, /* How long does this media last, in nanoseconds */
}

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    init()?;

    let mut data = CustomData::default();

    /* Create the elements */
    let playbin = ElementFactory::make("playbin").name("playbin").build()?;

    /* Set the URI to play */
    playbin.set_property(
        "uri",
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm",
    );

    /* Start playing */
    playbin
        .set_state(State::Playing)
        .map_err(|_| "Unable to set the pipeline to the playing state.")?;

    /* Listen to the bus */
    let bus = playbin.bus().unwrap();
    while !data.terminate {
        if let Some(msg) = bus.timed_pop_filtered(
            Some(ClockTime::from_mseconds(100)),
            &[
                MessageType::StateChanged,
                MessageType::Error,
                MessageType::Eos,
                MessageType::DurationChanged,
            ],
        ) {
            handle_message(&mut data, &playbin, msg);
        } else {
            /* We got no message, this means the timeout expired */
            if data.playing {
                if let Some(current) = playbin.query_position::<ClockTime>() {
                    /* If we didn't know it yet, query the stream duration */
                    if data.duration.is_none() {
                        data.duration = playbin.query_duration();
                        g_printerr!("Could not query current duration.\n");
                    }

                    /* Print current position and total duration */
                    g_print!("Position {} / {}\r", current, data.duration.unwrap());

                    /* If seeking is enabled, we have not done it yet, and the time is right, seek */
                    if data.seek_enabled && !data.seek_done && current > ClockTime::from_seconds(10)
                    {
                        g_print!("\nReached 10s, performing seek...\n");
                        playbin.seek_simple(
                            SeekFlags::FLUSH | SeekFlags::KEY_UNIT,
                            ClockTime::from_seconds(30),
                        )?;
                        data.seek_done = true;
                    }
                } else {
                    g_printerr!("Could not query current position.\n");
                }
            }
        }
    }

    /* Free resources */
    playbin.set_state(State::Null)?;

    Ok(())
}

fn handle_message(data: &mut CustomData, playbin: &Element, msg: Message) {
    match msg.view() {
        MessageView::Error(err) => {
            let src_name = err.src().map(|src| src.name()).unwrap_or_else(|| "".into());
            g_printerr!(
                "Error received from element {}: {}\n",
                src_name,
                err.error()
            );
            g_printerr!(
                "Debugging information: {}\n",
                err.debug().unwrap_or_else(|| "".into())
            );
            data.terminate = true;
        }
        MessageView::Eos(_) => {
            g_print!("\nnEnd-Of-Stream reached.\n");
            data.terminate = true;
        }
        MessageView::DurationChanged(_) => {
            /* The duration has changed, mark the current one as invalid */
            data.duration = ClockTime::NONE;
        }
        MessageView::StateChanged(msg) => {
            if msg
                .src()
                .map(|src| src.as_object_ref() == playbin.as_object_ref())
                .unwrap_or(false)
            {
                let new_state = msg.current();
                g_print!(
                    "Pipeline state changed from {:?} to {:?}:\n",
                    msg.old(),
                    new_state
                );

                /* Remember whether we are in the PLAYING state or not */
                data.playing = new_state == State::Playing;
                if data.playing {
                    /* We just moved to PLAYING. Check if seeking is possible */
                    let mut query = Seeking::new(Format::Time);
                    if playbin.query(query.query_mut()) {
                        let (seek_enabled, start, end) = query.result();
                        data.seek_enabled = seek_enabled;
                        if data.seek_enabled {
                            g_print!("Seeking is ENABLED from {} to {}\n", start, end);
                        } else {
                            g_print!("Seeking is DISABLED for this stream.\n");
                        }
                    } else {
                        g_printerr!("Seeking query failed.\n");
                    }
                }
            }
        }
        _ => {
            g_printerr!("Unexpected message received.\n");
            panic!();
        }
    }
}
