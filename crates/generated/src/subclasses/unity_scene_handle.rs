//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct UnitySceneHandle {
    pub value: i32,
}

impl crate::Parse for UnitySceneHandle {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let value = rdr.read_i32()?;

        Ok(UnitySceneHandle {
            value,
        })
    }
}
