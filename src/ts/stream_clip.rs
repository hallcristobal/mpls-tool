use std::{
    fmt::{Debug, Display},
    ptr,
};

use super::FromBinary;

#[repr(C, packed)]
#[derive(Default, Clone)]
pub struct ClipHeader {
    pub item_len: i16,
    pub item_name: [u8; 5],
    pub item_type: [u8; 4],
    _unk: [u8; 1],
    pub ma_c_flags: u8,
    _unk_1: [u8; 1],
    pub in_time: i32,
    pub out_time: i32,
    _unk_2: [u8; 12],
}

impl ClipHeader {
    pub fn item_len(&self) -> i16 {
        unsafe { ptr::read_unaligned(ptr::addr_of!(self.item_len)) }
    }
    pub fn stream_file_name(&self) -> String {
        format!("{}.M2TS", self.item_name())
    }
    pub fn stream_clip_file_name(&self) -> String {
        format!("{}.CLPI", self.item_name())
    }
    pub fn item_name(&self) -> String {
        String::from_utf8(self.item_name.to_vec()).unwrap_or_default()
    }
    pub fn item_type(&self) -> String {
        String::from_utf8(self.item_type.to_vec()).unwrap_or_default()
    }
    pub fn multi_angle(&self) -> u8 {
        (self.ma_c_flags >> 4) & 0x01
    }
    pub fn condition(&self) -> u8 {
        self.ma_c_flags & 0xF
    }
    pub fn time_in(&self) -> f64 {
        let in_time = if self.in_time < 0 {
            self.in_time & 0x7FFF_FFFF
        } else {
            self.in_time
        };
        in_time as f64 / 45_000.0
    }
    pub fn time_out(&self) -> f64 {
        let out_time = if self.out_time < 0 {
            self.out_time & 0x7FFF_FFFF
        } else {
            self.out_time
        };
        out_time as f64 / 45_000.0
    }
}

impl FromBinary for ClipHeader {
    fn sanitize(&mut self) {
        self.item_len = self.item_len.swap_bytes();
        self.in_time = self.in_time.swap_bytes();
        self.out_time = self.out_time.swap_bytes();
    }
}

impl Display for ClipHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", FriendlyClipHeader::new(self))
    }
}

impl Debug for ClipHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", FriendlyClipHeader::new(self))
    }
}

#[allow(unused)]
#[derive(Debug)]
struct FriendlyClipHeader {
    item_len: i16,
    item_name: String,
    item_type: String,
    stream_file_name: String,
    stream_clip_file_name: String,
    ma_c_flags: u8,
    multi_angle: u8,
    condition: u8,
    time_in: f64,
    time_out: f64,
}

impl FriendlyClipHeader {
    fn new(header: &ClipHeader) -> Self {
        FriendlyClipHeader {
            item_len: header.item_len(),
            item_name: header.item_name(),
            item_type: header.item_type(),
            stream_file_name: header.stream_file_name(),
            stream_clip_file_name: header.stream_clip_file_name(),
            ma_c_flags: header.ma_c_flags,
            multi_angle: header.multi_angle(),
            condition: header.condition(),
            time_in: header.time_in(),
            time_out: header.time_out(),
        }
    }
}

#[derive(Debug, Default)]
pub struct StreamClip {
    pub angle_index: i32,
    pub name: String,
    pub time_in: f64,
    pub time_out: f64,
    pub relative_time_in: f64,
    pub relative_time_out: f64,
    pub length: f64,
    pub relative_length: f64,
    pub file_size: u64,
    pub interleaved_file_size: u64,
    pub payload_bytes: u64,
    pub packet_count: u64,
    pub packet_seconds: f64,
    pub chapters: Vec<f64>,
    // TSStreamFile StreamFile = null;
    // TSStreamClipFile StreamClipFile = null;
}

impl StreamClip {
    pub fn new(header: &ClipHeader, total_length: f64) -> Self {
        let mut sc = StreamClip {
            name: header.stream_file_name(),
            time_in: header.time_in(),
            time_out: header.time_out(),
            relative_time_in: total_length,
            ..Default::default()
        };

        sc.length = sc.time_out - sc.time_in;
        sc.relative_time_out = sc.relative_time_in + sc.length;
        sc.relative_length = sc.length / total_length;

        sc
    }
    pub fn display_name(&self) -> String {
        self.name.to_owned()
    }
    pub fn packet_size(&self) -> u64 {
        self.packet_count * 192
    }
    pub fn packet_bit_rate(&self) -> f64 {
        if self.packet_seconds > 0.0 {
            ((self.packet_size() as f64 * 8.0) / self.packet_seconds).round()
        } else {
            0.0
        }
    }
}
