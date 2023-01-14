mod error;

use gstreamer::prelude::*;
use gstreamer::*;
use gstreamer_video::prelude::*;
use gstreamer_video::ColorBalance;
use std::cmp::{max, min};
use std::iter::Iterator;

/* Process a color balance command */
fn update_color_channel(channel_name: &str, increase: bool, cb: &ColorBalance) {
    /* Retrieve the list of channels and locate the requested one */
    let channels = cb.list_channels();
    if let Some(channel) = channels.iter().find(|tmp| tmp.label() == channel_name) {
        /* Change the channel's value */
        let step = 0.1 * (channel.max_value() - channel.min_value()) as f64;
        let mut value = cb.value(channel);
        if increase {
            value = min(value + step as i32, channel.max_value());
        } else {
            value = max(value - step as i32, channel.min_value());
        }
        cb.set_value(channel, value);
    }
}

/* Output the current values of all Color Balance channels */
fn print_current_values(pipeline: &Element) {
    /* Output Color Balance values */
    let color_balance = pipeline.dynamic_cast_ref::<ColorBalance>().unwrap();
    let channels = color_balance.list_channels();
    for channel in channels {
        let value = color_balance.value(&channel);
        print!(
            "{}: {:>3}% ",
            channel.label(),
            100 * (value - channel.min_value()) / (channel.max_value() - channel.min_value())
        );
    }
    println!();
}

/* Process keyboard input */
fn handle_keyboard(pipeline: &Element, main_loop: &glib::MainLoop) {
    loop {
        let mut str = String::new();
        match std::io::stdin().read_line(&mut str) {
            Ok(size) if 0 < size => {
                let c = str.chars().next().unwrap();
                let increase = c.is_uppercase();
                match c.to_lowercase().next() {
                    Some('c') => {
                        let cb = pipeline.dynamic_cast_ref::<ColorBalance>().unwrap();
                        update_color_channel("CONTRAST", increase, cb);
                    }
                    Some('b') => {
                        let cb = pipeline.dynamic_cast_ref::<ColorBalance>().unwrap();
                        update_color_channel("BRIGHTNESS", increase, cb);
                    }
                    Some('h') => {
                        let cb = pipeline.dynamic_cast_ref::<ColorBalance>().unwrap();
                        update_color_channel("HUE", increase, cb);
                    }
                    Some('s') => {
                        let cb = pipeline.dynamic_cast_ref::<ColorBalance>().unwrap();
                        update_color_channel("SATURATION", increase, cb);
                    }
                    Some('q') => {
                        main_loop.quit();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        print_current_values(pipeline);
    }
}

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    gstreamer::init()?;

    /* Print usage map */
    println!("USAGE: Choose one of the following options, then press enter:");
    println!(" 'C' to increase contrast, 'c' to decrease contrast");
    println!(" 'B' to increase brightness, 'b' to decrease brightness");
    println!(" 'H' to increase hue, 'h' to decrease hue");
    println!(" 'S' to increase saturation, 's' to decrease saturation");
    println!(" 'Q' to quit");

    /* Build the pipeline */
    let pipeline = parse_launch("playbin uri=https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm")?;

    /* Create a GLib Main Loop and set it to run */
    let main_loop = glib::MainLoop::new(None, false);

    /* Add a keyboard watch so we get notified of keystrokes */
    let pipeline_for_keyboard = pipeline.clone();
    let main_loop_for_keyboard = main_loop.clone();
    std::thread::spawn(move || handle_keyboard(&pipeline_for_keyboard, &main_loop_for_keyboard));

    /* Start playing */
    if pipeline.set_state(State::Playing).is_err() {
        panic!("Unable to set the pipeline to the playing state.");
    }
    print_current_values(&pipeline);

    /* Create a GLib Main Loop and set it to run */
    main_loop.run();

    /* Free resources */
    let _ = pipeline.set_state(State::Null);

    Ok(())
}
