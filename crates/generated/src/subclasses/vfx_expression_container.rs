//! 自动生成。请勿手动修改。

use crate::subclasses::expression::Expression;


/// 版本枚举：VfxExpressionContainerMNeedsWorldToLocalVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VfxExpressionContainerMNeedsWorldToLocalVariant {
    Default(bool),
    V2022_1(bool),
}

impl crate::Parse for VfxExpressionContainerMNeedsWorldToLocalVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2022_1(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct VfxExpressionContainer {
    pub m_expressions: Vec<Expression>,
    pub m_needs_local_to_world: bool,
    pub m_max_common_expressions_index: Option<u32>,
    pub m_needs_world_to_local: VfxExpressionContainerMNeedsWorldToLocalVariant,
    pub m_constant_bake_curve_count: Option<u32>,
    pub m_needed_main_camera_buffers: Option<i32>,
    pub m_constant_bake_gradient_count: Option<u32>,
    pub m_needs_main_camera: Option<bool>,
    pub m_dynamic_bake_curve_count: Option<u32>,
    pub m_dynamic_bake_gradient_count: Option<u32>,
}

impl crate::Parse for VfxExpressionContainer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_expressions = None;
        let mut m_needs_local_to_world = None;
        let mut m_max_common_expressions_index = None;
        let mut m_needs_world_to_local = None;
        let mut m_constant_bake_curve_count = None;
        let mut m_needed_main_camera_buffers = None;
        let mut m_constant_bake_gradient_count = None;
        let mut m_needs_main_camera = None;
        let mut m_dynamic_bake_curve_count = None;
        let mut m_dynamic_bake_gradient_count = None;
            m_expressions = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Expression::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_max_common_expressions_index = Some(rdr.read_u32()?);
            m_constant_bake_curve_count = Some(rdr.read_u32()?);
            m_constant_bake_gradient_count = Some(rdr.read_u32()?);
            m_dynamic_bake_curve_count = Some(rdr.read_u32()?);
            m_dynamic_bake_gradient_count = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            m_max_common_expressions_index = Some(rdr.read_u32()?);
        } else {
        }
            m_needs_local_to_world = Some(rdr.read_bool()?);
            m_needs_world_to_local = Some(VfxExpressionContainerMNeedsWorldToLocalVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_needs_main_camera = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_needed_main_camera_buffers = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_needed_main_camera_buffers = Some(rdr.read_i32()?);
        } else {
        }

        Ok(VfxExpressionContainer {
            m_expressions: m_expressions.unwrap(),
            m_needs_local_to_world: m_needs_local_to_world.unwrap(),
            m_max_common_expressions_index,
            m_needs_world_to_local: m_needs_world_to_local.unwrap(),
            m_constant_bake_curve_count,
            m_needed_main_camera_buffers,
            m_constant_bake_gradient_count,
            m_needs_main_camera,
            m_dynamic_bake_curve_count,
            m_dynamic_bake_gradient_count,
        })
    }
}
