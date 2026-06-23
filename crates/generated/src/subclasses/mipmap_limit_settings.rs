//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MipmapLimitSettings {
    pub limit_bias_mode: i32,
    pub limit_bias: i32,
}

impl crate::Parse for MipmapLimitSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let limit_bias_mode = r.read_i32()?;
        let limit_bias = r.read_i32()?;

        Ok(MipmapLimitSettings {
            limit_bias_mode,
            limit_bias,
        })
    }
}
