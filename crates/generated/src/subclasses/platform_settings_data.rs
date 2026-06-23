//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PlatformSettingsData {
    pub enabled: bool,
    pub settings: Vec<(String, String)>,
}

impl crate::Parse for PlatformSettingsData {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = r.read_bool()?;
        r.align(4)?;
        let settings = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(PlatformSettingsData {
            enabled,
            settings,
        })
    }
}
