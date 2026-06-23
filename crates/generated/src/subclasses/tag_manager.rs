//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct TagManager {
    pub tags: Vec<String>,
    pub builtin_layer_0: String,
    pub builtin_layer_1: String,
    pub builtin_layer_2: String,
    pub builtin_layer_3: String,
    pub builtin_layer_4: String,
    pub builtin_layer_5: String,
    pub builtin_layer_6: String,
    pub builtin_layer_7: String,
    pub user_layer_8: String,
    pub user_layer_9: String,
    pub user_layer_10: String,
    pub user_layer_11: String,
    pub user_layer_12: String,
    pub user_layer_13: String,
    pub user_layer_14: String,
    pub user_layer_15: String,
    pub user_layer_16: String,
    pub user_layer_17: String,
    pub user_layer_18: String,
    pub user_layer_19: String,
    pub user_layer_20: String,
    pub user_layer_21: String,
    pub user_layer_22: String,
    pub user_layer_23: String,
    pub user_layer_24: String,
    pub user_layer_25: String,
    pub user_layer_26: String,
    pub user_layer_27: String,
    pub user_layer_28: String,
    pub user_layer_29: String,
    pub user_layer_30: String,
    pub user_layer_31: String,
}

impl crate::Parse for TagManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let tags = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let builtin_layer_0 = r.read_aligned_string()?;
        r.align(4)?;
        let builtin_layer_1 = r.read_aligned_string()?;
        r.align(4)?;
        let builtin_layer_2 = r.read_aligned_string()?;
        r.align(4)?;
        let builtin_layer_3 = r.read_aligned_string()?;
        r.align(4)?;
        let builtin_layer_4 = r.read_aligned_string()?;
        r.align(4)?;
        let builtin_layer_5 = r.read_aligned_string()?;
        r.align(4)?;
        let builtin_layer_6 = r.read_aligned_string()?;
        r.align(4)?;
        let builtin_layer_7 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_8 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_9 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_10 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_11 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_12 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_13 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_14 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_15 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_16 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_17 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_18 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_19 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_20 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_21 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_22 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_23 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_24 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_25 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_26 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_27 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_28 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_29 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_30 = r.read_aligned_string()?;
        r.align(4)?;
        let user_layer_31 = r.read_aligned_string()?;
        r.align(4)?;

        Ok(TagManager {
            tags,
            builtin_layer_0,
            builtin_layer_1,
            builtin_layer_2,
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
