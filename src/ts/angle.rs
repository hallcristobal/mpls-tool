use std::fmt::{Debug, Display};

use super::FromBinary;

#[repr(C, packed)]
#[derive(Default)]
pub struct ClipAngleHeader {
    angle_name: [u8; 5],
    angle_type: [u8; 4],
    _unk_0: [u8; 1],
}

impl ClipAngleHeader {
    pub fn angle_name(&self) -> String {
        String::from_utf8_lossy(&self.angle_name).to_string()
    }
    pub fn angle_type(&self) -> String {
        String::from_utf8_lossy(&self.angle_type).to_string()
    }
}

impl FromBinary for ClipAngleHeader {
    fn sanitize(&mut self) {}
}

impl Display for ClipAngleHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", FriendlyClipAngleHeader::new(self))
    }
}

impl Debug for ClipAngleHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", FriendlyClipAngleHeader::new(self))
    }
}

#[allow(unused)]
#[derive(Debug)]
struct FriendlyClipAngleHeader {
    angle_name: String,
    angle_type: String,
}

impl FriendlyClipAngleHeader {
    pub fn new(header: &ClipAngleHeader) -> Self {
        FriendlyClipAngleHeader {
            angle_name: header.angle_name(),
            angle_type: header.angle_type(),
        }
    }
}
