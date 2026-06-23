//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct NavMeshBuildSettings {
    pub agent_radius: f32,
    pub agent_height: f32,
    pub agent_slope: f32,
    pub agent_climb: f32,
    pub ledge_drop_height: f32,
    pub max_jump_across_distance: f32,
    pub accurate_placement: bool,
    pub min_region_area: f32,
    pub width_inaccuracy: f32,
    pub height_inaccuracy: f32,
}

impl crate::Parse for NavMeshBuildSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let agent_radius = r.read_f32()?;
        let agent_height = r.read_f32()?;
        let agent_slope = r.read_f32()?;
        let agent_climb = r.read_f32()?;
        let ledge_drop_height = r.read_f32()?;
        let max_jump_across_distance = r.read_f32()?;
        let accurate_placement = r.read_bool()?;
        r.align(4)?;
        let min_region_area = r.read_f32()?;
        let width_inaccuracy = r.read_f32()?;
        let height_inaccuracy = r.read_f32()?;

        Ok(NavMeshBuildSettings {
            agent_radius,
            agent_height,
            agent_slope,
            agent_climb,
            ledge_drop_height,
            max_jump_across_distance,
            accurate_placement,
            min_region_area,
            width_inaccuracy,
            height_inaccuracy,
        })
    }
}
