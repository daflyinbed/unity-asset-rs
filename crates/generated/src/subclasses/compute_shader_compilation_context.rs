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

impl crate::ToValue for ComputeShaderCompilationContext {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("buildTarget".to_string(), self.build_target.to_value())),
            Some((
                "sourceFileName".to_string(),
                self.source_file_name.to_value(),
            )),
            Some(("source".to_string(), self.source.to_value())),
            Some(("sourceFile".to_string(), self.source_file.to_value())),
            Some(("kernels".to_string(), self.kernels.to_value())),
            Some(("kernelMacros".to_string(), self.kernel_macros.to_value())),
            Some((
                "compilationFlags".to_string(),
                self.compilation_flags.to_value(),
            )),
            Some(("apiMask".to_string(), self.api_mask.to_value())),
            Some(("supportedAPIs".to_string(), self.supported_ap_is.to_value())),
            Some(("includeHash0".to_string(), self.include_hash0.to_value())),
            Some(("includeHash1".to_string(), self.include_hash1.to_value())),
            Some(("includeHash2".to_string(), self.include_hash2.to_value())),
            Some(("includeHash3".to_string(), self.include_hash3.to_value())),
            Some(("includeFiles".to_string(), self.include_files.to_value())),
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
                "sourceFileName" => (self.source_file_name).inspect_value(path, offset, limit),
                "source" => (self.source).inspect_value(path, offset, limit),
                "sourceFile" => (self.source_file).inspect_value(path, offset, limit),
                "kernels" => (self.kernels).inspect_value(path, offset, limit),
                "kernelMacros" => (self.kernel_macros).inspect_value(path, offset, limit),
                "compilationFlags" => (self.compilation_flags).inspect_value(path, offset, limit),
                "apiMask" => (self.api_mask).inspect_value(path, offset, limit),
                "supportedAPIs" => (self.supported_ap_is).inspect_value(path, offset, limit),
                "includeHash0" => (self.include_hash0).inspect_value(path, offset, limit),
                "includeHash1" => (self.include_hash1).inspect_value(path, offset, limit),
                "includeHash2" => (self.include_hash2).inspect_value(path, offset, limit),
                "includeHash3" => (self.include_hash3).inspect_value(path, offset, limit),
                "includeFiles" => (self.include_files).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.build_target).inspect_value(path, offset, limit))
                .map(|v| ("buildTarget".to_string(), v)),
            ((self.source_file_name).inspect_value(path, offset, limit))
                .map(|v| ("sourceFileName".to_string(), v)),
            ((self.source).inspect_value(path, offset, limit)).map(|v| ("source".to_string(), v)),
            ((self.source_file).inspect_value(path, offset, limit))
                .map(|v| ("sourceFile".to_string(), v)),
            ((self.kernels).inspect_value(path, offset, limit)).map(|v| ("kernels".to_string(), v)),
            ((self.kernel_macros).inspect_value(path, offset, limit))
                .map(|v| ("kernelMacros".to_string(), v)),
            ((self.compilation_flags).inspect_value(path, offset, limit))
                .map(|v| ("compilationFlags".to_string(), v)),
            ((self.api_mask).inspect_value(path, offset, limit))
                .map(|v| ("apiMask".to_string(), v)),
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
            ((self.include_files).inspect_value(path, offset, limit))
                .map(|v| ("includeFiles".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
