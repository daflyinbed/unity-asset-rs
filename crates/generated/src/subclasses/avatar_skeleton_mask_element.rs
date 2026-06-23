//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AvatarSkeletonMaskElement {
    pub path: String,
    pub weight: f32,
}

impl crate::Parse for AvatarSkeletonMaskElement {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let path = r.read_aligned_string()?;
        r.align(4)?;
        let weight = r.read_f32()?;

        Ok(AvatarSkeletonMaskElement {
            path,
            weight,
        })
    }
}
