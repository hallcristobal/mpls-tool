#![allow(unused)]
use std::io::Read;

macro_rules! log {
    ($name:literal, $fmt:expr, $($arg:tt)*) => {
        crate::log::key_val_print(Some(crate::log::Color::Cyan), $name, &format!($fmt, $($arg)*));
    };
}

macro_rules! position {
    ($ident:ident) => {
        crate::log::key_val_print(
            Some(crate::log::Color::Yellow),
            "Reader Position",
            &format!("{:X}", $ident.stream_position()?),
        );
    };
}

macro_rules! impl_read {
    ($t:ty) => {
        ::paste::paste! {
            pub fn [<read_ $t>] <R: Read>(reader: &mut R) -> std::io::Result<$t> {
                let mut buffer = [0; std::mem::size_of::<$t>()];
                reader.read(&mut buffer)?;
                Ok($t::from_be_bytes(buffer))
            }
        }
    };
}

impl_read!(i8);
impl_read!(i16);
impl_read!(i32);
impl_read!(i64);
impl_read!(u8);
impl_read!(u16);
impl_read!(u32);
impl_read!(u64);

pub fn read_string<R: Read>(reader: &mut R, size: usize) -> std::io::Result<String> {
    let mut buffer = vec![0; size];
    reader.read(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}
