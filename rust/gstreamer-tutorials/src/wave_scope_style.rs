use gstreamer::glib;
use gstreamer::glib::translate::{from_glib, FromGlib, IntoGlib, ToGlibPtr, ToGlibPtrMut};
use gstreamer::glib::value::FromValue;
use gstreamer::glib::StaticType;
use std::ffi::{c_char, c_int};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum WaveScopeStyle {
    Dots,
    Lines,
    ColorDots,
    ColorLines,
    __Unknown(i32),
}

pub type GstWaveScopeStyle = c_int;
pub const GST_WAVE_SCOPE_STYLE_DOTS: GstWaveScopeStyle = 0;
pub const GST_WAVE_SCOPE_STYLE_LINES: GstWaveScopeStyle = 1;
pub const GST_WAVE_SCOPE_STYLE_COLOR_DOTS: GstWaveScopeStyle = 2;
pub const GST_WAVE_SCOPE_STYLE_COLOR_LINES: GstWaveScopeStyle = 3;

impl IntoGlib for WaveScopeStyle {
    type GlibType = GstWaveScopeStyle;

    #[inline]
    fn into_glib(self) -> Self::GlibType {
        match self {
            Self::Dots => GST_WAVE_SCOPE_STYLE_DOTS,
            Self::Lines => GST_WAVE_SCOPE_STYLE_LINES,
            Self::ColorDots => GST_WAVE_SCOPE_STYLE_COLOR_DOTS,
            Self::ColorLines => GST_WAVE_SCOPE_STYLE_COLOR_LINES,
            Self::__Unknown(value) => value,
        }
    }
}

impl FromGlib<GstWaveScopeStyle> for WaveScopeStyle {
    #[inline]
    unsafe fn from_glib(value: GstWaveScopeStyle) -> Self {
        match value {
            GST_WAVE_SCOPE_STYLE_DOTS => Self::Dots,
            GST_WAVE_SCOPE_STYLE_LINES => Self::Lines,
            GST_WAVE_SCOPE_STYLE_COLOR_DOTS => Self::ColorDots,
            GST_WAVE_SCOPE_STYLE_COLOR_LINES => Self::ColorLines,
            _ => Self::__Unknown(value),
        }
    }
}

impl StaticType for WaveScopeStyle {
    #[inline]
    fn static_type() -> glib::Type {
        unsafe {
            let value =
                glib::gobject_ffi::g_type_from_name("GstWaveScopeStyle".as_ptr() as *const c_char);
            assert_ne!(0, value);
            from_glib(value)
        }
    }
}

impl glib::ToValue for WaveScopeStyle {
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

unsafe impl<'a> FromValue<'a> for WaveScopeStyle {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl glib::value::ValueType for WaveScopeStyle {
    type Type = Self;
}
