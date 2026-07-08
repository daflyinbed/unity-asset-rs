//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::shader_info::ShaderInfo;

#[derive(Debug, Clone)]
pub struct ShaderVariantCollection {
    pub m_name: String,
    pub m_shaders: Vec<(PPtr, ShaderInfo)>,
}

impl crate::Parse for ShaderVariantCollection {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let m_shaders = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = crate::PPtr::parse(rdr, ver)?; let b = ShaderInfo::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(ShaderVariantCollection {
            m_name,
            m_shaders,
        })
    }
}
