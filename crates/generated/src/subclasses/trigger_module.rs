//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TriggerModule {
    pub enabled: bool,
    pub collision_shape0: Option<PPtr>,
    pub inside: i32,
    pub collision_shape1: Option<PPtr>,
    pub outside: i32,
    pub collision_shape2: Option<PPtr>,
    pub enter: i32,
    pub collision_shape3: Option<PPtr>,
    pub exit: i32,
    pub collision_shape4: Option<PPtr>,
    pub collider_query_mode: Option<i32>,
    pub collision_shape5: Option<PPtr>,
    pub radius_scale: f32,
    pub primitives: Option<Vec<PPtr>>,
}

impl crate::Parse for TriggerModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut collision_shape0 = None;
        let mut inside = None;
        let mut collision_shape1 = None;
        let mut outside = None;
        let mut collision_shape2 = None;
        let mut enter = None;
        let mut collision_shape3 = None;
        let mut exit = None;
        let mut collision_shape4 = None;
        let mut collider_query_mode = None;
        let mut collision_shape5 = None;
        let mut radius_scale = None;
        let mut primitives = None;
        enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
        } else {
            collision_shape0 = Some(crate::PPtr::parse(rdr, ver)?);
            collision_shape1 = Some(crate::PPtr::parse(rdr, ver)?);
            collision_shape2 = Some(crate::PPtr::parse(rdr, ver)?);
            collision_shape3 = Some(crate::PPtr::parse(rdr, ver)?);
            collision_shape4 = Some(crate::PPtr::parse(rdr, ver)?);
            collision_shape5 = Some(crate::PPtr::parse(rdr, ver)?);
        }
        inside = Some(rdr.read_i32()?);
        outside = Some(rdr.read_i32()?);
        enter = Some(rdr.read_i32()?);
        exit = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            collider_query_mode = Some(rdr.read_i32()?);
        } else {
        }
        radius_scale = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            primitives = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }

        Ok(TriggerModule {
            enabled: enabled.unwrap(),
            collision_shape0,
            inside: inside.unwrap(),
            collision_shape1,
            outside: outside.unwrap(),
            collision_shape2,
            enter: enter.unwrap(),
            collision_shape3,
            exit: exit.unwrap(),
            collision_shape4,
            collider_query_mode,
            collision_shape5,
            radius_scale: radius_scale.unwrap(),
            primitives,
        })
    }
}

impl crate::ToValue for TriggerModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            crate::opt_field("collisionShape0", &self.collision_shape0),
            Some(("inside".to_string(), self.inside.to_value())),
            crate::opt_field("collisionShape1", &self.collision_shape1),
            Some(("outside".to_string(), self.outside.to_value())),
            crate::opt_field("collisionShape2", &self.collision_shape2),
            Some(("enter".to_string(), self.enter.to_value())),
            crate::opt_field("collisionShape3", &self.collision_shape3),
            Some(("exit".to_string(), self.exit.to_value())),
            crate::opt_field("collisionShape4", &self.collision_shape4),
            crate::opt_field("colliderQueryMode", &self.collider_query_mode),
            crate::opt_field("collisionShape5", &self.collision_shape5),
            Some(("radiusScale".to_string(), self.radius_scale.to_value())),
            crate::opt_field("primitives", &self.primitives),
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
                "collisionShape0" => (self.collision_shape0).inspect_value(path, offset, limit),
                "inside" => (self.inside).inspect_value(path, offset, limit),
                "collisionShape1" => (self.collision_shape1).inspect_value(path, offset, limit),
                "outside" => (self.outside).inspect_value(path, offset, limit),
                "collisionShape2" => (self.collision_shape2).inspect_value(path, offset, limit),
                "enter" => (self.enter).inspect_value(path, offset, limit),
                "collisionShape3" => (self.collision_shape3).inspect_value(path, offset, limit),
                "exit" => (self.exit).inspect_value(path, offset, limit),
                "collisionShape4" => (self.collision_shape4).inspect_value(path, offset, limit),
                "colliderQueryMode" => {
                    (self.collider_query_mode).inspect_value(path, offset, limit)
                }
                "collisionShape5" => (self.collision_shape5).inspect_value(path, offset, limit),
                "radiusScale" => (self.radius_scale).inspect_value(path, offset, limit),
                "primitives" => (self.primitives).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.collision_shape0).inspect_value(path, offset, limit))
                .map(|v| ("collisionShape0".to_string(), v)),
            ((self.inside).inspect_value(path, offset, limit)).map(|v| ("inside".to_string(), v)),
            ((self.collision_shape1).inspect_value(path, offset, limit))
                .map(|v| ("collisionShape1".to_string(), v)),
            ((self.outside).inspect_value(path, offset, limit)).map(|v| ("outside".to_string(), v)),
            ((self.collision_shape2).inspect_value(path, offset, limit))
                .map(|v| ("collisionShape2".to_string(), v)),
            ((self.enter).inspect_value(path, offset, limit)).map(|v| ("enter".to_string(), v)),
            ((self.collision_shape3).inspect_value(path, offset, limit))
                .map(|v| ("collisionShape3".to_string(), v)),
            ((self.exit).inspect_value(path, offset, limit)).map(|v| ("exit".to_string(), v)),
            ((self.collision_shape4).inspect_value(path, offset, limit))
                .map(|v| ("collisionShape4".to_string(), v)),
            ((self.collider_query_mode).inspect_value(path, offset, limit))
                .map(|v| ("colliderQueryMode".to_string(), v)),
            ((self.collision_shape5).inspect_value(path, offset, limit))
                .map(|v| ("collisionShape5".to_string(), v)),
            ((self.radius_scale).inspect_value(path, offset, limit))
                .map(|v| ("radiusScale".to_string(), v)),
            ((self.primitives).inspect_value(path, offset, limit))
                .map(|v| ("primitives".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
