use super::stream_type::{
    AspectRatio, ChannelLayout, FrameRate, SampleRate, StreamType, VideoFormat,
};

// #[derive(Default, Debug)]
// pub struct Stream {
//     pub pid: u16,
//     pub stream_type: StreamType,
//     pub descriptors: Vec<String>,
//     pub bit_rate: i64,
//     pub active_bit_rate: i64,
//     pub is_vbr: bool,
//     pub is_initialized: bool,
//     pub language_name: String,
//     pub language_code: String,
//     pub is_hidden: bool,
//     pub payload_bytes: u64,
//     pub packet_count: u64,
//     pub packet_seconds: f64,
//     pub angle_index: i32,
// }

#[derive(Debug)]
pub struct TSVideoStream {
    pub pid: u16,
    pub stream_type: StreamType,
    pub video_format: VideoFormat,
    pub aspect_ratio: AspectRatio,
    pub frame_rate: FrameRate,
}

#[derive(Debug)]
pub struct TSAudioStream {
    pub pid: u16,
    pub stream_type: StreamType,
    pub channel_layout: ChannelLayout,
    pub sample_rate: SampleRate,
    pub lang_code: String,
}

#[derive(Debug)]
pub struct TSGraphicsStream {
    pub pid: u16,
    pub stream_type: StreamType,
    pub lang_code: String,
}

#[derive(Debug)]
pub struct TSTextStream {
    pub pid: u16,
    pub stream_type: StreamType,
    pub lang_code: String,
}

#[derive(Debug)]
pub enum Stream {
    VideoStream(TSVideoStream),
    AudioStream(TSAudioStream),
    GraphicsStream(TSGraphicsStream),
    TextStream(TSTextStream),
}

impl Stream {
    pub fn pid(&self) -> u16 {
        match self {
            Stream::VideoStream(s) => s.pid,
            Stream::AudioStream(s) => s.pid,
            Stream::GraphicsStream(s) => s.pid,
            Stream::TextStream(s) => s.pid,
        }
    }

    pub fn stream_type(&self) -> StreamType {
        match self {
            Stream::VideoStream(s) => s.stream_type,
            Stream::AudioStream(s) => s.stream_type,
            Stream::GraphicsStream(s) => s.stream_type,
            Stream::TextStream(s) => s.stream_type,
        }
    }
}
