mod error;

use gstreamer::prelude::*;
use gstreamer::*;

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    gstreamer::init()?;

    /* Build the pipeline */
    let pipeline = parse_launch("playbin uri=https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm")?;

    /* Create the elements inside the sink bin */
    let equalizer = ElementFactory::make("equalizer-3bands")
        .name("equalizer")
        .build()?;
    let convert = ElementFactory::make("audioconvert")
        .name("convert")
        .build()?;
    let sink = ElementFactory::make("autoaudiosink")
        .name("audio_sink")
        .build()?;

    /* Create the sink bin, add the elements and link them */
    let bin = Bin::new(Some("audio_sink_bin"));
    bin.add_many(&[&equalizer, &convert, &sink])?;
    Element::link_many(&[&equalizer, &convert, &sink])?;
    let pad = equalizer.static_pad("sink").unwrap();
    let ghost_pad = GhostPad::with_target(Some("sink"), &pad)?;
    ghost_pad.set_active(true)?;
    bin.add_pad(&ghost_pad)?;
    drop(pad);

    /* Configure the equalizer */
    equalizer.set_property("band1", -24.0);
    equalizer.set_property("band2", -24.0);

    /* Set playbin's audio sink to be our sink bin */
    pipeline.set_property("audio-sink", bin);

    /* Start playing */
    pipeline.set_state(State::Playing)?;

    /* Wait until error or EOS */
    let bus = pipeline.bus().unwrap();
    let _msg = bus.timed_pop_filtered(ClockTime::NONE, &[MessageType::Error, MessageType::Eos]);

    /* Free resources */
    let _ = pipeline.set_state(State::Null);

    Ok(())
}
