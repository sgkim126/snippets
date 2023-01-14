use gstreamer::glib;
use gstreamer::glib::translate::{from_glib, FromGlib, IntoGlib, ToGlibPtr, ToGlibPtrMut};
use gstreamer::glib::value::FromValue;
use gstreamer::glib::StaticType;
use std::ffi::{c_char, c_int};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum AudioVisualizerShader {
    None,
    Fade,
    FadeAndMoveUp,
    FadeAndMoveDown,
    FadeAndMoveLeft,
    FadeAndMoveRight,
    FadeAndMoveHorizOut,
    FadeAndMoveHorizIn,
    FadeAndMoveVertOut,
    FadeAndMoveVertIn,
    __Unknown(i32),
}

pub type GstAudioVisualizerShader = c_int;
pub const GST_AUDIO_VISUALIZER_SHADER_NONE: GstAudioVisualizerShader = 0;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE: GstAudioVisualizerShader = 1;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_UP: GstAudioVisualizerShader = 2;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_DOWN: GstAudioVisualizerShader = 3;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_LEFT: GstAudioVisualizerShader = 4;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_RIGHT: GstAudioVisualizerShader = 5;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_HORIZ_OUT: GstAudioVisualizerShader = 6;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_HORIZ_IN: GstAudioVisualizerShader = 7;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_VERT_OUT: GstAudioVisualizerShader = 8;
pub const GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_VERT_IN: GstAudioVisualizerShader = 9;

impl IntoGlib for AudioVisualizerShader {
    type GlibType = GstAudioVisualizerShader;

    #[inline]
    fn into_glib(self) -> Self::GlibType {
        match self {
            Self::None => GST_AUDIO_VISUALIZER_SHADER_NONE,
            Self::Fade => GST_AUDIO_VISUALIZER_SHADER_FADE,
            Self::FadeAndMoveUp => GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_UP,
            Self::FadeAndMoveDown => GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_DOWN,
            Self::FadeAndMoveLeft => GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_LEFT,
            Self::FadeAndMoveRight => GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_RIGHT,
            Self::FadeAndMoveHorizOut => GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_HORIZ_OUT,
            Self::FadeAndMoveHorizIn => GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_HORIZ_IN,
            Self::FadeAndMoveVertOut => GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_VERT_OUT,
            Self::FadeAndMoveVertIn => GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_VERT_IN,
            Self::__Unknown(value) => value,
        }
    }
}

impl FromGlib<GstAudioVisualizerShader> for AudioVisualizerShader {
    #[inline]
    unsafe fn from_glib(value: GstAudioVisualizerShader) -> Self {
        match value {
            GST_AUDIO_VISUALIZER_SHADER_NONE => Self::None,
            GST_AUDIO_VISUALIZER_SHADER_FADE => Self::Fade,
            GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_UP => Self::FadeAndMoveUp,
            GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_DOWN => Self::FadeAndMoveDown,
            GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_LEFT => Self::FadeAndMoveLeft,
            GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_RIGHT => Self::FadeAndMoveRight,
            GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_HORIZ_OUT => Self::FadeAndMoveHorizOut,
            GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_HORIZ_IN => Self::FadeAndMoveHorizIn,
            GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_VERT_OUT => Self::FadeAndMoveVertOut,
            GST_AUDIO_VISUALIZER_SHADER_FADE_AND_MOVE_VERT_IN => Self::FadeAndMoveVertIn,
            _ => Self::__Unknown(value),
        }
    }
}

impl StaticType for AudioVisualizerShader {
    #[inline]
    fn static_type() -> glib::Type {
        unsafe {
            let value = glib::gobject_ffi::g_type_from_name(
                "GstAudioVisualizerShader".as_ptr() as *const c_char
            );
            assert_ne!(0, value);
            from_glib(value)
        }
    }
}

impl glib::ToValue for AudioVisualizerShader {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

unsafe impl<'a> FromValue<'a> for AudioVisualizerShader {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl glib::value::ValueType for AudioVisualizerShader {
    type Type = Self;
}
