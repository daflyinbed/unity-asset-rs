//! 自动生成。请勿手动修改。

use crate::subclasses::input_axis::InputAxis;
use super::global_game_manager::GlobalGameManager;

#[derive(Debug, Clone)]
pub struct InputManager {
    pub parent: GlobalGameManager,
    pub m_axes: Vec<InputAxis>,
    pub m_use_physical_keys: Option<bool>,
}

impl crate::Parse for InputManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = GlobalGameManager::parse(r, ver)?;
        let m_axes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { InputAxis::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_use_physical_keys = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(InputManager {
            parent,
            m_axes,
            m_use_physical_keys,
        })
    }
}
