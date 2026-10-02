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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| GenericBinding::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| GenericBinding::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnimationClipBindingConstantGenericBindingsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for AnimationClipBindingConstantPptrCurveMappingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
        let pptr_curve_mapping =
            AnimationClipBindingConstantPptrCurveMappingVariant::parse(rdr, ver)?;

        Ok(AnimationClipBindingConstant {
            generic_bindings,
            pptr_curve_mapping,
        })
    }
}

impl crate::ToValue for AnimationClipBindingConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "genericBindings".to_string(),
                self.generic_bindings.to_value(),
            )),
            Some((
                "pptrCurveMapping".to_string(),
                self.pptr_curve_mapping.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "genericBindings" => (self.generic_bindings).inspect_value(path, offset, limit),
                "pptrCurveMapping" => (self.pptr_curve_mapping).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.generic_bindings).inspect_value(path, offset, limit))
                .map(|v| ("genericBindings".to_string(), v)),
            ((self.pptr_curve_mapping).inspect_value(path, offset, limit))
                .map(|v| ("pptrCurveMapping".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
