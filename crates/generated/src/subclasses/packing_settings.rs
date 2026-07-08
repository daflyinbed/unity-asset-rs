//! 自动生成。请勿手动修改。


/// 版本枚举：PackingSettingsEnableTightPackingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PackingSettingsEnableTightPackingVariant {
    Default(bool),
    V2021_1(bool),
}

impl crate::Parse for PackingSettingsEnableTightPackingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2021_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct PackingSettings {
    pub padding: i32,
    pub block_offset: i32,
    pub allow_alpha_splitting: bool,
    pub enable_rotation: bool,
    pub enable_tight_packing: PackingSettingsEnableTightPackingVariant,
    pub enable_alpha_dilation: Option<bool>,
}

impl crate::Parse for PackingSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let padding = rdr.read_i32()?;
        let block_offset = rdr.read_i32()?;
        let allow_alpha_splitting = rdr.read_bool()?;
        let enable_rotation = rdr.read_bool()?;
        let enable_tight_packing = PackingSettingsEnableTightPackingVariant::parse(rdr, ver)?;
        let enable_alpha_dilation = if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(PackingSettings {
            padding,
            block_offset,
            allow_alpha_splitting,
            enable_rotation,
            enable_tight_packing,
            enable_alpha_dilation,
        })
    }
}
