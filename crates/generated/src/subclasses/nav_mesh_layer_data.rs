//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct NavMeshLayerData {
    pub name: String,
    pub cost: f32,
    pub edit_type: i32,
}

impl crate::Parse for NavMeshLayerData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let cost = rdr.read_f32()?;
        let edit_type = rdr.read_i32()?;

        Ok(NavMeshLayerData {
            name,
            cost,
            edit_type,
        })
    }
}
