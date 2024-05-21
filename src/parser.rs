use std::{
    collections::HashMap,
    io::{Read, Result, Seek, SeekFrom},
};

use crate::{
    macros,
    ts::{
        self,
        angle::ClipAngleHeader,
        mpls::MplsFileHeader,
        plist::PlaylistHeader,
        stream::{TSAudioStream, TSGraphicsStream, TSTextStream, TSVideoStream},
        stream_clip::{ClipHeader, StreamClip},
        streams_header::StreamsHeader,
        FromBinary,
    },
};

use macros::*;
use ts::{
    plist::{ChapterHeader, ChapterSection},
    stream::Stream,
    stream_type::StreamType,
};

#[derive(Debug, Default)]
pub struct Parser {
    mpls_header: MplsFileHeader,
    playlist_header: PlaylistHeader,
    chapters_header: ChapterHeader,
    playlist_streams: HashMap<u16, Stream>,
    clips: Vec<StreamClip>,
    chapters: Vec<ChapterSection>,
}

impl Parser {
    pub fn total_length(&self) -> f64 {
        self.clips.iter().fold(0.0, |acc, c| acc + c.length)
    }

    pub fn read_header<R: Read + Seek>(&mut self, reader: &mut R) -> Result<()> {
        self.mpls_header = MplsFileHeader::read_from_binary(reader)?;
        let offsets = self.mpls_header.offsets();
        log!("magic string", "{}", self.mpls_header.magic_string());
        log!("Playlist offset", "{:X}", offsets.0);
        log!("Marks offset", "{:X}", offsets.1);
        log!("Extensions offset", "{:X}", offsets.2);
        log!("Misc Flags", "{:X}", self.mpls_header.misc_flags);
        assert_eq!(
            MplsFileHeader::expected_size() as u64,
            reader.stream_position()?
        );
        Ok(())
    }

    pub fn read_playlist_header<R: Read + Seek>(&mut self, reader: &mut R) -> Result<()> {
        reader.seek(SeekFrom::Start(self.mpls_header.playlist_offset.into()))?;
        self.playlist_header = PlaylistHeader::read_from_binary(reader)?;
        log!("play item entries", "{}", self.playlist_header.item_count());
        log!(
            "subpath entries",
            "{}",
            self.playlist_header.subitem_count()
        );
        assert_eq!(
            (self.mpls_header.playlist_offset + PlaylistHeader::expected_size()) as u64,
            reader.stream_position()?
        );
        Ok(())
    }

    pub fn read_play_item_entries<R: Read + Seek>(&mut self, reader: &mut R) -> Result<()> {
        reader.seek(SeekFrom::Start(
            (self.mpls_header.playlist_offset + PlaylistHeader::expected_size()) as u64,
        ))?;

        for i in 0..self.playlist_header.item_count {
            info!(&format!("Playlist Item {}", i), "",);
            self.read_stream_section(reader)?;
        }

        Ok(())
    }

    pub fn read_stream_section<R: Read + Seek>(&mut self, reader: &mut R) -> Result<()> {
        let item_start = reader.stream_position()?;

        let stream_header = ClipHeader::read_from_binary(reader)?;
        info!("Stream Section", "{}", stream_header.stream_file_name());
        info!("Start", "{}", stream_header.time_in());
        info!("End", "{}", stream_header.time_out());
        info!("Duration", "{}", stream_header.duration());

        let stream_clip = StreamClip::new(
            &stream_header,
            self.total_length(),
            &stream_header.stream_file_name(),
            &stream_header.stream_clip_file_name(),
        );
        let relative_length = stream_clip.relative_length;
        self.clips.push(stream_clip);
        let stream_clip = self.clips.last().unwrap();

        let mut _angle_count = 0;

        if stream_header.multi_angle() > 0 {
            log!("multi_angle", "{}", stream_header.multi_angle());

            let angles = read_u8(reader)?;
            reader.seek(SeekFrom::Current(1))?;
            log!("angles", "{}", angles);

            for angle in 0..angles {
                let header = ClipAngleHeader::read_from_binary(reader)?;
                let mut angle_clip = stream_clip.clone();
                angle_clip.angle_index = angle as i32 + 1;
                angle_clip.stream_file = format!("{}.M2TS", header.angle_name());
                angle_clip.stream_clip_file = format!("{}.CLPI", header.angle_name());
            }

            if angles - 1 > _angle_count {
                _angle_count = angles - 1;
            }
        } else {
            log!("multi_angle", "{}", stream_header.multi_angle());
        }

        let streams_header = StreamsHeader::read_from_binary(reader)?;
        log!("stream_info_len", "{}", streams_header.stream_info_len());
        log!(
            "stream_count_video",
            "{}",
            streams_header.stream_count_video
        );
        log!(
            "stream_count_audio",
            "{}",
            streams_header.stream_count_audio
        );
        log!("stream_count_pg", "{}", streams_header.stream_count_pg);
        log!("stream_count_ig", "{}", streams_header.stream_count_ig);
        log!(
            "stream_count_secondary_audio",
            "{}",
            streams_header.stream_count_secondary_audio
        );
        log!(
            "stream_count_secondary_video",
            "{}",
            streams_header.stream_count_secondary_video
        );
        log!("stream_count_pip", "{}", streams_header.stream_count_pip);

        let mut loop_streams = |reader: &mut R, identifier: &str, len: u8| -> Result<()> {
            for i in 0..len {
                let stream = self.create_playlist_stream(reader)?;
                if let Some(stream) = stream {
                    log!(&format!("{} Stream {}", identifier, i), "{:?}", stream);

                    if !self.playlist_streams.contains_key(&stream.pid()) || relative_length > 0.01
                    {
                        self.playlist_streams.insert(stream.pid(), stream);
                    } else {
                        log!(
                            &format!("Dup {} Stream {} pid", identifier, i),
                            "{}",
                            stream.pid()
                        );
                    }
                } else {
                    error!(
                        &format!("{} Stream {}", identifier, i),
                        "{}", "Returned None"
                    )
                }
            }
            Ok(())
        };

        loop_streams(reader, "Videos", streams_header.stream_count_video)?;
        loop_streams(reader, "Audio", streams_header.stream_count_audio)?;
        loop_streams(reader, "PG", streams_header.stream_count_pg)?;
        loop_streams(reader, "IG", streams_header.stream_count_ig)?;
        loop_streams(
            reader,
            "2nd Audio",
            streams_header.stream_count_secondary_audio,
        )?;
        reader.seek(SeekFrom::Current(2))?;
        loop_streams(
            reader,
            "2nd Video",
            streams_header.stream_count_secondary_video,
        )?;
        reader.seek(SeekFrom::Current(6))?;

        let current_pos = reader.stream_position()?;
        reader.seek(SeekFrom::Current(
            stream_header.item_len() as i64 - (current_pos as i64 - item_start as i64) + 2,
        ))?;

        Ok(())
    }

    pub fn create_playlist_stream<R: Read + Seek>(
        &mut self,
        reader: &mut R,
    ) -> Result<Option<Stream>> {
        let header_len = read_u8(reader)? as u64;
        let header_pos = reader.stream_position()?;
        let header_type = read_u8(reader)?;

        let mut pid = 0;
        let mut _subpath_id = 0;
        let mut _subclip_id = 0;

        match header_type {
            1 => {
                pid = read_u16(reader)?;
            }
            2 => {
                _subpath_id = read_u8(reader)?;
                _subclip_id = read_u8(reader)?;
                pid = read_u16(reader)?;
            }
            3 => {
                _subpath_id = read_u8(reader)?;
                pid = read_u16(reader)?;
            }
            4 => {
                _subpath_id = read_u8(reader)?;
                _subclip_id = read_u8(reader)?;
                pid = read_u16(reader)?;
            }
            _ => error!("Invalid header type", "{}", header_type),
        }

        reader.seek(SeekFrom::Start(header_pos + header_len))?;

        let stream_len = read_u8(reader)? as u64;
        let stream_pos = reader.stream_position()?;

        let stream_type: StreamType = read_u8(reader)?.into();
        let stream: Option<Stream> = match stream_type {
            StreamType::MVC_VIDEO => None,
            StreamType::HEVC_VIDEO
            | StreamType::AVC_VIDEO
            | StreamType::MPEG1_VIDEO
            | StreamType::MPEG2_VIDEO
            | StreamType::VC1_VIDEO => {
                let format_flags = read_u8(reader)?;
                let video_format = format_flags >> 4;
                let frame_rate = format_flags & 0xF;
                let aspect_ratio = read_u8(reader)? >> 4;
                Some(Stream::VideoStream(TSVideoStream {
                    pid,
                    stream_type,
                    video_format: video_format.into(),
                    frame_rate: frame_rate.into(),
                    aspect_ratio: aspect_ratio.into(),
                }))
            }

            StreamType::AC3_AUDIO
            | StreamType::AC3_PLUS_AUDIO
            | StreamType::AC3_PLUS_SECONDARY_AUDIO
            | StreamType::AC3_TRUE_HD_AUDIO
            | StreamType::DTS_AUDIO
            | StreamType::DTS_HD_AUDIO
            | StreamType::DTS_HD_MASTER_AUDIO
            | StreamType::DTS_HD_SECONDARY_AUDIO
            | StreamType::LPCM_AUDIO
            | StreamType::MPEG1_AUDIO
            | StreamType::MPEG2_AUDIO
            | StreamType::MPEG2_AAC_AUDIO
            | StreamType::MPEG4_AAC_AUDIO => {
                let audio_format_flags = read_u8(reader)?;
                let channel_layout = audio_format_flags >> 4;
                let sample_rate = audio_format_flags & 0xF;

                let lang_code = read_string(reader, 3)?;
                Some(Stream::AudioStream(TSAudioStream {
                    pid,
                    stream_type,
                    channel_layout: channel_layout.into(),
                    sample_rate: sample_rate.into(),
                    lang_code,
                }))
            }

            StreamType::INTERACTIVE_GRAPHICS | StreamType::PRESENTATION_GRAPHICS => {
                let lang_code = read_string(reader, 3)?;
                Some(Stream::GraphicsStream(TSGraphicsStream {
                    pid,
                    stream_type,
                    lang_code,
                }))
            }

            StreamType::SUBTITLE => {
                let _code = read_u8(reader)?;
                let lang_code = read_string(reader, 3)?;
                Some(Stream::TextStream(TSTextStream {
                    pid,
                    stream_type,
                    lang_code,
                }))
            }
            _ => None,
        };

        reader.seek(SeekFrom::Start(stream_pos + stream_len))?;
        Ok(stream)
    }

    pub fn read_chapters_header<R: Read + Seek>(&mut self, reader: &mut R) -> Result<()> {
        reader.seek(SeekFrom::Start(self.mpls_header.chapters_offset.into()))?;
        self.chapters_header = ChapterHeader::read_from_binary(reader)?;
        assert_eq!(
            (self.mpls_header.chapters_offset + ChapterHeader::expected_size()) as u64,
            reader.stream_position()?
        );
        Ok(())
    }

    pub fn read_chapters<R: Read + Seek>(&mut self, reader: &mut R) -> Result<()> {
        reader.seek(SeekFrom::Start(
            (self.mpls_header.chapters_offset + ChapterHeader::expected_size()) as u64,
        ))?;

        for i in 0..self.chapters_header.mark_count() {
            let mark = ChapterSection::read_from_binary(reader)?;
            log!(
                &format!("Marker: {}", i),
                "{:?}",
                (mark.play_item_id, mark.timestamp(), mark.valid_duration())
            );
            self.chapters.push(mark);
        }

        Ok(())
    }
}
