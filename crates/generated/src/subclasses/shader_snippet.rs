//! 自动生成。请勿手动修改。

use crate::subclasses::hash128::Hash128;

#[derive(Debug, Clone)]
pub struct ShaderSnippet {
    pub m_code: String,
    pub m_asset_path: String,
    pub m_platform_mask: u32,
    pub m_types_mask: u32,
    pub m_includes_hash: Hash128,
    pub m_target: i32,
    pub m_is_glsl: bool,
    pub m_from_other: bool,
    pub m_keyword_combinations_0: Vec<Vec<String>>,
    pub m_keyword_combinations_1: Vec<Vec<String>>,
    pub m_keyword_combinations_2: Vec<Vec<String>>,
    pub m_keyword_combinations_3: Vec<Vec<String>>,
    pub m_keyword_combinations_4: Vec<Vec<String>>,
    pub m_keyword_combinations_5: Vec<Vec<String>>,
}

impl crate::Parse for ShaderSnippet {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_code = rdr.read_aligned_string()?;
        let m_asset_path = rdr.read_aligned_string()?;
        let m_platform_mask = rdr.read_u32()?;
        let m_types_mask = rdr.read_u32()?;
        let m_includes_hash = Hash128::parse(rdr, ver)?;
        let m_target = rdr.read_i32()?;
        let m_is_glsl = rdr.read_bool()?;
        let m_from_other = rdr.read_bool()?;
        rdr.align(4)?;
        let m_keyword_combinations_0 = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_keyword_combinations_1 = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_keyword_combinations_2 = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_keyword_combinations_3 = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_keyword_combinations_4 = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_keyword_combinations_5 = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(ShaderSnippet {
            m_code,
            m_asset_path,
            m_platform_mask,
            m_types_mask,
            m_includes_hash,
            m_target,
            m_is_glsl,
            m_from_other,
            m_keyword_combinations_0,
            m_keyword_combinations_1,
            m_keyword_combinations_2,
            m_keyword_combinations_3,
            m_keyword_combinations_4,
            m_keyword_combinations_5,
        })
    }
}
