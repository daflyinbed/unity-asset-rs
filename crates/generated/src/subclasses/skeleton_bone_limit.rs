//! 自动生成。请勿手动修改。

use crate::subclasses::quaternionf::Quaternionf;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct SkeletonBoneLimit {
    pub m_min: Vector3f,
    pub m_max: Vector3f,
    pub m_value: Vector3f,
    pub m_pre_q: Option<Quaternionf>,
    pub m_length: f32,
    pub m_post_q: Option<Quaternionf>,
    pub m_modified: bool,
}

impl crate::Parse for SkeletonBoneLimit {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_min = None;
        let mut m_max = None;
        let mut m_value = None;
        let mut m_pre_q = None;
        let mut m_length = None;
        let mut m_post_q = None;
        let mut m_modified = None;
        m_min = Some(Vector3f::parse(rdr, ver)?);
        m_max = Some(Vector3f::parse(rdr, ver)?);
        m_value = Some(Vector3f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
        } else {
            m_pre_q = Some(Quaternionf::parse(rdr, ver)?);
            m_post_q = Some(Quaternionf::parse(rdr, ver)?);
        }
        m_length = Some(rdr.read_f32()?);
        m_modified = Some(rdr.read_bool()?);
        rdr.align(4)?;

        Ok(SkeletonBoneLimit {
            m_min: m_min.unwrap(),
            m_max: m_max.unwrap(),
            m_value: m_value.unwrap(),
            m_pre_q,
            m_length: m_length.unwrap(),
            m_post_q,
            m_modified: m_modified.unwrap(),
        })
    }
}

impl crate::ToValue for SkeletonBoneLimit {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Min".to_string(), self.m_min.to_value())),
            Some(("m_Max".to_string(), self.m_max.to_value())),
            Some(("m_Value".to_string(), self.m_value.to_value())),
            crate::opt_field("m_PreQ", &self.m_pre_q),
            Some(("m_Length".to_string(), self.m_length.to_value())),
            crate::opt_field("m_PostQ", &self.m_post_q),
            Some(("m_Modified".to_string(), self.m_modified.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Min" => (self.m_min).inspect_value(path, offset, limit),
                "m_Max" => (self.m_max).inspect_value(path, offset, limit),
                "m_Value" => (self.m_value).inspect_value(path, offset, limit),
                "m_PreQ" => (self.m_pre_q).inspect_value(path, offset, limit),
                "m_Length" => (self.m_length).inspect_value(path, offset, limit),
                "m_PostQ" => (self.m_post_q).inspect_value(path, offset, limit),
                "m_Modified" => (self.m_modified).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_min).inspect_value(path, offset, limit)).map(|v| ("m_Min".to_string(), v)),
            ((self.m_max).inspect_value(path, offset, limit)).map(|v| ("m_Max".to_string(), v)),
            ((self.m_value).inspect_value(path, offset, limit)).map(|v| ("m_Value".to_string(), v)),
            ((self.m_pre_q).inspect_value(path, offset, limit)).map(|v| ("m_PreQ".to_string(), v)),
            ((self.m_length).inspect_value(path, offset, limit))
                .map(|v| ("m_Length".to_string(), v)),
            ((self.m_post_q).inspect_value(path, offset, limit))
                .map(|v| ("m_PostQ".to_string(), v)),
            ((self.m_modified).inspect_value(path, offset, limit))
                .map(|v| ("m_Modified".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
