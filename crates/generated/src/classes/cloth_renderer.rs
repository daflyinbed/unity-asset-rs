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

impl crate::ToValue for ClothRendererMLightProbeAnchorVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_3(v) => (v).inspect_value(path, offset, limit),
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

impl crate::ToValue for ClothRendererMSortingOrderVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_5(v) => (v).inspect_value(path, offset, limit),
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
        rdr: &mut R,
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
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_bool()?);
        m_cast_shadows = Some(rdr.read_bool()?);
        m_receive_shadows = Some(rdr.read_bool()?);
        m_lightmap_index = Some(rdr.read_u8()?);
        m_lightmap_tiling_offset = Some(Vector4f::parse(rdr, ver)?);
        m_materials = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        m_subset_indices = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        m_static_batch_root = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(ClothRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer_id = Some(rdr.read_u32()?);
            m_sorting_order = Some(ClothRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(ClothRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
            m_sorting_layer = Some(rdr.read_i16()?);
            m_sorting_order = Some(ClothRendererMSortingOrderVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_use_light_probes = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_light_probe_anchor = Some(ClothRendererMLightProbeAnchorVariant::parse(rdr, ver)?);
        } else {
        }
        m_pause_when_not_visible = Some(rdr.read_bool()?);

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

impl crate::ToValue for ClothRenderer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_CastShadows".to_string(), self.m_cast_shadows.to_value())),
            Some((
                "m_ReceiveShadows".to_string(),
                self.m_receive_shadows.to_value(),
            )),
            Some((
                "m_LightmapIndex".to_string(),
                self.m_lightmap_index.to_value(),
            )),
            Some((
                "m_LightmapTilingOffset".to_string(),
                self.m_lightmap_tiling_offset.to_value(),
            )),
            Some(("m_Materials".to_string(), self.m_materials.to_value())),
            Some((
                "m_SubsetIndices".to_string(),
                self.m_subset_indices.to_value(),
            )),
            Some((
                "m_StaticBatchRoot".to_string(),
                self.m_static_batch_root.to_value(),
            )),
            Some((
                "m_PauseWhenNotVisible".to_string(),
                self.m_pause_when_not_visible.to_value(),
            )),
            crate::opt_field("m_UseLightProbes", &self.m_use_light_probes),
            crate::opt_field("m_LightProbeAnchor", &self.m_light_probe_anchor),
            crate::opt_field("m_SortingLayer", &self.m_sorting_layer),
            crate::opt_field("m_SortingLayerID", &self.m_sorting_layer_id),
            crate::opt_field("m_SortingOrder", &self.m_sorting_order),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_CastShadows" => (self.m_cast_shadows).inspect_value(path, offset, limit),
                "m_ReceiveShadows" => (self.m_receive_shadows).inspect_value(path, offset, limit),
                "m_LightmapIndex" => (self.m_lightmap_index).inspect_value(path, offset, limit),
                "m_LightmapTilingOffset" => {
                    (self.m_lightmap_tiling_offset).inspect_value(path, offset, limit)
                }
                "m_Materials" => (self.m_materials).inspect_value(path, offset, limit),
                "m_SubsetIndices" => (self.m_subset_indices).inspect_value(path, offset, limit),
                "m_StaticBatchRoot" => {
                    (self.m_static_batch_root).inspect_value(path, offset, limit)
                }
                "m_PauseWhenNotVisible" => {
                    (self.m_pause_when_not_visible).inspect_value(path, offset, limit)
                }
                "m_UseLightProbes" => (self.m_use_light_probes).inspect_value(path, offset, limit),
                "m_LightProbeAnchor" => {
                    (self.m_light_probe_anchor).inspect_value(path, offset, limit)
                }
                "m_SortingLayer" => (self.m_sorting_layer).inspect_value(path, offset, limit),
                "m_SortingLayerID" => (self.m_sorting_layer_id).inspect_value(path, offset, limit),
                "m_SortingOrder" => (self.m_sorting_order).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_cast_shadows).inspect_value(path, offset, limit))
                .map(|v| ("m_CastShadows".to_string(), v)),
            ((self.m_receive_shadows).inspect_value(path, offset, limit))
                .map(|v| ("m_ReceiveShadows".to_string(), v)),
            ((self.m_lightmap_index).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapIndex".to_string(), v)),
            ((self.m_lightmap_tiling_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_LightmapTilingOffset".to_string(), v)),
            ((self.m_materials).inspect_value(path, offset, limit))
                .map(|v| ("m_Materials".to_string(), v)),
            ((self.m_subset_indices).inspect_value(path, offset, limit))
                .map(|v| ("m_SubsetIndices".to_string(), v)),
            ((self.m_static_batch_root).inspect_value(path, offset, limit))
                .map(|v| ("m_StaticBatchRoot".to_string(), v)),
            ((self.m_pause_when_not_visible).inspect_value(path, offset, limit))
                .map(|v| ("m_PauseWhenNotVisible".to_string(), v)),
            ((self.m_use_light_probes).inspect_value(path, offset, limit))
                .map(|v| ("m_UseLightProbes".to_string(), v)),
            ((self.m_light_probe_anchor).inspect_value(path, offset, limit))
                .map(|v| ("m_LightProbeAnchor".to_string(), v)),
            ((self.m_sorting_layer).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayer".to_string(), v)),
            ((self.m_sorting_layer_id).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayerID".to_string(), v)),
            ((self.m_sorting_order).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingOrder".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
