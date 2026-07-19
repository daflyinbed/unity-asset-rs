//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AvatarSkeletonMaskElement {
    pub path: String,
    pub weight: f32,
}

impl crate::Parse for AvatarSkeletonMaskElement {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let path = rdr.read_aligned_string()?;
        let weight = rdr.read_f32()?;

        Ok(AvatarSkeletonMaskElement { path, weight })
    }
}
