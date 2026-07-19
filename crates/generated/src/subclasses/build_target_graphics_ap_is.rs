//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetGraphicsApIs {
    pub m_build_target: String,
    pub m_ap_is: Vec<i32>,
    pub m_automatic: bool,
}

impl crate::Parse for BuildTargetGraphicsApIs {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_ap_is = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_automatic = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(BuildTargetGraphicsApIs {
            m_build_target,
            m_ap_is,
            m_automatic,
        })
    }
}
