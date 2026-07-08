//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;
use crate::subclasses::hash128::Hash128;
use crate::subclasses::md_four::MdFour;


/// 版本枚举：CachedAssetMetaDataOriginalDigestVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum CachedAssetMetaDataOriginalDigestVariant {
    Default(Box<MdFour>),
    V5_0(Box<Hash128>),
}

impl crate::Parse for CachedAssetMetaDataOriginalDigestVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { Ok(Box::new(Hash128::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_0(v))
        } else {
            let v = ( { Ok(Box::new(MdFour::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct CachedAssetMetaData {
    pub guid: Guid,
    pub path_name: String,
    pub original_changeset: u32,
    pub original_name: String,
    pub original_parent: Guid,
    pub original_digest: CachedAssetMetaDataOriginalDigestVariant,
}

impl crate::Parse for CachedAssetMetaData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let guid = Guid::parse(rdr, ver)?;
        let path_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let original_changeset = rdr.read_u32()?;
        let original_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let original_parent = Guid::parse(rdr, ver)?;
        let original_digest = CachedAssetMetaDataOriginalDigestVariant::parse(rdr, ver)?;

        Ok(CachedAssetMetaData {
            guid,
            path_name,
            original_changeset,
            original_name,
            original_parent,
            original_digest,
        })
    }
}
