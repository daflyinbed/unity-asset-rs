//! 自动生成。请勿手动修改。

use crate::subclasses::sorting_layer_entry::SortingLayerEntry;

#[derive(Debug, Clone)]
pub struct TagManager {
    pub tags: Vec<String>,
    pub builtin_layer_0: Option<String>,
    pub layers: Option<Vec<String>>,
    pub builtin_layer_1: Option<String>,
    pub m_sorting_layers: Option<Vec<SortingLayerEntry>>,
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
        r: &mut R,
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
            tags = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            layers = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_sorting_layers = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SortingLayerEntry::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_rendering_layers = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            layers = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_sorting_layers = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SortingLayerEntry::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            builtin_layer_0 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_1 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_2 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_3 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_4 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_5 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_6 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_7 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_8 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_9 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_10 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_11 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_12 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_13 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_14 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_15 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_16 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_17 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_18 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_19 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_20 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_21 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_22 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_23 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_24 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_25 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_26 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_27 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_28 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_29 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_30 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_31 = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_sorting_layers = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { SortingLayerEntry::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else {
            builtin_layer_0 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_1 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_2 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_3 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_4 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_5 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_6 = Some(r.read_aligned_string()?);
            r.align(4)?;
            builtin_layer_7 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_8 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_9 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_10 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_11 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_12 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_13 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_14 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_15 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_16 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_17 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_18 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_19 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_20 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_21 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_22 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_23 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_24 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_25 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_26 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_27 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_28 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_29 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_30 = Some(r.read_aligned_string()?);
            r.align(4)?;
            user_layer_31 = Some(r.read_aligned_string()?);
            r.align(4)?;
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
