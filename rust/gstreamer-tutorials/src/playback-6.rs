mod error;
mod play_flag;

use crate::play_flag::{GstPlayFlags, GST_PLAY_FLAG_VIS};
use gstreamer::prelude::*;
use gstreamer::*;

/* Return TRUE if this is a Visualization element */
fn filter_vis_features(feature: &PluginFeature) -> bool {
    match feature.downcast_ref::<ElementFactory>() {
        Some(factory) => factory.klass() == "Visualization",
        None => false,
    }
}

fn main() -> Result<(), error::Error> {
    /* Initialize GStreamer */
    gstreamer::init()?;

    /* Get a list of all visualization plugins */
    let list = Registry::get().features_filtered(filter_vis_features, false);

    /* Print their names */
    println!("Available visualization plugins:");
    let mut selected_factory = None;
    for walk in list {
        let factory = walk.downcast::<ElementFactory>().unwrap();
        let name = factory.longname();
        println!("  {}", name);
        if selected_factory.is_none() || name.starts_with("GOOM") {
            selected_factory = Some(factory);
        }
    }

    /* Don't use the factory if it's still empty */
    /* e.g. no visualization plugins found */
    if selected_factory.is_none() {
        println!("No visualization plugins found!");
        panic!();
    }
    let selected_factory = selected_factory.unwrap();

    /* We have now selected a factory for the visualization element */
    println!("Selected '{}'", selected_factory.longname());
    let vis_plugin = selected_factory.create().build()?;

    /* Build the pipeline */
    let pipeline = parse_launch("playbin uri=http://radio.hbr1.com:19800/ambient.ogg").unwrap();

    /* Set the visualization flag */
    let mut flags = unsafe {
        use gstreamer::glib::translate::{ToGlibPtr, ToGlibPtrMut};

        let mut flags = 0.to_value();
        glib::gobject_ffi::g_object_get_property(
            pipeline.as_object_ref().to_glib_none().0,
            "flags\0".as_ptr() as *const _,
            flags.to_glib_none_mut().0,
        );
        flags.get::<GstPlayFlags>().unwrap()
    };
    flags |= GST_PLAY_FLAG_VIS;
    let flags = flags.to_value();
    unsafe {
        use gstreamer::glib::translate::ToGlibPtr;

        glib::gobject_ffi::g_object_set_property(
            pipeline.as_object_ref().to_glib_none().0,
            "flags\0".as_ptr() as *const _,
            flags.to_glib_none().0,
        );
    }

    /* set vis plugin for playbin */
    pipeline.set_property("vis-plugin", vis_plugin);

    /* Start playing */
    let _ = pipeline.set_state(State::Playing);

    /* Wait until error or EOS */
    let bus = pipeline.bus().unwrap();
    let _msg = bus.timed_pop_filtered(ClockTime::NONE, &[MessageType::Error, MessageType::Eos]);

    /* Free resources */
    let _ = pipeline.set_state(State::Null);

    Ok(())
}
