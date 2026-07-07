//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::lod::Lod;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct LodGroup {
    pub m_game_object: PPtr,
    pub m_local_reference_point: Vector3f,
    pub m_size: f32,
    pub m_screen_relative_transition_height: Option<f32>,
    pub m_lo_ds: Vec<Lod>,
    pub m_fade_mode: Option<i32>,
    pub m_enabled: bool,
    pub m_animate_cross_fading: Option<bool>,
    pub m_last_lod_is_billboard: Option<bool>,
    pub m_global_illumination_lod: Option<i32>,
}

impl crate::Parse for LodGroup {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_local_reference_point = Vector3f::parse(r, ver)?;
        let m_size = r.read_f32()?;
        let m_screen_relative_transition_height = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_lo_ds = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Lod::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_fade_mode = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_enabled = r.read_bool()?;
        r.align(4)?;
        let m_animate_cross_fading = if *ver >= crate::UnityVersion::new(5, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_last_lod_is_billboard = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_global_illumination_lod = if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };

        Ok(LodGroup {
            m_game_object,
            m_local_reference_point,
            m_size,
            m_screen_relative_transition_height,
            m_lo_ds,
            m_fade_mode,
            m_enabled,
            m_animate_cross_fading,
            m_last_lod_is_billboard,
            m_global_illumination_lod,
        })
    }
}
