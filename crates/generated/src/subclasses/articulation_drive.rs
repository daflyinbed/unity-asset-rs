//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ArticulationDrive {
    pub lower_limit: f32,
    pub upper_limit: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub force_limit: f32,
    pub target: f32,
    pub target_velocity: f32,
    pub drive_type: Option<i32>,
}

impl crate::Parse for ArticulationDrive {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let lower_limit = rdr.read_f32()?;
        let upper_limit = rdr.read_f32()?;
        let stiffness = rdr.read_f32()?;
        let damping = rdr.read_f32()?;
        let force_limit = rdr.read_f32()?;
        let target = rdr.read_f32()?;
        let target_velocity = rdr.read_f32()?;
        let drive_type = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(ArticulationDrive {
            lower_limit,
            upper_limit,
            stiffness,
            damping,
            force_limit,
            target,
            target_velocity,
            drive_type,
        })
    }
}
