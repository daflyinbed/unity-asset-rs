//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::network_view_id::NetworkViewId;

#[derive(Debug, Clone)]
pub struct NetworkView {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_state_synchronization: i32,
    pub m_observed: PPtr,
    pub m_view_id: NetworkViewId,
}

impl crate::Parse for NetworkView {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_state_synchronization = rdr.read_i32()?;
        let m_observed = crate::PPtr::parse(rdr, ver)?;
        let m_view_id = NetworkViewId::parse(rdr, ver)?;

        Ok(NetworkView {
            m_game_object,
            m_enabled,
            m_state_synchronization,
            m_observed,
            m_view_id,
        })
    }
}

impl crate::ToValue for NetworkView {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some((
                "m_StateSynchronization".to_string(),
                self.m_state_synchronization.to_value(),
            )),
            Some(("m_Observed".to_string(), self.m_observed.to_value())),
            Some(("m_ViewID".to_string(), self.m_view_id.to_value())),
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
                "m_StateSynchronization" => {
                    (self.m_state_synchronization).inspect_value(path, offset, limit)
                }
                "m_Observed" => (self.m_observed).inspect_value(path, offset, limit),
                "m_ViewID" => (self.m_view_id).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_state_synchronization).inspect_value(path, offset, limit))
                .map(|v| ("m_StateSynchronization".to_string(), v)),
            ((self.m_observed).inspect_value(path, offset, limit))
                .map(|v| ("m_Observed".to_string(), v)),
            ((self.m_view_id).inspect_value(path, offset, limit))
                .map(|v| ("m_ViewID".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
