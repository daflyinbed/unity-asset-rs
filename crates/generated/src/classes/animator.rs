//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Animator {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_avatar: PPtr,
    pub m_controller: PPtr,
    pub m_culling_mode: i32,
    pub m_apply_root_motion: bool,
    pub m_update_mode: Option<i32>,
    pub m_animate_physics: Option<bool>,
    pub m_has_transform_hierarchy: Option<bool>,
    pub m_linear_velocity_blending: Option<bool>,
    pub m_allow_constant_clip_sampling_optimization: Option<bool>,
    pub m_stabilize_feet: Option<bool>,
    pub m_keep_animator_controller_state_on_disable: Option<bool>,
    pub m_keep_animator_state_on_disable: Option<bool>,
    pub m_write_default_values_on_disable: Option<bool>,
}

impl crate::Parse for Animator {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_avatar = crate::PPtr::parse(r, ver)?;
        let m_controller = crate::PPtr::parse(r, ver)?;
        let m_culling_mode = r.read_i32()?;
        let m_apply_root_motion = r.read_bool()?;
        let m_update_mode = if *ver >= crate::UnityVersion::new(4, 5, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_animate_physics = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_has_transform_hierarchy = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_linear_velocity_blending = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_allow_constant_clip_sampling_optimization = if *ver >= crate::UnityVersion::new(4, 5, 3) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_stabilize_feet = if *ver >= crate::UnityVersion::new(2020, 3, 43) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_keep_animator_controller_state_on_disable = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_keep_animator_state_on_disable = if *ver >= crate::UnityVersion::new(2020, 3, 43) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_write_default_values_on_disable = if *ver >= crate::UnityVersion::new(2020, 3, 46) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(Animator {
            m_game_object,
            m_enabled,
            m_avatar,
            m_controller,
            m_culling_mode,
            m_apply_root_motion,
            m_update_mode,
            m_animate_physics,
            m_has_transform_hierarchy,
            m_linear_velocity_blending,
            m_allow_constant_clip_sampling_optimization,
            m_stabilize_feet,
            m_keep_animator_controller_state_on_disable,
            m_keep_animator_state_on_disable,
            m_write_default_values_on_disable,
        })
    }
}
