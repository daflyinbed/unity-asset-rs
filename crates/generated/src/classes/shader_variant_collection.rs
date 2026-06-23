//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::shader_info::ShaderInfo;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct ShaderVariantCollection {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_shaders: Vec<(PPtr, ShaderInfo)>,
}

impl crate::Parse for ShaderVariantCollection {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_shaders = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(r, ver)?; let b = ShaderInfo::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ShaderVariantCollection {
            parent,
            m_name,
            m_shaders,
        })
    }
}
