//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct PPtr_PrefabInstance {
    pub m_file_id: i32,
    pub m_path_id: i64,
}

impl crate::Parse for PPtr_PrefabInstance {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_file_id = rdr.read_i32()?;
        let m_path_id = rdr.read_i64()?;

        Ok(PPtr_PrefabInstance {
            m_file_id,
            m_path_id,
        })
    }
}
