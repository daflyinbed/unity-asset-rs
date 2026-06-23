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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_code = r.read_aligned_string()?;
        r.align(4)?;
        let m_asset_path = r.read_aligned_string()?;
        r.align(4)?;
        let m_platform_mask = r.read_u32()?;
        let m_types_mask = r.read_u32()?;
        let m_includes_hash = Hash128::parse(r, ver)?;
        let m_target = r.read_i32()?;
        let m_is_glsl = r.read_bool()?;
        let m_from_other = r.read_bool()?;
        r.align(4)?;
        let m_keyword_combinations_0 = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_keyword_combinations_1 = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_keyword_combinations_2 = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_keyword_combinations_3 = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_keyword_combinations_4 = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_keyword_combinations_5 = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

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
