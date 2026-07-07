//! 自动生成。请勿手动修改。

use crate::subclasses::annotation::Annotation;

#[derive(Debug, Clone)]
pub struct AnnotationManager {
    pub m_current_preset_m_annotation_list: Vec<Annotation>,
    pub m_recently_changed: Vec<Annotation>,
    pub m_icon_size: Option<f32>,
    pub m_world_icon_size: Option<f32>,
    pub m_use3d_gizmos: Option<bool>,
    pub m_fade_gizmo_size: Option<f32>,
    pub m_show_grid: Option<bool>,
    pub m_fade_gizmos: Option<bool>,
    pub m_show_selection_outline: Option<bool>,
    pub m_show_selection_wire: Option<bool>,
    pub m_show_lod_labels: Option<bool>,
}

impl crate::Parse for AnnotationManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_current_preset_m_annotation_list = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Annotation::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_recently_changed = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Annotation::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_icon_size = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_world_icon_size = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_use3d_gizmos = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_fade_gizmo_size = if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_show_grid = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_fade_gizmos = if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_show_selection_outline = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_show_selection_wire = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_show_lod_labels = if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(AnnotationManager {
            m_current_preset_m_annotation_list,
            m_recently_changed,
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
