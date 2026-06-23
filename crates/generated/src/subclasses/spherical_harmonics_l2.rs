//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SphericalHarmonicsL2 {
    pub sh_0: f32,
    pub sh_1: f32,
    pub sh_2: f32,
    pub sh_3: f32,
    pub sh_4: f32,
    pub sh_5: f32,
    pub sh_6: f32,
    pub sh_7: f32,
    pub sh_8: f32,
    pub sh_9: f32,
    pub sh_10: f32,
    pub sh_11: f32,
    pub sh_12: f32,
    pub sh_13: f32,
    pub sh_14: f32,
    pub sh_15: f32,
    pub sh_16: f32,
    pub sh_17: f32,
    pub sh_18: f32,
    pub sh_19: f32,
    pub sh_20: f32,
    pub sh_21: f32,
    pub sh_22: f32,
    pub sh_23: f32,
    pub sh_24: f32,
    pub sh_25: f32,
    pub sh_26: f32,
}

impl crate::Parse for SphericalHarmonicsL2 {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let sh_0 = r.read_f32()?;
        let sh_1 = r.read_f32()?;
        let sh_2 = r.read_f32()?;
        let sh_3 = r.read_f32()?;
        let sh_4 = r.read_f32()?;
        let sh_5 = r.read_f32()?;
        let sh_6 = r.read_f32()?;
        let sh_7 = r.read_f32()?;
        let sh_8 = r.read_f32()?;
        let sh_9 = r.read_f32()?;
        let sh_10 = r.read_f32()?;
        let sh_11 = r.read_f32()?;
        let sh_12 = r.read_f32()?;
        let sh_13 = r.read_f32()?;
        let sh_14 = r.read_f32()?;
        let sh_15 = r.read_f32()?;
        let sh_16 = r.read_f32()?;
        let sh_17 = r.read_f32()?;
        let sh_18 = r.read_f32()?;
        let sh_19 = r.read_f32()?;
        let sh_20 = r.read_f32()?;
        let sh_21 = r.read_f32()?;
        let sh_22 = r.read_f32()?;
        let sh_23 = r.read_f32()?;
        let sh_24 = r.read_f32()?;
        let sh_25 = r.read_f32()?;
        let sh_26 = r.read_f32()?;

        Ok(SphericalHarmonicsL2 {
            sh_0,
            sh_1,
            sh_2,
            sh_3,
            sh_4,
            sh_5,
            sh_6,
            sh_7,
            sh_8,
            sh_9,
            sh_10,
            sh_11,
            sh_12,
            sh_13,
            sh_14,
            sh_15,
            sh_16,
            sh_17,
            sh_18,
            sh_19,
            sh_20,
            sh_21,
            sh_22,
            sh_23,
            sh_24,
            sh_25,
            sh_26,
        })
    }
}
