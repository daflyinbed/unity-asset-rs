//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::generic_binding::GenericBinding;

#[derive(Debug, Clone)]
pub struct AnimationClipBindingConstant {
    pub generic_bindings: Vec<GenericBinding>,
    pub pptr_curve_mapping: Vec<PPtr>,
}

impl crate::Parse for AnimationClipBindingConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let generic_bindings = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { GenericBinding::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let pptr_curve_mapping = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(AnimationClipBindingConstant {
            generic_bindings,
            pptr_curve_mapping,
        })
    }
}
