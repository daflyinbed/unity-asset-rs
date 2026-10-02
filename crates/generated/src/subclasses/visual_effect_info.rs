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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxExposedMapping::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2023_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxMapping::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VisualEffectInfoMExposedExpressionsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct VisualEffectInfo {
    pub m_expressions: VfxExpressionContainer,
    pub m_renderer_settings: VfxRendererSettings,
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
            m_renderer_settings = Some(VfxRendererSettings::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
            m_pre_warm_delta_time = Some(rdr.read_f32()?);
            m_pre_warm_step_count = Some(rdr.read_u32()?);
            m_initial_event_name = Some(rdr.read_aligned_string()?);
            m_instancing_mode = Some(rdr.read_i32()?);
            m_instancing_capacity = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_renderer_settings = Some(VfxRendererSettings::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
            m_pre_warm_delta_time = Some(rdr.read_f32()?);
            m_pre_warm_step_count = Some(rdr.read_u32()?);
            m_initial_event_name = Some(rdr.read_aligned_string()?);
        } else {
        }
        m_expressions = Some(VfxExpressionContainer::parse(rdr, ver)?);
        m_property_sheet = Some(VfxPropertySheetSerializedBase::parse(rdr, ver)?);
        m_exposed_expressions = Some(VisualEffectInfoMExposedExpressionsVariant::parse(rdr, ver)?);
        m_buffers = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxgpuBufferDesc::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_temporary_buffers = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| VfxTemporaryGpuBufferDesc::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }
        m_cpu_buffers = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxcpuBufferDesc::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_events = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxEventDesc::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
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
            m_renderer_settings = Some(VfxRendererSettings::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
            m_pre_warm_delta_time = Some(rdr.read_f32()?);
            m_pre_warm_step_count = Some(rdr.read_u32()?);
            m_initial_event_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            m_runtime_version = Some(rdr.read_u32()?);
            m_renderer_settings = Some(VfxRendererSettings::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
            m_pre_warm_delta_time = Some(rdr.read_f32()?);
            m_pre_warm_step_count = Some(rdr.read_u32()?);
            m_initial_event_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(2019, 2, 17) {
            m_runtime_version = Some(rdr.read_u32()?);
            m_renderer_settings = Some(VfxRendererSettings::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
            m_pre_warm_delta_time = Some(rdr.read_f32()?);
            m_pre_warm_step_count = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_renderer_settings = Some(VfxRendererSettings::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
            m_pre_warm_delta_time = Some(rdr.read_f32()?);
            m_pre_warm_step_count = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 4, 15) {
            m_runtime_version = Some(rdr.read_u32()?);
            m_renderer_settings = Some(VfxRendererSettings::parse(rdr, ver)?);
            m_culling_flags = Some(rdr.read_i32()?);
            m_update_mode = Some(rdr.read_i32()?);
        } else {
            m_renderer_settings = Some(VfxRendererSettings::parse(rdr, ver)?);
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

impl crate::ToValue for VisualEffectInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Expressions".to_string(), self.m_expressions.to_value())),
            Some((
                "m_RendererSettings".to_string(),
                self.m_renderer_settings.to_value(),
            )),
            Some((
                "m_PropertySheet".to_string(),
                self.m_property_sheet.to_value(),
            )),
            Some((
                "m_CullingFlags".to_string(),
                self.m_culling_flags.to_value(),
            )),
            Some((
                "m_ExposedExpressions".to_string(),
                self.m_exposed_expressions.to_value(),
            )),
            Some(("m_UpdateMode".to_string(), self.m_update_mode.to_value())),
            Some(("m_Buffers".to_string(), self.m_buffers.to_value())),
            crate::opt_field("m_PreWarmDeltaTime", &self.m_pre_warm_delta_time),
            Some(("m_CPUBuffers".to_string(), self.m_cpu_buffers.to_value())),
            crate::opt_field("m_PreWarmStepCount", &self.m_pre_warm_step_count),
            crate::opt_field("m_TemporaryBuffers", &self.m_temporary_buffers),
            Some(("m_Events".to_string(), self.m_events.to_value())),
            crate::opt_field("m_InitialEventName", &self.m_initial_event_name),
            crate::opt_field("m_RuntimeVersion", &self.m_runtime_version),
            crate::opt_field("m_InstancingMode", &self.m_instancing_mode),
            crate::opt_field("m_InstancingCapacity", &self.m_instancing_capacity),
            crate::opt_field("m_CompilationVersion", &self.m_compilation_version),
            crate::opt_field(
                "m_InstancingDisabledReason",
                &self.m_instancing_disabled_reason,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Expressions" => (self.m_expressions).inspect_value(path, offset, limit),
                "m_RendererSettings" => {
                    (self.m_renderer_settings).inspect_value(path, offset, limit)
                }
                "m_PropertySheet" => (self.m_property_sheet).inspect_value(path, offset, limit),
                "m_CullingFlags" => (self.m_culling_flags).inspect_value(path, offset, limit),
                "m_ExposedExpressions" => {
                    (self.m_exposed_expressions).inspect_value(path, offset, limit)
                }
                "m_UpdateMode" => (self.m_update_mode).inspect_value(path, offset, limit),
                "m_Buffers" => (self.m_buffers).inspect_value(path, offset, limit),
                "m_PreWarmDeltaTime" => {
                    (self.m_pre_warm_delta_time).inspect_value(path, offset, limit)
                }
                "m_CPUBuffers" => (self.m_cpu_buffers).inspect_value(path, offset, limit),
                "m_PreWarmStepCount" => {
                    (self.m_pre_warm_step_count).inspect_value(path, offset, limit)
                }
                "m_TemporaryBuffers" => {
                    (self.m_temporary_buffers).inspect_value(path, offset, limit)
                }
                "m_Events" => (self.m_events).inspect_value(path, offset, limit),
                "m_InitialEventName" => {
                    (self.m_initial_event_name).inspect_value(path, offset, limit)
                }
                "m_RuntimeVersion" => (self.m_runtime_version).inspect_value(path, offset, limit),
                "m_InstancingMode" => (self.m_instancing_mode).inspect_value(path, offset, limit),
                "m_InstancingCapacity" => {
                    (self.m_instancing_capacity).inspect_value(path, offset, limit)
                }
                "m_CompilationVersion" => {
                    (self.m_compilation_version).inspect_value(path, offset, limit)
                }
                "m_InstancingDisabledReason" => {
                    (self.m_instancing_disabled_reason).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_expressions).inspect_value(path, offset, limit))
                .map(|v| ("m_Expressions".to_string(), v)),
            ((self.m_renderer_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_RendererSettings".to_string(), v)),
            ((self.m_property_sheet).inspect_value(path, offset, limit))
                .map(|v| ("m_PropertySheet".to_string(), v)),
            ((self.m_culling_flags).inspect_value(path, offset, limit))
                .map(|v| ("m_CullingFlags".to_string(), v)),
            ((self.m_exposed_expressions).inspect_value(path, offset, limit))
                .map(|v| ("m_ExposedExpressions".to_string(), v)),
            ((self.m_update_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_UpdateMode".to_string(), v)),
            ((self.m_buffers).inspect_value(path, offset, limit))
                .map(|v| ("m_Buffers".to_string(), v)),
            ((self.m_pre_warm_delta_time).inspect_value(path, offset, limit))
                .map(|v| ("m_PreWarmDeltaTime".to_string(), v)),
            ((self.m_cpu_buffers).inspect_value(path, offset, limit))
                .map(|v| ("m_CPUBuffers".to_string(), v)),
            ((self.m_pre_warm_step_count).inspect_value(path, offset, limit))
                .map(|v| ("m_PreWarmStepCount".to_string(), v)),
            ((self.m_temporary_buffers).inspect_value(path, offset, limit))
                .map(|v| ("m_TemporaryBuffers".to_string(), v)),
            ((self.m_events).inspect_value(path, offset, limit))
                .map(|v| ("m_Events".to_string(), v)),
            ((self.m_initial_event_name).inspect_value(path, offset, limit))
                .map(|v| ("m_InitialEventName".to_string(), v)),
            ((self.m_runtime_version).inspect_value(path, offset, limit))
                .map(|v| ("m_RuntimeVersion".to_string(), v)),
            ((self.m_instancing_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_InstancingMode".to_string(), v)),
            ((self.m_instancing_capacity).inspect_value(path, offset, limit))
                .map(|v| ("m_InstancingCapacity".to_string(), v)),
            ((self.m_compilation_version).inspect_value(path, offset, limit))
                .map(|v| ("m_CompilationVersion".to_string(), v)),
            ((self.m_instancing_disabled_reason).inspect_value(path, offset, limit))
                .map(|v| ("m_InstancingDisabledReason".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
