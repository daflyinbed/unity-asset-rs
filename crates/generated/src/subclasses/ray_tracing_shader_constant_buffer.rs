//! 自动生成。请勿手动修改。

use crate::subclasses::ray_tracing_shader_param::RayTracingShaderParam;

#[derive(Debug, Clone)]
pub struct RayTracingShaderConstantBuffer {
    pub name: String,
    pub byte_size: i32,
    pub params: Vec<RayTracingShaderParam>,
    pub hash: Option<u32>,
}

impl crate::Parse for RayTracingShaderConstantBuffer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut byte_size = None;
        let mut params = None;
        let mut hash = None;
            name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
            byte_size = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            hash = Some(rdr.read_u32()?);
        } else {
        }
            params = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { RayTracingShaderParam::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;

        Ok(RayTracingShaderConstantBuffer {
            name: name.unwrap(),
            byte_size: byte_size.unwrap(),
            params: params.unwrap(),
            hash,
        })
    }
}
