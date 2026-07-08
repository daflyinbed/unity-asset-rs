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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SortingLayerEntry::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SortingLayerEntry::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
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
            m_rendering_layers = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
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
