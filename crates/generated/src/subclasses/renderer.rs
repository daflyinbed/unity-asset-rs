//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector4f::Vector4f;


/// 版本枚举：RendererMLightProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RendererMLightProbeAnchorVariant {
    Default(PPtr),
    V4_3(PPtr),
}

impl crate::Parse for RendererMLightProbeAnchorVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::V4_3(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：RendererMSortingOrderVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RendererMSortingOrderVariant {
    Default(i16),
    V4_5(i16),
}

impl crate::Parse for RendererMSortingOrderVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = rdr.read_i16()?;
            rdr.align(4)?;
            Ok(Self::V4_5(v))
        } else {
            let v = rdr.read_i16()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Renderer {
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: bool,
    pub m_receive_shadows: bool,
    pub m_lightmap_index: u8,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_materials: Vec<PPtr>,
    pub m_subset_indices: Vec<u32>,
    pub m_static_batch_root: PPtr,
    pub m_use_light_probes: Option<bool>,
    pub m_light_probe_anchor: Option<RendererMLightProbeAnchorVariant>,
    pub m_sorting_layer: Option<i16>,
    pub m_sorting_layer_id: Option<u32>,
    pub m_sorting_order: Option<RendererMSortingOrderVariant>,
}

impl crate::Parse for Renderer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_bool()?;
        let m_cast_shadows = rdr.read_bool()?;
        let m_receive_shadows = rdr.read_bool()?;
        let m_lightmap_index = rdr.read_u8()?;
        let m_lightmap_tiling_offset = Vector4f::parse(rdr, ver)?;
        let m_materials = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_subset_indices = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_static_batch_root = crate::PPtr::parse(rdr, ver)?;
        let m_use_light_probes = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_light_probe_anchor = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(RendererMLightProbeAnchorVariant::parse(rdr, ver)?)
        } else {
            None
        };
        let m_sorting_layer = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(rdr.read_i16()?)
        } else {
            None
        };
        let m_sorting_layer_id = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(rdr.read_u32()?)
        } else {
            None
        };
        let m_sorting_order = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(RendererMSortingOrderVariant::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(Renderer {
            m_game_object,
            m_enabled,
            m_cast_shadows,
            m_receive_shadows,
            m_lightmap_index,
            m_lightmap_tiling_offset,
            m_materials,
            m_subset_indices,
            m_static_batch_root,
            m_use_light_probes,
            m_light_probe_anchor,
            m_sorting_layer,
            m_sorting_layer_id,
            m_sorting_order,
        })
    }
}
