//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct iOsDeviceRequirementForResourceVariant {
    pub variant_tag_name: String,
    pub variations: Vec<Vec<(String, String)>>,
}

impl crate::Parse for iOsDeviceRequirementForResourceVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let variant_tag_name = rdr.read_aligned_string()?;
        let variations = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(iOsDeviceRequirementForResourceVariant {
            variant_tag_name,
            variations,
        })
    }
}
