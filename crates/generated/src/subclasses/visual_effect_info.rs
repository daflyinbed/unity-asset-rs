//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_event_desc::VfxEventDesc;
use crate::subclasses::vfx_expression_container::VfxExpressionContainer;
use crate::subclasses::vfx_mapping::VfxMapping;
use crate::subclasses::vfx_property_sheet_serialized_base::VfxPropertySheetSerializedBase;
use crate::subclasses::vfx_renderer_settings::VfxRendererSettings;
use crate::subclasses::vfxcpu_buffer_desc::VfxcpuBufferDesc;
use crate::subclasses::vfxgpu_buffer_desc::VfxgpuBufferDesc;

#[derive(Debug, Clone)]
pub struct VisualEffectInfo {
    pub m_expressions: VfxExpressionContainer,
    pub m_property_sheet: VfxPropertySheetSerializedBase,
    pub m_exposed_expressions: Vec<VfxMapping>,
    pub m_buffers: Vec<VfxgpuBufferDesc>,
    pub m_cpu_buffers: Vec<VfxcpuBufferDesc>,
    pub m_events: Vec<VfxEventDesc>,
    pub m_runtime_version: u32,
    pub m_renderer_settings: VfxRendererSettings,
    pub m_culling_flags: i32,
    pub m_update_mode: i32,
}

impl crate::Parse for VisualEffectInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expressions = VfxExpressionContainer::parse(r, ver)?;
        r.align(4)?;
        let m_property_sheet = VfxPropertySheetSerializedBase::parse(r, ver)?;
        r.align(4)?;
        let m_exposed_expressions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxMapping::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_buffers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxgpuBufferDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_cpu_buffers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxcpuBufferDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_events = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxEventDesc::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_runtime_version = r.read_u32()?;
        let m_renderer_settings = VfxRendererSettings::parse(r, ver)?;
        r.align(4)?;
        let m_culling_flags = r.read_i32()?;
        let m_update_mode = r.read_i32()?;

        Ok(VisualEffectInfo {
            m_expressions,
            m_property_sheet,
            m_exposed_expressions,
            m_buffers,
            m_cpu_buffers,
            m_events,
            m_runtime_version,
            m_renderer_settings,
            m_culling_flags,
            m_update_mode,
        })
    }
}
