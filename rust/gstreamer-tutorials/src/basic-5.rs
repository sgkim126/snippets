mod error;

use gstreamer::message::Application;
use gstreamer::prelude::*;
use gstreamer::tags::{AudioCodec, Bitrate, LanguageCode, VideoCodec};
use gstreamer::{
    message, ClockTime, Element, ElementFactory, SeekFlags, State, Structure, TagList,
};
use gstreamer_video::prelude::*;
use gstreamer_video::VideoOverlay;
use gtk::prelude::*;
use gtk::{glib, WindowType};
use std::sync::{Arc, RwLock};

struct CustomData {
    duration: Option<ClockTime>, /* Duration of the clip, in nanoseconds */
    state: State,                /* Current state of the pipeline */
}

/* This function is called when the GUI toolkit creates the physical window that will hold the video.
 * At this point we can retrieve its handler (which has a different meaning depending on the windowing system)
 * and pass it to GStreamer through the VideoOverlay interface. */
fn realize_cb(playbin: Element) -> impl Fn(&gtk::DrawingArea) {
    let video_overlay = playbin.dynamic_cast::<VideoOverlay>().unwrap();
    move |widget| {
        let window = widget.window().unwrap();
        if !window.ensure_native() {
            panic!("Couldn't create native window needed for GstVideoOverlay!");
        }

        /* Retrieve window handler from GDK */
        let window_handle = window_handle(&window);

        /* Pass it to playbin, which implements VideoOverlay and will forward it to the video sink */
        unsafe {
            video_overlay.set_window_handle(window_handle);
        }
    }
}

/* This function is called when the PLAY button is clicked */
fn play_cb(playbin: Element) -> impl Fn(&gtk::Button) {
    move |_| {
        let _ = playbin.set_state(State::Playing);
    }
}

/* This function is called when the PAUSE button is clicked */
fn pause_cb(playbin: Element) -> impl Fn(&gtk::Button) {
    move |_| {
        let _ = playbin.set_state(State::Paused);
    }
}

/* This function is called when the STOP button is clicked */
fn stop_cb(playbin: &Element) {
    let _ = playbin.set_state(State::Ready);
}

/* This function is called when the main window is closed */
fn delete_event_cb(playbin: &Element) -> Inhibit {
    stop_cb(playbin);

    gtk::main_quit();
    Inhibit(false)
}

/* This function is called everytime the video window needs to be redrawn (due to damage/exposure,
 * rescaling, etc). GStreamer takes care of this in the PAUSED and PLAYING states, otherwise,
 * we simply draw a black rectangle to avoid garbage showing up. */
fn draw_cb(state: State, widget: &gtk::DrawingArea, cr: &gtk::cairo::Context) -> Inhibit {
    if state < State::Paused {
        /* Cairo is a 2D graphics library which we use here to clean the video window.
         * It is used by GStreamer for other reasons, so it will always be available to us. */
        let allocation = widget.allocation();
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.rectangle(0.0, 0.0, allocation.width() as _, allocation.height() as _);
        let _ = cr.fill();
    }

    Inhibit(false)
}

/* This function is called when the slider changes its position. We perform a seek to the
 * new position here. */
fn slider_cb(playbin: Element) -> impl Fn(&gtk::Scale) {
    move |slider: &gtk::Scale| {
        let value = slider.value();
        let _ = playbin.seek_simple(
            SeekFlags::FLUSH | SeekFlags::KEY_UNIT,
            ClockTime::from_seconds(value as _),
        );
    }
}

/* This creates all the GTK+ widgets that compose our application, and registers the callbacks */
fn create_ui(
    playbin: &Element,
    data: Arc<RwLock<CustomData>>,
) -> (gtk::TextView, gtk::Scale, glib::signal::SignalHandlerId) {
    let main_window = gtk::ApplicationWindow::builder()
        .type_(WindowType::Toplevel)
        .default_width(640)
        .default_height(480)
        .build();
    {
        let playbin = playbin.clone();
        main_window.connect_delete_event(move |_, _| delete_event_cb(&playbin));
    }

    let video_window = gtk::DrawingArea::new();
    video_window.connect_realize(realize_cb(playbin.clone()));
    video_window.connect_draw(move |widget, cr| {
        let data = data.read().unwrap();
        let state = data.state;
        draw_cb(state, widget, cr)
    });

    let play_button =
        gtk::Button::from_icon_name(Some("media-playback-start"), gtk::IconSize::SmallToolbar);
    play_button.connect_clicked(play_cb(playbin.clone()));

    let pause_button =
        gtk::Button::from_icon_name(Some("media-playback-pause"), gtk::IconSize::SmallToolbar);
    pause_button.connect_clicked(pause_cb(playbin.clone()));

    let stop_button =
        gtk::Button::from_icon_name(Some("media-playback-stop"), gtk::IconSize::SmallToolbar);
    {
        let playbin = playbin.clone();
        stop_button.connect_clicked(move |_| stop_cb(&playbin));
    }

    let slider = gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 100.0, 1.0);
    slider.set_draw_value(false);
    let slider_update_signal_id = slider.connect_value_changed(slider_cb(playbin.clone()));

    let streams_list = gtk::TextView::builder().editable(false).build();

    let controls = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(0)
        .build();
    controls.pack_start(&play_button, false, false, 2);
    controls.pack_start(&pause_button, false, false, 2);
    controls.pack_start(&stop_button, false, false, 2);
    controls.pack_start(&slider, true, true, 2);

    let main_hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(0)
        .build();
    main_hbox.pack_start(&video_window, true, true, 0);
    main_hbox.pack_start(&streams_list, false, false, 2);

    let main_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(0)
        .build();
    main_box.pack_start(&main_hbox, true, true, 0);
    main_box.pack_start(&controls, false, false, 0);
    main_window.add(&main_box);

    main_window.show_all();

    (streams_list, slider, slider_update_signal_id)
}

/* This function is called periodically to refresh the GUI */
fn refresh_ui(
    playbin: &Element,
    slider: &gtk::Scale,
    slider_update_signal_id: &glib::signal::SignalHandlerId,
    state: &State,
    duration: &mut Option<ClockTime>,
) {
    /* We do not want to update anything unless we are in the PAUSED or PLAYING states */
    if state < &State::Paused {
        return;
    }

    /* If we didn't know it yet, query the stream duration */
    if duration.is_none() {
        *duration = playbin.query_duration();
        if let Some(duration) = duration {
            slider.set_range(0.0, duration.seconds() as _);
        } else {
            eprintln!("Could not query current duration.");
        }
    }

    if let Some(current) = playbin.query_position::<ClockTime>() {
        /* Block the "value-changed" signal, so the slider_cb function is not called
         * (which would trigger a seek the user has not requested) */
        slider.block_signal(slider_update_signal_id);
        /* Set the position of the slider to the current pipeline position, in SECONDS */
        slider.set_value(current.seconds() as _);
        /* Re-enable the signal */
        slider.unblock_signal(slider_update_signal_id);
    }
}

/* This function is called when new metadata is discovered in the stream */
fn tags_cb(playbin: &Element) {
    /* We are possibly in a GStreamer working thread, so we notify the main
     * thread of this event through a message in the bus */
    let structure = Structure::new_empty("tags-changed");
    let message = message::Application::new(structure);
    let _ = playbin.post_message(message);
}

/* This function is called when an error message is posted on the bus */
fn error_cb(playbin: &Element, error: &message::Error) {
    /* Print error details on the screen */
    let src_name = error
        .src()
        .map(|src| src.name())
        .unwrap_or_else(|| "".into());
    let msg = error.message();
    let debug_info = error.debug().unwrap_or_else(|| "".into());
    eprintln!("Error received from element {}: {:?}", src_name, msg);
    eprintln!("Debugging information: {}", debug_info);

    /* Set the pipeline to READY (which stops playback) */
    let _ = playbin.set_state(State::Ready);
}

/* This function is called when an End-Of-Stream message is posted on the bus.
 * We just set the pipeline to READY (which stops playback) */
fn eos_cb(playbin: &Element) {
    println!("End-Of-Stream reached.\n");
    let _ = playbin.set_state(State::Ready);
}

/* This function is called when the pipeline changes states. We use it to
 * keep track of the current state. */
fn state_changed_cb(
    playbin: &Element,
    slider: &gtk::Scale,
    slider_update_signal_id: &glib::signal::SignalHandlerId,
    state_changed: &message::StateChanged,
    state: &mut State,
    duration: &mut Option<ClockTime>,
) {
    if state_changed.src().unwrap() == *playbin {
        *state = state_changed.current();
        println!("State set to {:?}", *state);
        if state_changed.old() == State::Ready && *state == State::Paused {
            /* For extra responsiveness, we refresh the GUI as soon as we reach the PAUSED state */
            refresh_ui(playbin, slider, slider_update_signal_id, state, duration);
        }
    }
}

/* Extract metadata from all the streams and write it to the text widget in the GUI */
fn analyze_streams(playbin: &Element, streams_list: &gtk::TextView) {
    let text = streams_list.buffer().unwrap();

    /* Clean current contents of the widget */
    text.set_text("");

    /* Read some properties */
    let n_video: i32 = playbin.property("n-video");
    let n_audio: i32 = playbin.property("n-audio");
    let n_text: i32 = playbin.property("n-text");

    for i in 0..n_video {
        if let Some(tags) = playbin.emit_by_name::<Option<TagList>>("get-video-tags", &[&i]) {
            text.insert_at_cursor(&format!("video stream {}\n", i));
            text.insert_at_cursor(&format!(
                "  codec: {}\n",
                tags.get::<VideoCodec>()
                    .map(|tag| tag.get().to_owned())
                    .unwrap_or_else(|| "unknown".into())
            ));
        }
    }

    for i in 0..n_audio {
        /* Retrieve the stream's audio tags */
        if let Some(tags) = playbin.emit_by_name::<Option<TagList>>("get-audio-tags", &[&i]) {
            text.insert_at_cursor(&format!("\naudio stream {}\n", i));
            if let Some(codec) = tags.get::<AudioCodec>() {
                text.insert_at_cursor(&format!("  codec: {}\n", codec.get()));
            }
            if let Some(language) = tags.get::<LanguageCode>() {
                text.insert_at_cursor(&format!("  language: {}\n", language.get()));
            }
            if let Some(rate) = tags.get::<Bitrate>() {
                text.insert_at_cursor(&format!("  bitrate: {}\n", rate.get()));
            }
        }
    }

    for i in 0..n_text {
        /* Retrieve the stream's subtitle tags */
        if let Some(tags) = playbin.emit_by_name::<Option<TagList>>("get-text-tags", &[&i]) {
            text.insert_at_cursor(&format!("\nsubtitle stream {}\n", i));
            if let Some(language) = tags.get::<LanguageCode>() {
                text.insert_at_cursor(&format!("  language: {}\n", language.get()));
            }
        }
    }
}

/* This function is called when an "application" message is posted on the bus.
 * Here we retrieve the message posted by the tags_cb callback */
fn application_cb(msg: &Application, playbin: &Element, streams_list: &gtk::TextView) {
    if msg
        .structure()
        .map(|structure| structure.name() == "tags-changed")
        .unwrap_or(false)
    {
        /* If the message is the "tags-changed" (only one we are currently issuing), update
         * the stream info GUI */
        analyze_streams(playbin, streams_list);
    }
}

fn main() -> Result<(), error::Error> {
    /* Initialize GTK */
    gtk::init()?;

    /* Initialize GStreamer */
    gstreamer::init()?;

    /* Initialize our data structure */
    let data = Arc::new(RwLock::new(CustomData {
        duration: ClockTime::NONE,
        state: State::VoidPending,
    }));

    /* Create the elements */
    /* Our one and only pipeline */
    let playbin = ElementFactory::make("playbin").name("playbin").build()?;

    /* Set the URI to play */
    playbin.set_property(
        "uri",
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm",
    );
    {
        let playbin_for_cb = playbin.clone();
        playbin.connect("video-tags-changed", false, move |_| {
            tags_cb(&playbin_for_cb);
            None
        });
    }
    {
        let playbin_for_cb = playbin.clone();
        playbin.connect("audio-tags-changed", false, move |_| {
            tags_cb(&playbin_for_cb);
            None
        });
    }
    {
        let playbin_for_cb = playbin.clone();
        playbin.connect("text-tags-changed", false, move |_| {
            tags_cb(&playbin_for_cb);
            None
        });
    }

    /* Create the GUI */
    let (
        streams_list,            /* Text widget to display info about the streams */
        slider,                  /* Slider widget to keep track of current position */
        slider_update_signal_id, /* Signal ID for the slider update signal */
    ) = create_ui(&playbin, Arc::clone(&data));

    /* Instruct the bus to emit signals for each received message, and connect to the interesting signals */
    {
        let bus = playbin.bus().unwrap();
        bus.add_signal_watch();
        let playbin = playbin.clone();
        let streams_list =
            glib::object::SendWeakRef::<gtk::TextView>::from(streams_list.downgrade());
        let slider = glib::object::SendWeakRef::<gtk::Scale>::from(slider.downgrade());
        bus.connect_message(None, move |_bus, message| match message.view() {
            message::MessageView::Error(error) => {
                error_cb(&playbin, error);
            }
            message::MessageView::Eos(_eos) => {
                eos_cb(&playbin);
            }
            message::MessageView::StateChanged(state_changed) => {
                let mut data = data.write().unwrap();
                let CustomData { state, duration } = &mut *data;
                let slider = slider.upgrade().unwrap();
                state_changed_cb(
                    &playbin,
                    &slider,
                    &slider_update_signal_id,
                    state_changed,
                    state,
                    duration,
                );
            }
            message::MessageView::Application(application) => {
                let streams_list = streams_list.upgrade().unwrap();
                application_cb(application, &playbin, &streams_list);
            }
            _ => {}
        });
    }

    /* Start playing */
    playbin.set_state(State::Playing)?;

    /* Register a function that GLib will call every second */
    glib::timeout_add_seconds(1, move || Continue(true));

    /* Start the GTK main loop. We will not regain control until gtk_main_quit is called. */
    gtk::main();

    /* Free resources */
    let _ = playbin.set_state(State::Null);

    Ok(())
}

#[cfg(unix)]
fn window_handle(window: &gtk::gdk::Window) -> usize {
    unsafe { gdkx11::ffi::gdk_x11_window_get_xid(window.as_ptr() as _) as _ }
}
