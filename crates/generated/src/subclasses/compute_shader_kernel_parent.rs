//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_kernel::ComputeShaderKernel;

#[derive(Debug, Clone)]
pub struct ComputeShaderKernelParent {
    pub name: String,
    pub variant_map: Vec<(String, ComputeShaderKernel)>,
    pub valid_keywords: Vec<String>,
}

impl crate::Parse for ComputeShaderKernelParent {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let variant_map = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = ComputeShaderKernel::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let valid_keywords = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ComputeShaderKernelParent {
            name,
            variant_map,
            valid_keywords,
        })
    }
}
