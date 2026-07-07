//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct AudioResource {
    pub m_name: String,
}

impl crate::Parse for AudioResource {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(AudioResource {
            m_name,
        })
    }
}
