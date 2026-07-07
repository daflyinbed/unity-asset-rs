//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::guid::Guid;
use crate::subclasses::serialized_shader::SerializedShader;

#[derive(Debug, Clone)]
pub struct Shader {
    pub m_name: String,
    pub m_script: Option<String>,
    pub m_parsed_form: Option<SerializedShader>,
    pub m_path_name: Option<String>,
    pub platforms: Option<Vec<u32>>,
    pub m_dependencies: Option<Vec<PPtr>>,
    pub decompressed_size: Option<u32>,
    pub offsets: Option<Vec<Vec<u32>>>,
    pub m_shader_is_baked: Option<bool>,
    pub m_sub_program_blob: Option<Vec<u8>>,
    pub compressed_lengths: Option<Vec<Vec<u32>>>,
    pub decompressed_lengths: Option<Vec<Vec<u32>>>,
    pub compressed_blob: Option<Vec<u8>>,
    pub stage_counts: Option<Vec<u32>>,
    pub m_non_modifiable_textures: Option<Vec<(String, PPtr)>>,
    pub m_asset_guid: Option<Guid>,
    pub m_asset_local_identifier_in_file: Option<i64>,
}

impl crate::Parse for Shader {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_script = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_parsed_form = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = SerializedShader::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_path_name = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let platforms = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_dependencies = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let decompressed_size = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let offsets = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_shader_is_baked = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_sub_program_blob = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let compressed_lengths = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let decompressed_lengths = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let compressed_blob = if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let stage_counts = if *ver >= crate::UnityVersion::new(2021, 3, 12) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_non_modifiable_textures = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = crate::PPtr::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_asset_guid = if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            Some(Guid::parse(r, ver)?)
        } else {
            None
        };
        let m_asset_local_identifier_in_file = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            Some(r.read_i64()?)
        } else {
            None
        };

        Ok(Shader {
            m_name,
            m_script,
            m_parsed_form,
            m_path_name,
            platforms,
            m_dependencies,
            decompressed_size,
            offsets,
            m_shader_is_baked,
            m_sub_program_blob,
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
