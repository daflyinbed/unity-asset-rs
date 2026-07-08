//! 自动生成。请勿手动修改。

use crate::subclasses::streamed_resource::StreamedResource;


/// 版本枚举：AudioClipMIsTrackerFormatVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioClipMIsTrackerFormatVariant {
    Default(bool),
    V2017_1(bool),
}

impl crate::Parse for AudioClipMIsTrackerFormatVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AudioClip {
    pub m_name: String,
    pub m_format: Option<i32>,
    pub m_load_type: Option<i32>,
    pub m_type: Option<i32>,
    pub m_channels: Option<i32>,
    pub m_3d: Option<bool>,
    pub m_frequency: Option<i32>,
    pub m_use_hardware: Option<bool>,
    pub m_bits_per_sample: Option<i32>,
    pub m_stream: Option<i32>,
    pub m_length: Option<f32>,
    pub m_audio_data: Option<Vec<u8>>,
    pub m_is_tracker_format: Option<AudioClipMIsTrackerFormatVariant>,
    pub m_subsound_index: Option<i32>,
    pub m_ambisonic: Option<bool>,
    pub m_preload_audio_data: Option<bool>,
    pub m_load_in_background: Option<bool>,
    pub m_legacy3_d: Option<bool>,
    pub m_resource: Option<StreamedResource>,
    pub m_compression_format: Option<i32>,
}

impl crate::Parse for AudioClip {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_format = None;
        let mut m_load_type = None;
        let mut m_type = None;
        let mut m_channels = None;
        let mut m_3d = None;
        let mut m_frequency = None;
        let mut m_use_hardware = None;
        let mut m_bits_per_sample = None;
        let mut m_stream = None;
        let mut m_length = None;
        let mut m_audio_data = None;
        let mut m_is_tracker_format = None;
        let mut m_subsound_index = None;
        let mut m_ambisonic = None;
        let mut m_preload_audio_data = None;
        let mut m_load_in_background = None;
        let mut m_legacy3_d = None;
        let mut m_resource = None;
        let mut m_compression_format = None;
            m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_load_type = Some(rdr.read_i32()?);
            m_channels = Some(rdr.read_i32()?);
            m_frequency = Some(rdr.read_i32()?);
            m_bits_per_sample = Some(rdr.read_i32()?);
            m_length = Some(rdr.read_f32()?);
            m_is_tracker_format = Some(AudioClipMIsTrackerFormatVariant::parse(rdr, ver)?);
            m_ambisonic = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_subsound_index = Some(rdr.read_i32()?);
            m_preload_audio_data = Some(rdr.read_bool()?);
            m_load_in_background = Some(rdr.read_bool()?);
            m_legacy3_d = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_resource = Some(StreamedResource::parse(rdr, ver)?);
            m_compression_format = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_load_type = Some(rdr.read_i32()?);
            m_channels = Some(rdr.read_i32()?);
            m_frequency = Some(rdr.read_i32()?);
            m_bits_per_sample = Some(rdr.read_i32()?);
            m_length = Some(rdr.read_f32()?);
            m_is_tracker_format = Some(AudioClipMIsTrackerFormatVariant::parse(rdr, ver)?);
            m_subsound_index = Some(rdr.read_i32()?);
            m_preload_audio_data = Some(rdr.read_bool()?);
            m_load_in_background = Some(rdr.read_bool()?);
            m_legacy3_d = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_resource = Some(StreamedResource::parse(rdr, ver)?);
            m_compression_format = Some(rdr.read_i32()?);
            rdr.align(4)?;
        } else {
            m_format = Some(rdr.read_i32()?);
            m_type = Some(rdr.read_i32()?);
            m_3d = Some(rdr.read_bool()?);
            m_use_hardware = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_stream = Some(rdr.read_i32()?);
            m_audio_data = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        }

        Ok(AudioClip {
            m_name: m_name.unwrap(),
            m_format,
            m_load_type,
            m_type,
            m_channels,
            m_3d,
            m_frequency,
            m_use_hardware,
            m_bits_per_sample,
            m_stream,
            m_length,
            m_audio_data,
            m_is_tracker_format,
            m_subsound_index,
            m_ambisonic,
            m_preload_audio_data,
            m_load_in_background,
            m_legacy3_d,
            m_resource,
            m_compression_format,
        })
    }
}
