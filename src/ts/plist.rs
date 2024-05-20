/// https://en.wikibooks.org/wiki/User:Bdinfo/mpls
use super::FromBinary;

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

#[repr(C, packed)]
#[derive(Default, Debug, Clone)]
pub struct PlaylistMarkHeader {
    pub length: u32,
    pub mark_count: u16,
}

impl FromBinary for PlaylistMarkHeader {
    fn sanitize(&mut self) {
        self.length = self.length.swap_bytes();
        self.mark_count = self.mark_count.swap_bytes();
    }
}
