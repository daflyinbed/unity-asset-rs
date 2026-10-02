//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ClothAttachment {
    pub m_collider: PPtr,
    pub m_two_way_interaction: bool,
    pub m_tearable: bool,
}

impl crate::Parse for ClothAttachment {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_collider = crate::PPtr::parse(rdr, ver)?;
        let m_two_way_interaction = rdr.read_bool()?;
        let m_tearable = rdr.read_bool()?;

        Ok(ClothAttachment {
            m_collider,
            m_two_way_interaction,
            m_tearable,
        })
    }
}

impl crate::ToValue for ClothAttachment {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Collider".to_string(), self.m_collider.to_value())),
            Some((
                "m_TwoWayInteraction".to_string(),
                self.m_two_way_interaction.to_value(),
            )),
            Some(("m_Tearable".to_string(), self.m_tearable.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Collider" => (self.m_collider).inspect_value(path, offset, limit),
                "m_TwoWayInteraction" => {
                    (self.m_two_way_interaction).inspect_value(path, offset, limit)
                }
                "m_Tearable" => (self.m_tearable).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_collider).inspect_value(path, offset, limit))
                .map(|v| ("m_Collider".to_string(), v)),
            ((self.m_two_way_interaction).inspect_value(path, offset, limit))
                .map(|v| ("m_TwoWayInteraction".to_string(), v)),
            ((self.m_tearable).inspect_value(path, offset, limit))
                .map(|v| ("m_Tearable".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
