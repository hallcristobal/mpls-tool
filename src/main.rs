use std::{
    env::args,
    fs::File,
    io::{BufReader, Read, Seek},
};

use crate::ts::{
    angle::ClipAngleHeader,
    mpls::MplsFileHeader,
    plist::PlaylistHeader,
    stream_clip::{ClipHeader, StreamClip},
    FromBinary,
};

#[macro_use]
mod macros;

mod log;
mod ts;

use macros::*;

#[derive(Debug, Default)]
struct Parser {
    mpls_header: MplsFileHeader,
    playlist_header: PlaylistHeader,
    clips: Vec<StreamClip>,
}

impl Parser {
    fn total_length(&self) -> f64 {
        self.clips.iter().fold(0.0, |acc, c| acc + c.length)
    }

    fn read_header<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.mpls_header = MplsFileHeader::read_from_binary(reader)?;
        log!("header", "{}", self.mpls_header);
        Ok(())
    }

    fn read_playlist_header<R: Read>(&mut self, reader: &mut R) -> std::io::Result<()> {
        self.playlist_header = PlaylistHeader::read_from_binary(reader)?;
        log!("playlist_header", "{:?}", self.playlist_header);
        Ok(())
    }

    fn read_stream_section(&mut self, reader: &mut BufReader<File>) -> std::io::Result<()> {
        let _starting_position = reader.stream_position()?;
        position!(reader);

        let stream_header = ClipHeader::read_from_binary(reader)?;
        log!("stream_header", "{}", stream_header);

        let stream_clip = StreamClip::new(&stream_header, self.total_length());
        self.clips.push(stream_clip);

        reader.seek_relative(12)?;
        position!(reader);
        let mut angle_count = 0;
        if stream_header.multi_angle() > 0 {
            let angles = read_u8(reader)?;
            reader.seek_relative(1)?;
            for _ in 0..angles {
                let header = ClipAngleHeader::read_from_binary(reader)?;
                log!("header", "{:?}", header);
            }
            if angles - 1 > angle_count {
                angle_count = angles - 1;
            }
        }
        log!("angle count", "{}", angle_count);

        position!(reader);
        let stream_info_len = read_i16(reader)?;
        let _ = read_u16(reader)?;
        let stream_count_video = read_u8(reader)?;
        let stream_count_audio = read_u8(reader)?;
        let stream_count_pg = read_u8(reader)?;
        let stream_count_ig = read_u8(reader)?;
        let stream_count_secondary_audio = read_u8(reader)?;
        let stream_count_secondary_video = read_u8(reader)?;
        let stream_count_pip = read_u8(reader)?;
        reader.seek_relative(5)?;

        log!("stream_info_len", "{}", stream_info_len);
        log!("stream_count_video", "{}", stream_count_video);
        log!("stream_count_audio", "{}", stream_count_audio);
        log!("stream_count_pg", "{}", stream_count_pg);
        log!("stream_count_ig", "{}", stream_count_ig);
        log!(
            "stream_count_secondary_audio",
            "{}",
            stream_count_secondary_audio
        );
        log!(
            "stream_count_secondary_video",
            "{}",
            stream_count_secondary_video
        );
        log!("stream_count_pip", "{}", stream_count_pip);

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut args = args();
    let path = args.nth(1).expect("Path not provided as first argument");

    let f = File::open(path)?;
    let reader = &mut BufReader::new(f);
    let mut parser = Parser::default();
    parser.read_header(reader)?;

    reader.seek(std::io::SeekFrom::Start(
        parser.mpls_header.playlist_offset as u64,
    ))?;
    position!(reader);
    parser.read_playlist_header(reader)?;

    for _ in 0..parser.playlist_header.item_count {
        parser.read_stream_section(reader)?;
        position!(reader);
    }

    Ok(())
}
