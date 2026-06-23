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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let build_target = BuildTargetSelection::parse(r, ver)?;
        let platform_group = r.read_i32()?;
        let source_file_name = r.read_aligned_string()?;
        r.align(4)?;
        let source = r.read_aligned_string()?;
        r.align(4)?;
        let source_file = r.read_aligned_string()?;
        r.align(4)?;
        let compilation_flags = r.read_u64()?;
        let max_recursion_depth = r.read_u32()?;
        let enable_ray_payload_size_checks = r.read_bool()?;
        r.align(4)?;
        let color_space = r.read_i32()?;
        let shader_precision_model = r.read_i32()?;
        let supported_ap_is = r.read_u32()?;
        let include_hash0 = r.read_u32()?;
        let include_hash1 = r.read_u32()?;
        let include_hash2 = r.read_u32()?;
        let include_hash3 = r.read_u32()?;
        let build_target_api_mask = r.read_u32()?;

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
