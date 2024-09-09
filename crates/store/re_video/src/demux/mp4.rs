//! MP4 support.

#![allow(clippy::map_err_ignore)]

use crate::TimeMs;

use super::Codec;
use super::{Config, Sample, Segment, VideoData, VideoLoadError};
use ::mp4;

use mp4::TrackKind;

pub fn load_mp4(bytes: &[u8]) -> Result<VideoData, VideoLoadError> {
    let mp4 = ::mp4::read(bytes)?;

    let video_track = mp4
        .tracks()
        .values()
        .find(|t| t.kind == TrackKind::Video)
        .ok_or_else(|| VideoLoadError::NoVideoTrack)?;

    let (codec, codec_string, description) =
        if let Some(::mp4::Av01Box { av1c, av1c_raw, .. }) =
            &video_track.trak(&mp4).mdia.minf.stbl.stsd.av01
        {
            let profile = av1c.profile;
            let level = av1c.level;
            let tier = if av1c.tier == 0 { "M" } else { "H" };
            let bit_depth = av1c.bit_depth;

            (
                Codec::Av01,
                format!("av01.{profile}.{level:02}{tier}.{bit_depth:02}"),
                av1c_raw.clone(),
            )
        } else {
            // TODO(jan): support h.264, h.265, vp8, vp9
            let stsd = &video_track.trak(&mp4).mdia.minf.stbl.stsd;
            let codec_name = if stsd.avc1.is_some() {
                "avc"
            } else if stsd.hev1.is_some() {
                "hevc"
            } else if stsd.vp09.is_some() {
                "vp9"
            } else {
                "unknown"
            };
            return Err(VideoLoadError::UnsupportedCodec(codec_name.into()));
        };

    let coded_height = video_track.height;
    let coded_width = video_track.width;

    let config = Config {
        codec,
        codec_string,
        description,
        coded_height,
        coded_width,
    };

    let duration = TimeMs::new(video_track.duration_ms());
    let mut samples = Vec::<Sample>::new();
    let mut segment_start = 0;
    let mut segments = Vec::<Segment>::new();
    let data = video_track.data.clone();

    for sample in &video_track.samples {
        if sample.is_sync && !samples.is_empty() {
            let start = segment_start;
            let len = samples.len() - segment_start;
            segments.push(Segment {
                timestamp: samples[0].timestamp,
                start,
                len,
            });
            segment_start = samples.len();
        }

        let timestamp = TimeMs::new(sample.timestamp_ms());
        let duration = TimeMs::new(sample.duration_ms());

        let byte_offset = sample.offset as u32;
        let byte_length = sample.size as u32;

        samples.push(Sample {
            timestamp,
            duration,
            byte_offset,
            byte_length,
        });
    }

    if !samples.is_empty() {
        let start = segment_start;
        let len = samples.len() - segment_start;
        segments.push(Segment {
            timestamp: samples[0].timestamp,
            start,
            len,
        });
    }

    Ok(VideoData {
        config,
        duration,
        segments,
        samples,
        data,
    })
}
