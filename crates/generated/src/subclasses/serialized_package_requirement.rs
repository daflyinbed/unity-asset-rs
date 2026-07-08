//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedPackageRequirement {
    pub name: String,
    pub version: String,
    pub status: i8,
}

impl crate::Parse for SerializedPackageRequirement {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let version = rdr.read_aligned_string()?;
        let status = rdr.read_i8()?;

        Ok(SerializedPackageRequirement {
            name,
            version,
            status,
        })
    }
}
