//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxMapping {
    pub name_id: String,
    pub index: i32,
}

impl crate::Parse for VfxMapping {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name_id = rdr.read_aligned_string()?;
        let index = rdr.read_i32()?;

        Ok(VfxMapping {
            name_id,
            index,
        })
    }
}
