//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;
use crate::subclasses::vector4f::Vector4f;

/// 版本枚举：BillboardAssetImageTexCoordsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardAssetImageTexCoordsVariant {
    Default(Vec<Vector4f>),
    V2017_1(Vec<Vector4f>),
}

impl crate::Parse for BillboardAssetImageTexCoordsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector4f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector4f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BillboardAssetImageTexCoordsVariant {
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

/// 版本枚举：BillboardAssetVerticesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BillboardAssetVerticesVariant {
    Default(Vec<Vector2f>),
    V5_3(Vec<Vector2f>),
    V2017_1(Vec<Vector2f>),
}

impl crate::Parse for BillboardAssetVerticesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector2f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector2f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::V5_3(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector2f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for BillboardAssetVerticesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_3(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_3(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BillboardAsset {
    pub m_name: String,
    pub width: f32,
    pub bottom: f32,
    pub height: f32,
    pub image_tex_coords: BillboardAssetImageTexCoordsVariant,
    pub rotated: Option<Vec<u8>>,
    pub vertices: BillboardAssetVerticesVariant,
    pub indices: Vec<u16>,
    pub material: PPtr,
}

impl crate::Parse for BillboardAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut width = None;
        let mut bottom = None;
        let mut height = None;
        let mut image_tex_coords = None;
        let mut rotated = None;
        let mut vertices = None;
        let mut indices = None;
        let mut material = None;
        m_name = Some(rdr.read_aligned_string()?);
        width = Some(rdr.read_f32()?);
        bottom = Some(rdr.read_f32()?);
        height = Some(rdr.read_f32()?);
        image_tex_coords = Some(BillboardAssetImageTexCoordsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 3, 0) {
        } else {
            rotated = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        }
        vertices = Some(BillboardAssetVerticesVariant::parse(rdr, ver)?);
        indices = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u16())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        material = Some(crate::PPtr::parse(rdr, ver)?);

        Ok(BillboardAsset {
            m_name: m_name.unwrap(),
            width: width.unwrap(),
            bottom: bottom.unwrap(),
            height: height.unwrap(),
            image_tex_coords: image_tex_coords.unwrap(),
            rotated,
            vertices: vertices.unwrap(),
            indices: indices.unwrap(),
            material: material.unwrap(),
        })
    }
}

impl crate::ToValue for BillboardAsset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("width".to_string(), self.width.to_value())),
            Some(("bottom".to_string(), self.bottom.to_value())),
            Some(("height".to_string(), self.height.to_value())),
            Some((
                "imageTexCoords".to_string(),
                self.image_tex_coords.to_value(),
            )),
            crate::opt_field("rotated", &self.rotated),
            Some(("vertices".to_string(), self.vertices.to_value())),
            Some(("indices".to_string(), self.indices.to_value())),
            Some(("material".to_string(), self.material.to_value())),
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
                "width" => (self.width).inspect_value(path, offset, limit),
                "bottom" => (self.bottom).inspect_value(path, offset, limit),
                "height" => (self.height).inspect_value(path, offset, limit),
                "imageTexCoords" => (self.image_tex_coords).inspect_value(path, offset, limit),
                "rotated" => (self.rotated).inspect_value(path, offset, limit),
                "vertices" => (self.vertices).inspect_value(path, offset, limit),
                "indices" => (self.indices).inspect_value(path, offset, limit),
                "material" => (self.material).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.width).inspect_value(path, offset, limit)).map(|v| ("width".to_string(), v)),
            ((self.bottom).inspect_value(path, offset, limit)).map(|v| ("bottom".to_string(), v)),
            ((self.height).inspect_value(path, offset, limit)).map(|v| ("height".to_string(), v)),
            ((self.image_tex_coords).inspect_value(path, offset, limit))
                .map(|v| ("imageTexCoords".to_string(), v)),
            ((self.rotated).inspect_value(path, offset, limit)).map(|v| ("rotated".to_string(), v)),
            ((self.vertices).inspect_value(path, offset, limit))
                .map(|v| ("vertices".to_string(), v)),
            ((self.indices).inspect_value(path, offset, limit)).map(|v| ("indices".to_string(), v)),
            ((self.material).inspect_value(path, offset, limit))
                .map(|v| ("material".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
