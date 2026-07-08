//! 自动生成。请勿手动修改。

use crate::subclasses::build_target_selection::BuildTargetSelection;

#[derive(Debug, Clone)]
pub struct RayTracingShaderCompilationContext {
    pub build_target: BuildTargetSelection,
    pub platform_group: i32,
    pub source_file_name: String,
    pub source: String,
    pub source_file: String,
    pub compilation_flags: u64,
    pub max_recursion_depth: u32,
    pub enable_ray_payload_size_checks: bool,
    pub color_space: i32,
    pub shader_precision_model: i32,
    pub supported_ap_is: u32,
    pub include_hash0: u32,
    pub include_hash1: u32,
    pub include_hash2: u32,
    pub include_hash3: u32,
    pub build_target_api_mask: u32,
}

impl crate::Parse for RayTracingShaderCompilationContext {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let build_target = BuildTargetSelection::parse(rdr, ver)?;
        let platform_group = rdr.read_i32()?;
        let source_file_name = rdr.read_aligned_string()?;
        let source = rdr.read_aligned_string()?;
        let source_file = rdr.read_aligned_string()?;
        let compilation_flags = rdr.read_u64()?;
        let max_recursion_depth = rdr.read_u32()?;
        let enable_ray_payload_size_checks = rdr.read_bool()?;
        rdr.align(4)?;
        let color_space = rdr.read_i32()?;
        let shader_precision_model = rdr.read_i32()?;
        let supported_ap_is = rdr.read_u32()?;
        let include_hash0 = rdr.read_u32()?;
        let include_hash1 = rdr.read_u32()?;
        let include_hash2 = rdr.read_u32()?;
        let include_hash3 = rdr.read_u32()?;
        let build_target_api_mask = rdr.read_u32()?;

        Ok(RayTracingShaderCompilationContext {
            build_target,
            platform_group,
            source_file_name,
            source,
            source_file,
            compilation_flags,
            max_recursion_depth,
            enable_ray_payload_size_checks,
            color_space,
            shader_precision_model,
            supported_ap_is,
            include_hash0,
            include_hash1,
            include_hash2,
            include_hash3,
            build_target_api_mask,
        })
    }
}
