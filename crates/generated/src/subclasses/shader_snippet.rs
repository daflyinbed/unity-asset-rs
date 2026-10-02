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
        let m_keyword_combinations_0 = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let n = rdr.read_i32()? as usize;
                        (0..n)
                            .map(|_| rdr.read_aligned_string())
                            .collect::<Result<Vec<_>, _>>()
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_keyword_combinations_1 = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let n = rdr.read_i32()? as usize;
                        (0..n)
                            .map(|_| rdr.read_aligned_string())
                            .collect::<Result<Vec<_>, _>>()
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_keyword_combinations_2 = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let n = rdr.read_i32()? as usize;
                        (0..n)
                            .map(|_| rdr.read_aligned_string())
                            .collect::<Result<Vec<_>, _>>()
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_keyword_combinations_3 = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let n = rdr.read_i32()? as usize;
                        (0..n)
                            .map(|_| rdr.read_aligned_string())
                            .collect::<Result<Vec<_>, _>>()
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_keyword_combinations_4 = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let n = rdr.read_i32()? as usize;
                        (0..n)
                            .map(|_| rdr.read_aligned_string())
                            .collect::<Result<Vec<_>, _>>()
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_keyword_combinations_5 = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let n = rdr.read_i32()? as usize;
                        (0..n)
                            .map(|_| rdr.read_aligned_string())
                            .collect::<Result<Vec<_>, _>>()
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

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

impl crate::ToValue for ShaderSnippet {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Code".to_string(), self.m_code.to_value())),
            Some(("m_AssetPath".to_string(), self.m_asset_path.to_value())),
            Some((
                "m_PlatformMask".to_string(),
                self.m_platform_mask.to_value(),
            )),
            Some(("m_TypesMask".to_string(), self.m_types_mask.to_value())),
            Some((
                "m_IncludesHash".to_string(),
                self.m_includes_hash.to_value(),
            )),
            Some(("m_Target".to_string(), self.m_target.to_value())),
            Some(("m_IsGLSL".to_string(), self.m_is_glsl.to_value())),
            Some(("m_FromOther".to_string(), self.m_from_other.to_value())),
            Some((
                "m_KeywordCombinations[0]".to_string(),
                self.m_keyword_combinations_0.to_value(),
            )),
            Some((
                "m_KeywordCombinations[1]".to_string(),
                self.m_keyword_combinations_1.to_value(),
            )),
            Some((
                "m_KeywordCombinations[2]".to_string(),
                self.m_keyword_combinations_2.to_value(),
            )),
            Some((
                "m_KeywordCombinations[3]".to_string(),
                self.m_keyword_combinations_3.to_value(),
            )),
            Some((
                "m_KeywordCombinations[4]".to_string(),
                self.m_keyword_combinations_4.to_value(),
            )),
            Some((
                "m_KeywordCombinations[5]".to_string(),
                self.m_keyword_combinations_5.to_value(),
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
                "m_Code" => (self.m_code).inspect_value(path, offset, limit),
                "m_AssetPath" => (self.m_asset_path).inspect_value(path, offset, limit),
                "m_PlatformMask" => (self.m_platform_mask).inspect_value(path, offset, limit),
                "m_TypesMask" => (self.m_types_mask).inspect_value(path, offset, limit),
                "m_IncludesHash" => (self.m_includes_hash).inspect_value(path, offset, limit),
                "m_Target" => (self.m_target).inspect_value(path, offset, limit),
                "m_IsGLSL" => (self.m_is_glsl).inspect_value(path, offset, limit),
                "m_FromOther" => (self.m_from_other).inspect_value(path, offset, limit),
                "m_KeywordCombinations[0]" => {
                    (self.m_keyword_combinations_0).inspect_value(path, offset, limit)
                }
                "m_KeywordCombinations[1]" => {
                    (self.m_keyword_combinations_1).inspect_value(path, offset, limit)
                }
                "m_KeywordCombinations[2]" => {
                    (self.m_keyword_combinations_2).inspect_value(path, offset, limit)
                }
                "m_KeywordCombinations[3]" => {
                    (self.m_keyword_combinations_3).inspect_value(path, offset, limit)
                }
                "m_KeywordCombinations[4]" => {
                    (self.m_keyword_combinations_4).inspect_value(path, offset, limit)
                }
                "m_KeywordCombinations[5]" => {
                    (self.m_keyword_combinations_5).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_code).inspect_value(path, offset, limit)).map(|v| ("m_Code".to_string(), v)),
            ((self.m_asset_path).inspect_value(path, offset, limit))
                .map(|v| ("m_AssetPath".to_string(), v)),
            ((self.m_platform_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_PlatformMask".to_string(), v)),
            ((self.m_types_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_TypesMask".to_string(), v)),
            ((self.m_includes_hash).inspect_value(path, offset, limit))
                .map(|v| ("m_IncludesHash".to_string(), v)),
            ((self.m_target).inspect_value(path, offset, limit))
                .map(|v| ("m_Target".to_string(), v)),
            ((self.m_is_glsl).inspect_value(path, offset, limit))
                .map(|v| ("m_IsGLSL".to_string(), v)),
            ((self.m_from_other).inspect_value(path, offset, limit))
                .map(|v| ("m_FromOther".to_string(), v)),
            ((self.m_keyword_combinations_0).inspect_value(path, offset, limit))
                .map(|v| ("m_KeywordCombinations[0]".to_string(), v)),
            ((self.m_keyword_combinations_1).inspect_value(path, offset, limit))
                .map(|v| ("m_KeywordCombinations[1]".to_string(), v)),
            ((self.m_keyword_combinations_2).inspect_value(path, offset, limit))
                .map(|v| ("m_KeywordCombinations[2]".to_string(), v)),
            ((self.m_keyword_combinations_3).inspect_value(path, offset, limit))
                .map(|v| ("m_KeywordCombinations[3]".to_string(), v)),
            ((self.m_keyword_combinations_4).inspect_value(path, offset, limit))
                .map(|v| ("m_KeywordCombinations[4]".to_string(), v)),
            ((self.m_keyword_combinations_5).inspect_value(path, offset, limit))
                .map(|v| ("m_KeywordCombinations[5]".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
