//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BoneWeights4 {
    pub weight_0: f32,
    pub weight_1: f32,
    pub weight_2: f32,
    pub weight_3: f32,
    pub bone_index_0: i32,
    pub bone_index_1: i32,
    pub bone_index_2: i32,
    pub bone_index_3: i32,
}

impl crate::Parse for BoneWeights4 {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let weight_0 = r.read_f32()?;
        let weight_1 = r.read_f32()?;
        let weight_2 = r.read_f32()?;
        let weight_3 = r.read_f32()?;
        let bone_index_0 = r.read_i32()?;
        let bone_index_1 = r.read_i32()?;
        let bone_index_2 = r.read_i32()?;
        let bone_index_3 = r.read_i32()?;

        Ok(BoneWeights4 {
            weight_0,
            weight_1,
            weight_2,
            weight_3,
            bone_index_0,
            bone_index_1,
            bone_index_2,
            bone_index_3,
        })
    }
}
