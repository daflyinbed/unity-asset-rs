//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：TransformMLocalScaleVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TransformMLocalScaleVariant {
    Default(Box<Vector3f>),
    V2021_2(Box<Vector3f>),
}

impl crate::Parse for TransformMLocalScaleVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({ Ok(Box::new(Vector3f::parse(rdr, ver)?)) })?;
            rdr.align(4)?;
            Ok(Self::V2021_2(v))
        } else {
            let v = ({ Ok(Box::new(Vector3f::parse(rdr, ver)?)) })?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：TransformMChildrenVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TransformMChildrenVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for TransformMChildrenVariant {
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
#[derive(Debug, Clone)]
pub struct Transform {
    pub m_game_object: PPtr,
    pub m_local_rotation: Quaternionf,
    pub m_local_position: Vector3f,
    pub m_local_scale: TransformMLocalScaleVariant,
    pub m_children: TransformMChildrenVariant,
    pub m_father: PPtr,
}

impl crate::Parse for Transform {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_local_rotation = Quaternionf::parse(rdr, ver)?;
        let m_local_position = Vector3f::parse(rdr, ver)?;
        let m_local_scale = TransformMLocalScaleVariant::parse(rdr, ver)?;
        let m_children = TransformMChildrenVariant::parse(rdr, ver)?;
        let m_father = crate::PPtr::parse(rdr, ver)?;

        Ok(Transform {
            m_game_object,
            m_local_rotation,
            m_local_position,
            m_local_scale,
            m_children,
            m_father,
        })
    }
}
