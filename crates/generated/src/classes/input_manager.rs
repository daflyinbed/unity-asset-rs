//! 自动生成。请勿手动修改。

use crate::subclasses::input_axis::InputAxis;

#[derive(Debug, Clone)]
pub struct InputManager {
    pub m_axes: Vec<InputAxis>,
    pub m_use_physical_keys: Option<bool>,
}

impl crate::Parse for InputManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_axes = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { InputAxis::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_use_physical_keys = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(InputManager {
            m_axes,
            m_use_physical_keys,
        })
    }
}
