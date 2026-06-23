//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::articulation_drive::ArticulationDrive;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ArticulationBody {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_instance: PPtr,
    pub m_prefab_asset: PPtr,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_mass: f32,
    pub m_parent_anchor_position: Vector3f,
    pub m_parent_anchor_rotation: Quaternionf,
    pub m_anchor_position: Vector3f,
    pub m_anchor_rotation: Quaternionf,
    pub m_compute_parent_anchor: bool,
    pub m_articulation_joint_type: i32,
    pub m_linear_x: i32,
    pub m_linear_y: i32,
    pub m_linear_z: i32,
    pub m_swing_y: i32,
    pub m_swing_z: i32,
    pub m_twist: i32,
    pub m_x_drive: ArticulationDrive,
    pub m_y_drive: ArticulationDrive,
    pub m_z_drive: ArticulationDrive,
    pub m_linear_damping: f32,
    pub m_angular_damping: f32,
    pub m_joint_friction: f32,
    pub m_immovable: bool,
}

impl crate::Parse for ArticulationBody {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_instance = crate::PPtr::parse(r, ver)?;
        let m_prefab_asset = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_mass = r.read_f32()?;
        let m_parent_anchor_position = Vector3f::parse(r, ver)?;
        let m_parent_anchor_rotation = Quaternionf::parse(r, ver)?;
        let m_anchor_position = Vector3f::parse(r, ver)?;
        let m_anchor_rotation = Quaternionf::parse(r, ver)?;
        let m_compute_parent_anchor = r.read_bool()?;
        r.align(4)?;
        let m_articulation_joint_type = r.read_i32()?;
        let m_linear_x = r.read_i32()?;
        let m_linear_y = r.read_i32()?;
        let m_linear_z = r.read_i32()?;
        let m_swing_y = r.read_i32()?;
        let m_swing_z = r.read_i32()?;
        let m_twist = r.read_i32()?;
        let m_x_drive = ArticulationDrive::parse(r, ver)?;
        let m_y_drive = ArticulationDrive::parse(r, ver)?;
        let m_z_drive = ArticulationDrive::parse(r, ver)?;
        let m_linear_damping = r.read_f32()?;
        let m_angular_damping = r.read_f32()?;
        let m_joint_friction = r.read_f32()?;
        let m_immovable = r.read_bool()?;

        Ok(ArticulationBody {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_instance,
            m_prefab_asset,
            m_game_object,
            m_enabled,
            m_mass,
            m_parent_anchor_position,
            m_parent_anchor_rotation,
            m_anchor_position,
            m_anchor_rotation,
            m_compute_parent_anchor,
            m_articulation_joint_type,
            m_linear_x,
            m_linear_y,
            m_linear_z,
            m_swing_y,
            m_swing_z,
            m_twist,
            m_x_drive,
            m_y_drive,
            m_z_drive,
            m_linear_damping,
            m_angular_damping,
            m_joint_friction,
            m_immovable,
        })
    }
}
