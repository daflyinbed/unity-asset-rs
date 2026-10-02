//! 自动生成。请勿手动修改。

use crate::PPtr;

/// 版本枚举：SortingGroupMSortingOrderVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SortingGroupMSortingOrderVariant {
    Default(i16),
    V2022_1(i16),
}

impl crate::Parse for SortingGroupMSortingOrderVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = rdr.read_i16()?;
            Ok(Self::V2022_1(v))
        } else {
            let v = rdr.read_i16()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SortingGroupMSortingOrderVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：SortingGroupMSortAtRootVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SortingGroupMSortAtRootVariant {
    Default(bool),
    V6000_3(bool),
}

impl crate::Parse for SortingGroupMSortAtRootVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SortingGroupMSortAtRootVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SortingGroup {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_sorting_layer_id: i32,
    pub m_sorting_layer: i16,
    pub m_sorting_order: SortingGroupMSortingOrderVariant,
    pub m_sort_at_root: Option<SortingGroupMSortAtRootVariant>,
    pub m_sort3_d_as2_d: Option<bool>,
}

impl crate::Parse for SortingGroup {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_sorting_layer_id = rdr.read_i32()?;
        let m_sorting_layer = rdr.read_i16()?;
        let m_sorting_order = SortingGroupMSortingOrderVariant::parse(rdr, ver)?;
        let m_sort_at_root = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(SortingGroupMSortAtRootVariant::parse(rdr, ver)?)
        } else {
            None
        };
        let m_sort3_d_as2_d = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SortingGroup {
            m_game_object,
            m_enabled,
            m_sorting_layer_id,
            m_sorting_layer,
            m_sorting_order,
            m_sort_at_root,
            m_sort3_d_as2_d,
        })
    }
}

impl crate::ToValue for SortingGroup {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some((
                "m_SortingLayerID".to_string(),
                self.m_sorting_layer_id.to_value(),
            )),
            Some((
                "m_SortingLayer".to_string(),
                self.m_sorting_layer.to_value(),
            )),
            Some((
                "m_SortingOrder".to_string(),
                self.m_sorting_order.to_value(),
            )),
            crate::opt_field("m_SortAtRoot", &self.m_sort_at_root),
            crate::opt_field("m_Sort3DAs2D", &self.m_sort3_d_as2_d),
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
                "m_SortingLayerID" => (self.m_sorting_layer_id).inspect_value(path, offset, limit),
                "m_SortingLayer" => (self.m_sorting_layer).inspect_value(path, offset, limit),
                "m_SortingOrder" => (self.m_sorting_order).inspect_value(path, offset, limit),
                "m_SortAtRoot" => (self.m_sort_at_root).inspect_value(path, offset, limit),
                "m_Sort3DAs2D" => (self.m_sort3_d_as2_d).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_sorting_layer_id).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayerID".to_string(), v)),
            ((self.m_sorting_layer).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayer".to_string(), v)),
            ((self.m_sorting_order).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingOrder".to_string(), v)),
            ((self.m_sort_at_root).inspect_value(path, offset, limit))
                .map(|v| ("m_SortAtRoot".to_string(), v)),
            ((self.m_sort3_d_as2_d).inspect_value(path, offset, limit))
                .map(|v| ("m_Sort3DAs2D".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
