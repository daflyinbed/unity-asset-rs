//! 自动生成。请勿手动修改。

use crate::subclasses::streamed_resource::StreamedResource;

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
    pub m_is_tracker_format: Option<bool>,
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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_format = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_load_type = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_type = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_channels = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_3d = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_frequency = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_use_hardware = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_bits_per_sample = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_stream = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_length = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_audio_data = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_is_tracker_format = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_subsound_index = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_ambisonic = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_preload_audio_data = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_load_in_background = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_legacy3_d = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_resource = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = StreamedResource::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_compression_format = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(AudioClip {
            m_name,
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
