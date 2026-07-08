//! 自动生成。请勿手动修改。

use crate::subclasses::sample_settings::SampleSettings;
use crate::subclasses::streamed_resource::StreamedResource;


/// 版本枚举：AudioImporterOutputOutputSettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioImporterOutputOutputSettingsVariant {
    Default(Box<SampleSettings>),
    V2022_2(Box<SampleSettings>),
}

impl crate::Parse for AudioImporterOutputOutputSettingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ( { Ok(Box::new(SampleSettings::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V2022_2(v))
        } else {
            let v = ( { Ok(Box::new(SampleSettings::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AudioImporterOutputEditorOutputSettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AudioImporterOutputEditorOutputSettingsVariant {
    Default(Box<SampleSettings>),
    V2022_2(Box<SampleSettings>),
}

impl crate::Parse for AudioImporterOutputEditorOutputSettingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ( { Ok(Box::new(SampleSettings::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V2022_2(v))
        } else {
            let v = ( { Ok(Box::new(SampleSettings::parse(rdr, ver)?)) } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AudioImporterOutput {
    pub output_settings: AudioImporterOutputOutputSettingsVariant,
    pub output_container_format: i32,
    pub editor_output_settings: AudioImporterOutputEditorOutputSettingsVariant,
    pub editor_output_container_format: i32,
    pub player_resource: Option<StreamedResource>,
}

impl crate::Parse for AudioImporterOutput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let output_settings = AudioImporterOutputOutputSettingsVariant::parse(rdr, ver)?;
        let output_container_format = rdr.read_i32()?;
        let editor_output_settings = AudioImporterOutputEditorOutputSettingsVariant::parse(rdr, ver)?;
        let editor_output_container_format = rdr.read_i32()?;
        let player_resource = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = StreamedResource::parse(rdr, ver)?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(AudioImporterOutput {
            output_settings,
            output_container_format,
            editor_output_settings,
            editor_output_container_format,
            player_resource,
        })
    }
}
