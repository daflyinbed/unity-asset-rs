//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;

/// 版本枚举：ClampVelocityModuleSeparateAxisVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClampVelocityModuleSeparateAxisVariant {
    Default(bool),
    V4_0(bool),
}

impl crate::Parse for ClampVelocityModuleSeparateAxisVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V4_0(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ClampVelocityModuleSeparateAxisVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V4_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V4_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ClampVelocityModuleInWorldSpaceVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClampVelocityModuleInWorldSpaceVariant {
    Default(bool),
    V2017_2(bool),
}

impl crate::Parse for ClampVelocityModuleInWorldSpaceVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2017_2(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ClampVelocityModuleInWorldSpaceVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClampVelocityModule {
    pub enabled: bool,
    pub x: MinMaxCurve,
    pub y: MinMaxCurve,
    pub z: MinMaxCurve,
    pub magnitude: MinMaxCurve,
    pub separate_axis: ClampVelocityModuleSeparateAxisVariant,
    pub dampen: f32,
    pub in_world_space: Option<ClampVelocityModuleInWorldSpaceVariant>,
    pub multiply_drag_by_particle_size: Option<bool>,
    pub multiply_drag_by_particle_velocity: Option<bool>,
    pub drag: Option<MinMaxCurve>,
}

impl crate::Parse for ClampVelocityModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut x = None;
        let mut y = None;
        let mut z = None;
        let mut magnitude = None;
        let mut separate_axis = None;
        let mut dampen = None;
        let mut in_world_space = None;
        let mut multiply_drag_by_particle_size = None;
        let mut multiply_drag_by_particle_velocity = None;
        let mut drag = None;
        enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        x = Some(MinMaxCurve::parse(rdr, ver)?);
        y = Some(MinMaxCurve::parse(rdr, ver)?);
        z = Some(MinMaxCurve::parse(rdr, ver)?);
        magnitude = Some(MinMaxCurve::parse(rdr, ver)?);
        separate_axis = Some(ClampVelocityModuleSeparateAxisVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            in_world_space = Some(ClampVelocityModuleInWorldSpaceVariant::parse(rdr, ver)?);
            multiply_drag_by_particle_size = Some(rdr.read_bool()?);
            multiply_drag_by_particle_velocity = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            in_world_space = Some(ClampVelocityModuleInWorldSpaceVariant::parse(rdr, ver)?);
        } else {
        }
        dampen = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            drag = Some(MinMaxCurve::parse(rdr, ver)?);
        } else {
        }

        Ok(ClampVelocityModule {
            enabled: enabled.unwrap(),
            x: x.unwrap(),
            y: y.unwrap(),
            z: z.unwrap(),
            magnitude: magnitude.unwrap(),
            separate_axis: separate_axis.unwrap(),
            dampen: dampen.unwrap(),
            in_world_space,
            multiply_drag_by_particle_size,
            multiply_drag_by_particle_velocity,
            drag,
        })
    }
}

impl crate::ToValue for ClampVelocityModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("x".to_string(), self.x.to_value())),
            Some(("y".to_string(), self.y.to_value())),
            Some(("z".to_string(), self.z.to_value())),
            Some(("magnitude".to_string(), self.magnitude.to_value())),
            Some(("separateAxis".to_string(), self.separate_axis.to_value())),
            Some(("dampen".to_string(), self.dampen.to_value())),
            crate::opt_field("inWorldSpace", &self.in_world_space),
            crate::opt_field(
                "multiplyDragByParticleSize",
                &self.multiply_drag_by_particle_size,
            ),
            crate::opt_field(
                "multiplyDragByParticleVelocity",
                &self.multiply_drag_by_particle_velocity,
            ),
            crate::opt_field("drag", &self.drag),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enabled" => (self.enabled).inspect_value(path, offset, limit),
                "x" => (self.x).inspect_value(path, offset, limit),
                "y" => (self.y).inspect_value(path, offset, limit),
                "z" => (self.z).inspect_value(path, offset, limit),
                "magnitude" => (self.magnitude).inspect_value(path, offset, limit),
                "separateAxis" => (self.separate_axis).inspect_value(path, offset, limit),
                "dampen" => (self.dampen).inspect_value(path, offset, limit),
                "inWorldSpace" => (self.in_world_space).inspect_value(path, offset, limit),
                "multiplyDragByParticleSize" => {
                    (self.multiply_drag_by_particle_size).inspect_value(path, offset, limit)
                }
                "multiplyDragByParticleVelocity" => {
                    (self.multiply_drag_by_particle_velocity).inspect_value(path, offset, limit)
                }
                "drag" => (self.drag).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.x).inspect_value(path, offset, limit)).map(|v| ("x".to_string(), v)),
            ((self.y).inspect_value(path, offset, limit)).map(|v| ("y".to_string(), v)),
            ((self.z).inspect_value(path, offset, limit)).map(|v| ("z".to_string(), v)),
            ((self.magnitude).inspect_value(path, offset, limit))
                .map(|v| ("magnitude".to_string(), v)),
            ((self.separate_axis).inspect_value(path, offset, limit))
                .map(|v| ("separateAxis".to_string(), v)),
            ((self.dampen).inspect_value(path, offset, limit)).map(|v| ("dampen".to_string(), v)),
            ((self.in_world_space).inspect_value(path, offset, limit))
                .map(|v| ("inWorldSpace".to_string(), v)),
            ((self.multiply_drag_by_particle_size).inspect_value(path, offset, limit))
                .map(|v| ("multiplyDragByParticleSize".to_string(), v)),
            ((self.multiply_drag_by_particle_velocity).inspect_value(path, offset, limit))
                .map(|v| ("multiplyDragByParticleVelocity".to_string(), v)),
            ((self.drag).inspect_value(path, offset, limit)).map(|v| ("drag".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
