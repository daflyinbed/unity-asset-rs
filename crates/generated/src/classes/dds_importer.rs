//! 自动生成。请勿手动修改。

use crate::subclasses::md_four::MdFour;

#[derive(Debug, Clone)]
pub struct DdsImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_user_data: Option<String>,
    pub m_is_readable: Option<bool>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_asset_bundle_name: Option<String>,
    pub m_new_hash_identity: Option<MdFour>,
    pub m_asset_bundle_variant: Option<String>,
}

impl crate::Parse for DdsImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_user_data = None;
        let mut m_is_readable = None;
        let mut m_old_hash_identity = None;
        let mut m_asset_bundle_name = None;
        let mut m_new_hash_identity = None;
        let mut m_asset_bundle_variant = None;
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_is_readable = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            m_asset_bundle_variant = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_user_data = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_file_id_to_recycle_name = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_i32()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_old_hash_identity = Some(MdFour::parse(rdr, ver)?);
            rdr.align(4)?;
            m_new_hash_identity = Some(MdFour::parse(rdr, ver)?);
            rdr.align(4)?;
        }

        Ok(DdsImporter {
            m_name: m_name.unwrap(),
            m_file_id_to_recycle_name,
            m_user_data,
            m_is_readable,
            m_old_hash_identity,
            m_asset_bundle_name,
            m_new_hash_identity,
            m_asset_bundle_variant,
        })
    }
}
