//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct MeshCollider {
    pub m_game_object: PPtr,
    pub m_material: PPtr,
    pub m_is_trigger: bool,
    pub m_include_layers: Option<BitField>,
    pub m_enabled: bool,
    pub m_exclude_layers: Option<BitField>,
    pub m_smooth_sphere_collisions: Option<bool>,
    pub m_convex: bool,
    pub m_layer_override_priority: Option<i32>,
    pub m_mesh: PPtr,
    pub m_inflate_mesh: Option<bool>,
    pub m_cooking_options: Option<i32>,
    pub m_skin_width: Option<f32>,
    pub m_provides_contacts: Option<bool>,
}

impl crate::Parse for MeshCollider {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_material = crate::PPtr::parse(r, ver)?;
        let m_is_trigger = r.read_bool()?;
        let m_include_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_enabled = r.read_bool()?;
        r.align(4)?;
        let m_exclude_layers = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(BitField::parse(r, ver)?)
        } else {
            None
        };
        let m_smooth_sphere_collisions = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_convex = r.read_bool()?;
        r.align(4)?;
        let m_layer_override_priority = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_mesh = crate::PPtr::parse(r, ver)?;
        let m_inflate_mesh = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_cooking_options = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = r.read_i32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_skin_width = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_provides_contacts = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(MeshCollider {
            m_game_object,
            m_material,
            m_is_trigger,
            m_include_layers,
            m_enabled,
            m_exclude_layers,
            m_smooth_sphere_collisions,
            m_convex,
            m_layer_override_priority,
            m_mesh,
            m_inflate_mesh,
            m_cooking_options,
            m_skin_width,
            m_provides_contacts,
        })
    }
}
