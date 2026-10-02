//! 自动生成。请勿手动修改。

use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct AutoOffMeshLinkData {
    pub m_start: Vector3f,
    pub m_end: Vector3f,
    pub m_radius: f32,
    pub m_link_type: u16,
    pub m_area: u8,
    pub m_link_direction: u8,
}

impl crate::Parse for AutoOffMeshLinkData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_start = Vector3f::parse(rdr, ver)?;
        let m_end = Vector3f::parse(rdr, ver)?;
        let m_radius = rdr.read_f32()?;
        let m_link_type = rdr.read_u16()?;
        let m_area = rdr.read_u8()?;
        let m_link_direction = rdr.read_u8()?;

        Ok(AutoOffMeshLinkData {
            m_start,
            m_end,
            m_radius,
            m_link_type,
            m_area,
            m_link_direction,
        })
    }
}

impl crate::ToValue for AutoOffMeshLinkData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Start".to_string(), self.m_start.to_value())),
            Some(("m_End".to_string(), self.m_end.to_value())),
            Some(("m_Radius".to_string(), self.m_radius.to_value())),
            Some(("m_LinkType".to_string(), self.m_link_type.to_value())),
            Some(("m_Area".to_string(), self.m_area.to_value())),
            Some((
                "m_LinkDirection".to_string(),
                self.m_link_direction.to_value(),
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
                "m_Start" => (self.m_start).inspect_value(path, offset, limit),
                "m_End" => (self.m_end).inspect_value(path, offset, limit),
                "m_Radius" => (self.m_radius).inspect_value(path, offset, limit),
                "m_LinkType" => (self.m_link_type).inspect_value(path, offset, limit),
                "m_Area" => (self.m_area).inspect_value(path, offset, limit),
                "m_LinkDirection" => (self.m_link_direction).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_start).inspect_value(path, offset, limit)).map(|v| ("m_Start".to_string(), v)),
            ((self.m_end).inspect_value(path, offset, limit)).map(|v| ("m_End".to_string(), v)),
            ((self.m_radius).inspect_value(path, offset, limit))
                .map(|v| ("m_Radius".to_string(), v)),
            ((self.m_link_type).inspect_value(path, offset, limit))
                .map(|v| ("m_LinkType".to_string(), v)),
            ((self.m_area).inspect_value(path, offset, limit)).map(|v| ("m_Area".to_string(), v)),
            ((self.m_link_direction).inspect_value(path, offset, limit))
                .map(|v| ("m_LinkDirection".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
