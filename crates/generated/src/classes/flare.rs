//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::flare_element::FlareElement;

/// 版本枚举：FlareMElementsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FlareMElementsVariant {
    Default(Vec<FlareElement>),
    V2017_1(Vec<FlareElement>),
}

impl crate::Parse for FlareMElementsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| FlareElement::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| FlareElement::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for FlareMElementsVariant {
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
pub struct Flare {
    pub m_name: String,
    pub m_flare_texture: PPtr,
    pub m_texture_layout: i32,
    pub m_elements: FlareMElementsVariant,
    pub m_use_fog: bool,
}

impl crate::Parse for Flare {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_flare_texture = crate::PPtr::parse(rdr, ver)?;
        let m_texture_layout = rdr.read_i32()?;
        let m_elements = FlareMElementsVariant::parse(rdr, ver)?;
        let m_use_fog = rdr.read_bool()?;

        Ok(Flare {
            m_name,
            m_flare_texture,
            m_texture_layout,
            m_elements,
            m_use_fog,
        })
    }
}

impl crate::ToValue for Flare {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_FlareTexture".to_string(),
                self.m_flare_texture.to_value(),
            )),
            Some((
                "m_TextureLayout".to_string(),
                self.m_texture_layout.to_value(),
            )),
            Some(("m_Elements".to_string(), self.m_elements.to_value())),
            Some(("m_UseFog".to_string(), self.m_use_fog.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_FlareTexture" => (self.m_flare_texture).inspect_value(path, offset, limit),
                "m_TextureLayout" => (self.m_texture_layout).inspect_value(path, offset, limit),
                "m_Elements" => (self.m_elements).inspect_value(path, offset, limit),
                "m_UseFog" => (self.m_use_fog).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_flare_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_FlareTexture".to_string(), v)),
            ((self.m_texture_layout).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureLayout".to_string(), v)),
            ((self.m_elements).inspect_value(path, offset, limit))
                .map(|v| ("m_Elements".to_string(), v)),
            ((self.m_use_fog).inspect_value(path, offset, limit))
                .map(|v| ("m_UseFog".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
