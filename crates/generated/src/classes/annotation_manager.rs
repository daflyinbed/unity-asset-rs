//! 自动生成。请勿手动修改。

use crate::subclasses::annotation::Annotation;

/// 版本枚举：AnnotationManagerMCurrentPresetMAnnotationListVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnnotationManagerMCurrentPresetMAnnotationListVariant {
    Default(Vec<Annotation>),
    V2017_1(Vec<Annotation>),
}

impl crate::Parse for AnnotationManagerMCurrentPresetMAnnotationListVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Annotation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Annotation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnnotationManagerMCurrentPresetMAnnotationListVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AnnotationManagerMRecentlyChangedVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnnotationManagerMRecentlyChangedVariant {
    Default(Vec<Annotation>),
    V2017_1(Vec<Annotation>),
}

impl crate::Parse for AnnotationManagerMRecentlyChangedVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Annotation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Annotation::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnnotationManagerMRecentlyChangedVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AnnotationManagerMUse3dGizmosVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnnotationManagerMUse3dGizmosVariant {
    Default(bool),
    V5_5(bool),
}

impl crate::Parse for AnnotationManagerMUse3dGizmosVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnnotationManagerMUse3dGizmosVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AnnotationManagerMShowGridVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnnotationManagerMShowGridVariant {
    Default(bool),
    V5_5(bool),
}

impl crate::Parse for AnnotationManagerMShowGridVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnnotationManagerMShowGridVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：AnnotationManagerMShowSelectionWireVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnnotationManagerMShowSelectionWireVariant {
    Default(bool),
    V6000_4(bool),
}

impl crate::Parse for AnnotationManagerMShowSelectionWireVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_4(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnnotationManagerMShowSelectionWireVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_4(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnnotationManager {
    pub m_current_preset_m_annotation_list: AnnotationManagerMCurrentPresetMAnnotationListVariant,
    pub m_recently_changed: AnnotationManagerMRecentlyChangedVariant,
    pub m_icon_size: Option<f32>,
    pub m_world_icon_size: Option<f32>,
    pub m_use3d_gizmos: Option<AnnotationManagerMUse3dGizmosVariant>,
    pub m_fade_gizmo_size: Option<f32>,
    pub m_show_grid: Option<AnnotationManagerMShowGridVariant>,
    pub m_fade_gizmos: Option<bool>,
    pub m_show_selection_outline: Option<bool>,
    pub m_show_selection_wire: Option<AnnotationManagerMShowSelectionWireVariant>,
    pub m_show_lod_labels: Option<bool>,
}

impl crate::Parse for AnnotationManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_current_preset_m_annotation_list = None;
        let mut m_recently_changed = None;
        let mut m_icon_size = None;
        let mut m_world_icon_size = None;
        let mut m_use3d_gizmos = None;
        let mut m_fade_gizmo_size = None;
        let mut m_show_grid = None;
        let mut m_fade_gizmos = None;
        let mut m_show_selection_outline = None;
        let mut m_show_selection_wire = None;
        let mut m_show_lod_labels = None;
        m_current_preset_m_annotation_list =
            Some(AnnotationManagerMCurrentPresetMAnnotationListVariant::parse(rdr, ver)?);
        m_recently_changed = Some(AnnotationManagerMRecentlyChangedVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_world_icon_size = Some(rdr.read_f32()?);
            m_fade_gizmo_size = Some(rdr.read_f32()?);
            m_fade_gizmos = Some(rdr.read_bool()?);
            m_use3d_gizmos = Some(AnnotationManagerMUse3dGizmosVariant::parse(rdr, ver)?);
            m_show_grid = Some(AnnotationManagerMShowGridVariant::parse(rdr, ver)?);
            m_show_selection_outline = Some(rdr.read_bool()?);
            m_show_selection_wire =
                Some(AnnotationManagerMShowSelectionWireVariant::parse(rdr, ver)?);
            m_show_lod_labels = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_world_icon_size = Some(rdr.read_f32()?);
            m_fade_gizmo_size = Some(rdr.read_f32()?);
            m_fade_gizmos = Some(rdr.read_bool()?);
            m_use3d_gizmos = Some(AnnotationManagerMUse3dGizmosVariant::parse(rdr, ver)?);
            m_show_grid = Some(AnnotationManagerMShowGridVariant::parse(rdr, ver)?);
            m_show_selection_outline = Some(rdr.read_bool()?);
            m_show_selection_wire =
                Some(AnnotationManagerMShowSelectionWireVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_world_icon_size = Some(rdr.read_f32()?);
            m_use3d_gizmos = Some(AnnotationManagerMUse3dGizmosVariant::parse(rdr, ver)?);
            m_show_grid = Some(AnnotationManagerMShowGridVariant::parse(rdr, ver)?);
            m_show_selection_outline = Some(rdr.read_bool()?);
            m_show_selection_wire =
                Some(AnnotationManagerMShowSelectionWireVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_world_icon_size = Some(rdr.read_f32()?);
            m_use3d_gizmos = Some(AnnotationManagerMUse3dGizmosVariant::parse(rdr, ver)?);
            m_show_grid = Some(AnnotationManagerMShowGridVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_world_icon_size = Some(rdr.read_f32()?);
            m_use3d_gizmos = Some(AnnotationManagerMUse3dGizmosVariant::parse(rdr, ver)?);
        } else {
            m_icon_size = Some(rdr.read_f32()?);
        }

        Ok(AnnotationManager {
            m_current_preset_m_annotation_list: m_current_preset_m_annotation_list.unwrap(),
            m_recently_changed: m_recently_changed.unwrap(),
            m_icon_size,
            m_world_icon_size,
            m_use3d_gizmos,
            m_fade_gizmo_size,
            m_show_grid,
            m_fade_gizmos,
            m_show_selection_outline,
            m_show_selection_wire,
            m_show_lod_labels,
        })
    }
}

impl crate::ToValue for AnnotationManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_CurrentPreset_m_AnnotationList".to_string(),
                self.m_current_preset_m_annotation_list.to_value(),
            )),
            Some((
                "m_RecentlyChanged".to_string(),
                self.m_recently_changed.to_value(),
            )),
            crate::opt_field("m_IconSize", &self.m_icon_size),
            crate::opt_field("m_WorldIconSize", &self.m_world_icon_size),
            crate::opt_field("m_Use3dGizmos", &self.m_use3d_gizmos),
            crate::opt_field("m_FadeGizmoSize", &self.m_fade_gizmo_size),
            crate::opt_field("m_ShowGrid", &self.m_show_grid),
            crate::opt_field("m_FadeGizmos", &self.m_fade_gizmos),
            crate::opt_field("m_ShowSelectionOutline", &self.m_show_selection_outline),
            crate::opt_field("m_ShowSelectionWire", &self.m_show_selection_wire),
            crate::opt_field("m_ShowLODLabels", &self.m_show_lod_labels),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_CurrentPreset_m_AnnotationList" => {
                    (self.m_current_preset_m_annotation_list).inspect_value(path, offset, limit)
                }
                "m_RecentlyChanged" => (self.m_recently_changed).inspect_value(path, offset, limit),
                "m_IconSize" => (self.m_icon_size).inspect_value(path, offset, limit),
                "m_WorldIconSize" => (self.m_world_icon_size).inspect_value(path, offset, limit),
                "m_Use3dGizmos" => (self.m_use3d_gizmos).inspect_value(path, offset, limit),
                "m_FadeGizmoSize" => (self.m_fade_gizmo_size).inspect_value(path, offset, limit),
                "m_ShowGrid" => (self.m_show_grid).inspect_value(path, offset, limit),
                "m_FadeGizmos" => (self.m_fade_gizmos).inspect_value(path, offset, limit),
                "m_ShowSelectionOutline" => {
                    (self.m_show_selection_outline).inspect_value(path, offset, limit)
                }
                "m_ShowSelectionWire" => {
                    (self.m_show_selection_wire).inspect_value(path, offset, limit)
                }
                "m_ShowLODLabels" => (self.m_show_lod_labels).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_current_preset_m_annotation_list).inspect_value(path, offset, limit))
                .map(|v| ("m_CurrentPreset_m_AnnotationList".to_string(), v)),
            ((self.m_recently_changed).inspect_value(path, offset, limit))
                .map(|v| ("m_RecentlyChanged".to_string(), v)),
            ((self.m_icon_size).inspect_value(path, offset, limit))
                .map(|v| ("m_IconSize".to_string(), v)),
            ((self.m_world_icon_size).inspect_value(path, offset, limit))
                .map(|v| ("m_WorldIconSize".to_string(), v)),
            ((self.m_use3d_gizmos).inspect_value(path, offset, limit))
                .map(|v| ("m_Use3dGizmos".to_string(), v)),
            ((self.m_fade_gizmo_size).inspect_value(path, offset, limit))
                .map(|v| ("m_FadeGizmoSize".to_string(), v)),
            ((self.m_show_grid).inspect_value(path, offset, limit))
                .map(|v| ("m_ShowGrid".to_string(), v)),
            ((self.m_fade_gizmos).inspect_value(path, offset, limit))
                .map(|v| ("m_FadeGizmos".to_string(), v)),
            ((self.m_show_selection_outline).inspect_value(path, offset, limit))
                .map(|v| ("m_ShowSelectionOutline".to_string(), v)),
            ((self.m_show_selection_wire).inspect_value(path, offset, limit))
                .map(|v| ("m_ShowSelectionWire".to_string(), v)),
            ((self.m_show_lod_labels).inspect_value(path, offset, limit))
                .map(|v| ("m_ShowLODLabels".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
