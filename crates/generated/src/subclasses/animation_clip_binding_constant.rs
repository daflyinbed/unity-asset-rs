//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::generic_binding::GenericBinding;


/// 版本枚举：AnimationClipBindingConstantGenericBindingsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationClipBindingConstantGenericBindingsVariant {
    Default(Vec<GenericBinding>),
    V2017_1(Vec<GenericBinding>),
}

impl crate::Parse for AnimationClipBindingConstantGenericBindingsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { GenericBinding::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { GenericBinding::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：AnimationClipBindingConstantPptrCurveMappingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AnimationClipBindingConstantPptrCurveMappingVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for AnimationClipBindingConstantPptrCurveMappingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnimationClipBindingConstant {
    pub generic_bindings: AnimationClipBindingConstantGenericBindingsVariant,
    pub pptr_curve_mapping: AnimationClipBindingConstantPptrCurveMappingVariant,
}

impl crate::Parse for AnimationClipBindingConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let generic_bindings = AnimationClipBindingConstantGenericBindingsVariant::parse(rdr, ver)?;
        let pptr_curve_mapping = AnimationClipBindingConstantPptrCurveMappingVariant::parse(rdr, ver)?;

        Ok(AnimationClipBindingConstant {
            generic_bindings,
            pptr_curve_mapping,
        })
    }
}
