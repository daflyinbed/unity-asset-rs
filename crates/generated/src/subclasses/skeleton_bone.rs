//! 自动生成。请勿手动修改。

use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SkeletonBone {
    pub m_name: String,
    pub m_position: Vector3f,
    pub m_parent_name: Option<String>,
    pub m_rotation: Quaternionf,
    pub m_scale: Vector3f,
    pub m_transform_modified: Option<bool>,
}

impl crate::Parse for SkeletonBone {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_name = None;
        let mut m_position = None;
        let mut m_parent_name = None;
        let mut m_rotation = None;
        let mut m_scale = None;
        let mut m_transform_modified = None;
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_parent_name = Some(rdr.read_aligned_string()?);
        } else {
        }
        m_position = Some(Vector3f::parse(rdr, ver)?);
        m_rotation = Some(Quaternionf::parse(rdr, ver)?);
        m_scale = Some(Vector3f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else {
            m_transform_modified = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }

        Ok(SkeletonBone {
            m_name: m_name.unwrap(),
            m_position: m_position.unwrap(),
            m_parent_name,
            m_rotation: m_rotation.unwrap(),
            m_scale: m_scale.unwrap(),
            m_transform_modified,
        })
    }
}

impl crate::ToValue for SkeletonBone {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Position".to_string(), self.m_position.to_value())),
            crate::opt_field("m_ParentName", &self.m_parent_name),
            Some(("m_Rotation".to_string(), self.m_rotation.to_value())),
            Some(("m_Scale".to_string(), self.m_scale.to_value())),
            crate::opt_field("m_TransformModified", &self.m_transform_modified),
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
                "m_Position" => (self.m_position).inspect_value(path, offset, limit),
                "m_ParentName" => (self.m_parent_name).inspect_value(path, offset, limit),
                "m_Rotation" => (self.m_rotation).inspect_value(path, offset, limit),
                "m_Scale" => (self.m_scale).inspect_value(path, offset, limit),
                "m_TransformModified" => {
                    (self.m_transform_modified).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_position).inspect_value(path, offset, limit))
                .map(|v| ("m_Position".to_string(), v)),
            ((self.m_parent_name).inspect_value(path, offset, limit))
                .map(|v| ("m_ParentName".to_string(), v)),
            ((self.m_rotation).inspect_value(path, offset, limit))
                .map(|v| ("m_Rotation".to_string(), v)),
            ((self.m_scale).inspect_value(path, offset, limit)).map(|v| ("m_Scale".to_string(), v)),
            ((self.m_transform_modified).inspect_value(path, offset, limit))
                .map(|v| ("m_TransformModified".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
