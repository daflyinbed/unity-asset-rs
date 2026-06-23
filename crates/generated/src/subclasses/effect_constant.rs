//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct EffectConstant {
    pub r#type: i32,
    pub group_constant_index: u32,
    pub send_target_effect_index: u32,
    pub wet_mix_level_index: u32,
    pub prev_effect_index: u32,
    pub bypass: bool,
    pub parameter_indices: Vec<u32>,
}

impl crate::Parse for EffectConstant {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = r.read_i32()?;
        let group_constant_index = r.read_u32()?;
        let send_target_effect_index = r.read_u32()?;
        let wet_mix_level_index = r.read_u32()?;
        let prev_effect_index = r.read_u32()?;
        let bypass = r.read_bool()?;
        r.align(4)?;
        let parameter_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(EffectConstant {
            r#type,
            group_constant_index,
            send_target_effect_index,
            wet_mix_level_index,
            prev_effect_index,
            bypass,
            parameter_indices,
        })
    }
}
