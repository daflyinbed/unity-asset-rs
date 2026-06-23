//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetVrSettings {
    pub m_build_target: String,
    pub m_enabled: bool,
    pub m_devices: Vec<String>,
}

impl crate::Parse for BuildTargetVrSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = r.read_aligned_string()?;
        r.align(4)?;
        let m_enabled = r.read_bool()?;
        r.align(4)?;
        let m_devices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(BuildTargetVrSettings {
            m_build_target,
            m_enabled,
            m_devices,
        })
    }
}
