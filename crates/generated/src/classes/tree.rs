//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Tree {
    pub m_game_object: PPtr,
    pub m_speed_tree_wind_asset: Option<PPtr>,
}

impl crate::Parse for Tree {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_speed_tree_wind_asset = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(crate::PPtr::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(Tree {
            m_game_object,
            m_speed_tree_wind_asset,
        })
    }
}

impl crate::ToValue for Tree {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            crate::opt_field("m_SpeedTreeWindAsset", &self.m_speed_tree_wind_asset),
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
                "m_SpeedTreeWindAsset" => {
                    (self.m_speed_tree_wind_asset).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_speed_tree_wind_asset).inspect_value(path, offset, limit))
                .map(|v| ("m_SpeedTreeWindAsset".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
