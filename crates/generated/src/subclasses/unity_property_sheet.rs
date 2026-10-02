//! 自动生成。请勿手动修改。

use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::fast_property_name::FastPropertyName;
use crate::subclasses::unity_tex_env::UnityTexEnv;

/// 版本枚举：UnityPropertySheetMTexEnvsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum UnityPropertySheetMTexEnvsVariant {
    Default(Vec<(FastPropertyName, UnityTexEnv)>),
    V5_6(Vec<(String, UnityTexEnv)>),
}

impl crate::Parse for UnityPropertySheetMTexEnvsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = UnityTexEnv::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = FastPropertyName::parse(rdr, ver)?;
                            let b = UnityTexEnv::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for UnityPropertySheetMTexEnvsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：UnityPropertySheetMFloatsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum UnityPropertySheetMFloatsVariant {
    Default(Vec<(FastPropertyName, f32)>),
    V5_6(Vec<(String, f32)>),
}

impl crate::Parse for UnityPropertySheetMFloatsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = rdr.read_f32()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = FastPropertyName::parse(rdr, ver)?;
                            let b = rdr.read_f32()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for UnityPropertySheetMFloatsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：UnityPropertySheetMColorsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum UnityPropertySheetMColorsVariant {
    Default(Vec<(FastPropertyName, ColorRgbAf)>),
    V5_6(Vec<(String, ColorRgbAf)>),
}

impl crate::Parse for UnityPropertySheetMColorsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = rdr.read_aligned_string()?;
                            let b = ColorRgbAf::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_6(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = FastPropertyName::parse(rdr, ver)?;
                            let b = ColorRgbAf::parse(rdr, ver)?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for UnityPropertySheetMColorsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct UnityPropertySheet {
    pub m_tex_envs: UnityPropertySheetMTexEnvsVariant,
    pub m_floats: UnityPropertySheetMFloatsVariant,
    pub m_ints: Option<Vec<(String, i32)>>,
    pub m_colors: UnityPropertySheetMColorsVariant,
}

impl crate::Parse for UnityPropertySheet {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_tex_envs = None;
        let mut m_floats = None;
        let mut m_ints = None;
        let mut m_colors = None;
        m_tex_envs = Some(UnityPropertySheetMTexEnvsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_ints = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = rdr.read_i32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
        }
        m_floats = Some(UnityPropertySheetMFloatsVariant::parse(rdr, ver)?);
        m_colors = Some(UnityPropertySheetMColorsVariant::parse(rdr, ver)?);

        Ok(UnityPropertySheet {
            m_tex_envs: m_tex_envs.unwrap(),
            m_floats: m_floats.unwrap(),
            m_ints,
            m_colors: m_colors.unwrap(),
        })
    }
}

impl crate::ToValue for UnityPropertySheet {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_TexEnvs".to_string(), self.m_tex_envs.to_value())),
            Some(("m_Floats".to_string(), self.m_floats.to_value())),
            crate::opt_field("m_Ints", &self.m_ints),
            Some(("m_Colors".to_string(), self.m_colors.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_TexEnvs" => (self.m_tex_envs).inspect_value(path, offset, limit),
                "m_Floats" => (self.m_floats).inspect_value(path, offset, limit),
                "m_Ints" => (self.m_ints).inspect_value(path, offset, limit),
                "m_Colors" => (self.m_colors).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_tex_envs).inspect_value(path, offset, limit))
                .map(|v| ("m_TexEnvs".to_string(), v)),
            ((self.m_floats).inspect_value(path, offset, limit))
                .map(|v| ("m_Floats".to_string(), v)),
            ((self.m_ints).inspect_value(path, offset, limit)).map(|v| ("m_Ints".to_string(), v)),
            ((self.m_colors).inspect_value(path, offset, limit))
                .map(|v| ("m_Colors".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
