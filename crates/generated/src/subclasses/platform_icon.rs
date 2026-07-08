//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct PlatformIcon {
    pub m_textures: Vec<PPtr>,
    pub m_width: i32,
    pub m_height: i32,
    pub m_kind: i32,
    pub m_sub_kind: String,
}

impl crate::Parse for PlatformIcon {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_textures = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_width = rdr.read_i32()?;
        let m_height = rdr.read_i32()?;
        let m_kind = rdr.read_i32()?;
        let m_sub_kind = rdr.read_aligned_string()?;

        Ok(PlatformIcon {
            m_textures,
            m_width,
            m_height,
            m_kind,
            m_sub_kind,
        })
    }
}
