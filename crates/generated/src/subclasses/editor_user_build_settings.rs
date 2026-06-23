//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct EditorUserBuildSettings {
    pub m_object_hide_flags: u32,
    pub m_build_location: Vec<String>,
    pub m_active_build_target: i32,
    pub m_selected_build_target_group: i32,
    pub m_selected_standalone_target: i32,
    pub m_architecture_flags: i32,
    pub m_selected_wii_subtarget: i32,
    pub m_development: bool,
    pub m_connect_profiler: bool,
    pub m_allow_debugging: bool,
    pub m_web_player_streamed: bool,
    pub m_web_player_offline_deployment: bool,
    pub m_web_player_na_cl_support: bool,
    pub m_install_in_build_folder: bool,
    pub m_symlink_libraries: bool,
    pub m_selected_android_subtarget: i32,
}

impl crate::Parse for EditorUserBuildSettings {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_build_location = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_active_build_target = r.read_i32()?;
        let m_selected_build_target_group = r.read_i32()?;
        let m_selected_standalone_target = r.read_i32()?;
        let m_architecture_flags = r.read_i32()?;
        let m_selected_wii_subtarget = r.read_i32()?;
        let m_development = r.read_bool()?;
        let m_connect_profiler = r.read_bool()?;
        let m_allow_debugging = r.read_bool()?;
        let m_web_player_streamed = r.read_bool()?;
        let m_web_player_offline_deployment = r.read_bool()?;
        let m_web_player_na_cl_support = r.read_bool()?;
        let m_install_in_build_folder = r.read_bool()?;
        let m_symlink_libraries = r.read_bool()?;
        r.align(4)?;
        let m_selected_android_subtarget = r.read_i32()?;

        Ok(EditorUserBuildSettings {
            m_object_hide_flags,
            m_build_location,
            m_active_build_target,
            m_selected_build_target_group,
            m_selected_standalone_target,
            m_architecture_flags,
            m_selected_wii_subtarget,
            m_development,
            m_connect_profiler,
            m_allow_debugging,
            m_web_player_streamed,
            m_web_player_offline_deployment,
            m_web_player_na_cl_support,
            m_install_in_build_folder,
            m_symlink_libraries,
            m_selected_android_subtarget,
        })
    }
}
