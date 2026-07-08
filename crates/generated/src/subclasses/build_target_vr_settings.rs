//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetVrSettings {
    pub m_build_target: String,
    pub m_enabled: bool,
    pub m_devices: Vec<String>,
}

impl crate::Parse for BuildTargetVrSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let m_devices = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(BuildTargetVrSettings {
            m_build_target,
            m_enabled,
            m_devices,
        })
    }
}
