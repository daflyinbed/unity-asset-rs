//! 自动生成。请勿手动修改。

use crate::subclasses::offset_ptr::OffsetPtr;


/// 版本枚举：BlendTreeConstantMNodeArrayVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum BlendTreeConstantMNodeArrayVariant {
    Default(Vec<OffsetPtr>),
    V4_1_3(Vec<OffsetPtr>),
}

impl crate::Parse for BlendTreeConstantMNodeArrayVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 1, 3) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V4_1_3(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { OffsetPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct BlendTreeConstant {
    pub m_node_array: BlendTreeConstantMNodeArrayVariant,
    pub m_blend_event_array_constant: Option<OffsetPtr>,
}

impl crate::Parse for BlendTreeConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_node_array = None;
        let mut m_blend_event_array_constant = None;
            m_node_array = Some(BlendTreeConstantMNodeArrayVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
        } else {
            m_blend_event_array_constant = Some(OffsetPtr::parse(rdr, ver)?);
        }

        Ok(BlendTreeConstant {
            m_node_array: m_node_array.unwrap(),
            m_blend_event_array_constant,
        })
    }
}
