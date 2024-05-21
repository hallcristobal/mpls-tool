use std::fmt::Debug;

use super::FromBinary;

#[repr(C, packed)]
#[derive(Default)]
pub struct StreamsHeader {
    pub stream_info_len: u16,
    _pad_0: [u8; 0x2],
    pub stream_count_video: u8,
    pub stream_count_audio: u8,
    pub stream_count_pg: u8,
    pub stream_count_ig: u8,
    pub stream_count_secondary_audio: u8,
    pub stream_count_secondary_video: u8,
    pub stream_count_pip: u8,
    _pad_1: [u8; 0x5],
}

impl FromBinary for StreamsHeader {
    fn sanitize(&mut self) {
        self.stream_info_len = self.stream_info_len.swap_bytes();
    }
}

impl Debug for StreamsHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", FriendlyStreamsHeader::new(self))
    }
}

impl StreamsHeader {
    pub fn stream_info_len(&self) -> u16 {
        self.stream_info_len
    }
}

#[allow(unused)]
#[derive(Debug)]
struct FriendlyStreamsHeader {
    stream_info_len: u16,
    stream_count_video: u8,
    stream_count_audio: u8,
    stream_count_pg: u8,
    stream_count_ig: u8,
    stream_count_secondary_audio: u8,
    stream_count_secondary_video: u8,
    stream_count_pip: u8,
}

impl FriendlyStreamsHeader {
    fn new(header: &StreamsHeader) -> Self {
        Self {
            stream_info_len: header.stream_info_len,
            stream_count_video: header.stream_count_video,
            stream_count_audio: header.stream_count_audio,
            stream_count_pg: header.stream_count_pg,
            stream_count_ig: header.stream_count_ig,
            stream_count_secondary_audio: header.stream_count_secondary_audio,
            stream_count_secondary_video: header.stream_count_secondary_video,
            stream_count_pip: header.stream_count_pip,
        }
    }
}
