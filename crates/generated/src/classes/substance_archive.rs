//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SubstanceArchive {
    pub m_name: String,
    pub m_package_data: Option<Vec<u8>>,
}

impl crate::Parse for SubstanceArchive {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_package_data = None;
            m_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
        } else {
            m_package_data = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        }

        Ok(SubstanceArchive {
            m_name: m_name.unwrap(),
            m_package_data,
        })
    }
}
