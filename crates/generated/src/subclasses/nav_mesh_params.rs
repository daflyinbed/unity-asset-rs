//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct NavMeshParams {
    pub tile_size: f32,
    pub walkable_height: f32,
    pub walkable_radius: f32,
    pub walkable_climb: f32,
    pub cell_size: f32,
}

impl crate::Parse for NavMeshParams {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let tile_size = r.read_f32()?;
        let walkable_height = r.read_f32()?;
        let walkable_radius = r.read_f32()?;
        let walkable_climb = r.read_f32()?;
        let cell_size = r.read_f32()?;

        Ok(NavMeshParams {
            tile_size,
            walkable_height,
            walkable_radius,
            walkable_climb,
            cell_size,
        })
    }
}
