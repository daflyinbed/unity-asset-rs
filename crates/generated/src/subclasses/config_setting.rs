//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ConfigSetting {
    pub value: String,
    pub flags: u32,
}

impl crate::Parse for ConfigSetting {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let value = rdr.read_aligned_string()?;
        let flags = rdr.read_u32()?;

        Ok(ConfigSetting {
            value,
            flags,
        })
    }
}
