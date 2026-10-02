//! 自动生成。请勿手动修改。

use crate::subclasses::skeleton_bone_limit::SkeletonBoneLimit;

#[derive(Debug, Clone)]
pub struct HumanBone {
    pub m_bone_name: String,
    pub m_human_name: String,
    pub m_limit: SkeletonBoneLimit,
}

impl crate::Parse for HumanBone {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_bone_name = rdr.read_aligned_string()?;
        let m_human_name = rdr.read_aligned_string()?;
        let m_limit = SkeletonBoneLimit::parse(rdr, ver)?;

        Ok(HumanBone {
            m_bone_name,
            m_human_name,
            m_limit,
        })
    }
}

impl crate::ToValue for HumanBone {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BoneName".to_string(), self.m_bone_name.to_value())),
            Some(("m_HumanName".to_string(), self.m_human_name.to_value())),
            Some(("m_Limit".to_string(), self.m_limit.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_BoneName" => (self.m_bone_name).inspect_value(path, offset, limit),
                "m_HumanName" => (self.m_human_name).inspect_value(path, offset, limit),
                "m_Limit" => (self.m_limit).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_bone_name).inspect_value(path, offset, limit))
                .map(|v| ("m_BoneName".to_string(), v)),
            ((self.m_human_name).inspect_value(path, offset, limit))
                .map(|v| ("m_HumanName".to_string(), v)),
            ((self.m_limit).inspect_value(path, offset, limit)).map(|v| ("m_Limit".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
