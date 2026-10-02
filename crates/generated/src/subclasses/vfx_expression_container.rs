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

impl crate::ToValue for VfxExpressionContainerMNeedsWorldToLocalVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1(v) => (v).inspect_value(path, offset, limit),
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
        m_expressions = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Expression::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
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
        m_needs_world_to_local = Some(VfxExpressionContainerMNeedsWorldToLocalVariant::parse(
            rdr, ver,
        )?);
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

impl crate::ToValue for VfxExpressionContainer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Expressions".to_string(), self.m_expressions.to_value())),
            Some((
                "m_NeedsLocalToWorld".to_string(),
                self.m_needs_local_to_world.to_value(),
            )),
            crate::opt_field(
                "m_MaxCommonExpressionsIndex",
                &self.m_max_common_expressions_index,
            ),
            Some((
                "m_NeedsWorldToLocal".to_string(),
                self.m_needs_world_to_local.to_value(),
            )),
            crate::opt_field(
                "m_ConstantBakeCurveCount",
                &self.m_constant_bake_curve_count,
            ),
            crate::opt_field(
                "m_NeededMainCameraBuffers",
                &self.m_needed_main_camera_buffers,
            ),
            crate::opt_field(
                "m_ConstantBakeGradientCount",
                &self.m_constant_bake_gradient_count,
            ),
            crate::opt_field("m_NeedsMainCamera", &self.m_needs_main_camera),
            crate::opt_field("m_DynamicBakeCurveCount", &self.m_dynamic_bake_curve_count),
            crate::opt_field(
                "m_DynamicBakeGradientCount",
                &self.m_dynamic_bake_gradient_count,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Expressions" => (self.m_expressions).inspect_value(path, offset, limit),
                "m_NeedsLocalToWorld" => {
                    (self.m_needs_local_to_world).inspect_value(path, offset, limit)
                }
                "m_MaxCommonExpressionsIndex" => {
                    (self.m_max_common_expressions_index).inspect_value(path, offset, limit)
                }
                "m_NeedsWorldToLocal" => {
                    (self.m_needs_world_to_local).inspect_value(path, offset, limit)
                }
                "m_ConstantBakeCurveCount" => {
                    (self.m_constant_bake_curve_count).inspect_value(path, offset, limit)
                }
                "m_NeededMainCameraBuffers" => {
                    (self.m_needed_main_camera_buffers).inspect_value(path, offset, limit)
                }
                "m_ConstantBakeGradientCount" => {
                    (self.m_constant_bake_gradient_count).inspect_value(path, offset, limit)
                }
                "m_NeedsMainCamera" => {
                    (self.m_needs_main_camera).inspect_value(path, offset, limit)
                }
                "m_DynamicBakeCurveCount" => {
                    (self.m_dynamic_bake_curve_count).inspect_value(path, offset, limit)
                }
                "m_DynamicBakeGradientCount" => {
                    (self.m_dynamic_bake_gradient_count).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_expressions).inspect_value(path, offset, limit))
                .map(|v| ("m_Expressions".to_string(), v)),
            ((self.m_needs_local_to_world).inspect_value(path, offset, limit))
                .map(|v| ("m_NeedsLocalToWorld".to_string(), v)),
            ((self.m_max_common_expressions_index).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxCommonExpressionsIndex".to_string(), v)),
            ((self.m_needs_world_to_local).inspect_value(path, offset, limit))
                .map(|v| ("m_NeedsWorldToLocal".to_string(), v)),
            ((self.m_constant_bake_curve_count).inspect_value(path, offset, limit))
                .map(|v| ("m_ConstantBakeCurveCount".to_string(), v)),
            ((self.m_needed_main_camera_buffers).inspect_value(path, offset, limit))
                .map(|v| ("m_NeededMainCameraBuffers".to_string(), v)),
            ((self.m_constant_bake_gradient_count).inspect_value(path, offset, limit))
                .map(|v| ("m_ConstantBakeGradientCount".to_string(), v)),
            ((self.m_needs_main_camera).inspect_value(path, offset, limit))
                .map(|v| ("m_NeedsMainCamera".to_string(), v)),
            ((self.m_dynamic_bake_curve_count).inspect_value(path, offset, limit))
                .map(|v| ("m_DynamicBakeCurveCount".to_string(), v)),
            ((self.m_dynamic_bake_gradient_count).inspect_value(path, offset, limit))
                .map(|v| ("m_DynamicBakeGradientCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
