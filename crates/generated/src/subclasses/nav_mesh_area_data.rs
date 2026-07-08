//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct NavMeshAreaData {
    pub name: String,
    pub cost: f32,
}

impl crate::Parse for NavMeshAreaData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let cost = rdr.read_f32()?;

        Ok(NavMeshAreaData {
            name,
            cost,
        })
    }
}
