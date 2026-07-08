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
            thumbnail_class_id = Some(LibraryRepresentationThumbnailClassIdVariant::parse(rdr, ver)?);
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
