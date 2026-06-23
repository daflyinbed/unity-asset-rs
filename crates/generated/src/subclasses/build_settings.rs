//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildSettings {
    pub m_object_hide_flags: u32,
    pub levels: Vec<String>,
    pub has_render_texture: bool,
    pub has_pro_version: bool,
    pub is_educational_build: bool,
    pub has_publishing_rights: bool,
    pub has_shadows: bool,
    pub has_advanced_version: bool,
    pub enable_dynamic_batching: bool,
    pub is_debug_build: bool,
}

impl crate::Parse for BuildSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let levels = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let has_render_texture = r.read_bool()?;
        let has_pro_version = r.read_bool()?;
        let is_educational_build = r.read_bool()?;
        let has_publishing_rights = r.read_bool()?;
        let has_shadows = r.read_bool()?;
        let has_advanced_version = r.read_bool()?;
        let enable_dynamic_batching = r.read_bool()?;
        let is_debug_build = r.read_bool()?;
        r.align(4)?;

        Ok(BuildSettings {
            m_object_hide_flags,
            levels,
            has_render_texture,
            has_pro_version,
            is_educational_build,
            has_publishing_rights,
            has_shadows,
            has_advanced_version,
            enable_dynamic_batching,
            is_debug_build,
        })
    }
}
