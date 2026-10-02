//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct OcclusionPortal {
    pub m_game_object: PPtr,
    pub m_open: bool,
    pub m_center: Vector3f,
    pub m_size: Vector3f,
}

impl crate::Parse for OcclusionPortal {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_open = rdr.read_bool()?;
        rdr.align(4)?;
        let m_center = Vector3f::parse(rdr, ver)?;
        let m_size = Vector3f::parse(rdr, ver)?;

        Ok(OcclusionPortal {
            m_game_object,
            m_open,
            m_center,
            m_size,
        })
    }
}

impl crate::ToValue for OcclusionPortal {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Open".to_string(), self.m_open.to_value())),
            Some(("m_Center".to_string(), self.m_center.to_value())),
            Some(("m_Size".to_string(), self.m_size.to_value())),
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
                "m_Open" => (self.m_open).inspect_value(path, offset, limit),
                "m_Center" => (self.m_center).inspect_value(path, offset, limit),
                "m_Size" => (self.m_size).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_open).inspect_value(path, offset, limit)).map(|v| ("m_Open".to_string(), v)),
            ((self.m_center).inspect_value(path, offset, limit))
                .map(|v| ("m_Center".to_string(), v)),
            ((self.m_size).inspect_value(path, offset, limit)).map(|v| ("m_Size".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
