mod error;

use gstreamer::prelude::*;
use gstreamer::*;
use std::ops::Deref;

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    init()?;

    /* Build the pipeline */
    let pipeline = SetPipelineStateNullOnDrop(parse_launch("playbin uri=https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm")?);

    /* Start playing */
    pipeline.set_state(State::Playing)?;

    /* Wait until error or EOS */
    let bus = pipeline.bus().unwrap();
    let msg = bus.timed_pop_filtered(None, &[MessageType::Error, MessageType::Eos]);

    /* See next tutorial for proper error message handling/parsing */
    if let Some(msg) = msg {
        if let MessageView::Error(_) = msg.view() {
            eprintln!(
                "An error occurred! Re-run with the GST_DEBUG=*:WARN environment \
                         variable set for more details."
            );
        }
    }

    /* Free resources */
    // I added a structure which set the state of the pipleline null before freeing it. Rust will
    // free the resources.

    Ok(())
}

struct SetPipelineStateNullOnDrop(Element);

impl Drop for SetPipelineStateNullOnDrop {
    fn drop(&mut self) {
        self.0.set_state(State::Null).unwrap();
    }
}

impl Deref for SetPipelineStateNullOnDrop {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
