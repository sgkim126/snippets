mod error;

use gstreamer::glib::SendValue;
use gstreamer::prelude::*;
use gstreamer::*;

/* Functions below print the Capabilities in a human-friendly format */
fn print_field(field: &str, value: &SendValue, pfx: &str) {
    let str = value.serialize().unwrap();
    println!("{}  {:>15}: {}", pfx, field, str);
}

fn print_caps(caps: Caps, pfx: &str) {
    if caps.is_any() {
        println!("{}ANY", pfx);
        return;
    }

    if caps.is_empty() {
        println!("{}EMPTY", pfx);
        return;
    }
    for structure in caps.iter() {
        println!("{}{}", pfx, structure.name());
        for (field, value) in structure.iter() {
            print_field(field, value, pfx);
        }
    }
}

/* Prints information about a Pad Template, including its Capabilities */
fn print_pad_templates_information(factory: &ElementFactory) {
    println!("Pad Templates for {}:", factory.longname());
    if factory.num_pad_templates() == 0 {
        println!("  none");
        return;
    }

    let pads = factory.static_pad_templates();
    for pad_template in pads {
        match pad_template.direction() {
            PadDirection::Src => {
                println!("  SRC template: '{}'", pad_template.name_template());
            }
            PadDirection::Sink => {
                println!("  SINK template: '{}'", pad_template.name_template());
            }
            _ => {
                println!("  UNKNOWN!!! template: '{}'", pad_template.name_template());
            }
        }

        match pad_template.presence() {
            PadPresence::Always => {
                println!("    Availability: Always");
            }
            PadPresence::Sometimes => {
                println!("    Availability: Sometimes");
            }
            PadPresence::Request => {
                println!("    Availability: On request");
            }
            _ => {
                println!("    Availability: UNKNOWN!!!");
            }
        }

        println!("    Capabilities:");
        print_caps(pad_template.caps(), "      ");
        println!();
    }
}

/* Shows the CURRENT capabilities of the requested pad in the given element */
fn print_pad_capabilities(element: &Element, pad_name: &str) {
    /* Retrieve pad */
    if let Some(pad) = element.static_pad(pad_name) {
        /* Retrieve negotiated caps (or acceptable caps if negotiation is not finished yet) */
        let caps = pad.current_caps().unwrap_or_else(|| pad.query_caps(None));

        /* Print and free */
        println!("Caps for the {} pad:", pad_name);
        print_caps(caps, "      ");
    } else {
        eprintln!("Could not retrieve pad '{}'", pad_name);
    }
}

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    init()?;

    /* Create the element factories */
    let source_factory =
        ElementFactory::find("audiotestsrc").expect("Not all element factories could be created.");
    let sink_factory =
        ElementFactory::find("autoaudiosink").expect("Not all element factories could be created.");

    /* Print information about the pad templates of these factories */
    print_pad_templates_information(&source_factory);
    print_pad_templates_information(&sink_factory);

    /* Ask the factories to instantiate actual elements */
    let source = source_factory.create_with_name(Some("source"))?;
    let sink = sink_factory.create_with_name(Some("sink"))?;

    /* Create the empty pipeline */
    let pipeline = Pipeline::new(Some("test-pipeline"));

    /* Build the pipeline */
    pipeline.add_many(&[&source, &sink])?;
    source.link(&sink).expect("Elements could not be linked.");

    /* Print initial negotiated caps (in NULL state) */
    println!("In NULL state:");
    print_pad_capabilities(&sink, "sink");

    /* Start playing */
    if pipeline.set_state(State::Playing).is_err() {
        eprintln!(
            "Unable to set the pipeline to the playing state (check the bus for error messages)."
        );
    }

    /* Wait until error, EOS or State Change */
    let bus = pipeline.bus().unwrap();
    loop {
        if let Some(msg) = bus.timed_pop_filtered(
            ClockTime::NONE,
            &[
                MessageType::Error,
                MessageType::Eos,
                MessageType::StateChanged,
            ],
        ) {
            match msg.view() {
                MessageView::Error(err) => {
                    let src_name = err.src().unwrap().name();
                    eprintln!("Error received from element {}: {}", src_name, err.error());
                    eprintln!(
                        "Debugging information: {}",
                        err.debug().unwrap_or_else(|| "none".into())
                    );
                    break;
                }
                MessageView::Eos(_) => {
                    println!("End-Of-Stream reached.");
                    break;
                }
                MessageView::StateChanged(state_changed) => {
                    /* We are only interested in state-changed messages from the pipeline */
                    if state_changed
                        .src()
                        .map(|src| src == pipeline)
                        .unwrap_or(false)
                    {
                        let old_state = state_changed.old();
                        println!(
                            "\nPipeline state changed from {:?} to {:?}:",
                            old_state,
                            state_changed.current()
                        );
                        /* Print the current capabilities of the sink element */
                        print_pad_capabilities(&sink, "sink");
                    }
                }
                _ => {
                    /* We should not reach here because we only asked for ERRORs, EOS and STATE_CHANGED */
                    eprintln!("Unexpected message received.");
                }
            }
        }
    }

    /* Free resources */
    let _ = pipeline.set_state(State::Null);

    Ok(())
}
