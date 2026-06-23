//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PackingSettings {
    pub padding: i32,
    pub block_offset: i32,
    pub allow_alpha_splitting: bool,
    pub enable_rotation: bool,
    pub enable_tight_packing: bool,
}

impl crate::Parse for PackingSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let padding = r.read_i32()?;
        let block_offset = r.read_i32()?;
        let allow_alpha_splitting = r.read_bool()?;
        let enable_rotation = r.read_bool()?;
        let enable_tight_packing = r.read_bool()?;
        r.align(4)?;

        Ok(PackingSettings {
            padding,
            block_offset,
            allow_alpha_splitting,
            enable_rotation,
            enable_tight_packing,
        })
    }
}
