//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::image::Image;

/// 版本枚举：LibraryRepresentationObjectVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LibraryRepresentationObjectVariant {
    Default(PPtr),
    V3_5(PPtr),
}

impl crate::Parse for LibraryRepresentationObjectVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(3, 5, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::V3_5(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for LibraryRepresentationObjectVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V3_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V3_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：LibraryRepresentationThumbnailClassIdVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LibraryRepresentationThumbnailClassIdVariant {
    Default(i32),
    V4_0(i16),
    V5_5(i32),
}

impl crate::Parse for LibraryRepresentationThumbnailClassIdVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_5(v))
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_i16()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for LibraryRepresentationThumbnailClassIdVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_0(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_0(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：LibraryRepresentationFlagsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LibraryRepresentationFlagsVariant {
    Default(u16),
    V5_5(u16),
}

impl crate::Parse for LibraryRepresentationFlagsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = rdr.read_u16()?;
            rdr.align(4)?;
            Ok(Self::V5_5(v))
        } else {
            let v = rdr.read_u16()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for LibraryRepresentationFlagsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct LibraryRepresentation {
    pub name: String,
    pub thumbnail: Image,
    pub object: Option<LibraryRepresentationObjectVariant>,
    pub guid: Option<Guid>,
    pub thumbnail_class_id: LibraryRepresentationThumbnailClassIdVariant,
    pub path: Option<String>,
    pub script_class_name: String,
    pub flags: Option<LibraryRepresentationFlagsVariant>,
    pub local_identifier: Option<i64>,
}

impl crate::Parse for LibraryRepresentation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut thumbnail = None;
        let mut object = None;
        let mut guid = None;
        let mut thumbnail_class_id = None;
        let mut path = None;
        let mut script_class_name = None;
        let mut flags = None;
        let mut local_identifier = None;
        name = Some(rdr.read_aligned_string()?);
        thumbnail = Some(Image::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 1, 0) {
            guid = Some(Guid::parse(rdr, ver)?);
            path = Some(rdr.read_aligned_string()?);
            local_identifier = Some(rdr.read_i64()?);
        } else {
            object = Some(LibraryRepresentationObjectVariant::parse(rdr, ver)?);
        }
        thumbnail_class_id = Some(LibraryRepresentationThumbnailClassIdVariant::parse(
            rdr, ver,
        )?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            flags = Some(LibraryRepresentationFlagsVariant::parse(rdr, ver)?);
        } else {
        }
        script_class_name = Some(rdr.read_aligned_string()?);

        Ok(LibraryRepresentation {
            name: name.unwrap(),
            thumbnail: thumbnail.unwrap(),
            object,
            guid,
            thumbnail_class_id: thumbnail_class_id.unwrap(),
            path,
            script_class_name: script_class_name.unwrap(),
            flags,
            local_identifier,
        })
    }
}

impl crate::ToValue for LibraryRepresentation {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("thumbnail".to_string(), self.thumbnail.to_value())),
            crate::opt_field("object", &self.object),
            crate::opt_field("guid", &self.guid),
            Some((
                "thumbnailClassID".to_string(),
                self.thumbnail_class_id.to_value(),
            )),
            crate::opt_field("path", &self.path),
            Some((
                "scriptClassName".to_string(),
                self.script_class_name.to_value(),
            )),
            crate::opt_field("flags", &self.flags),
            crate::opt_field("localIdentifier", &self.local_identifier),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "thumbnail" => (self.thumbnail).inspect_value(path, offset, limit),
                "object" => (self.object).inspect_value(path, offset, limit),
                "guid" => (self.guid).inspect_value(path, offset, limit),
                "thumbnailClassID" => (self.thumbnail_class_id).inspect_value(path, offset, limit),
                "path" => (self.path).inspect_value(path, offset, limit),
                "scriptClassName" => (self.script_class_name).inspect_value(path, offset, limit),
                "flags" => (self.flags).inspect_value(path, offset, limit),
                "localIdentifier" => (self.local_identifier).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.thumbnail).inspect_value(path, offset, limit))
                .map(|v| ("thumbnail".to_string(), v)),
            ((self.object).inspect_value(path, offset, limit)).map(|v| ("object".to_string(), v)),
            ((self.guid).inspect_value(path, offset, limit)).map(|v| ("guid".to_string(), v)),
            ((self.thumbnail_class_id).inspect_value(path, offset, limit))
                .map(|v| ("thumbnailClassID".to_string(), v)),
            ((self.path).inspect_value(path, offset, limit)).map(|v| ("path".to_string(), v)),
            ((self.script_class_name).inspect_value(path, offset, limit))
                .map(|v| ("scriptClassName".to_string(), v)),
            ((self.flags).inspect_value(path, offset, limit)).map(|v| ("flags".to_string(), v)),
            ((self.local_identifier).inspect_value(path, offset, limit))
                .map(|v| ("localIdentifier".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
