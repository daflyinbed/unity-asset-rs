//! 自动生成。请勿手动修改。

use crate::subclasses::annotation::Annotation;

#[derive(Debug, Clone)]
pub struct AnnotationManager {
    pub m_object_hide_flags: u32,
    pub m_current_preset_m_annotation_list: Vec<Annotation>,
    pub m_recently_changed: Vec<Annotation>,
    pub m_icon_size: f32,
}

impl crate::Parse for AnnotationManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_current_preset_m_annotation_list = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Annotation::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_recently_changed = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Annotation::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_icon_size = r.read_f32()?;

        Ok(AnnotationManager {
            m_object_hide_flags,
            m_current_preset_m_annotation_list,
            m_recently_changed,
            m_icon_size,
        })
    }
}
