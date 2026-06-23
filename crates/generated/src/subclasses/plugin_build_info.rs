//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PluginBuildInfo {
    pub m_runtime_plugins: Vec<String>,
    pub m_editor_plugins: Vec<String>,
}

impl crate::Parse for PluginBuildInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_runtime_plugins = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_editor_plugins = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(PluginBuildInfo {
            m_runtime_plugins,
            m_editor_plugins,
        })
    }
}
