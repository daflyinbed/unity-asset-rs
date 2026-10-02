//! 自动生成。请勿手动修改。

use crate::subclasses::input_axis::InputAxis;

/// 版本枚举：InputManagerMAxesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum InputManagerMAxesVariant {
    Default(Vec<InputAxis>),
    V2017_1(Vec<InputAxis>),
}

impl crate::Parse for InputManagerMAxesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| InputAxis::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| InputAxis::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for InputManagerMAxesVariant {
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
pub struct InputManager {
    pub m_axes: InputManagerMAxesVariant,
    pub m_use_physical_keys: Option<bool>,
}

impl crate::Parse for InputManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_axes = InputManagerMAxesVariant::parse(rdr, ver)?;
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

impl crate::ToValue for InputManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Axes".to_string(), self.m_axes.to_value())),
            crate::opt_field("m_UsePhysicalKeys", &self.m_use_physical_keys),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Axes" => (self.m_axes).inspect_value(path, offset, limit),
                "m_UsePhysicalKeys" => {
                    (self.m_use_physical_keys).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_axes).inspect_value(path, offset, limit)).map(|v| ("m_Axes".to_string(), v)),
            ((self.m_use_physical_keys).inspect_value(path, offset, limit))
                .map(|v| ("m_UsePhysicalKeys".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
