//! 自动生成。请勿手动修改。


/// 版本枚举：ComputeShaderBuiltinSamplerSamplerVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ComputeShaderBuiltinSamplerSamplerVariant {
    Default(i32),
    V2017_1(u32),
}

impl crate::Parse for ComputeShaderBuiltinSamplerSamplerVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = rdr.read_u32()?;
            Ok(Self::V2017_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ComputeShaderBuiltinSampler {
    pub sampler: ComputeShaderBuiltinSamplerSamplerVariant,
    pub bind_point: i32,
}

impl crate::Parse for ComputeShaderBuiltinSampler {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let sampler = ComputeShaderBuiltinSamplerSamplerVariant::parse(rdr, ver)?;
        let bind_point = rdr.read_i32()?;

        Ok(ComputeShaderBuiltinSampler {
            sampler,
            bind_point,
        })
    }
}
