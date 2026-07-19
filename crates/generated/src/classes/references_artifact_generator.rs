//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::source_asset_identifier::SourceAssetIdentifier;

#[derive(Debug, Clone)]
pub struct ReferencesArtifactGenerator {
    pub m_name: String,
    pub m_external_objects: Vec<(SourceAssetIdentifier, PPtr)>,
    pub m_used_file_i_ds: Vec<i64>,
    pub m_user_data: String,
    pub m_asset_bundle_name: String,
    pub m_asset_bundle_variant: String,
}

impl crate::Parse for ReferencesArtifactGenerator {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_external_objects = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = SourceAssetIdentifier::parse(rdr, ver)?;
                        let b = crate::PPtr::parse(rdr, ver)?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_used_file_i_ds = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i64())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_user_data = rdr.read_aligned_string()?;
        let m_asset_bundle_name = rdr.read_aligned_string()?;
        let m_asset_bundle_variant = rdr.read_aligned_string()?;

        Ok(ReferencesArtifactGenerator {
            m_name,
            m_external_objects,
            m_used_file_i_ds,
            m_user_data,
            m_asset_bundle_name,
            m_asset_bundle_variant,
        })
    }
}
