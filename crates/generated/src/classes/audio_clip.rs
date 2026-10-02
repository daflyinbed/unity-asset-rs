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

impl crate::ToValue for AudioClipMIsTrackerFormatVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// pre-5.0 AudioClip.m_AudioData 的值依赖读取结果。
/// 对齐 AssetRipper `Pass505_FixOldAudioClip` + `AudioClipHelper.ReadOldByteArray`：
/// `m_Stream==2`（Streaming）时磁盘是 `u32 size; u32 offset;` 指向
/// `<collection.name>.resS`，由 Environment 按 offset/size 回填字节。
#[derive(Debug, Clone)]
pub enum AudioClipMAudioData {
    /// 内联（pre-5.0 且 m_Stream != 2）：i32 count + bytes + align。
    Inline(Vec<u8>),
    /// 流式（pre-5.0 且 m_Stream == 2）：u32 size; u32 offset;
    /// 指向 `<collection.name>.resS`，字节由 Environment 取。
    Streamed { size: u32, offset: u32 },
}

impl crate::ToValue for AudioClipMAudioData {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Inline(v) => crate::Value::Bytes(v.clone()),
            Self::Streamed { size, offset } => crate::Value::Struct(vec![
                ("size".to_string(), crate::Value::U32(*size)),
                ("offset".to_string(), crate::Value::U32(*offset)),
            ]),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Inline(v) => crate::inspect_bytes(v, path, offset, limit),
            Self::Streamed { .. } => self.to_value().inspect_value(path, offset, limit),
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
    pub m_audio_data: Option<AudioClipMAudioData>,
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
            m_audio_data = Some(if m_stream == Some(2) {
                // Streaming：u32 size; u32 offset; 指向 <collection>.resS
                let size = rdr.read_u32()?;
                let offset = rdr.read_u32()?;
                crate::classes::audio_clip::AudioClipMAudioData::Streamed { size, offset }
            } else {
                let v = ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?;
                crate::classes::audio_clip::AudioClipMAudioData::Inline(v)
            });
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

impl crate::ToValue for AudioClip {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_Format", &self.m_format),
            crate::opt_field("m_LoadType", &self.m_load_type),
            crate::opt_field("m_Type", &self.m_type),
            crate::opt_field("m_Channels", &self.m_channels),
            crate::opt_field("m_3D", &self.m_3d),
            crate::opt_field("m_Frequency", &self.m_frequency),
            crate::opt_field("m_UseHardware", &self.m_use_hardware),
            crate::opt_field("m_BitsPerSample", &self.m_bits_per_sample),
            crate::opt_field("m_Stream", &self.m_stream),
            crate::opt_field("m_Length", &self.m_length),
            crate::opt_field("m_AudioData", &self.m_audio_data),
            crate::opt_field("m_IsTrackerFormat", &self.m_is_tracker_format),
            crate::opt_field("m_SubsoundIndex", &self.m_subsound_index),
            crate::opt_field("m_Ambisonic", &self.m_ambisonic),
            crate::opt_field("m_PreloadAudioData", &self.m_preload_audio_data),
            crate::opt_field("m_LoadInBackground", &self.m_load_in_background),
            crate::opt_field("m_Legacy3D", &self.m_legacy3_d),
            crate::opt_field("m_Resource", &self.m_resource),
            crate::opt_field("m_CompressionFormat", &self.m_compression_format),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Format" => (self.m_format).inspect_value(path, offset, limit),
                "m_LoadType" => (self.m_load_type).inspect_value(path, offset, limit),
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                "m_Channels" => (self.m_channels).inspect_value(path, offset, limit),
                "m_3D" => (self.m_3d).inspect_value(path, offset, limit),
                "m_Frequency" => (self.m_frequency).inspect_value(path, offset, limit),
                "m_UseHardware" => (self.m_use_hardware).inspect_value(path, offset, limit),
                "m_BitsPerSample" => (self.m_bits_per_sample).inspect_value(path, offset, limit),
                "m_Stream" => (self.m_stream).inspect_value(path, offset, limit),
                "m_Length" => (self.m_length).inspect_value(path, offset, limit),
                "m_AudioData" => self.m_audio_data.inspect_value(path, offset, limit),
                "m_IsTrackerFormat" => {
                    (self.m_is_tracker_format).inspect_value(path, offset, limit)
                }
                "m_SubsoundIndex" => (self.m_subsound_index).inspect_value(path, offset, limit),
                "m_Ambisonic" => (self.m_ambisonic).inspect_value(path, offset, limit),
                "m_PreloadAudioData" => {
                    (self.m_preload_audio_data).inspect_value(path, offset, limit)
                }
                "m_LoadInBackground" => {
                    (self.m_load_in_background).inspect_value(path, offset, limit)
                }
                "m_Legacy3D" => (self.m_legacy3_d).inspect_value(path, offset, limit),
                "m_Resource" => (self.m_resource).inspect_value(path, offset, limit),
                "m_CompressionFormat" => {
                    (self.m_compression_format).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_format).inspect_value(path, offset, limit))
                .map(|v| ("m_Format".to_string(), v)),
            ((self.m_load_type).inspect_value(path, offset, limit))
                .map(|v| ("m_LoadType".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
            ((self.m_channels).inspect_value(path, offset, limit))
                .map(|v| ("m_Channels".to_string(), v)),
            ((self.m_3d).inspect_value(path, offset, limit)).map(|v| ("m_3D".to_string(), v)),
            ((self.m_frequency).inspect_value(path, offset, limit))
                .map(|v| ("m_Frequency".to_string(), v)),
            ((self.m_use_hardware).inspect_value(path, offset, limit))
                .map(|v| ("m_UseHardware".to_string(), v)),
            ((self.m_bits_per_sample).inspect_value(path, offset, limit))
                .map(|v| ("m_BitsPerSample".to_string(), v)),
            ((self.m_stream).inspect_value(path, offset, limit))
                .map(|v| ("m_Stream".to_string(), v)),
            ((self.m_length).inspect_value(path, offset, limit))
                .map(|v| ("m_Length".to_string(), v)),
            (self.m_audio_data.inspect_value(path, offset, limit))
                .map(|v| ("m_AudioData".to_string(), v)),
            ((self.m_is_tracker_format).inspect_value(path, offset, limit))
                .map(|v| ("m_IsTrackerFormat".to_string(), v)),
            ((self.m_subsound_index).inspect_value(path, offset, limit))
                .map(|v| ("m_SubsoundIndex".to_string(), v)),
            ((self.m_ambisonic).inspect_value(path, offset, limit))
                .map(|v| ("m_Ambisonic".to_string(), v)),
            ((self.m_preload_audio_data).inspect_value(path, offset, limit))
                .map(|v| ("m_PreloadAudioData".to_string(), v)),
            ((self.m_load_in_background).inspect_value(path, offset, limit))
                .map(|v| ("m_LoadInBackground".to_string(), v)),
            ((self.m_legacy3_d).inspect_value(path, offset, limit))
                .map(|v| ("m_Legacy3D".to_string(), v)),
            ((self.m_resource).inspect_value(path, offset, limit))
                .map(|v| ("m_Resource".to_string(), v)),
            ((self.m_compression_format).inspect_value(path, offset, limit))
                .map(|v| ("m_CompressionFormat".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
