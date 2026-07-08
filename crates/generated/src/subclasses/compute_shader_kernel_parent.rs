//! 自动生成。请勿手动修改。

use crate::subclasses::compute_shader_kernel::ComputeShaderKernel;

#[derive(Debug, Clone)]
pub struct ComputeShaderKernelParent {
    pub name: String,
    pub variant_map: Option<Vec<(String, ComputeShaderKernel)>>,
    pub unique_variants: Option<Vec<ComputeShaderKernel>>,
    pub valid_keywords: Option<Vec<String>>,
    pub global_keywords: Option<Vec<String>>,
    pub variant_indices: Option<Vec<(String, u32)>>,
    pub local_keywords: Option<Vec<String>>,
    pub dynamic_keywords: Option<Vec<String>>,
}

impl crate::Parse for ComputeShaderKernelParent {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut variant_map = None;
        let mut unique_variants = None;
        let mut valid_keywords = None;
        let mut global_keywords = None;
        let mut variant_indices = None;
        let mut local_keywords = None;
        let mut dynamic_keywords = None;
            name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            unique_variants = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ComputeShaderKernel::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            variant_indices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = rdr.read_u32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            global_keywords = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            local_keywords = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            dynamic_keywords = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            variant_map = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = ComputeShaderKernel::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            global_keywords = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            local_keywords = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else {
            variant_map = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let a = rdr.read_aligned_string()?; let b = ComputeShaderKernel::parse(rdr, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            valid_keywords = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        }

        Ok(ComputeShaderKernelParent {
            name: name.unwrap(),
            variant_map,
            unique_variants,
            valid_keywords,
            global_keywords,
            variant_indices,
            local_keywords,
            dynamic_keywords,
        })
    }
}
