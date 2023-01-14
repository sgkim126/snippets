pub type GstPlayFlags = i32;
#[allow(unused)]
pub const GST_PLAY_FLAG_VIDEO: GstPlayFlags = 0b00_0000_0001;
#[allow(unused)]
pub const GST_PLAY_FLAG_AUDIO: GstPlayFlags = 0b00_0000_0010;
#[allow(unused)]
pub const GST_PLAY_FLAG_TEXT: GstPlayFlags = 0b00_0000_0100;
#[allow(unused)]
pub const GST_PLAY_FLAG_DOWNLOAD: GstPlayFlags = 0b00_1000_0000; /* Enable progressive download (on selected formats) */
#[allow(unused)]
pub const GST_PLAY_FLAG_VIS: GstPlayFlags = 0b00_0000_1000; /* Enable rendering of visualizations when there is no video stream. */
