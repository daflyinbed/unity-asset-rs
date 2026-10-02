//! 自动生成。请勿手动修改。

use crate::subclasses::attachment_info::AttachmentInfo;
use crate::subclasses::sub_pass_descriptor::SubPassDescriptor;

/// 版本枚举：RenderPassInfoSampleCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RenderPassInfoSampleCountVariant {
    Default(i32),
    V6000_6(i32),
}

impl crate::Parse for RenderPassInfoSampleCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            let v = rdr.read_i32()?;
            rdr.align(4)?;
            Ok(Self::V6000_6(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for RenderPassInfoSampleCountVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RenderPassInfo {
    pub attachments: Vec<AttachmentInfo>,
    pub attachment_count: i32,
    pub sub_passes: Vec<SubPassDescriptor>,
    pub sub_pass_count: i32,
    pub depth_attachment_index: i32,
    pub shading_rate_index: i32,
    pub multiview_count: i32,
    pub foveation_image_index: Option<i32>,
    pub sample_count: RenderPassInfoSampleCountVariant,
    pub has_eye_texture: Option<bool>,
}

impl crate::Parse for RenderPassInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut attachments = None;
        let mut attachment_count = None;
        let mut sub_passes = None;
        let mut sub_pass_count = None;
        let mut depth_attachment_index = None;
        let mut shading_rate_index = None;
        let mut multiview_count = None;
        let mut foveation_image_index = None;
        let mut sample_count = None;
        let mut has_eye_texture = None;
        attachments = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| AttachmentInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        attachment_count = Some(rdr.read_i32()?);
        sub_passes = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubPassDescriptor::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        sub_pass_count = Some(rdr.read_i32()?);
        depth_attachment_index = Some(rdr.read_i32()?);
        shading_rate_index = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            foveation_image_index = Some(rdr.read_i32()?);
        } else {
        }
        multiview_count = Some(rdr.read_i32()?);
        sample_count = Some(RenderPassInfoSampleCountVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
        } else if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            has_eye_texture = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(RenderPassInfo {
            attachments: attachments.unwrap(),
            attachment_count: attachment_count.unwrap(),
            sub_passes: sub_passes.unwrap(),
            sub_pass_count: sub_pass_count.unwrap(),
            depth_attachment_index: depth_attachment_index.unwrap(),
            shading_rate_index: shading_rate_index.unwrap(),
            multiview_count: multiview_count.unwrap(),
            foveation_image_index,
            sample_count: sample_count.unwrap(),
            has_eye_texture,
        })
    }
}

impl crate::ToValue for RenderPassInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("attachments".to_string(), self.attachments.to_value())),
            Some((
                "attachmentCount".to_string(),
                self.attachment_count.to_value(),
            )),
            Some(("subPasses".to_string(), self.sub_passes.to_value())),
            Some(("subPassCount".to_string(), self.sub_pass_count.to_value())),
            Some((
                "depthAttachmentIndex".to_string(),
                self.depth_attachment_index.to_value(),
            )),
            Some((
                "shadingRateIndex".to_string(),
                self.shading_rate_index.to_value(),
            )),
            Some((
                "multiviewCount".to_string(),
                self.multiview_count.to_value(),
            )),
            crate::opt_field("foveationImageIndex", &self.foveation_image_index),
            Some(("sampleCount".to_string(), self.sample_count.to_value())),
            crate::opt_field("hasEyeTexture", &self.has_eye_texture),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "attachments" => (self.attachments).inspect_value(path, offset, limit),
                "attachmentCount" => (self.attachment_count).inspect_value(path, offset, limit),
                "subPasses" => (self.sub_passes).inspect_value(path, offset, limit),
                "subPassCount" => (self.sub_pass_count).inspect_value(path, offset, limit),
                "depthAttachmentIndex" => {
                    (self.depth_attachment_index).inspect_value(path, offset, limit)
                }
                "shadingRateIndex" => (self.shading_rate_index).inspect_value(path, offset, limit),
                "multiviewCount" => (self.multiview_count).inspect_value(path, offset, limit),
                "foveationImageIndex" => {
                    (self.foveation_image_index).inspect_value(path, offset, limit)
                }
                "sampleCount" => (self.sample_count).inspect_value(path, offset, limit),
                "hasEyeTexture" => (self.has_eye_texture).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.attachments).inspect_value(path, offset, limit))
                .map(|v| ("attachments".to_string(), v)),
            ((self.attachment_count).inspect_value(path, offset, limit))
                .map(|v| ("attachmentCount".to_string(), v)),
            ((self.sub_passes).inspect_value(path, offset, limit))
                .map(|v| ("subPasses".to_string(), v)),
            ((self.sub_pass_count).inspect_value(path, offset, limit))
                .map(|v| ("subPassCount".to_string(), v)),
            ((self.depth_attachment_index).inspect_value(path, offset, limit))
                .map(|v| ("depthAttachmentIndex".to_string(), v)),
            ((self.shading_rate_index).inspect_value(path, offset, limit))
                .map(|v| ("shadingRateIndex".to_string(), v)),
            ((self.multiview_count).inspect_value(path, offset, limit))
                .map(|v| ("multiviewCount".to_string(), v)),
            ((self.foveation_image_index).inspect_value(path, offset, limit))
                .map(|v| ("foveationImageIndex".to_string(), v)),
            ((self.sample_count).inspect_value(path, offset, limit))
                .map(|v| ("sampleCount".to_string(), v)),
            ((self.has_eye_texture).inspect_value(path, offset, limit))
                .map(|v| ("hasEyeTexture".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
