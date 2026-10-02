//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct UpdateZoneInfo {
    pub update_zone_center: Vector3f,
    pub update_zone_size: Vector3f,
    pub rotation: f32,
    pub pass_index: i32,
    pub need_swap: bool,
}

impl crate::Parse for UpdateZoneInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let update_zone_center = Vector3f::parse(rdr, ver)?;
        let update_zone_size = Vector3f::parse(rdr, ver)?;
        let rotation = rdr.read_f32()?;
        let pass_index = rdr.read_i32()?;
        let need_swap = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(UpdateZoneInfo {
            update_zone_center,
            update_zone_size,
            rotation,
            pass_index,
            need_swap,
        })
    }
}

impl crate::ToValue for UpdateZoneInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "updateZoneCenter".to_string(),
                self.update_zone_center.to_value(),
            )),
            Some((
                "updateZoneSize".to_string(),
                self.update_zone_size.to_value(),
            )),
            Some(("rotation".to_string(), self.rotation.to_value())),
            Some(("passIndex".to_string(), self.pass_index.to_value())),
            Some(("needSwap".to_string(), self.need_swap.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "updateZoneCenter" => (self.update_zone_center).inspect_value(path, offset, limit),
                "updateZoneSize" => (self.update_zone_size).inspect_value(path, offset, limit),
                "rotation" => (self.rotation).inspect_value(path, offset, limit),
                "passIndex" => (self.pass_index).inspect_value(path, offset, limit),
                "needSwap" => (self.need_swap).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.update_zone_center).inspect_value(path, offset, limit))
                .map(|v| ("updateZoneCenter".to_string(), v)),
            ((self.update_zone_size).inspect_value(path, offset, limit))
                .map(|v| ("updateZoneSize".to_string(), v)),
            ((self.rotation).inspect_value(path, offset, limit))
                .map(|v| ("rotation".to_string(), v)),
            ((self.pass_index).inspect_value(path, offset, limit))
                .map(|v| ("passIndex".to_string(), v)),
            ((self.need_swap).inspect_value(path, offset, limit))
                .map(|v| ("needSwap".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
