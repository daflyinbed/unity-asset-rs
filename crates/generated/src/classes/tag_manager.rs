//! 自动生成。请勿手动修改。

use crate::subclasses::sorting_layer_entry::SortingLayerEntry;

/// 版本枚举：TagManagerTagsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TagManagerTagsVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for TagManagerTagsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for TagManagerTagsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：TagManagerLayersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TagManagerLayersVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for TagManagerLayersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for TagManagerLayersVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：TagManagerMSortingLayersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum TagManagerMSortingLayersVariant {
    Default(Vec<SortingLayerEntry>),
    V2017_1(Vec<SortingLayerEntry>),
}

impl crate::Parse for TagManagerMSortingLayersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SortingLayerEntry::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SortingLayerEntry::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for TagManagerMSortingLayersVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TagManager {
    pub tags: TagManagerTagsVariant,
    pub builtin_layer_0: Option<String>,
    pub layers: Option<TagManagerLayersVariant>,
    pub builtin_layer_1: Option<String>,
    pub m_sorting_layers: Option<TagManagerMSortingLayersVariant>,
    pub builtin_layer_2: Option<String>,
    pub m_rendering_layers: Option<Vec<String>>,
    pub builtin_layer_3: Option<String>,
    pub builtin_layer_4: Option<String>,
    pub builtin_layer_5: Option<String>,
    pub builtin_layer_6: Option<String>,
    pub builtin_layer_7: Option<String>,
    pub user_layer_8: Option<String>,
    pub user_layer_9: Option<String>,
    pub user_layer_10: Option<String>,
    pub user_layer_11: Option<String>,
    pub user_layer_12: Option<String>,
    pub user_layer_13: Option<String>,
    pub user_layer_14: Option<String>,
    pub user_layer_15: Option<String>,
    pub user_layer_16: Option<String>,
    pub user_layer_17: Option<String>,
    pub user_layer_18: Option<String>,
    pub user_layer_19: Option<String>,
    pub user_layer_20: Option<String>,
    pub user_layer_21: Option<String>,
    pub user_layer_22: Option<String>,
    pub user_layer_23: Option<String>,
    pub user_layer_24: Option<String>,
    pub user_layer_25: Option<String>,
    pub user_layer_26: Option<String>,
    pub user_layer_27: Option<String>,
    pub user_layer_28: Option<String>,
    pub user_layer_29: Option<String>,
    pub user_layer_30: Option<String>,
    pub user_layer_31: Option<String>,
}

impl crate::Parse for TagManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut tags = None;
        let mut builtin_layer_0 = None;
        let mut layers = None;
        let mut builtin_layer_1 = None;
        let mut m_sorting_layers = None;
        let mut builtin_layer_2 = None;
        let mut m_rendering_layers = None;
        let mut builtin_layer_3 = None;
        let mut builtin_layer_4 = None;
        let mut builtin_layer_5 = None;
        let mut builtin_layer_6 = None;
        let mut builtin_layer_7 = None;
        let mut user_layer_8 = None;
        let mut user_layer_9 = None;
        let mut user_layer_10 = None;
        let mut user_layer_11 = None;
        let mut user_layer_12 = None;
        let mut user_layer_13 = None;
        let mut user_layer_14 = None;
        let mut user_layer_15 = None;
        let mut user_layer_16 = None;
        let mut user_layer_17 = None;
        let mut user_layer_18 = None;
        let mut user_layer_19 = None;
        let mut user_layer_20 = None;
        let mut user_layer_21 = None;
        let mut user_layer_22 = None;
        let mut user_layer_23 = None;
        let mut user_layer_24 = None;
        let mut user_layer_25 = None;
        let mut user_layer_26 = None;
        let mut user_layer_27 = None;
        let mut user_layer_28 = None;
        let mut user_layer_29 = None;
        let mut user_layer_30 = None;
        let mut user_layer_31 = None;
        tags = Some(TagManagerTagsVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            layers = Some(TagManagerLayersVariant::parse(rdr, ver)?);
            m_sorting_layers = Some(TagManagerMSortingLayersVariant::parse(rdr, ver)?);
            m_rendering_layers = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_aligned_string())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            layers = Some(TagManagerLayersVariant::parse(rdr, ver)?);
            m_sorting_layers = Some(TagManagerMSortingLayersVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            builtin_layer_0 = Some(rdr.read_aligned_string()?);
            builtin_layer_1 = Some(rdr.read_aligned_string()?);
            builtin_layer_2 = Some(rdr.read_aligned_string()?);
            builtin_layer_3 = Some(rdr.read_aligned_string()?);
            builtin_layer_4 = Some(rdr.read_aligned_string()?);
            builtin_layer_5 = Some(rdr.read_aligned_string()?);
            builtin_layer_6 = Some(rdr.read_aligned_string()?);
            builtin_layer_7 = Some(rdr.read_aligned_string()?);
            user_layer_8 = Some(rdr.read_aligned_string()?);
            user_layer_9 = Some(rdr.read_aligned_string()?);
            user_layer_10 = Some(rdr.read_aligned_string()?);
            user_layer_11 = Some(rdr.read_aligned_string()?);
            user_layer_12 = Some(rdr.read_aligned_string()?);
            user_layer_13 = Some(rdr.read_aligned_string()?);
            user_layer_14 = Some(rdr.read_aligned_string()?);
            user_layer_15 = Some(rdr.read_aligned_string()?);
            user_layer_16 = Some(rdr.read_aligned_string()?);
            user_layer_17 = Some(rdr.read_aligned_string()?);
            user_layer_18 = Some(rdr.read_aligned_string()?);
            user_layer_19 = Some(rdr.read_aligned_string()?);
            user_layer_20 = Some(rdr.read_aligned_string()?);
            user_layer_21 = Some(rdr.read_aligned_string()?);
            user_layer_22 = Some(rdr.read_aligned_string()?);
            user_layer_23 = Some(rdr.read_aligned_string()?);
            user_layer_24 = Some(rdr.read_aligned_string()?);
            user_layer_25 = Some(rdr.read_aligned_string()?);
            user_layer_26 = Some(rdr.read_aligned_string()?);
            user_layer_27 = Some(rdr.read_aligned_string()?);
            user_layer_28 = Some(rdr.read_aligned_string()?);
            user_layer_29 = Some(rdr.read_aligned_string()?);
            user_layer_30 = Some(rdr.read_aligned_string()?);
            user_layer_31 = Some(rdr.read_aligned_string()?);
            m_sorting_layers = Some(TagManagerMSortingLayersVariant::parse(rdr, ver)?);
        } else {
            builtin_layer_0 = Some(rdr.read_aligned_string()?);
            builtin_layer_1 = Some(rdr.read_aligned_string()?);
            builtin_layer_2 = Some(rdr.read_aligned_string()?);
            builtin_layer_3 = Some(rdr.read_aligned_string()?);
            builtin_layer_4 = Some(rdr.read_aligned_string()?);
            builtin_layer_5 = Some(rdr.read_aligned_string()?);
            builtin_layer_6 = Some(rdr.read_aligned_string()?);
            builtin_layer_7 = Some(rdr.read_aligned_string()?);
            user_layer_8 = Some(rdr.read_aligned_string()?);
            user_layer_9 = Some(rdr.read_aligned_string()?);
            user_layer_10 = Some(rdr.read_aligned_string()?);
            user_layer_11 = Some(rdr.read_aligned_string()?);
            user_layer_12 = Some(rdr.read_aligned_string()?);
            user_layer_13 = Some(rdr.read_aligned_string()?);
            user_layer_14 = Some(rdr.read_aligned_string()?);
            user_layer_15 = Some(rdr.read_aligned_string()?);
            user_layer_16 = Some(rdr.read_aligned_string()?);
            user_layer_17 = Some(rdr.read_aligned_string()?);
            user_layer_18 = Some(rdr.read_aligned_string()?);
            user_layer_19 = Some(rdr.read_aligned_string()?);
            user_layer_20 = Some(rdr.read_aligned_string()?);
            user_layer_21 = Some(rdr.read_aligned_string()?);
            user_layer_22 = Some(rdr.read_aligned_string()?);
            user_layer_23 = Some(rdr.read_aligned_string()?);
            user_layer_24 = Some(rdr.read_aligned_string()?);
            user_layer_25 = Some(rdr.read_aligned_string()?);
            user_layer_26 = Some(rdr.read_aligned_string()?);
            user_layer_27 = Some(rdr.read_aligned_string()?);
            user_layer_28 = Some(rdr.read_aligned_string()?);
            user_layer_29 = Some(rdr.read_aligned_string()?);
            user_layer_30 = Some(rdr.read_aligned_string()?);
            user_layer_31 = Some(rdr.read_aligned_string()?);
        }

        Ok(TagManager {
            tags: tags.unwrap(),
            builtin_layer_0,
            layers,
            builtin_layer_1,
            m_sorting_layers,
            builtin_layer_2,
            m_rendering_layers,
            builtin_layer_3,
            builtin_layer_4,
            builtin_layer_5,
            builtin_layer_6,
            builtin_layer_7,
            user_layer_8,
            user_layer_9,
            user_layer_10,
            user_layer_11,
            user_layer_12,
            user_layer_13,
            user_layer_14,
            user_layer_15,
            user_layer_16,
            user_layer_17,
            user_layer_18,
            user_layer_19,
            user_layer_20,
            user_layer_21,
            user_layer_22,
            user_layer_23,
            user_layer_24,
            user_layer_25,
            user_layer_26,
            user_layer_27,
            user_layer_28,
            user_layer_29,
            user_layer_30,
            user_layer_31,
        })
    }
}

impl crate::ToValue for TagManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("tags".to_string(), self.tags.to_value())),
            crate::opt_field("Builtin Layer 0", &self.builtin_layer_0),
            crate::opt_field("layers", &self.layers),
            crate::opt_field("Builtin Layer 1", &self.builtin_layer_1),
            crate::opt_field("m_SortingLayers", &self.m_sorting_layers),
            crate::opt_field("Builtin Layer 2", &self.builtin_layer_2),
            crate::opt_field("m_RenderingLayers", &self.m_rendering_layers),
            crate::opt_field("Builtin Layer 3", &self.builtin_layer_3),
            crate::opt_field("Builtin Layer 4", &self.builtin_layer_4),
            crate::opt_field("Builtin Layer 5", &self.builtin_layer_5),
            crate::opt_field("Builtin Layer 6", &self.builtin_layer_6),
            crate::opt_field("Builtin Layer 7", &self.builtin_layer_7),
            crate::opt_field("User Layer 8", &self.user_layer_8),
            crate::opt_field("User Layer 9", &self.user_layer_9),
            crate::opt_field("User Layer 10", &self.user_layer_10),
            crate::opt_field("User Layer 11", &self.user_layer_11),
            crate::opt_field("User Layer 12", &self.user_layer_12),
            crate::opt_field("User Layer 13", &self.user_layer_13),
            crate::opt_field("User Layer 14", &self.user_layer_14),
            crate::opt_field("User Layer 15", &self.user_layer_15),
            crate::opt_field("User Layer 16", &self.user_layer_16),
            crate::opt_field("User Layer 17", &self.user_layer_17),
            crate::opt_field("User Layer 18", &self.user_layer_18),
            crate::opt_field("User Layer 19", &self.user_layer_19),
            crate::opt_field("User Layer 20", &self.user_layer_20),
            crate::opt_field("User Layer 21", &self.user_layer_21),
            crate::opt_field("User Layer 22", &self.user_layer_22),
            crate::opt_field("User Layer 23", &self.user_layer_23),
            crate::opt_field("User Layer 24", &self.user_layer_24),
            crate::opt_field("User Layer 25", &self.user_layer_25),
            crate::opt_field("User Layer 26", &self.user_layer_26),
            crate::opt_field("User Layer 27", &self.user_layer_27),
            crate::opt_field("User Layer 28", &self.user_layer_28),
            crate::opt_field("User Layer 29", &self.user_layer_29),
            crate::opt_field("User Layer 30", &self.user_layer_30),
            crate::opt_field("User Layer 31", &self.user_layer_31),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "tags" => (self.tags).inspect_value(path, offset, limit),
                "Builtin Layer 0" => (self.builtin_layer_0).inspect_value(path, offset, limit),
                "layers" => (self.layers).inspect_value(path, offset, limit),
                "Builtin Layer 1" => (self.builtin_layer_1).inspect_value(path, offset, limit),
                "m_SortingLayers" => (self.m_sorting_layers).inspect_value(path, offset, limit),
                "Builtin Layer 2" => (self.builtin_layer_2).inspect_value(path, offset, limit),
                "m_RenderingLayers" => (self.m_rendering_layers).inspect_value(path, offset, limit),
                "Builtin Layer 3" => (self.builtin_layer_3).inspect_value(path, offset, limit),
                "Builtin Layer 4" => (self.builtin_layer_4).inspect_value(path, offset, limit),
                "Builtin Layer 5" => (self.builtin_layer_5).inspect_value(path, offset, limit),
                "Builtin Layer 6" => (self.builtin_layer_6).inspect_value(path, offset, limit),
                "Builtin Layer 7" => (self.builtin_layer_7).inspect_value(path, offset, limit),
                "User Layer 8" => (self.user_layer_8).inspect_value(path, offset, limit),
                "User Layer 9" => (self.user_layer_9).inspect_value(path, offset, limit),
                "User Layer 10" => (self.user_layer_10).inspect_value(path, offset, limit),
                "User Layer 11" => (self.user_layer_11).inspect_value(path, offset, limit),
                "User Layer 12" => (self.user_layer_12).inspect_value(path, offset, limit),
                "User Layer 13" => (self.user_layer_13).inspect_value(path, offset, limit),
                "User Layer 14" => (self.user_layer_14).inspect_value(path, offset, limit),
                "User Layer 15" => (self.user_layer_15).inspect_value(path, offset, limit),
                "User Layer 16" => (self.user_layer_16).inspect_value(path, offset, limit),
                "User Layer 17" => (self.user_layer_17).inspect_value(path, offset, limit),
                "User Layer 18" => (self.user_layer_18).inspect_value(path, offset, limit),
                "User Layer 19" => (self.user_layer_19).inspect_value(path, offset, limit),
                "User Layer 20" => (self.user_layer_20).inspect_value(path, offset, limit),
                "User Layer 21" => (self.user_layer_21).inspect_value(path, offset, limit),
                "User Layer 22" => (self.user_layer_22).inspect_value(path, offset, limit),
                "User Layer 23" => (self.user_layer_23).inspect_value(path, offset, limit),
                "User Layer 24" => (self.user_layer_24).inspect_value(path, offset, limit),
                "User Layer 25" => (self.user_layer_25).inspect_value(path, offset, limit),
                "User Layer 26" => (self.user_layer_26).inspect_value(path, offset, limit),
                "User Layer 27" => (self.user_layer_27).inspect_value(path, offset, limit),
                "User Layer 28" => (self.user_layer_28).inspect_value(path, offset, limit),
                "User Layer 29" => (self.user_layer_29).inspect_value(path, offset, limit),
                "User Layer 30" => (self.user_layer_30).inspect_value(path, offset, limit),
                "User Layer 31" => (self.user_layer_31).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.tags).inspect_value(path, offset, limit)).map(|v| ("tags".to_string(), v)),
            ((self.builtin_layer_0).inspect_value(path, offset, limit))
                .map(|v| ("Builtin Layer 0".to_string(), v)),
            ((self.layers).inspect_value(path, offset, limit)).map(|v| ("layers".to_string(), v)),
            ((self.builtin_layer_1).inspect_value(path, offset, limit))
                .map(|v| ("Builtin Layer 1".to_string(), v)),
            ((self.m_sorting_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_SortingLayers".to_string(), v)),
            ((self.builtin_layer_2).inspect_value(path, offset, limit))
                .map(|v| ("Builtin Layer 2".to_string(), v)),
            ((self.m_rendering_layers).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderingLayers".to_string(), v)),
            ((self.builtin_layer_3).inspect_value(path, offset, limit))
                .map(|v| ("Builtin Layer 3".to_string(), v)),
            ((self.builtin_layer_4).inspect_value(path, offset, limit))
                .map(|v| ("Builtin Layer 4".to_string(), v)),
            ((self.builtin_layer_5).inspect_value(path, offset, limit))
                .map(|v| ("Builtin Layer 5".to_string(), v)),
            ((self.builtin_layer_6).inspect_value(path, offset, limit))
                .map(|v| ("Builtin Layer 6".to_string(), v)),
            ((self.builtin_layer_7).inspect_value(path, offset, limit))
                .map(|v| ("Builtin Layer 7".to_string(), v)),
            ((self.user_layer_8).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 8".to_string(), v)),
            ((self.user_layer_9).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 9".to_string(), v)),
            ((self.user_layer_10).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 10".to_string(), v)),
            ((self.user_layer_11).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 11".to_string(), v)),
            ((self.user_layer_12).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 12".to_string(), v)),
            ((self.user_layer_13).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 13".to_string(), v)),
            ((self.user_layer_14).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 14".to_string(), v)),
            ((self.user_layer_15).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 15".to_string(), v)),
            ((self.user_layer_16).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 16".to_string(), v)),
            ((self.user_layer_17).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 17".to_string(), v)),
            ((self.user_layer_18).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 18".to_string(), v)),
            ((self.user_layer_19).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 19".to_string(), v)),
            ((self.user_layer_20).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 20".to_string(), v)),
            ((self.user_layer_21).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 21".to_string(), v)),
            ((self.user_layer_22).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 22".to_string(), v)),
            ((self.user_layer_23).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 23".to_string(), v)),
            ((self.user_layer_24).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 24".to_string(), v)),
            ((self.user_layer_25).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 25".to_string(), v)),
            ((self.user_layer_26).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 26".to_string(), v)),
            ((self.user_layer_27).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 27".to_string(), v)),
            ((self.user_layer_28).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 28".to_string(), v)),
            ((self.user_layer_29).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 29".to_string(), v)),
            ((self.user_layer_30).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 30".to_string(), v)),
            ((self.user_layer_31).inspect_value(path, offset, limit))
                .map(|v| ("User Layer 31".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
