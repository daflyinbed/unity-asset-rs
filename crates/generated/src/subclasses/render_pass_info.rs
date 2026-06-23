//! 自动生成。请勿手动修改。

use crate::subclasses::attachment_info::AttachmentInfo;
use crate::subclasses::sub_pass_descriptor::SubPassDescriptor;

#[derive(Debug, Clone)]
pub struct RenderPassInfo {
    pub attachments: Vec<AttachmentInfo>,
    pub attachment_count: i32,
    pub sub_passes: Vec<SubPassDescriptor>,
    pub sub_pass_count: i32,
    pub depth_attachment_index: i32,
    pub shading_rate_index: i32,
    pub multiview_count: i32,
    pub sample_count: i32,
}

impl crate::Parse for RenderPassInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let attachments = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AttachmentInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let attachment_count = r.read_i32()?;
        let sub_passes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SubPassDescriptor::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let sub_pass_count = r.read_i32()?;
        let depth_attachment_index = r.read_i32()?;
        let shading_rate_index = r.read_i32()?;
        let multiview_count = r.read_i32()?;
        let sample_count = r.read_i32()?;

        Ok(RenderPassInfo {
            attachments,
            attachment_count,
            sub_passes,
            sub_pass_count,
            depth_attachment_index,
            shading_rate_index,
            multiview_count,
            sample_count,
        })
    }
}
