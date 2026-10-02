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

impl crate::ToValue for RayTracingShaderCompilationContext {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("buildTarget".to_string(), self.build_target.to_value())),
            Some(("platformGroup".to_string(), self.platform_group.to_value())),
            Some((
                "sourceFileName".to_string(),
                self.source_file_name.to_value(),
            )),
            Some(("source".to_string(), self.source.to_value())),
            Some(("sourceFile".to_string(), self.source_file.to_value())),
            Some((
                "compilationFlags".to_string(),
                self.compilation_flags.to_value(),
            )),
            Some((
                "maxRecursionDepth".to_string(),
                self.max_recursion_depth.to_value(),
            )),
            Some((
                "enableRayPayloadSizeChecks".to_string(),
                self.enable_ray_payload_size_checks.to_value(),
            )),
            Some(("colorSpace".to_string(), self.color_space.to_value())),
            Some((
                "shaderPrecisionModel".to_string(),
                self.shader_precision_model.to_value(),
            )),
            Some(("supportedAPIs".to_string(), self.supported_ap_is.to_value())),
            Some(("includeHash0".to_string(), self.include_hash0.to_value())),
            Some(("includeHash1".to_string(), self.include_hash1.to_value())),
            Some(("includeHash2".to_string(), self.include_hash2.to_value())),
            Some(("includeHash3".to_string(), self.include_hash3.to_value())),
            Some((
                "buildTargetAPIMask".to_string(),
                self.build_target_api_mask.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "buildTarget" => (self.build_target).inspect_value(path, offset, limit),
                "platformGroup" => (self.platform_group).inspect_value(path, offset, limit),
                "sourceFileName" => (self.source_file_name).inspect_value(path, offset, limit),
                "source" => (self.source).inspect_value(path, offset, limit),
                "sourceFile" => (self.source_file).inspect_value(path, offset, limit),
                "compilationFlags" => (self.compilation_flags).inspect_value(path, offset, limit),
                "maxRecursionDepth" => {
                    (self.max_recursion_depth).inspect_value(path, offset, limit)
                }
                "enableRayPayloadSizeChecks" => {
                    (self.enable_ray_payload_size_checks).inspect_value(path, offset, limit)
                }
                "colorSpace" => (self.color_space).inspect_value(path, offset, limit),
                "shaderPrecisionModel" => {
                    (self.shader_precision_model).inspect_value(path, offset, limit)
                }
                "supportedAPIs" => (self.supported_ap_is).inspect_value(path, offset, limit),
                "includeHash0" => (self.include_hash0).inspect_value(path, offset, limit),
                "includeHash1" => (self.include_hash1).inspect_value(path, offset, limit),
                "includeHash2" => (self.include_hash2).inspect_value(path, offset, limit),
                "includeHash3" => (self.include_hash3).inspect_value(path, offset, limit),
                "buildTargetAPIMask" => {
                    (self.build_target_api_mask).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.build_target).inspect_value(path, offset, limit))
                .map(|v| ("buildTarget".to_string(), v)),
            ((self.platform_group).inspect_value(path, offset, limit))
                .map(|v| ("platformGroup".to_string(), v)),
            ((self.source_file_name).inspect_value(path, offset, limit))
                .map(|v| ("sourceFileName".to_string(), v)),
            ((self.source).inspect_value(path, offset, limit)).map(|v| ("source".to_string(), v)),
            ((self.source_file).inspect_value(path, offset, limit))
                .map(|v| ("sourceFile".to_string(), v)),
            ((self.compilation_flags).inspect_value(path, offset, limit))
                .map(|v| ("compilationFlags".to_string(), v)),
            ((self.max_recursion_depth).inspect_value(path, offset, limit))
                .map(|v| ("maxRecursionDepth".to_string(), v)),
            ((self.enable_ray_payload_size_checks).inspect_value(path, offset, limit))
                .map(|v| ("enableRayPayloadSizeChecks".to_string(), v)),
            ((self.color_space).inspect_value(path, offset, limit))
                .map(|v| ("colorSpace".to_string(), v)),
            ((self.shader_precision_model).inspect_value(path, offset, limit))
                .map(|v| ("shaderPrecisionModel".to_string(), v)),
            ((self.supported_ap_is).inspect_value(path, offset, limit))
                .map(|v| ("supportedAPIs".to_string(), v)),
            ((self.include_hash0).inspect_value(path, offset, limit))
                .map(|v| ("includeHash0".to_string(), v)),
            ((self.include_hash1).inspect_value(path, offset, limit))
                .map(|v| ("includeHash1".to_string(), v)),
            ((self.include_hash2).inspect_value(path, offset, limit))
                .map(|v| ("includeHash2".to_string(), v)),
            ((self.include_hash3).inspect_value(path, offset, limit))
                .map(|v| ("includeHash3".to_string(), v)),
            ((self.build_target_api_mask).inspect_value(path, offset, limit))
                .map(|v| ("buildTargetAPIMask".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
