#![allow(unused, non_camel_case_types)]
macro_rules! impl_default {
    ($ident:ident) => {
        impl Default for $ident {
            fn default() -> Self {
                $ident::Unknown
            }
        }
    };
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum StreamType {
    Unknown = 0,
    MPEG1_VIDEO = 0x01,
    MPEG2_VIDEO = 0x02,
    AVC_VIDEO = 0x1b,
    MVC_VIDEO = 0x20,
    HEVC_VIDEO = 0x24,
    VC1_VIDEO = 0xea,

    MPEG1_AUDIO = 0x03,
    MPEG2_AUDIO = 0x04,
    MPEG2_AAC_AUDIO = 0x0F,
    MPEG4_AAC_AUDIO = 0x11,
    LPCM_AUDIO = 0x80,
    AC3_AUDIO = 0x81,
    AC3_PLUS_AUDIO = 0x84,
    AC3_PLUS_SECONDARY_AUDIO = 0xA1,
    AC3_TRUE_HD_AUDIO = 0x83,
    DTS_AUDIO = 0x82,
    DTS_HD_AUDIO = 0x85,
    DTS_HD_SECONDARY_AUDIO = 0xA2,
    DTS_HD_MASTER_AUDIO = 0x86,

    PRESENTATION_GRAPHICS = 0x90,
    INTERACTIVE_GRAPHICS = 0x91,
    SUBTITLE = 0x92,
}

impl From<u8> for StreamType {
    fn from(value: u8) -> Self {
        match value {
            0 => StreamType::Unknown,
            0x01 => StreamType::MPEG1_VIDEO,
            0x02 => StreamType::MPEG2_VIDEO,
            0x1b => StreamType::AVC_VIDEO,
            0x20 => StreamType::MVC_VIDEO,
            0x24 => StreamType::HEVC_VIDEO,
            0xea => StreamType::VC1_VIDEO,
            0x03 => StreamType::MPEG1_AUDIO,
            0x04 => StreamType::MPEG2_AUDIO,
            0x0F => StreamType::MPEG2_AAC_AUDIO,
            0x11 => StreamType::MPEG4_AAC_AUDIO,
            0x80 => StreamType::LPCM_AUDIO,
            0x81 => StreamType::AC3_AUDIO,
            0x84 => StreamType::AC3_PLUS_AUDIO,
            0xA1 => StreamType::AC3_PLUS_SECONDARY_AUDIO,
            0x83 => StreamType::AC3_TRUE_HD_AUDIO,
            0x82 => StreamType::DTS_AUDIO,
            0x85 => StreamType::DTS_HD_AUDIO,
            0xA2 => StreamType::DTS_HD_SECONDARY_AUDIO,
            0x86 => StreamType::DTS_HD_MASTER_AUDIO,
            0x90 => StreamType::PRESENTATION_GRAPHICS,
            0x91 => StreamType::INTERACTIVE_GRAPHICS,
            0x92 => StreamType::SUBTITLE,
            _ => {
                error!("Unknown value for StreamType", "{}", value);
                StreamType::Unknown
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum VideoFormat {
    Unknown = 0x00,
    VIDEOFORMAT_480i = 0x01,
    VIDEOFORMAT_576i = 0x02,
    VIDEOFORMAT_480p = 0x03,
    VIDEOFORMAT_1080i = 0x04,
    VIDEOFORMAT_720p = 0x05,
    VIDEOFORMAT_1080p = 0x06,
    VIDEOFORMAT_576p = 0x07,
    VIDEOFORMAT_2160p = 0x08,
}
impl From<u8> for VideoFormat {
    fn from(value: u8) -> Self {
        match value {
            0x00 => VideoFormat::Unknown,
            0x01 => VideoFormat::VIDEOFORMAT_480i,
            0x02 => VideoFormat::VIDEOFORMAT_576i,
            0x03 => VideoFormat::VIDEOFORMAT_480p,
            0x04 => VideoFormat::VIDEOFORMAT_1080i,
            0x05 => VideoFormat::VIDEOFORMAT_720p,
            0x06 => VideoFormat::VIDEOFORMAT_1080p,
            0x07 => VideoFormat::VIDEOFORMAT_576p,
            0x08 => VideoFormat::VIDEOFORMAT_2160p,
            _ => {
                error!("Unknown value for StreamType", "{}", value);
                VideoFormat::Unknown
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum FrameRate {
    Unknown = 0x00,
    FRAMERATE_23_976 = 0x01,
    FRAMERATE_24 = 0x02,
    FRAMERATE_25 = 0x03,
    FRAMERATE_29_97 = 0x04,
    FRAMERATE_50 = 0x06,
    FRAMERATE_59_94 = 0x07,
}
impl From<u8> for FrameRate {
    fn from(value: u8) -> Self {
        match value {
            0x00 => FrameRate::Unknown,
            0x01 => FrameRate::FRAMERATE_23_976,
            0x02 => FrameRate::FRAMERATE_24,
            0x03 => FrameRate::FRAMERATE_25,
            0x04 => FrameRate::FRAMERATE_29_97,
            0x06 => FrameRate::FRAMERATE_50,
            0x07 => FrameRate::FRAMERATE_59_94,
            _ => {
                error!("Unknown value for FrameRate", "{}", value);
                FrameRate::Unknown
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum ChannelLayout {
    Unknown = 0x00,
    CHANNELLAYOUT_MONO = 0x01,
    CHANNELLAYOUT_STEREO = 0x03,
    CHANNELLAYOUT_MULTI = 0x06,
    CHANNELLAYOUT_COMBO = 0x0C,
}
impl From<u8> for ChannelLayout {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ChannelLayout::Unknown,
            0x01 => ChannelLayout::CHANNELLAYOUT_MONO,
            0x03 => ChannelLayout::CHANNELLAYOUT_STEREO,
            0x06 => ChannelLayout::CHANNELLAYOUT_MULTI,
            0x0C => ChannelLayout::CHANNELLAYOUT_COMBO,
            _ => {
                error!("Unknown value for FrameRate", "{}", value);
                ChannelLayout::Unknown
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum SampleRate {
    Unknown = 0x00,
    SAMPLERATE_48 = 0x01,
    SAMPLERATE_96 = 0x04,
    SAMPLERATE_192 = 0x05,
    SAMPLERATE_48_192 = 0xC,
    SAMPLERATE_48_96 = 0xE,
}
impl From<u8> for SampleRate {
    fn from(value: u8) -> Self {
        match value {
            0x00 => SampleRate::Unknown,
            0x01 => SampleRate::SAMPLERATE_48,
            0x04 => SampleRate::SAMPLERATE_96,
            0x05 => SampleRate::SAMPLERATE_192,
            0x0C => SampleRate::SAMPLERATE_48_192,
            0x0E => SampleRate::SAMPLERATE_48_96,
            _ => {
                error!("Unknown value for SampleRate", "{}", value);
                SampleRate::Unknown
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum AspectRatio {
    Unknown = 0x00,
    ASPECT_4_3 = 0x02,
    ASPECT_16_9 = 0x03,
    ASPECT_2_21 = 0x04,
}
impl From<u8> for AspectRatio {
    fn from(value: u8) -> Self {
        match value {
            0x00 => AspectRatio::Unknown,
            0x02 => AspectRatio::ASPECT_4_3,
            0x03 => AspectRatio::ASPECT_16_9,
            0x04 => AspectRatio::ASPECT_2_21,
            _ => {
                error!("Unknown value for AspectRatio", "{}", value);
                AspectRatio::Unknown
            }
        }
    }
}

impl_default!(StreamType);
impl_default!(VideoFormat);
impl_default!(FrameRate);
impl_default!(ChannelLayout);
impl_default!(SampleRate);
impl_default!(AspectRatio);
