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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let locale_iso_code = r.read_aligned_string()?;
        r.align(4)?;
        let editor_asset = r.read_bool()?;
        r.align(4)?;
        let string_table = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(LocalizationAsset {
            m_name,
            locale_iso_code,
            editor_asset,
            string_table,
        })
    }
}
