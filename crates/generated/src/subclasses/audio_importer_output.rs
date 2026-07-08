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
        let mut output_settings = None;
        let mut output_container_format = None;
        let mut editor_output_settings = None;
        let mut editor_output_container_format = None;
        let mut player_resource = None;
            output_settings = Some(AudioImporterOutputOutputSettingsVariant::parse(rdr, ver)?);
            output_container_format = Some(rdr.read_i32()?);
            editor_output_settings = Some(AudioImporterOutputEditorOutputSettingsVariant::parse(rdr, ver)?);
            editor_output_container_format = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
        } else {
            player_resource = Some(StreamedResource::parse(rdr, ver)?);
            rdr.align(4)?;
        }

        Ok(AudioImporterOutput {
            output_settings: output_settings.unwrap(),
            output_container_format: output_container_format.unwrap(),
            editor_output_settings: editor_output_settings.unwrap(),
            editor_output_container_format: editor_output_container_format.unwrap(),
            player_resource,
        })
    }
}
