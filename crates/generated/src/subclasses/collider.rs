//! 自动生成。请勿手动修改。

use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct Collider {
    pub m_x: xform,
    pub m_type: u32,
    pub m_x_motion_type: u32,
    pub m_y_motion_type: u32,
    pub m_z_motion_type: u32,
    pub m_min_limit_x: f32,
    pub m_max_limit_x: f32,
    pub m_max_limit_y: f32,
    pub m_max_limit_z: f32,
}

impl crate::Parse for Collider {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_x = xform::parse(rdr, ver)?;
        let m_type = rdr.read_u32()?;
        let m_x_motion_type = rdr.read_u32()?;
        let m_y_motion_type = rdr.read_u32()?;
        let m_z_motion_type = rdr.read_u32()?;
        let m_min_limit_x = rdr.read_f32()?;
        let m_max_limit_x = rdr.read_f32()?;
        let m_max_limit_y = rdr.read_f32()?;
        let m_max_limit_z = rdr.read_f32()?;

        Ok(Collider {
            m_x,
            m_type,
            m_x_motion_type,
            m_y_motion_type,
            m_z_motion_type,
            m_min_limit_x,
            m_max_limit_x,
            m_max_limit_y,
            m_max_limit_z,
        })
    }
}

impl crate::ToValue for Collider {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_X".to_string(), self.m_x.to_value())),
            Some(("m_Type".to_string(), self.m_type.to_value())),
            Some(("m_XMotionType".to_string(), self.m_x_motion_type.to_value())),
            Some(("m_YMotionType".to_string(), self.m_y_motion_type.to_value())),
            Some(("m_ZMotionType".to_string(), self.m_z_motion_type.to_value())),
            Some(("m_MinLimitX".to_string(), self.m_min_limit_x.to_value())),
            Some(("m_MaxLimitX".to_string(), self.m_max_limit_x.to_value())),
            Some(("m_MaxLimitY".to_string(), self.m_max_limit_y.to_value())),
            Some(("m_MaxLimitZ".to_string(), self.m_max_limit_z.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_X" => (self.m_x).inspect_value(path, offset, limit),
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                "m_XMotionType" => (self.m_x_motion_type).inspect_value(path, offset, limit),
                "m_YMotionType" => (self.m_y_motion_type).inspect_value(path, offset, limit),
                "m_ZMotionType" => (self.m_z_motion_type).inspect_value(path, offset, limit),
                "m_MinLimitX" => (self.m_min_limit_x).inspect_value(path, offset, limit),
                "m_MaxLimitX" => (self.m_max_limit_x).inspect_value(path, offset, limit),
                "m_MaxLimitY" => (self.m_max_limit_y).inspect_value(path, offset, limit),
                "m_MaxLimitZ" => (self.m_max_limit_z).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_x).inspect_value(path, offset, limit)).map(|v| ("m_X".to_string(), v)),
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
            ((self.m_x_motion_type).inspect_value(path, offset, limit))
                .map(|v| ("m_XMotionType".to_string(), v)),
            ((self.m_y_motion_type).inspect_value(path, offset, limit))
                .map(|v| ("m_YMotionType".to_string(), v)),
            ((self.m_z_motion_type).inspect_value(path, offset, limit))
                .map(|v| ("m_ZMotionType".to_string(), v)),
            ((self.m_min_limit_x).inspect_value(path, offset, limit))
                .map(|v| ("m_MinLimitX".to_string(), v)),
            ((self.m_max_limit_x).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxLimitX".to_string(), v)),
            ((self.m_max_limit_y).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxLimitY".to_string(), v)),
            ((self.m_max_limit_z).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxLimitZ".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
