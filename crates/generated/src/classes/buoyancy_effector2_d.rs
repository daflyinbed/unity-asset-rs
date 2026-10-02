//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;

#[derive(Debug, Clone)]
pub struct BuoyancyEffector2D {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_use_collider_mask: bool,
    pub m_collider_mask: BitField,
    pub m_surface_level: f32,
    pub m_density: f32,
    pub m_linear_drag: Option<f32>,
    pub m_linear_damping: Option<f32>,
    pub m_angular_drag: Option<f32>,
    pub m_angular_damping: Option<f32>,
    pub m_flow_angle: f32,
    pub m_flow_magnitude: f32,
    pub m_flow_variation: f32,
}

impl crate::Parse for BuoyancyEffector2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_use_collider_mask = None;
        let mut m_collider_mask = None;
        let mut m_surface_level = None;
        let mut m_density = None;
        let mut m_linear_drag = None;
        let mut m_linear_damping = None;
        let mut m_angular_drag = None;
        let mut m_angular_damping = None;
        let mut m_flow_angle = None;
        let mut m_flow_magnitude = None;
        let mut m_flow_variation = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_use_collider_mask = Some(rdr.read_bool()?);
        rdr.align(4)?;
        m_collider_mask = Some(BitField::parse(rdr, ver)?);
        m_surface_level = Some(rdr.read_f32()?);
        m_density = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            m_linear_damping = Some(rdr.read_f32()?);
            m_angular_damping = Some(rdr.read_f32()?);
        } else {
            m_linear_drag = Some(rdr.read_f32()?);
            m_angular_drag = Some(rdr.read_f32()?);
        }
        m_flow_angle = Some(rdr.read_f32()?);
        m_flow_magnitude = Some(rdr.read_f32()?);
        m_flow_variation = Some(rdr.read_f32()?);

        Ok(BuoyancyEffector2D {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_use_collider_mask: m_use_collider_mask.unwrap(),
            m_collider_mask: m_collider_mask.unwrap(),
            m_surface_level: m_surface_level.unwrap(),
            m_density: m_density.unwrap(),
            m_linear_drag,
            m_linear_damping,
            m_angular_drag,
            m_angular_damping,
            m_flow_angle: m_flow_angle.unwrap(),
            m_flow_magnitude: m_flow_magnitude.unwrap(),
            m_flow_variation: m_flow_variation.unwrap(),
        })
    }
}

impl crate::ToValue for BuoyancyEffector2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some((
                "m_UseColliderMask".to_string(),
                self.m_use_collider_mask.to_value(),
            )),
            Some((
                "m_ColliderMask".to_string(),
                self.m_collider_mask.to_value(),
            )),
            Some((
                "m_SurfaceLevel".to_string(),
                self.m_surface_level.to_value(),
            )),
            Some(("m_Density".to_string(), self.m_density.to_value())),
            crate::opt_field("m_LinearDrag", &self.m_linear_drag),
            crate::opt_field("m_LinearDamping", &self.m_linear_damping),
            crate::opt_field("m_AngularDrag", &self.m_angular_drag),
            crate::opt_field("m_AngularDamping", &self.m_angular_damping),
            Some(("m_FlowAngle".to_string(), self.m_flow_angle.to_value())),
            Some((
                "m_FlowMagnitude".to_string(),
                self.m_flow_magnitude.to_value(),
            )),
            Some((
                "m_FlowVariation".to_string(),
                self.m_flow_variation.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_UseColliderMask" => {
                    (self.m_use_collider_mask).inspect_value(path, offset, limit)
                }
                "m_ColliderMask" => (self.m_collider_mask).inspect_value(path, offset, limit),
                "m_SurfaceLevel" => (self.m_surface_level).inspect_value(path, offset, limit),
                "m_Density" => (self.m_density).inspect_value(path, offset, limit),
                "m_LinearDrag" => (self.m_linear_drag).inspect_value(path, offset, limit),
                "m_LinearDamping" => (self.m_linear_damping).inspect_value(path, offset, limit),
                "m_AngularDrag" => (self.m_angular_drag).inspect_value(path, offset, limit),
                "m_AngularDamping" => (self.m_angular_damping).inspect_value(path, offset, limit),
                "m_FlowAngle" => (self.m_flow_angle).inspect_value(path, offset, limit),
                "m_FlowMagnitude" => (self.m_flow_magnitude).inspect_value(path, offset, limit),
                "m_FlowVariation" => (self.m_flow_variation).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_use_collider_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_UseColliderMask".to_string(), v)),
            ((self.m_collider_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_ColliderMask".to_string(), v)),
            ((self.m_surface_level).inspect_value(path, offset, limit))
                .map(|v| ("m_SurfaceLevel".to_string(), v)),
            ((self.m_density).inspect_value(path, offset, limit))
                .map(|v| ("m_Density".to_string(), v)),
            ((self.m_linear_drag).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearDrag".to_string(), v)),
            ((self.m_linear_damping).inspect_value(path, offset, limit))
                .map(|v| ("m_LinearDamping".to_string(), v)),
            ((self.m_angular_drag).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularDrag".to_string(), v)),
            ((self.m_angular_damping).inspect_value(path, offset, limit))
                .map(|v| ("m_AngularDamping".to_string(), v)),
            ((self.m_flow_angle).inspect_value(path, offset, limit))
                .map(|v| ("m_FlowAngle".to_string(), v)),
            ((self.m_flow_magnitude).inspect_value(path, offset, limit))
                .map(|v| ("m_FlowMagnitude".to_string(), v)),
            ((self.m_flow_variation).inspect_value(path, offset, limit))
                .map(|v| ("m_FlowVariation".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
