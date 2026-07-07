use crate::{EndianReader, UnityVersion};

/// PPtr\<X\> —— 跨引用指针。所有 PPtr 在磁盘上都是 m_FileID + m_PathID 两整数。
///
/// 不带 generic：避免依赖 target class 存在（如 PrefabInstance 在某些 TPK 版本被改名）。
/// 用户拿不到类型区分（PPtr\<GameObject\> vs PPtr\<Texture2D\> 都变成 PPtr），但能编译。
#[derive(Debug, Clone)]
pub struct PPtr {
    pub m_file_id: i32,
    pub m_path_id: i64,
}

impl PPtr {
    pub fn parse<R: EndianReader>(r: &mut R, _ver: &UnityVersion) -> Result<Self, R::Error> {
        let m_file_id = r.read_i32()?;
        // 简化：m_PathID 永远按 i64 读。
        // 老版本（ver < 5）实际是 i32，但生成器暂不做版本分支。
        let m_path_id = r.read_i64()?;
        Ok(PPtr { m_file_id, m_path_id })
    }
}
