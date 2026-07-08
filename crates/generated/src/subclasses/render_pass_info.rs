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
            attachments = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { AttachmentInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            attachment_count = Some(rdr.read_i32()?);
            sub_passes = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SubPassDescriptor::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
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
