//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MuscleClipInfo {
    pub m_start_time: f32,
    pub m_stop_time: f32,
    pub m_orientation_offset_y: f32,
    pub m_level: f32,
    pub m_cycle_offset: f32,
    pub m_loop_blend: bool,
    pub m_loop_blend_orientation: bool,
    pub m_loop_blend_position_y: bool,
    pub m_loop_blend_position_xz: bool,
    pub m_keep_original_orientation: bool,
    pub m_keep_original_position_y: bool,
    pub m_keep_original_position_xz: bool,
    pub m_height_from_feet: bool,
    pub m_mirror: bool,
    pub m_keep_additional_bones_animation: bool,
}

impl crate::Parse for MuscleClipInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_start_time = rdr.read_f32()?;
        let m_stop_time = rdr.read_f32()?;
        let m_orientation_offset_y = rdr.read_f32()?;
        let m_level = rdr.read_f32()?;
        let m_cycle_offset = rdr.read_f32()?;
        let m_loop_blend = rdr.read_bool()?;
        let m_loop_blend_orientation = rdr.read_bool()?;
        let m_loop_blend_position_y = rdr.read_bool()?;
        let m_loop_blend_position_xz = rdr.read_bool()?;
        let m_keep_original_orientation = rdr.read_bool()?;
        let m_keep_original_position_y = rdr.read_bool()?;
        let m_keep_original_position_xz = rdr.read_bool()?;
        let m_height_from_feet = rdr.read_bool()?;
        let m_mirror = rdr.read_bool()?;
        let m_keep_additional_bones_animation = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(MuscleClipInfo {
            m_start_time,
            m_stop_time,
            m_orientation_offset_y,
            m_level,
            m_cycle_offset,
            m_loop_blend,
            m_loop_blend_orientation,
            m_loop_blend_position_y,
            m_loop_blend_position_xz,
            m_keep_original_orientation,
            m_keep_original_position_y,
            m_keep_original_position_xz,
            m_height_from_feet,
            m_mirror,
            m_keep_additional_bones_animation,
        })
    }
}
