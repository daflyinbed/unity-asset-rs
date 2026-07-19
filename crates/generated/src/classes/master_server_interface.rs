//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MasterServerInterface {}

impl crate::Parse for MasterServerInterface {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        Ok(MasterServerInterface {})
    }
}
