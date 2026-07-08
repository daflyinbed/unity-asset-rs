//! 自动生成。请勿手动修改。

use crate::subclasses::md_four::MdFour;

#[derive(Debug, Clone)]
pub struct AssetImporter {
    pub m_name: String,
    pub m_file_id_to_recycle_name: Option<Vec<(i32, String)>>,
    pub m_old_hash_identity: Option<MdFour>,
    pub m_new_hash_identity: Option<MdFour>,
}

impl crate::Parse for AssetImporter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_file_id_to_recycle_name = None;
        let mut m_old_hash_identity = None;
        let mut m_new_hash_identity = None;
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
        } else {
            m_file_id_to_recycle_name = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_i32()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_old_hash_identity = Some(MdFour::parse(rdr, ver)?);
            rdr.align(4)?;
            m_new_hash_identity = Some(MdFour::parse(rdr, ver)?);
            rdr.align(4)?;
        }

        Ok(AssetImporter {
            m_name: m_name.unwrap(),
            m_file_id_to_recycle_name,
            m_old_hash_identity,
            m_new_hash_identity,
        })
    }
}
