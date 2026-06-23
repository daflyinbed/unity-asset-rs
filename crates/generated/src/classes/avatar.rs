//! 自动生成。请勿手动修改。

use crate::subclasses::avatar_constant::AvatarConstant;
use crate::subclasses::human_description::HumanDescription;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct Avatar {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_avatar_size: u32,
    pub m_avatar: AvatarConstant,
    pub m_tos: Vec<(u32, String)>,
    pub m_human_description: Option<HumanDescription>,
}

impl crate::Parse for Avatar {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_avatar_size = r.read_u32()?;
        let m_avatar = AvatarConstant::parse(r, ver)?;
        r.align(4)?;
        let m_tos = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_u32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_human_description = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = HumanDescription::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(Avatar {
            parent,
            m_name,
            m_avatar_size,
            m_avatar,
            m_tos,
            m_human_description,
        })
    }
}
