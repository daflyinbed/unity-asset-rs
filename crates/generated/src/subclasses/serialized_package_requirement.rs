//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SerializedPackageRequirement {
    pub name: String,
    pub version: String,
    pub status: i8,
}

impl crate::Parse for SerializedPackageRequirement {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let version = r.read_aligned_string()?;
        r.align(4)?;
        let status = r.read_i8()?;

        Ok(SerializedPackageRequirement {
            name,
            version,
            status,
        })
    }
}
