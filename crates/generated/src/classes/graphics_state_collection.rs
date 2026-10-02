//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;
use crate::subclasses::render_pass_info::RenderPassInfo;
use crate::subclasses::render_state_info::RenderStateInfo;
use crate::subclasses::variant_info::VariantInfo;
use crate::subclasses::vertex_layout_info::VertexLayoutInfo;

#[derive(Debug, Clone)]
pub struct GraphicsStateCollection {
    pub m_name: String,
    pub m_version: i32,
    pub m_device_renderer: i32,
    pub m_runtime_platform: i32,
    pub m_quality_level_name: String,
    pub m_variant_info_map: Vec<(Hash128, VariantInfo)>,
    pub m_vertex_layout_info_map: Vec<(u64, VertexLayoutInfo)>,
    pub m_render_pass_info_map: Vec<(u64, RenderPassInfo)>,
    pub m_render_state_map: Vec<(u64, RenderStateInfo)>,
}

impl crate::Parse for GraphicsStateCollection {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_version = rdr.read_i32()?;
        let m_device_renderer = rdr.read_i32()?;
        let m_runtime_platform = rdr.read_i32()?;
        let m_quality_level_name = rdr.read_aligned_string()?;
        let m_variant_info_map = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = Hash128::parse(rdr, ver)?;
                        let b = VariantInfo::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_vertex_layout_info_map = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_u64()?;
                        let b = VertexLayoutInfo::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_render_pass_info_map = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_u64()?;
                        let b = RenderPassInfo::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_render_state_map = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_u64()?;
                        let b = RenderStateInfo::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(GraphicsStateCollection {
            m_name,
            m_version,
            m_device_renderer,
            m_runtime_platform,
            m_quality_level_name,
            m_variant_info_map,
            m_vertex_layout_info_map,
            m_render_pass_info_map,
            m_render_state_map,
        })
    }
}

impl crate::ToValue for GraphicsStateCollection {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Version".to_string(), self.m_version.to_value())),
            Some((
                "m_DeviceRenderer".to_string(),
                self.m_device_renderer.to_value(),
            )),
            Some((
                "m_RuntimePlatform".to_string(),
                self.m_runtime_platform.to_value(),
            )),
            Some((
                "m_QualityLevelName".to_string(),
                self.m_quality_level_name.to_value(),
            )),
            Some((
                "m_VariantInfoMap".to_string(),
                self.m_variant_info_map.to_value(),
            )),
            Some((
                "m_VertexLayoutInfoMap".to_string(),
                self.m_vertex_layout_info_map.to_value(),
            )),
            Some((
                "m_RenderPassInfoMap".to_string(),
                self.m_render_pass_info_map.to_value(),
            )),
            Some((
                "m_RenderStateMap".to_string(),
                self.m_render_state_map.to_value(),
            )),
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
                "m_Version" => (self.m_version).inspect_value(path, offset, limit),
                "m_DeviceRenderer" => (self.m_device_renderer).inspect_value(path, offset, limit),
                "m_RuntimePlatform" => (self.m_runtime_platform).inspect_value(path, offset, limit),
                "m_QualityLevelName" => {
                    (self.m_quality_level_name).inspect_value(path, offset, limit)
                }
                "m_VariantInfoMap" => (self.m_variant_info_map).inspect_value(path, offset, limit),
                "m_VertexLayoutInfoMap" => {
                    (self.m_vertex_layout_info_map).inspect_value(path, offset, limit)
                }
                "m_RenderPassInfoMap" => {
                    (self.m_render_pass_info_map).inspect_value(path, offset, limit)
                }
                "m_RenderStateMap" => (self.m_render_state_map).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_version).inspect_value(path, offset, limit))
                .map(|v| ("m_Version".to_string(), v)),
            ((self.m_device_renderer).inspect_value(path, offset, limit))
                .map(|v| ("m_DeviceRenderer".to_string(), v)),
            ((self.m_runtime_platform).inspect_value(path, offset, limit))
                .map(|v| ("m_RuntimePlatform".to_string(), v)),
            ((self.m_quality_level_name).inspect_value(path, offset, limit))
                .map(|v| ("m_QualityLevelName".to_string(), v)),
            ((self.m_variant_info_map).inspect_value(path, offset, limit))
                .map(|v| ("m_VariantInfoMap".to_string(), v)),
            ((self.m_vertex_layout_info_map).inspect_value(path, offset, limit))
                .map(|v| ("m_VertexLayoutInfoMap".to_string(), v)),
            ((self.m_render_pass_info_map).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderPassInfoMap".to_string(), v)),
            ((self.m_render_state_map).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderStateMap".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
