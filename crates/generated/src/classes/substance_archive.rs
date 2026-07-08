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
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_package_data = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SubstanceArchive {
            m_name,
            m_package_data,
        })
    }
}
