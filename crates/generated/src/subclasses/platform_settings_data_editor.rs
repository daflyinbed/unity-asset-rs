//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PlatformSettingsData_Editor {
    pub enabled: Option<bool>,
    pub settings: Vec<(String, String)>,
}

impl crate::Parse for PlatformSettingsData_Editor {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut settings = None;
        if *ver >= crate::UnityVersion::new(2021, 1, 5) {
        } else if *ver >= crate::UnityVersion::new(2021, 1, 1) {
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 4, 38) {
        } else if *ver >= crate::UnityVersion::new(2019, 4, 27) {
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(2019, 1, 4) {
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
        } else if *ver >= crate::UnityVersion::new(2018, 3, 10) {
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 4, 0) {
        } else {
            enabled = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        settings = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = rdr.read_aligned_string()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );

        Ok(PlatformSettingsData_Editor {
            enabled,
            settings: settings.unwrap(),
        })
    }
}
