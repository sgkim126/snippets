mod error;

use gstreamer::prelude::*;
use gstreamer::*;

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    init()?;

    /* Create the elements */
    let source = ElementFactory::make("uridecodebin")
        .name("source")
        .build()?;
    let convert = ElementFactory::make("audioconvert")
        .name("convert")
        .build()?;
    let resample = ElementFactory::make("audioresample")
        .name("resample")
        .build()?;
    let sink = ElementFactory::make("autoaudiosink").name("sink").build()?;

    /* Create the empty pipeline */
    let pipeline = Pipeline::new(Some("test-pipeline"));

    /* Build the pipeline. Note that we are NOT linking the source at this
     * point. We will do it later. */
    pipeline.add_many(&[&source, &convert, &resample, &sink])?;
    Element::link_many(&[&convert, &resample, &sink])
        .map_err(|_| "Elements could not be linked.")?;

    /* Set the URI to play */
    source.set_property(
        "uri",
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm",
    );

    /* Connect to the pad-added signal */
    source.connect_pad_added(pad_added_handler(convert));

    /* Start playing */
    pipeline
        .set_state(State::Playing)
        .map_err(|_| "Unable to set the pipeline to the playing state.")?;

    /* Listen to the bus */
    let bus = pipeline.bus().unwrap();
    loop {
        match bus
            .timed_pop_filtered(
                None,
                &[
                    MessageType::StateChanged,
                    MessageType::Error,
                    MessageType::Eos,
                ],
            )
            .as_ref()
            .map(|msg| msg.view())
        {
            /* Parse message */
            Some(MessageView::Error(err)) => {
                println!(
                    "Error received from element {}: {:?}",
                    err.src().unwrap().name(),
                    err.error()
                );
                println!(
                    "Debugging information: {}",
                    err.debug().unwrap_or_else(|| "none".to_string())
                );
                break;
            }
            Some(MessageView::Eos(_)) => {
                println!("End-Of-Stream reached.");
                break;
            }
            Some(MessageView::StateChanged(message)) => {
                /* We are only interested in state-changed messages from the pipeline */
                if message
                    .src()
                    .map(|object| object.as_object_ref() == pipeline.as_object_ref())
                    .unwrap_or(false)
                {
                    let old_state = message.old();
                    let new_state = message.current();
                    let _pending_state = message.pending();
                    println!(
                        "Pipeline state changed from {:?} to {:?}:",
                        old_state, new_state
                    );
                }
            }
            _ => {
                /* We should not reach here */
                panic!("Unexpected message received.");
            }
        }
    }

    /* Free resources */
    pipeline.set_state(State::Null)?;

    Ok(())
}

/* This function will be called by the pad-added signal */
fn pad_added_handler(convert: Element) -> impl Fn(&Element, &Pad) + Send + Sync + 'static {
    move |src: &Element, new_pad: &Pad| {
        if let Some(sink_pad) = convert.static_pad("sink") {
            println!(
                "Received new pad '{}' from '{}':\n",
                new_pad.name(),
                src.name()
            );

            /* If our converter is already linked, we have nothing to do here */
            if sink_pad.is_linked() {
                println!("We are already linked. Ignoring.");
                return;
            }

            /* Check the new pad's type */
            let new_pad_caps = new_pad.current_caps().unwrap();
            let new_pad_struct = new_pad_caps.structure(0).unwrap();
            let new_pad_type = new_pad_struct.name();
            if !new_pad_type.starts_with("audio/x-raw") {
                println!(
                    "It has type '{}' which is not raw audio. Ignoring.",
                    new_pad_type
                );
                return;
            }

            /* Attempt the link */
            if new_pad.link(&sink_pad).is_err() {
                println!("Type is '{}' but link failed.", new_pad_type);
            } else {
                println!("Link succeeded (type '{}').", new_pad_type);
            }
        }
    }
}
