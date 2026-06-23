//! 自动生成。请勿手动修改。

use crate::subclasses::input_axis::InputAxis;

#[derive(Debug, Clone)]
pub struct InputManager {
    pub m_object_hide_flags: u32,
    pub m_axes: Vec<InputAxis>,
}

impl crate::Parse for InputManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_axes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { InputAxis::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(InputManager {
            m_object_hide_flags,
            m_axes,
        })
    }
}
