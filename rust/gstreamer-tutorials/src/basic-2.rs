mod error;
mod video_test_src_pattern;

use gstreamer::prelude::*;
use gstreamer::*;
use video_test_src_pattern::VideoTestSrcPattern;

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    init()?;

    /* Create the elements */
    let source = ElementFactory::make("videotestsrc")
        .name("source")
        .build()?;
    let sink = ElementFactory::make("autovideosink").name("sink").build()?;

    /* Create the empty pipeline */
    let pipeline = Pipeline::new(Some("test-pipeline"));

    /* Build the pipeline */
    pipeline.add_many(&[&source, &sink])?;
    source
        .link(&sink)
        .map_err(|_| "Elements could not be linked.")?;

    /* Modify the source's properties */
    source.set_property("pattern", VideoTestSrcPattern::Smpte);

    /* Start playing */
    pipeline
        .set_state(State::Playing)
        .map_err(|_| "Unable to set the pipeline to the playing state.")?;

    /* Wait until error or EOS */
    let bus = pipeline.bus().unwrap();
    match bus
        .timed_pop_filtered(None, &[MessageType::Error, MessageType::Eos])
        .as_ref()
        .map(|msg| msg.view())
    {
        Some(MessageView::Error(err)) => {
            eprintln!(
                "Error received from element {}: {}",
                err.src().unwrap().name(),
                err.error()
            );
            eprintln!(
                "Debugging information: {}",
                err.debug().unwrap_or_else(|| "none".into())
            );
        }
        Some(MessageView::Eos(_)) => {
            println!("End-Of-Stream reached.");
        }
        _ => {}
    }

    /* Free resources */
    pipeline.set_state(State::Null)?;

    Ok(())
}
