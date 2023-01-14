mod audio_visualizer_shader;
mod error;
mod wave_scope_style;

use gstreamer::prelude::*;
use gstreamer::*;

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    init()?;

    /* Create the elements */
    let audio_source = ElementFactory::make("audiotestsrc")
        .name("audio_source")
        .build()?;
    let tee = ElementFactory::make("tee").name("tee").build()?;
    let audio_queue = ElementFactory::make("queue").name("audio_queue").build()?;
    let audio_convert = ElementFactory::make("audioconvert")
        .name("audio_convert")
        .build()?;
    let audio_resample = ElementFactory::make("audioresample")
        .name("audio_resample")
        .build()?;
    let audio_sink = ElementFactory::make("autoaudiosink")
        .name("audio_sink")
        .build()?;
    let video_queue = ElementFactory::make("queue").name("video_queue").build()?;
    let visual = ElementFactory::make("wavescope").name("visual").build()?;
    let video_convert = ElementFactory::make("videoconvert").name("csp").build()?;
    let video_sink = ElementFactory::make("autovideosink")
        .name("video_sink")
        .build()?;

    /* Create the empty pipeline */
    let pipeline = Pipeline::new(Some("test-pipeline"));

    /* Configure elements */
    audio_source.set_property("freq", 215.0);
    visual.set_property_from_str("shader", "none");
    visual.set_property_from_str("style", "lines");

    /* Link all elements that can be automatically linked because they have "Always" pads */
    pipeline.add_many(&[
        &audio_source,
        &tee,
        &audio_queue,
        &audio_convert,
        &audio_resample,
        &audio_sink,
        &video_queue,
        &visual,
        &video_convert,
        &video_sink,
    ])?;

    if audio_source.link(&tee).is_err()
        || Element::link_many(&[&audio_queue, &audio_convert, &audio_resample, &audio_sink])
            .is_err()
        || Element::link_many(&[&video_queue, &visual, &video_convert, &video_sink]).is_err()
    {
        eprintln!("Elements could not be linked.");
        panic!()
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
    if tee_audio_pad.link(&queue_audio_pad).is_err()
        || tee_video_pad.link(&queue_video_pad).is_err()
    {
        eprintln!("Tee could not be linked.");
        panic!();
    }

    /* Start playing the pipeline */
    let _ = pipeline.set_state(State::Playing);

    /* Wait until error or EOS */
    let bus = pipeline.bus().unwrap();
    bus.timed_pop_filtered(ClockTime::NONE, &[MessageType::Error, MessageType::Eos]);

    /* Release the request pads from the Tee, and unref them */
    tee.release_request_pad(&tee_audio_pad);
    tee.release_request_pad(&tee_video_pad);
    drop(tee_audio_pad);
    drop(tee_video_pad);

    /* Free resources */
    let _ = pipeline.set_state(State::Null);

    Ok(())
}
