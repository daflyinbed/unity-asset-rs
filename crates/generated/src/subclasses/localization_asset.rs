//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct LocalizationAsset {
    pub m_object_hide_flags: u32,
    pub m_corresponding_source_object: PPtr,
    pub m_prefab_internal: PPtr,
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
        let m_object_hide_flags = r.read_u32()?;
        let m_corresponding_source_object = crate::PPtr::parse(r, ver)?;
        let m_prefab_internal = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let locale_iso_code = r.read_aligned_string()?;
        r.align(4)?;
        let editor_asset = r.read_bool()?;
        r.align(4)?;
        let string_table = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(LocalizationAsset {
            m_object_hide_flags,
            m_corresponding_source_object,
            m_prefab_internal,
            m_name,
            locale_iso_code,
            editor_asset,
            string_table,
        })
    }
}
