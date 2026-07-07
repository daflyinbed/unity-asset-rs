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
        let tags = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let builtin_layer_0 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let layers = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let builtin_layer_1 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sorting_layers = if *ver >= crate::UnityVersion::new(4, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SortingLayerEntry::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let builtin_layer_2 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_rendering_layers = if *ver >= crate::UnityVersion::new(2023, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let builtin_layer_3 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let builtin_layer_4 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let builtin_layer_5 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let builtin_layer_6 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let builtin_layer_7 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_8 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_9 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_10 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_11 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_12 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_13 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_14 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_15 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_16 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_17 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_18 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_19 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_20 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_21 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_22 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_23 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_24 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_25 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_26 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_27 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_28 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_29 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_30 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let user_layer_31 = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(TagManager {
            tags,
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
