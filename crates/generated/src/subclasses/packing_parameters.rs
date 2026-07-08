//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PackingParameters {
    pub padding_power: u32,
    pub block_offset: u32,
    pub allow_alpha_splitting: i32,
    pub enable_rotation: i32,
    pub enable_tight_packing: i32,
}

impl crate::Parse for PackingParameters {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let padding_power = rdr.read_u32()?;
        let block_offset = rdr.read_u32()?;
        let allow_alpha_splitting = rdr.read_i32()?;
        let enable_rotation = rdr.read_i32()?;
        let enable_tight_packing = rdr.read_i32()?;

        Ok(PackingParameters {
            padding_power,
            block_offset,
            allow_alpha_splitting,
            enable_rotation,
            enable_tight_packing,
        })
    }
}
