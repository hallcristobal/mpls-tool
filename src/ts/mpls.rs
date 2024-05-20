use std::fmt::{Debug, Display};

use super::FromBinary;

#[repr(C, packed)]
#[derive(Clone)]
pub struct MplsFileHeader {
    magic_string: [u8; 8],     // 0x0
    pub playlist_offset: u32,  // 0x8
    pub chapters_offset: u32,  // 0xC
    pub extnsions_offset: u32, // 0x10
    _unk_0: [u8; 0x24],        // 0x14
    pub misc_flags: u8,        // 0x38
}

impl MplsFileHeader {
    pub fn magic_string(&self) -> String {
        String::from_utf8_lossy(&self.magic_string).to_string()
    }
    pub fn mvc_base_view_r(&self) -> bool {
        (self.misc_flags & (1 << 4)) == 1
    }
}

impl Default for MplsFileHeader {
    fn default() -> Self {
        Self {
            magic_string: Default::default(),
            playlist_offset: Default::default(),
            chapters_offset: Default::default(),
            extnsions_offset: Default::default(),
            _unk_0: [0; 0x24],
            misc_flags: Default::default(),
        }
    }
}

impl FromBinary for MplsFileHeader {
    fn sanitize(&mut self) {
        self.playlist_offset = self.playlist_offset.swap_bytes();
        self.chapters_offset = self.chapters_offset.swap_bytes();
        self.extnsions_offset = self.extnsions_offset.swap_bytes();
        self.misc_flags = self.misc_flags.swap_bytes();
    }
}

impl Display for MplsFileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", FriendlyMplsFileHeader::new(self))
    }
}

impl Debug for MplsFileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", FriendlyMplsFileHeader::new(self))
    }
}

#[allow(unused)]
#[derive(Debug)]
struct FriendlyMplsFileHeader {
    magic_string: String,
    playlist_offset: u32,
    chapters_offset: u32,
    extnsions_offset: u32,
    misc_flags: u8,
    mvc_base_view_r: bool,
}

impl FriendlyMplsFileHeader {
    pub fn new(header: &MplsFileHeader) -> Self {
        FriendlyMplsFileHeader {
            magic_string: header.magic_string(),
            playlist_offset: header.playlist_offset,
            chapters_offset: header.chapters_offset,
            extnsions_offset: header.extnsions_offset,
            misc_flags: header.misc_flags,
            mvc_base_view_r: header.mvc_base_view_r(),
        }
    }
}
