mod error;
mod play_flag;

use crate::play_flag::{
    GstPlayFlags, GST_PLAY_FLAG_AUDIO, GST_PLAY_FLAG_TEXT, GST_PLAY_FLAG_VIDEO,
};
use gstreamer::prelude::*;
use gstreamer::tags::{AudioCodec, Bitrate, LanguageCode};
use gstreamer::*;
use std::sync::{Arc, RwLock};

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    init().unwrap();

    /* Create the elements */
    let playbin = ElementFactory::make("playbin")
        .name("playbin")
        .build()
        .unwrap();

    /* Set the URI to play */
    playbin.set_property(
        "uri",
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.ogv",
    );

    /* Set the subtitle URI to play and some font description */
    playbin.set_property(
        "suburi",
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer_gr.srt",
    );
    playbin.set_property("subtitle-font-desc", "Sans, 18");

    /* Set flags to show Audio and Video but ignore Subtitles */
    let mut flags = unsafe {
        use gstreamer::glib::translate::{ToGlibPtr, ToGlibPtrMut};

        let mut flags = 0.to_value();
        glib::gobject_ffi::g_object_get_property(
            playbin.as_object_ref().to_glib_none().0,
            "flags\0".as_ptr() as *const _,
            flags.to_glib_none_mut().0,
        );
        flags.get::<GstPlayFlags>().unwrap()
    };
    flags |= GST_PLAY_FLAG_VIDEO | GST_PLAY_FLAG_AUDIO | GST_PLAY_FLAG_TEXT;
    let flags = flags.to_value();
    unsafe {
        use gstreamer::glib::translate::ToGlibPtr;

        glib::gobject_ffi::g_object_set_property(
            playbin.as_object_ref().to_glib_none().0,
            "flags\0".as_ptr() as *const _,
            flags.to_glib_none().0,
        );
    }

    /* Set connection speed. This will affect some internal decisions of playbin */
    playbin.set_property("connection-speed", 56_u64);

    /* Create a GLib Main Loop and set it to run */
    let main_loop = glib::MainLoop::new(None, false);

    /* Add a bus watch, so we get notified when a message arrives */
    let bus = playbin.bus().unwrap();
    let main_loop_for_msg = main_loop.clone();
    let playbin_for_msg = playbin.clone();
    let n_text = Arc::new(RwLock::new(0));
    let n_text_for_msg = Arc::clone(&n_text);
    bus.add_watch(move |_, msg| {
        handle_message(msg, &main_loop_for_msg, &playbin_for_msg, &n_text_for_msg)
    })
    .unwrap();

    /* Add a keyboard watch so we get notified of keystrokes */
    let playbin_for_keyboard = playbin.clone();
    std::thread::spawn(move || {
        handle_keyboard(&playbin_for_keyboard, n_text);
    });

    /* Start playing */
    if playbin.set_state(State::Playing).is_err() {
        eprintln!("Unable to set the pipeline to the playing state.");
        panic!();
    }

    main_loop.run();

    /* Free resources */
    let _ = playbin.set_state(State::Null);

    Ok(())
}

/* Extract some metadata from the streams and print it on the screen */
fn analyze_streams(playbin: &Element, n_text: &Arc<RwLock<i32>>) {
    let mut n_text = n_text.write().unwrap();

    /* Read some properties */
    let n_video: i32 = playbin.property("n-video"); /* Number of embedded video streams */
    let n_audio: i32 = playbin.property("n-audio"); /* Number of embedded audio streams */
    *n_text = playbin.property("n-text"); /* Number of embedded subtitle streams */

    println!(
        "{} video stream(s), {} audio stream(s), {} text stream(s)",
        n_video, n_text, n_text
    );

    println!();
    for i in 0..n_video {
        if let Some(tags) = playbin.emit_by_name::<Option<TagList>>("get-video-tags", &[&i]) {
            println!("video stream {}:", i);
            let codec = tags.get::<tags::VideoCodec>();
            let str = codec
                .map(|codec| codec.get().to_owned())
                .unwrap_or_else(|| "unknown".into());
            println!("  codec: {}", str);
        }
    }

    println!();
    for i in 0..n_audio {
        /* Retrieve the stream's audio tags */
        if let Some(tags) = playbin.emit_by_name::<Option<TagList>>("get-audio-tags", &[&i]) {
            println!("audio stream {}", i);
            if let Some(codec) = tags.get::<AudioCodec>() {
                println!("  codec: {}", codec.get());
            }
            if let Some(language) = tags.get::<LanguageCode>() {
                println!("  language: {}", language.get());
            }
            if let Some(rate) = tags.get::<Bitrate>() {
                println!("  bitrate: {}", rate.get());
            }
        }
    }

    println!();
    for i in 0..*n_text {
        /* Retrieve the stream's subtitle tags */
        println!("subtitle stream {}", i);
        if let Some(tags) = playbin.emit_by_name::<Option<TagList>>("get-text-tags", &[&i]) {
            if let Some(language) = tags.get::<LanguageCode>() {
                println!("  language: {}", language.get());
            }
        } else {
            println!("  no tags found");
        }
    }

    let current_video = playbin.property::<i32>("current-video"); /* Currently playing video stream */
    let current_audio = playbin.property::<i32>("current-audio"); /* Currently playing audio stream */
    let current_text = playbin.property::<i32>("current-text"); /* Currently playing text stream */
    println!();
    println!(
        "Currently playing video stream {}, audio stream {} and text stream {}",
        current_video, current_audio, current_text
    );
    println!("Type any number and hit ENTER to select a different audio stream");
}

/* Process messages from GStreamer */
fn handle_message(
    msg: &Message,
    main_loop: &glib::MainLoop,
    playbin: &Element,
    n_text: &Arc<RwLock<i32>>,
) -> Continue {
    match msg.view() {
        MessageView::Error(err) => {
            let src_name = err.src().unwrap().name();
            let debug_info = err.debug().unwrap_or_else(|| "none".into());
            eprintln!("Error received from element {}:{}", src_name, err.error());
            eprintln!("Debugging information: {}", debug_info);
            main_loop.quit();
        }
        MessageView::Eos(_) => {
            println!("End-Of-Stream reached.");
            main_loop.quit();
        }
        MessageView::StateChanged(msg) => {
            if msg.src().unwrap() == *playbin && msg.current() == State::Playing {
                /* Once we are in the playing state, analyze the streams */
                analyze_streams(playbin, n_text);
            }
        }
        _ => {}
    }

    /* We want to keep receiving messages */
    Continue(true)
}

/* Process keyboard input */
fn handle_keyboard(playbin: &Element, n_text: Arc<RwLock<i32>>) {
    loop {
        let mut str = String::new();
        if std::io::stdin().read_line(&mut str).is_ok() {
            match str.trim().parse::<i32>() {
                Ok(index) if 0 <= index && index < *n_text.read().unwrap() => {
                    /* If the input was a valid audio stream index, set the current audio stream */
                    println!("Setting current subtitle stream to {}", index);
                    playbin.set_property("current-text", index);
                }
                _ => {
                    eprintln!("Index out of bounds");
                }
            }
        }
    }
}
