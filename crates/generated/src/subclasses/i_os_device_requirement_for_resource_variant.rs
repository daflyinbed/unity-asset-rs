//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct iOsDeviceRequirementForResourceVariant {
    pub variant_tag_name: String,
    pub variations: Vec<Vec<(String, String)>>,
}

impl crate::Parse for iOsDeviceRequirementForResourceVariant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let variant_tag_name = r.read_aligned_string()?;
        r.align(4)?;
        let variations = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(iOsDeviceRequirementForResourceVariant {
            variant_tag_name,
            variations,
        })
    }
}
