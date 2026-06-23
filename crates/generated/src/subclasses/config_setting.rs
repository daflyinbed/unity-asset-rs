//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ConfigSetting {
    pub value: String,
    pub flags: u32,
}

impl crate::Parse for ConfigSetting {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let value = r.read_aligned_string()?;
        r.align(4)?;
        let flags = r.read_u32()?;

        Ok(ConfigSetting {
            value,
            flags,
        })
    }
}
