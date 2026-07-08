//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PlatformSettingsData {
    pub enabled: Option<bool>,
    pub settings: Vec<(String, String)>,
}

impl crate::Parse for PlatformSettingsData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let settings = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(PlatformSettingsData {
            enabled,
            settings,
        })
    }
}
