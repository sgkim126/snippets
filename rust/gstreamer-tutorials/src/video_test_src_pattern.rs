use gstreamer::glib;
use gstreamer::glib::translate::{from_glib, FromGlib, IntoGlib, ToGlibPtr, ToGlibPtrMut};
use gstreamer::glib::value::FromValue;
use gstreamer::glib::StaticType;
use std::ffi::{c_char, c_int};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum VideoTestSrcPattern {
    Smpte,
    Snow,
    Black,
    White,
    Red,
    Green,
    Blue,
    Checkers1,
    Checkers2,
    Checkers4,
    Checkers8,
    Circular,
    Blink,
    Smpte75,
    ZonePlate,
    Gamut,
    ChromaZonePlate,
    SolidColor,
    Ball,
    Smpte100,
    Bar,
    Pinwheel,
    Spokes,
    Gradient,
    Colors,
    SmpteRP219,
    __Unknown(i32),
}

pub type GstVideoTestSrcPattern = c_int;
pub const GST_VIDEO_TEST_SRC_PATTERN_SMPTE: GstVideoTestSrcPattern = 0;
pub const GST_VIDEO_TEST_SRC_PATTERN_SNOW: GstVideoTestSrcPattern = 1;
pub const GST_VIDEO_TEST_SRC_PATTERN_BLACK: GstVideoTestSrcPattern = 2;
pub const GST_VIDEO_TEST_SRC_PATTERN_WHITE: GstVideoTestSrcPattern = 3;
pub const GST_VIDEO_TEST_SRC_PATTERN_RED: GstVideoTestSrcPattern = 4;
pub const GST_VIDEO_TEST_SRC_PATTERN_GREEN: GstVideoTestSrcPattern = 5;
pub const GST_VIDEO_TEST_SRC_PATTERN_BLUE: GstVideoTestSrcPattern = 6;
pub const GST_VIDEO_TEST_SRC_PATTERN_CHECKERS1: GstVideoTestSrcPattern = 7;
pub const GST_VIDEO_TEST_SRC_PATTERN_CHECKERS2: GstVideoTestSrcPattern = 8;
pub const GST_VIDEO_TEST_SRC_PATTERN_CHECKERS4: GstVideoTestSrcPattern = 9;
pub const GST_VIDEO_TEST_SRC_PATTERN_CHECKERS8: GstVideoTestSrcPattern = 10;
pub const GST_VIDEO_TEST_SRC_PATTERN_CIRCULAR: GstVideoTestSrcPattern = 11;
pub const GST_VIDEO_TEST_SRC_PATTERN_BLINK: GstVideoTestSrcPattern = 12;
pub const GST_VIDEO_TEST_SRC_PATTERN_SMPTE_75: GstVideoTestSrcPattern = 13;
pub const GST_VIDEO_TEST_SRC_PATTERN_ZONE_PLATE: GstVideoTestSrcPattern = 14;
pub const GST_VIDEO_TEST_SRC_PATTERN_GAMUT: GstVideoTestSrcPattern = 15;
pub const GST_VIDEO_TEST_SRC_PATTERN_CHROMA_ZONE_GAMUT: GstVideoTestSrcPattern = 16;
pub const GST_VIDEO_TEST_SRC_PATTERN_SOLID_COLOR: GstVideoTestSrcPattern = 17;
pub const GST_VIDEO_TEST_SRC_PATTERN_BALL: GstVideoTestSrcPattern = 18;
pub const GST_VIDEO_TEST_SRC_PATTERN_SMPTE_100: GstVideoTestSrcPattern = 19;
pub const GST_VIDEO_TEST_SRC_PATTERN_BAR: GstVideoTestSrcPattern = 20;
pub const GST_VIDEO_TEST_SRC_PATTERN_PINWHEEL: GstVideoTestSrcPattern = 21;
pub const GST_VIDEO_TEST_SRC_PATTERN_SPOKES: GstVideoTestSrcPattern = 22;
pub const GST_VIDEO_TEST_SRC_PATTERN_GRADIENT: GstVideoTestSrcPattern = 23;
pub const GST_VIDEO_TEST_SRC_PATTERN_COLORS: GstVideoTestSrcPattern = 24;
pub const GST_VIDEO_TEST_SRC_PATTERN_SMPTE_RP_219: GstVideoTestSrcPattern = 25;

impl IntoGlib for VideoTestSrcPattern {
    type GlibType = GstVideoTestSrcPattern;

    #[inline]
    fn into_glib(self) -> Self::GlibType {
        match self {
            Self::Smpte => GST_VIDEO_TEST_SRC_PATTERN_SMPTE,
            Self::Snow => GST_VIDEO_TEST_SRC_PATTERN_SNOW,
            Self::Black => GST_VIDEO_TEST_SRC_PATTERN_BLACK,
            Self::White => GST_VIDEO_TEST_SRC_PATTERN_WHITE,
            Self::Red => GST_VIDEO_TEST_SRC_PATTERN_RED,
            Self::Green => GST_VIDEO_TEST_SRC_PATTERN_GREEN,
            Self::Blue => GST_VIDEO_TEST_SRC_PATTERN_BLUE,
            Self::Checkers1 => GST_VIDEO_TEST_SRC_PATTERN_CHECKERS1,
            Self::Checkers2 => GST_VIDEO_TEST_SRC_PATTERN_CHECKERS2,
            Self::Checkers4 => GST_VIDEO_TEST_SRC_PATTERN_CHECKERS4,
            Self::Checkers8 => GST_VIDEO_TEST_SRC_PATTERN_CHECKERS8,
            Self::Circular => GST_VIDEO_TEST_SRC_PATTERN_CIRCULAR,
            Self::Blink => GST_VIDEO_TEST_SRC_PATTERN_BLINK,
            Self::Smpte75 => GST_VIDEO_TEST_SRC_PATTERN_SMPTE_75,
            Self::ZonePlate => GST_VIDEO_TEST_SRC_PATTERN_ZONE_PLATE,
            Self::Gamut => GST_VIDEO_TEST_SRC_PATTERN_GAMUT,
            Self::ChromaZonePlate => GST_VIDEO_TEST_SRC_PATTERN_CHROMA_ZONE_GAMUT,
            Self::SolidColor => GST_VIDEO_TEST_SRC_PATTERN_SOLID_COLOR,
            Self::Ball => GST_VIDEO_TEST_SRC_PATTERN_BALL,
            Self::Smpte100 => GST_VIDEO_TEST_SRC_PATTERN_SMPTE_100,
            Self::Bar => GST_VIDEO_TEST_SRC_PATTERN_BAR,
            Self::Pinwheel => GST_VIDEO_TEST_SRC_PATTERN_PINWHEEL,
            Self::Spokes => GST_VIDEO_TEST_SRC_PATTERN_SPOKES,
            Self::Gradient => GST_VIDEO_TEST_SRC_PATTERN_GRADIENT,
            Self::Colors => GST_VIDEO_TEST_SRC_PATTERN_COLORS,
            Self::SmpteRP219 => GST_VIDEO_TEST_SRC_PATTERN_SMPTE_RP_219,
            Self::__Unknown(value) => value,
        }
    }
}

impl FromGlib<GstVideoTestSrcPattern> for VideoTestSrcPattern {
    #[inline]
    unsafe fn from_glib(value: GstVideoTestSrcPattern) -> Self {
        match value {
            GST_VIDEO_TEST_SRC_PATTERN_SMPTE => Self::Smpte,
            GST_VIDEO_TEST_SRC_PATTERN_SNOW => Self::Snow,
            GST_VIDEO_TEST_SRC_PATTERN_BLACK => Self::Black,
            GST_VIDEO_TEST_SRC_PATTERN_WHITE => Self::White,
            GST_VIDEO_TEST_SRC_PATTERN_RED => Self::Red,
            GST_VIDEO_TEST_SRC_PATTERN_GREEN => Self::Green,
            GST_VIDEO_TEST_SRC_PATTERN_BLUE => Self::Blue,
            GST_VIDEO_TEST_SRC_PATTERN_CHECKERS1 => Self::Checkers1,
            GST_VIDEO_TEST_SRC_PATTERN_CHECKERS2 => Self::Checkers2,
            GST_VIDEO_TEST_SRC_PATTERN_CHECKERS4 => Self::Checkers4,
            GST_VIDEO_TEST_SRC_PATTERN_CHECKERS8 => Self::Checkers8,
            GST_VIDEO_TEST_SRC_PATTERN_CIRCULAR => Self::Circular,
            GST_VIDEO_TEST_SRC_PATTERN_BLINK => Self::Blink,
            GST_VIDEO_TEST_SRC_PATTERN_SMPTE_75 => Self::Smpte75,
            GST_VIDEO_TEST_SRC_PATTERN_ZONE_PLATE => Self::ZonePlate,
            GST_VIDEO_TEST_SRC_PATTERN_GAMUT => Self::Gamut,
            GST_VIDEO_TEST_SRC_PATTERN_CHROMA_ZONE_GAMUT => Self::ChromaZonePlate,
            GST_VIDEO_TEST_SRC_PATTERN_SOLID_COLOR => Self::SolidColor,
            GST_VIDEO_TEST_SRC_PATTERN_BALL => Self::Ball,
            GST_VIDEO_TEST_SRC_PATTERN_SMPTE_100 => Self::Smpte100,
            GST_VIDEO_TEST_SRC_PATTERN_BAR => Self::Bar,
            GST_VIDEO_TEST_SRC_PATTERN_PINWHEEL => Self::Pinwheel,
            GST_VIDEO_TEST_SRC_PATTERN_SPOKES => Self::Spokes,
            GST_VIDEO_TEST_SRC_PATTERN_GRADIENT => Self::Gradient,
            GST_VIDEO_TEST_SRC_PATTERN_COLORS => Self::Colors,
            GST_VIDEO_TEST_SRC_PATTERN_SMPTE_RP_219 => Self::SmpteRP219,
            _ => Self::__Unknown(value),
        }
    }
}

impl StaticType for VideoTestSrcPattern {
    #[inline]
    fn static_type() -> glib::Type {
        unsafe {
            let value = glib::gobject_ffi::g_type_from_name(
                "GstVideoTestSrcPattern".as_ptr() as *const c_char
            );
            assert_ne!(0, value);
            from_glib(value)
        }
    }
}

impl glib::ToValue for VideoTestSrcPattern {
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

unsafe impl<'a> FromValue<'a> for VideoTestSrcPattern {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl glib::value::ValueType for VideoTestSrcPattern {
    type Type = Self;
}
