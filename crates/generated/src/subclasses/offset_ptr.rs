//! 自动生成。请勿手动修改。

use crate::subclasses::clip::Clip;
use crate::subclasses::skeleton_pose::SkeletonPose;
use crate::subclasses::value_array::ValueArray;
use crate::subclasses::value_array_constant::ValueArrayConstant;


/// 版本枚举：OffsetPtrDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum OffsetPtrDataVariant {
    Default(Box<ValueArray>),
    V5_3(Box<ValueArrayConstant>),
    V5_3_5(Box<ValueArray>),
    V5_6(Box<ValueArrayConstant>),
    V5_6_1(Box<SkeletonPose>),
    V2017_1(Box<ValueArrayConstant>),
    V2017_2(Box<ValueArray>),
    V2017_3(Box<ValueArrayConstant>),
    V2018_3(Box<Clip>),
    V6000_2(Box<ValueArray>),
    V6000_5(Box<Clip>),
}

impl crate::Parse for OffsetPtrDataVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = ( { Ok(Box::new(Clip::parse(rdr, ver)?)) } )?;
            Ok(Self::V6000_5(v))
        } else if *ver >= crate::UnityVersion::new(6000, 2, 0) {
            let v = ( { Ok(Box::new(ValueArray::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V6000_2(v))
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = ( { Ok(Box::new(Clip::parse(rdr, ver)?)) } )?;
            Ok(Self::V2018_3(v))
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = ( { Ok(Box::new(ValueArrayConstant::parse(rdr, ver)?)) } )?;
            Ok(Self::V2017_3(v))
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = ( { Ok(Box::new(ValueArray::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V2017_2(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { Ok(Box::new(ValueArrayConstant::parse(rdr, ver)?)) } )?;
            Ok(Self::V2017_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 6, 1) {
            let v = ( { Ok(Box::new(SkeletonPose::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_6_1(v))
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ( { Ok(Box::new(ValueArrayConstant::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_6(v))
        } else if *ver >= crate::UnityVersion::new(5, 3, 5) {
            let v = ( { Ok(Box::new(ValueArray::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::V5_3_5(v))
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = ( { Ok(Box::new(ValueArrayConstant::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_3(v))
        } else {
            let v = ( { Ok(Box::new(ValueArray::parse(rdr, ver)?)) } )?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct OffsetPtr {
    pub data: OffsetPtrDataVariant,
}

impl crate::Parse for OffsetPtr {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let data = OffsetPtrDataVariant::parse(rdr, ver)?;

        Ok(OffsetPtr {
            data,
        })
    }
}
