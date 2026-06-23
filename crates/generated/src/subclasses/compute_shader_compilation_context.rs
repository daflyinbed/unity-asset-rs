//! 自动生成。请勿手动修改。

use crate::subclasses::build_target_selection::BuildTargetSelection;

#[derive(Debug, Clone)]
pub struct ComputeShaderCompilationContext {
    pub build_target: BuildTargetSelection,
    pub source_file_name: String,
    pub source: String,
    pub source_file: String,
    pub kernels: Vec<String>,
    pub kernel_macros: Vec<Vec<(String, String)>>,
    pub compilation_flags: i32,
    pub api_mask: u32,
    pub supported_ap_is: u32,
    pub include_hash0: u32,
    pub include_hash1: u32,
    pub include_hash2: u32,
    pub include_hash3: u32,
    pub include_files: Vec<String>,
}

impl crate::Parse for ComputeShaderCompilationContext {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let build_target = BuildTargetSelection::parse(r, ver)?;
        let source_file_name = r.read_aligned_string()?;
        r.align(4)?;
        let source = r.read_aligned_string()?;
        r.align(4)?;
        let source_file = r.read_aligned_string()?;
        r.align(4)?;
        let kernels = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let kernel_macros = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let compilation_flags = r.read_i32()?;
        let api_mask = r.read_u32()?;
        let supported_ap_is = r.read_u32()?;
        let include_hash0 = r.read_u32()?;
        let include_hash1 = r.read_u32()?;
        let include_hash2 = r.read_u32()?;
        let include_hash3 = r.read_u32()?;
        let include_files = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(ComputeShaderCompilationContext {
            build_target,
            source_file_name,
            source,
            source_file,
            kernels,
            kernel_macros,
            compilation_flags,
            api_mask,
            supported_ap_is,
            include_hash0,
            include_hash1,
            include_hash2,
            include_hash3,
            include_files,
        })
    }
}
