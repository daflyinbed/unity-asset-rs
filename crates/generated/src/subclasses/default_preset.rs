//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct DefaultPreset {
    pub m_preset: PPtr,
    pub m_filter: Option<String>,
    pub m_disabled: Option<bool>,
}

impl crate::Parse for DefaultPreset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_preset = crate::PPtr::parse(rdr, ver)?;
        let m_filter = if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_disabled = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(DefaultPreset {
            m_preset,
            m_filter,
            m_disabled,
        })
    }
}
