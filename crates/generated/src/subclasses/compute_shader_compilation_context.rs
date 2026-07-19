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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let build_target = BuildTargetSelection::parse(rdr, ver)?;
        let source_file_name = rdr.read_aligned_string()?;
        let source = rdr.read_aligned_string()?;
        let source_file = rdr.read_aligned_string()?;
        let kernels = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_aligned_string())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let kernel_macros = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let n = rdr.read_i32()? as usize;
                        (0..n)
                            .map(|_| {
                                ({
                                    let a = rdr.read_aligned_string()?;
                                    let b = rdr.read_aligned_string()?;
                                    Ok((a, b))
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let compilation_flags = rdr.read_i32()?;
        let api_mask = rdr.read_u32()?;
        let supported_ap_is = rdr.read_u32()?;
        let include_hash0 = rdr.read_u32()?;
        let include_hash1 = rdr.read_u32()?;
        let include_hash2 = rdr.read_u32()?;
        let include_hash3 = rdr.read_u32()?;
        let include_files = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_aligned_string())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

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
