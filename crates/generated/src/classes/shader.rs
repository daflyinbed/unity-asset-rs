//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::serialized_shader::SerializedShader;

/// 版本枚举：ShaderPlatformsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ShaderPlatformsVariant {
    Default(Vec<u32>),
    V2017_1(Vec<u32>),
}

impl crate::Parse for ShaderPlatformsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ShaderPlatformsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ShaderMDependenciesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ShaderMDependenciesVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for ShaderMDependenciesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ShaderMDependenciesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ShaderOffsetsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ShaderOffsetsVariant {
    Default(Vec<u32>),
    V2017_1(Vec<u32>),
    V2019_3(Vec<Vec<u32>>),
}

impl crate::Parse for ShaderOffsetsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let n = rdr.read_i32()? as usize;
                            (0..n)
                                .map(|_| rdr.read_u32())
                                .collect::<Result<Vec<_>, _>>()
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ShaderOffsetsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ShaderCompressedLengthsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ShaderCompressedLengthsVariant {
    Default(Vec<u32>),
    V2017_1(Vec<u32>),
    V2019_3(Vec<Vec<u32>>),
}

impl crate::Parse for ShaderCompressedLengthsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let n = rdr.read_i32()? as usize;
                            (0..n)
                                .map(|_| rdr.read_u32())
                                .collect::<Result<Vec<_>, _>>()
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ShaderCompressedLengthsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ShaderDecompressedLengthsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ShaderDecompressedLengthsVariant {
    Default(Vec<u32>),
    V2017_1(Vec<u32>),
    V2019_3(Vec<Vec<u32>>),
}

impl crate::Parse for ShaderDecompressedLengthsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 3, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let n = rdr.read_i32()? as usize;
                            (0..n)
                                .map(|_| rdr.read_u32())
                                .collect::<Result<Vec<_>, _>>()
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2019_3(v))
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ShaderDecompressedLengthsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
            Self::V2019_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Shader {
    pub m_name: String,
    pub m_script: Option<String>,
    pub m_parsed_form: Option<SerializedShader>,
    pub m_path_name: Option<String>,
    pub platforms: Option<ShaderPlatformsVariant>,
    pub m_dependencies: Option<ShaderMDependenciesVariant>,
    pub decompressed_size: Option<u32>,
    pub offsets: Option<ShaderOffsetsVariant>,
    pub m_shader_is_baked: Option<bool>,
    pub m_compressed_blob: Option<Vec<u8>>,
    pub compressed_lengths: Option<ShaderCompressedLengthsVariant>,
    pub decompressed_lengths: Option<ShaderDecompressedLengthsVariant>,
    pub compressed_blob: Option<Vec<u8>>,
    pub stage_counts: Option<Vec<u32>>,
    pub m_non_modifiable_textures: Option<Vec<(String, PPtr)>>,
    pub m_asset_guid: Option<Guid>,
    pub m_asset_local_identifier_in_file: Option<i64>,
}

impl crate::Parse for Shader {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_script = None;
        let mut m_parsed_form = None;
        let mut m_path_name = None;
        let mut platforms = None;
        let mut m_dependencies = None;
        let mut decompressed_size = None;
        let mut offsets = None;
        let mut m_shader_is_baked = None;
        let mut m_compressed_blob = None;
        let mut compressed_lengths = None;
        let mut decompressed_lengths = None;
        let mut compressed_blob = None;
        let mut stage_counts = None;
        let mut m_non_modifiable_textures = None;
        let mut m_asset_guid = None;
        let mut m_asset_local_identifier_in_file = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            m_parsed_form = Some(SerializedShader::parse(rdr, ver)?);
            platforms = Some(ShaderPlatformsVariant::parse(rdr, ver)?);
            offsets = Some(ShaderOffsetsVariant::parse(rdr, ver)?);
            compressed_lengths = Some(ShaderCompressedLengthsVariant::parse(rdr, ver)?);
            decompressed_lengths = Some(ShaderDecompressedLengthsVariant::parse(rdr, ver)?);
            compressed_blob = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            stage_counts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_dependencies = Some(ShaderMDependenciesVariant::parse(rdr, ver)?);
            m_non_modifiable_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_shader_is_baked = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_guid = Some(Guid::parse(rdr, ver)?);
            m_asset_local_identifier_in_file = Some(rdr.read_i64()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            m_parsed_form = Some(SerializedShader::parse(rdr, ver)?);
            platforms = Some(ShaderPlatformsVariant::parse(rdr, ver)?);
            offsets = Some(ShaderOffsetsVariant::parse(rdr, ver)?);
            compressed_lengths = Some(ShaderCompressedLengthsVariant::parse(rdr, ver)?);
            decompressed_lengths = Some(ShaderDecompressedLengthsVariant::parse(rdr, ver)?);
            compressed_blob = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            stage_counts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_dependencies = Some(ShaderMDependenciesVariant::parse(rdr, ver)?);
            m_non_modifiable_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_shader_is_baked = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_asset_guid = Some(Guid::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 21) {
            m_parsed_form = Some(SerializedShader::parse(rdr, ver)?);
            platforms = Some(ShaderPlatformsVariant::parse(rdr, ver)?);
            offsets = Some(ShaderOffsetsVariant::parse(rdr, ver)?);
            compressed_lengths = Some(ShaderCompressedLengthsVariant::parse(rdr, ver)?);
            decompressed_lengths = Some(ShaderDecompressedLengthsVariant::parse(rdr, ver)?);
            compressed_blob = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            stage_counts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_dependencies = Some(ShaderMDependenciesVariant::parse(rdr, ver)?);
            m_non_modifiable_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_shader_is_baked = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_parsed_form = Some(SerializedShader::parse(rdr, ver)?);
            platforms = Some(ShaderPlatformsVariant::parse(rdr, ver)?);
            offsets = Some(ShaderOffsetsVariant::parse(rdr, ver)?);
            compressed_lengths = Some(ShaderCompressedLengthsVariant::parse(rdr, ver)?);
            decompressed_lengths = Some(ShaderDecompressedLengthsVariant::parse(rdr, ver)?);
            compressed_blob = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_dependencies = Some(ShaderMDependenciesVariant::parse(rdr, ver)?);
            m_non_modifiable_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_shader_is_baked = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2021, 3, 12) {
            m_parsed_form = Some(SerializedShader::parse(rdr, ver)?);
            platforms = Some(ShaderPlatformsVariant::parse(rdr, ver)?);
            offsets = Some(ShaderOffsetsVariant::parse(rdr, ver)?);
            compressed_lengths = Some(ShaderCompressedLengthsVariant::parse(rdr, ver)?);
            decompressed_lengths = Some(ShaderDecompressedLengthsVariant::parse(rdr, ver)?);
            compressed_blob = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            stage_counts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| rdr.read_u32())
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_dependencies = Some(ShaderMDependenciesVariant::parse(rdr, ver)?);
            m_non_modifiable_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_shader_is_baked = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_parsed_form = Some(SerializedShader::parse(rdr, ver)?);
            platforms = Some(ShaderPlatformsVariant::parse(rdr, ver)?);
            offsets = Some(ShaderOffsetsVariant::parse(rdr, ver)?);
            compressed_lengths = Some(ShaderCompressedLengthsVariant::parse(rdr, ver)?);
            decompressed_lengths = Some(ShaderDecompressedLengthsVariant::parse(rdr, ver)?);
            compressed_blob = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_dependencies = Some(ShaderMDependenciesVariant::parse(rdr, ver)?);
            m_non_modifiable_textures = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_aligned_string()?;
                                let b = crate::PPtr::parse(rdr, ver)?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_shader_is_baked = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_parsed_form = Some(SerializedShader::parse(rdr, ver)?);
            platforms = Some(ShaderPlatformsVariant::parse(rdr, ver)?);
            offsets = Some(ShaderOffsetsVariant::parse(rdr, ver)?);
            compressed_lengths = Some(ShaderCompressedLengthsVariant::parse(rdr, ver)?);
            decompressed_lengths = Some(ShaderDecompressedLengthsVariant::parse(rdr, ver)?);
            compressed_blob = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_dependencies = Some(ShaderMDependenciesVariant::parse(rdr, ver)?);
            m_shader_is_baked = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_script = Some(rdr.read_aligned_string()?);
            m_path_name = Some(rdr.read_aligned_string()?);
            decompressed_size = Some(rdr.read_u32()?);
            m_compressed_blob = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_dependencies = Some(ShaderMDependenciesVariant::parse(rdr, ver)?);
            m_shader_is_baked = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_script = Some(rdr.read_aligned_string()?);
            m_path_name = Some(rdr.read_aligned_string()?);
            m_dependencies = Some(ShaderMDependenciesVariant::parse(rdr, ver)?);
            m_shader_is_baked = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_script = Some(rdr.read_aligned_string()?);
            m_path_name = Some(rdr.read_aligned_string()?);
        }

        Ok(Shader {
            m_name: m_name.unwrap(),
            m_script,
            m_parsed_form,
            m_path_name,
            platforms,
            m_dependencies,
            decompressed_size,
            offsets,
            m_shader_is_baked,
            m_compressed_blob,
            compressed_lengths,
            decompressed_lengths,
            compressed_blob,
            stage_counts,
            m_non_modifiable_textures,
            m_asset_guid,
            m_asset_local_identifier_in_file,
        })
    }
}

impl crate::ToValue for Shader {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            crate::opt_field("m_Script", &self.m_script),
            crate::opt_field("m_ParsedForm", &self.m_parsed_form),
            crate::opt_field("m_PathName", &self.m_path_name),
            crate::opt_field("platforms", &self.platforms),
            crate::opt_field("m_Dependencies", &self.m_dependencies),
            crate::opt_field("decompressedSize", &self.decompressed_size),
            crate::opt_field("offsets", &self.offsets),
            crate::opt_field("m_ShaderIsBaked", &self.m_shader_is_baked),
            crate::opt_field("m_CompressedBlob", &self.m_compressed_blob),
            crate::opt_field("compressedLengths", &self.compressed_lengths),
            crate::opt_field("decompressedLengths", &self.decompressed_lengths),
            crate::opt_field("compressedBlob", &self.compressed_blob),
            crate::opt_field("stageCounts", &self.stage_counts),
            crate::opt_field("m_NonModifiableTextures", &self.m_non_modifiable_textures),
            crate::opt_field("m_AssetGUID", &self.m_asset_guid),
            crate::opt_field(
                "m_AssetLocalIdentifierInFile",
                &self.m_asset_local_identifier_in_file,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Script" => (self.m_script).inspect_value(path, offset, limit),
                "m_ParsedForm" => (self.m_parsed_form).inspect_value(path, offset, limit),
                "m_PathName" => (self.m_path_name).inspect_value(path, offset, limit),
                "platforms" => (self.platforms).inspect_value(path, offset, limit),
                "m_Dependencies" => (self.m_dependencies).inspect_value(path, offset, limit),
                "decompressedSize" => (self.decompressed_size).inspect_value(path, offset, limit),
                "offsets" => (self.offsets).inspect_value(path, offset, limit),
                "m_ShaderIsBaked" => (self.m_shader_is_baked).inspect_value(path, offset, limit),
                "m_CompressedBlob" => (self.m_compressed_blob).inspect_value(path, offset, limit),
                "compressedLengths" => (self.compressed_lengths).inspect_value(path, offset, limit),
                "decompressedLengths" => {
                    (self.decompressed_lengths).inspect_value(path, offset, limit)
                }
                "compressedBlob" => (self.compressed_blob).inspect_value(path, offset, limit),
                "stageCounts" => (self.stage_counts).inspect_value(path, offset, limit),
                "m_NonModifiableTextures" => {
                    (self.m_non_modifiable_textures).inspect_value(path, offset, limit)
                }
                "m_AssetGUID" => (self.m_asset_guid).inspect_value(path, offset, limit),
                "m_AssetLocalIdentifierInFile" => {
                    (self.m_asset_local_identifier_in_file).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_script).inspect_value(path, offset, limit))
                .map(|v| ("m_Script".to_string(), v)),
            ((self.m_parsed_form).inspect_value(path, offset, limit))
                .map(|v| ("m_ParsedForm".to_string(), v)),
            ((self.m_path_name).inspect_value(path, offset, limit))
                .map(|v| ("m_PathName".to_string(), v)),
            ((self.platforms).inspect_value(path, offset, limit))
                .map(|v| ("platforms".to_string(), v)),
            ((self.m_dependencies).inspect_value(path, offset, limit))
                .map(|v| ("m_Dependencies".to_string(), v)),
            ((self.decompressed_size).inspect_value(path, offset, limit))
                .map(|v| ("decompressedSize".to_string(), v)),
            ((self.offsets).inspect_value(path, offset, limit)).map(|v| ("offsets".to_string(), v)),
            ((self.m_shader_is_baked).inspect_value(path, offset, limit))
                .map(|v| ("m_ShaderIsBaked".to_string(), v)),
            ((self.m_compressed_blob).inspect_value(path, offset, limit))
                .map(|v| ("m_CompressedBlob".to_string(), v)),
            ((self.compressed_lengths).inspect_value(path, offset, limit))
                .map(|v| ("compressedLengths".to_string(), v)),
            ((self.decompressed_lengths).inspect_value(path, offset, limit))
                .map(|v| ("decompressedLengths".to_string(), v)),
            ((self.compressed_blob).inspect_value(path, offset, limit))
                .map(|v| ("compressedBlob".to_string(), v)),
            ((self.stage_counts).inspect_value(path, offset, limit))
                .map(|v| ("stageCounts".to_string(), v)),
            ((self.m_non_modifiable_textures).inspect_value(path, offset, limit))
                .map(|v| ("m_NonModifiableTextures".to_string(), v)),
            ((self.m_asset_guid).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetGUID".to_string(), v)),
            ((self.m_asset_local_identifier_in_file).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetLocalIdentifierInFile".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
