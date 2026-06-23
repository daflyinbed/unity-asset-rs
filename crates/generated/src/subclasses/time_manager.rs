//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TimeManager {
    pub m_object_hide_flags: u32,
    pub fixed_timestep: f32,
    pub maximum_allowed_timestep: f32,
    pub m_time_scale: f32,
}

impl crate::Parse for TimeManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let fixed_timestep = r.read_f32()?;
        let maximum_allowed_timestep = r.read_f32()?;
        let m_time_scale = r.read_f32()?;

        Ok(TimeManager {
            m_object_hide_flags,
            fixed_timestep,
            maximum_allowed_timestep,
            m_time_scale,
        })
    }
}
