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
