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
