//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct Grid {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_cell_size: Vector3f,
    pub m_cell_gap: Vector3f,
    pub m_cell_layout: i32,
    pub m_cell_swizzle: i32,
}

impl crate::Parse for Grid {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_cell_size = Vector3f::parse(rdr, ver)?;
        let m_cell_gap = Vector3f::parse(rdr, ver)?;
        let m_cell_layout = rdr.read_i32()?;
        let m_cell_swizzle = rdr.read_i32()?;

        Ok(Grid {
            m_game_object,
            m_enabled,
            m_cell_size,
            m_cell_gap,
            m_cell_layout,
            m_cell_swizzle,
        })
    }
}

impl crate::ToValue for Grid {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_CellSize".to_string(), self.m_cell_size.to_value())),
            Some(("m_CellGap".to_string(), self.m_cell_gap.to_value())),
            Some(("m_CellLayout".to_string(), self.m_cell_layout.to_value())),
            Some(("m_CellSwizzle".to_string(), self.m_cell_swizzle.to_value())),
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
                "m_CellSize" => (self.m_cell_size).inspect_value(path, offset, limit),
                "m_CellGap" => (self.m_cell_gap).inspect_value(path, offset, limit),
                "m_CellLayout" => (self.m_cell_layout).inspect_value(path, offset, limit),
                "m_CellSwizzle" => (self.m_cell_swizzle).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_cell_size).inspect_value(path, offset, limit))
                .map(|v| ("m_CellSize".to_string(), v)),
            ((self.m_cell_gap).inspect_value(path, offset, limit))
                .map(|v| ("m_CellGap".to_string(), v)),
            ((self.m_cell_layout).inspect_value(path, offset, limit))
                .map(|v| ("m_CellLayout".to_string(), v)),
            ((self.m_cell_swizzle).inspect_value(path, offset, limit))
                .map(|v| ("m_CellSwizzle".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
