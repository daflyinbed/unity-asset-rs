//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ParticleSystemEmissionBurst {
    pub time: f32,
    pub min_count: u32,
    pub max_count: u32,
    pub cycle_count: u32,
    pub repeat_interval: f32,
}

impl crate::Parse for ParticleSystemEmissionBurst {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let time = r.read_f32()?;
        let min_count = r.read_u32()?;
        let max_count = r.read_u32()?;
        let cycle_count = r.read_u32()?;
        let repeat_interval = r.read_f32()?;

        Ok(ParticleSystemEmissionBurst {
            time,
            min_count,
            max_count,
            cycle_count,
            repeat_interval,
        })
    }
}
