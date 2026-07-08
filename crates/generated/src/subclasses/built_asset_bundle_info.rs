//! 自动生成。请勿手动修改。


/// 版本枚举：BuiltAssetBundleInfoPackagedFileIndicesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BuiltAssetBundleInfoPackagedFileIndicesVariant {
    Default(Vec<u32>),
    V2017_1(Vec<u32>),
}

impl crate::Parse for BuiltAssetBundleInfoPackagedFileIndicesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct BuiltAssetBundleInfo {
    pub bundle_name: String,
    pub bundle_archive_file: u32,
    pub packaged_file_indices: BuiltAssetBundleInfoPackagedFileIndicesVariant,
}

impl crate::Parse for BuiltAssetBundleInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bundle_name = rdr.read_aligned_string()?;
        let bundle_archive_file = rdr.read_u32()?;
        let packaged_file_indices = BuiltAssetBundleInfoPackagedFileIndicesVariant::parse(rdr, ver)?;

        Ok(BuiltAssetBundleInfo {
            bundle_name,
            bundle_archive_file,
            packaged_file_indices,
        })
    }
}
