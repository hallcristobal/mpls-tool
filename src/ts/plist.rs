use super::FromBinary;

#[repr(C, packed)]
#[derive(Default, Debug, Clone)]
pub struct PlaylistHeader {
    pub playlist_len: i32,
    pub playlist_reserved: i16,
    pub item_count: i16,
    pub subitem_count: i16,
}

impl FromBinary for PlaylistHeader {
    fn sanitize(&mut self) {
        self.playlist_len = self.playlist_len.swap_bytes();
        self.playlist_reserved = self.playlist_reserved.swap_bytes();
        self.item_count = self.item_count.swap_bytes();
        self.subitem_count = self.subitem_count.swap_bytes();
    }
}
