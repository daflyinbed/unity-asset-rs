//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::fast_property_name::FastPropertyName;
use crate::subclasses::unity_tex_env::UnityTexEnv;

#[derive(Debug, Clone)]
pub struct UnityPropertySheet {
    pub m_tex_envs: Vec<(FastPropertyName, UnityTexEnv)>,
    pub m_floats: Vec<(FastPropertyName, f32)>,
    pub m_colors: Vec<(FastPropertyName, ColorRgba)>,
}

impl crate::Parse for UnityPropertySheet {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_tex_envs = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = FastPropertyName::parse(r, ver)?; let b = UnityTexEnv::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_floats = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = FastPropertyName::parse(r, ver)?; let b = r.read_f32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_colors = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = FastPropertyName::parse(r, ver)?; let b = ColorRgba::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(UnityPropertySheet {
            m_tex_envs,
            m_floats,
            m_colors,
        })
    }
}
