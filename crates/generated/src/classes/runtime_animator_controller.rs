//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::controller_constant::ControllerConstant;

#[derive(Debug, Clone)]
pub struct RuntimeAnimatorController {
    pub m_name: String,
    pub m_controller_size: u32,
    pub m_controller: ControllerConstant,
    pub m_tos: Vec<(u32, String)>,
    pub m_animation_clips: Vec<PPtr>,
}

impl crate::Parse for RuntimeAnimatorController {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_controller_size = r.read_u32()?;
        let m_controller = ControllerConstant::parse(r, ver)?;
        r.align(4)?;
        let m_tos = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_u32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_animation_clips = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(RuntimeAnimatorController {
            m_name,
            m_controller_size,
            m_controller,
            m_tos,
            m_animation_clips,
        })
    }
}
