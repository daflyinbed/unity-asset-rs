//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct MonoBehaviour {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_script: PPtr,
    pub m_name: String,
}

impl crate::Parse for MonoBehaviour {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        // MonoBehaviour 用户字段不在 TPK 中，token 保留原字节供运行时解释。
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_script = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;

        Ok(MonoBehaviour {
            m_game_object,
            m_enabled,
            m_script,
            m_name,
        })
    }
}
