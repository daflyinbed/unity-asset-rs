//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SceneObjectIdentifier {
    pub target_object: i64,
    pub target_prefab: i64,
}

impl crate::Parse for SceneObjectIdentifier {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_object = r.read_i64()?;
        let target_prefab = r.read_i64()?;

        Ok(SceneObjectIdentifier {
            target_object,
            target_prefab,
        })
    }
}
