use super::language_codes::get_language_code;

#[derive(Default, Debug)]
pub struct Stream {
    pid: u16,
    stream_type: u8,
    descriptors: Vec<String>,
    bit_rate: i64,
    active_bit_rate: i64,
    is_vbr: bool,
    is_initialized: bool,
    language_name: String,
    language_code: String,
    is_hidden: bool,
    payload_bytes: u64,
    packet_count: u64,
    packet_seconds: f64,
    angle_index: i32,
}

impl Stream {
    pub fn packet_size(&self) -> u64 {
        self.packet_count * 192
    }
    pub fn set_language_code(&mut self, code: &str) {
        self.language_code = get_language_code(code);
    }
}
