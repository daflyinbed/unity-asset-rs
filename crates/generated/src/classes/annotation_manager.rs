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
