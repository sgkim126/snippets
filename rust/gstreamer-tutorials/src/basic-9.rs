mod error;

use gstreamer::glib::SendValue;
use gstreamer::prelude::*;
use gstreamer::*;
use gstreamer_pbutils::prelude::*;
use gstreamer_pbutils::{
    pb_utils_get_codec_description, Discoverer, DiscovererContainerInfo, DiscovererInfo,
    DiscovererResult, DiscovererStreamInfo,
};
use std::env::args;
use std::iter::Iterator;

/* Print a tag in a human-readable format (name: value) */
fn print_tag_foreach(val: &SendValue, tag: &str, depth: usize) {
    let str: String = if let Ok(str) = val.get::<&str>() {
        str.into()
    } else {
        val.serialize().unwrap().into()
    };
    let indent = 2 * depth;
    println!("{:>indent$}{}: {}", ' ', tag_get_nick(tag), str);
}

/* Print information regarding a stream */
fn print_stream_info(info: &DiscovererStreamInfo, depth: usize) {
    let desc = if let Some(caps) = info.caps() {
        if caps.is_fixed() {
            pb_utils_get_codec_description(&caps).into()
        } else {
            caps.to_string()
        }
    } else {
        "".into()
    };
    let indent = 2 * depth;
    println!("{:<indent$}{}: {}", ' ', info.stream_type_nick(), desc);
    drop(desc);

    if let Some(tags) = info.tags() {
        let indent = 2 * (depth + 1);
        println!("{:<indent$}Tags:", ' ');
        for (tag, value) in tags.iter() {
            print_tag_foreach(&value, tag, depth + 2);
        }
    }
}

/* Print information regarding a stream and its substreams, if any */
fn print_topology(info: &DiscovererStreamInfo, depth: usize) {
    print_stream_info(info, depth);

    let next = info.next();
    if let Some(next) = next {
        print_topology(&next, depth + 1);
    } else if let Some(info) = info.downcast_ref::<DiscovererContainerInfo>() {
        let streams = info.streams();
        for tmpinf in streams {
            print_topology(&tmpinf, depth + 1);
        }
    }
}

/* This function is called every time the discoverer has information regarding
 * one of the URIs we provided.*/
fn on_discovered_cb(info: &DiscovererInfo, err: Option<&glib::Error>) {
    let uri = info.uri();
    let result = info.result();
    match result {
        DiscovererResult::UriInvalid => {
            println!("Invalid URI '{}'", uri);
        }
        DiscovererResult::Error => {
            println!("Discoverer error: {}", err.unwrap().message());
        }
        DiscovererResult::Timeout => {
            println!("Timeout");
        }
        DiscovererResult::Busy => {
            println!("Busy");
        }
        DiscovererResult::MissingPlugins => {
            println!("Missing plugins: {}", info.misc().unwrap());
        }
        DiscovererResult::Ok => {
            println!("Discovered '{}'", uri);
        }
        _ => panic!(),
    }

    if result != DiscovererResult::Ok {
        eprintln!("This URI cannot be played");
        return;
    }

    /* If we got no error, show the retrieved information */
    println!("\nDuration: {}", info.duration().unwrap());

    if let Some(tags) = info.tags() {
        println!("Tags:");
        for (tag, value) in tags.iter() {
            print_tag_foreach(&value, tag, 1);
        }
    }

    if info.is_seekable() {
        println!("Seekable: yes");
    } else {
        println!("Seekable: no");
    }

    println!("\n");

    let sinfo = info.stream_info();
    if sinfo.is_none() {
        return;
    }
    let sinfo = sinfo.unwrap();
    println!("Stream information:");
    print_topology(&sinfo, 1);
    println!();
}

/* This function is called when the discoverer has finished examining
 * all the URIs we provided.*/
fn on_finished_cb(main_loop: &glib::MainLoop) {
    println!("Finished discovering");
    main_loop.quit();
}

fn main() -> Result<(), error::Error> {
    let uri = if let Some(arg) = args().collect::<Vec<_>>().get(1) {
        /* if a URI was provided, use it instead of the default one */
        arg.to_string()
    } else {
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm"
            .into()
    };

    /* Initialize GStreamer */
    gstreamer::init().unwrap();

    println!("Discovering '{}'", uri);

    /* Instantiate the Discoverer */
    let discoverer = Discoverer::new(ClockTime::from_seconds(5))
        .map_err(|err| format!("Error creating discoverer instance: {}\n", err))
        .unwrap();

    /* Connect to the interesting signals */
    discoverer.connect_discovered(|_, info, err| {
        on_discovered_cb(info, err);
    });

    /* Create a GLib Main Loop and set it to run, so we can wait for the signals */
    let main_loop = glib::MainLoop::new(None, false);
    let main_loop_for_finished = main_loop.clone();

    discoverer.connect_finished(move |_| {
        on_finished_cb(&main_loop_for_finished);
    });

    /* Start the discoverer process (nothing to do yet) */
    discoverer.start();

    /* Add a request to process asynchronously the URI passed through the command line */
    if let Err(_) = discoverer.discover_uri_async(&uri) {
        println!("Failed to start discovering URI '{}'", uri);
        panic!();
    }
    main_loop.run();

    /* Stop the discoverer process */
    discoverer.stop();

    /* Free resources */

    Ok(())
}
