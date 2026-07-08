//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_event_desc::VfxEventDesc;
use crate::subclasses::vfx_exposed_mapping::VfxExposedMapping;
use crate::subclasses::vfx_expression_container::VfxExpressionContainer;
use crate::subclasses::vfx_mapping::VfxMapping;
use crate::subclasses::vfx_property_sheet_serialized_base::VfxPropertySheetSerializedBase;
use crate::subclasses::vfx_renderer_settings::VfxRendererSettings;
use crate::subclasses::vfx_temporary_gpu_buffer_desc::VfxTemporaryGpuBufferDesc;
use crate::subclasses::vfxcpu_buffer_desc::VfxcpuBufferDesc;
use crate::subclasses::vfxgpu_buffer_desc::VfxgpuBufferDesc;


/// 版本枚举：VisualEffectInfoMRendererSettingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VisualEffectInfoMRendererSettingsVariant {
    Default(Box<VfxRendererSettings>),
    V6000_3(Box<VfxRendererSettings>),
}

impl crate::Parse for VisualEffectInfoMRendererSettingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = ( { Ok(Box::new(VfxRendererSettings::parse(rdr, ver)?)) } )?;
            Ok(Self::V6000_3(v))
        } else {
            let v = ( { Ok(Box::new(VfxRendererSettings::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：VisualEffectInfoMExposedExpressionsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VisualEffectInfoMExposedExpressionsVariant {
    Default(Vec<VfxMapping>),
    V2023_1(Vec<VfxExposedMapping>),
}

impl crate::Parse for VisualEffectInfoMExposedExpressionsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxExposedMapping::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2023_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxMapping::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct VisualEffectInfo {
    pub m_expressions: VfxExpressionContainer,
    pub m_renderer_settings: VisualEffectInfoMRendererSettingsVariant,
    pub m_property_sheet: VfxPropertySheetSerializedBase,
    pub m_culling_flags: i32,
    pub m_exposed_expressions: VisualEffectInfoMExposedExpressionsVariant,
    pub m_update_mode: i32,
    pub m_buffers: Vec<VfxgpuBufferDesc>,
    pub m_pre_warm_delta_time: Option<f32>,
    pub m_cpu_buffers: Vec<VfxcpuBufferDesc>,
    pub m_pre_warm_step_count: Option<u32>,
    pub m_temporary_buffers: Option<Vec<VfxTemporaryGpuBufferDesc>>,
    pub m_events: Vec<VfxEventDesc>,
    pub m_initial_event_name: Option<String>,
    pub m_runtime_version: Option<u32>,
    pub m_instancing_mode: Option<i32>,
    pub m_instancing_capacity: Option<u32>,
    pub m_compilation_version: Option<u32>,
    pub m_instancing_disabled_reason: Option<i32>,
}

impl crate::Parse for VisualEffectInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_expressions = None;
        let mut m_renderer_settings = None;
        let mut m_property_sheet = None;
        let mut m_culling_flags = None;
        let mut m_exposed_expressions = None;
        let mut m_update_mode = None;
        let mut m_buffers = None;
        let mut m_pre_warm_delta_time = None;
        let mut m_cpu_buffers = None;
        let mut m_pre_warm_step_count = None;
        let mut m_temporary_buffers = None;
        let mut m_events = None;
        let mut m_initial_event_name = None;
        let mut m_runtime_version = None;
        let mut m_instancing_mode = None;
        let mut m_instancing_capacity = None;
        let mut m_compilation_version = None;
        let mut m_instancing_disabled_reason = None;
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_renderer_settings = Some(VisualEffectInfoMRendererSettingsVariant::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
            m_pre_warm_delta_time = Some(rdr.read_f32()?);
            m_pre_warm_step_count = Some(rdr.read_u32()?);
            m_initial_event_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_instancing_mode = Some(rdr.read_i32()?);
            m_instancing_capacity = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_renderer_settings = Some(VisualEffectInfoMRendererSettingsVariant::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
            m_pre_warm_delta_time = Some(rdr.read_f32()?);
            m_pre_warm_step_count = Some(rdr.read_u32()?);
            m_initial_event_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else {
        }
            m_expressions = Some(VfxExpressionContainer::parse(rdr, ver)?);
            rdr.align(4)?;
            m_property_sheet = Some(VfxPropertySheetSerializedBase::parse(rdr, ver)?);
            rdr.align(4)?;
            m_exposed_expressions = Some(VisualEffectInfoMExposedExpressionsVariant::parse(rdr, ver)?);
            m_buffers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxgpuBufferDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_temporary_buffers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxTemporaryGpuBufferDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
        }
            m_cpu_buffers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxcpuBufferDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_events = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { VfxEventDesc::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_instancing_disabled_reason = Some(rdr.read_i32()?);
            m_runtime_version = Some(rdr.read_u32()?);
            m_compilation_version = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_runtime_version = Some(rdr.read_u32()?);
            m_compilation_version = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_runtime_version = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_renderer_settings = Some(VisualEffectInfoMRendererSettingsVariant::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
            m_pre_warm_delta_time = Some(rdr.read_f32()?);
            m_pre_warm_step_count = Some(rdr.read_u32()?);
            m_initial_event_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_runtime_version = Some(rdr.read_u32()?);
            m_renderer_settings = Some(VisualEffectInfoMRendererSettingsVariant::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
            m_pre_warm_delta_time = Some(rdr.read_f32()?);
            m_pre_warm_step_count = Some(rdr.read_u32()?);
            m_initial_event_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 2, 17) {
            m_runtime_version = Some(rdr.read_u32()?);
            m_renderer_settings = Some(VisualEffectInfoMRendererSettingsVariant::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
            m_pre_warm_delta_time = Some(rdr.read_f32()?);
            m_pre_warm_step_count = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_renderer_settings = Some(VisualEffectInfoMRendererSettingsVariant::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
            m_pre_warm_delta_time = Some(rdr.read_f32()?);
            m_pre_warm_step_count = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 4, 15) {
            m_runtime_version = Some(rdr.read_u32()?);
            m_renderer_settings = Some(VisualEffectInfoMRendererSettingsVariant::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
        } else {
            m_renderer_settings = Some(VisualEffectInfoMRendererSettingsVariant::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
        }

        Ok(VisualEffectInfo {
            m_expressions: m_expressions.unwrap(),
            m_renderer_settings: m_renderer_settings.unwrap(),
            m_property_sheet: m_property_sheet.unwrap(),
            m_culling_flags: m_culling_flags.unwrap(),
            m_exposed_expressions: m_exposed_expressions.unwrap(),
            m_update_mode: m_update_mode.unwrap(),
            m_buffers: m_buffers.unwrap(),
            m_pre_warm_delta_time,
            m_cpu_buffers: m_cpu_buffers.unwrap(),
            m_pre_warm_step_count,
            m_temporary_buffers,
            m_events: m_events.unwrap(),
            m_initial_event_name,
            m_runtime_version,
            m_instancing_mode,
            m_instancing_capacity,
            m_compilation_version,
            m_instancing_disabled_reason,
        })
    }
}
