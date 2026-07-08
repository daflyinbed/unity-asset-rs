//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LocalizationAsset {
    pub m_name: String,
    pub locale_iso_code: String,
    pub editor_asset: bool,
    pub string_table: Vec<(String, String)>,
}

impl crate::Parse for LocalizationAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let locale_iso_code = rdr.read_aligned_string()?;
        let editor_asset = rdr.read_bool()?;
        rdr.align(4)?;
        let string_table = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(LocalizationAsset {
            m_name,
            locale_iso_code,
            editor_asset,
            string_table,
        })
    }
}
