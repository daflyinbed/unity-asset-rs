//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector4f::Vector4f;


/// 版本枚举：ClothRendererMLightProbeAnchorVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClothRendererMLightProbeAnchorVariant {
    Default(PPtr),
    V4_3(PPtr),
}

impl crate::Parse for ClothRendererMLightProbeAnchorVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = crate::PPtr::parse(r, ver)?;
            r.align(4)?;
            Ok(Self::V4_3(v))
        } else {
            let v = crate::PPtr::parse(r, ver)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：ClothRendererMSortingOrderVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClothRendererMSortingOrderVariant {
    Default(i16),
    V4_5(i16),
}

impl crate::Parse for ClothRendererMSortingOrderVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            let v = r.read_i16()?;
            r.align(4)?;
            Ok(Self::V4_5(v))
        } else {
            let v = r.read_i16()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClothRenderer {
    pub m_game_object: PPtr,
    pub m_enabled: bool,
    pub m_cast_shadows: bool,
    pub m_receive_shadows: bool,
    pub m_lightmap_index: u8,
    pub m_lightmap_tiling_offset: Vector4f,
    pub m_materials: Vec<PPtr>,
    pub m_subset_indices: Vec<u32>,
    pub m_static_batch_root: PPtr,
    pub m_pause_when_not_visible: bool,
    pub m_use_light_probes: Option<bool>,
    pub m_light_probe_anchor: Option<ClothRendererMLightProbeAnchorVariant>,
    pub m_sorting_layer: Option<i16>,
    pub m_sorting_layer_id: Option<u32>,
    pub m_sorting_order: Option<ClothRendererMSortingOrderVariant>,
}

impl crate::Parse for ClothRenderer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_cast_shadows = None;
        let mut m_receive_shadows = None;
        let mut m_lightmap_index = None;
        let mut m_lightmap_tiling_offset = None;
        let mut m_materials = None;
        let mut m_subset_indices = None;
        let mut m_static_batch_root = None;
        let mut m_pause_when_not_visible = None;
        let mut m_use_light_probes = None;
        let mut m_light_probe_anchor = None;
        let mut m_sorting_layer = None;
        let mut m_sorting_layer_id = None;
        let mut m_sorting_order = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_bool()?);
            m_cast_shadows = Some(r.read_bool()?);
            m_receive_shadows = Some(r.read_bool()?);
            m_lightmap_index = Some(r.read_u8()?);
            m_lightmap_tiling_offset = Some(Vector4f::parse(r, ver)?);
            m_materials = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            m_subset_indices = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
            m_static_batch_root = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_use_light_probes = Some(r.read_bool()?);
            r.align(4)?;
            m_light_probe_anchor = Some(ClothRendererMLightProbeAnchorVariant::parse(r, ver)?);
            m_sorting_layer_id = Some(r.read_u32()?);
            m_sorting_order = Some(ClothRendererMSortingOrderVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_use_light_probes = Some(r.read_bool()?);
            r.align(4)?;
            m_light_probe_anchor = Some(ClothRendererMLightProbeAnchorVariant::parse(r, ver)?);
            m_sorting_layer = Some(r.read_i16()?);
            m_sorting_order = Some(ClothRendererMSortingOrderVariant::parse(r, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_use_light_probes = Some(r.read_bool()?);
            r.align(4)?;
            m_light_probe_anchor = Some(ClothRendererMLightProbeAnchorVariant::parse(r, ver)?);
        } else {
        }
            m_pause_when_not_visible = Some(r.read_bool()?);

        Ok(ClothRenderer {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_cast_shadows: m_cast_shadows.unwrap(),
            m_receive_shadows: m_receive_shadows.unwrap(),
            m_lightmap_index: m_lightmap_index.unwrap(),
            m_lightmap_tiling_offset: m_lightmap_tiling_offset.unwrap(),
            m_materials: m_materials.unwrap(),
            m_subset_indices: m_subset_indices.unwrap(),
            m_static_batch_root: m_static_batch_root.unwrap(),
            m_pause_when_not_visible: m_pause_when_not_visible.unwrap(),
            m_use_light_probes,
            m_light_probe_anchor,
            m_sorting_layer,
            m_sorting_layer_id,
            m_sorting_order,
        })
    }
}
