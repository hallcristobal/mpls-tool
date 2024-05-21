use std::fmt::{Debug, Display};

/// https://en.wikibooks.org/wiki/User:Bdinfo/mpls
use super::{stream_clip::Duration, FromBinary};

#[repr(C, packed)]
#[derive(Default, Debug, Clone)]
pub struct PlaylistHeader {
    pub length: u32,
    _unk_0: u16,
    pub item_count: u16,
    pub subitem_count: u16,
}

impl FromBinary for PlaylistHeader {
    fn sanitize(&mut self) {
        self.length = self.length.swap_bytes();
        self.item_count = self.item_count.swap_bytes();
        self.subitem_count = self.subitem_count.swap_bytes();
    }
}

impl PlaylistHeader {
    pub const fn expected_size() -> u32 {
        10
    }
    pub fn item_count(&self) -> u16 {
        self.item_count
    }
    pub fn subitem_count(&self) -> u16 {
        self.subitem_count
    }
}

#[repr(C, packed)]
#[derive(Default, Debug, Clone)]
pub struct ChapterHeader {
    pub length: u32,
    pub mark_count: u16,
}

impl FromBinary for ChapterHeader {
    fn sanitize(&mut self) {
        self.length = self.length.swap_bytes();
        self.mark_count = self.mark_count.swap_bytes();
    }
}

impl ChapterHeader {
    pub const fn expected_size() -> u32 {
        6
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn mark_count(&self) -> u16 {
        self.mark_count
    }
}

#[repr(C, packed)]
#[derive(Default, Clone)]
pub struct ChapterSection {
    _unk_0: u8,
    pub mark_type: u8,
    pub play_item_id: u16,
    timestamp: i32,
    _entry_espid: u16,
    duration: i32,
}

impl Display for ChapterSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", FriendlyChapterSection::new(self))
    }
}

impl Debug for ChapterSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", FriendlyChapterSection::new(self))
    }
}

impl FromBinary for ChapterSection {
    fn sanitize(&mut self) {
        self.play_item_id = self.play_item_id.swap_bytes();
        self.timestamp = self.timestamp.swap_bytes();
        self._entry_espid = self._entry_espid.swap_bytes();
        self.duration = self.duration.swap_bytes();
    }
}

impl ChapterSection {
    pub fn timestamp(&self) -> f64 {
        let timestamp = if self.timestamp < 0 {
            self.timestamp & 0x7FFF_FFFF
        } else {
            self.timestamp
        };
        timestamp as f64 / 45_000.0
    }

    pub fn duration(&self) -> f64 {
        let duration = if self.duration < 0 {
            self.duration & 0x7FFF_FFFF
        } else {
            self.duration
        };
        duration as f64 / 45_000.0
    }

    pub fn valid_duration(&self) -> Option<Duration> {
        if self.duration() < f64::EPSILON {
            None
        } else {
            Some(Duration::from_secs_f64(self.duration()))
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
struct FriendlyChapterSection {
    mark_type: u8,
    play_item_id: u16,
    timestamp: f64,
    duration: Option<f64>,
}

impl FriendlyChapterSection {
    fn new(body: &ChapterSection) -> Self {
        FriendlyChapterSection {
            mark_type: body.mark_type,
            play_item_id: body.play_item_id,
            timestamp: body.timestamp(),
            duration: if body.duration() < f64::EPSILON {
                None
            } else {
                Some(body.duration())
            },
        }
    }
}
