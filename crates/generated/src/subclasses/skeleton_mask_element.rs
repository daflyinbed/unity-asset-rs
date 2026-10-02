//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SkeletonMaskElement {
    pub m_index: Option<u32>,
    pub m_path_hash: Option<u32>,
    pub m_weight: f32,
}

impl crate::Parse for SkeletonMaskElement {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_index = None;
        let mut m_path_hash = None;
        let mut m_weight = None;
        if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_path_hash = Some(rdr.read_u32()?);
        } else {
            m_index = Some(rdr.read_u32()?);
        }
        m_weight = Some(rdr.read_f32()?);

        Ok(SkeletonMaskElement {
            m_index,
            m_path_hash,
            m_weight: m_weight.unwrap(),
        })
    }
}

impl crate::ToValue for SkeletonMaskElement {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_Index", &self.m_index),
            crate::opt_field("m_PathHash", &self.m_path_hash),
            Some(("m_Weight".to_string(), self.m_weight.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Index" => (self.m_index).inspect_value(path, offset, limit),
                "m_PathHash" => (self.m_path_hash).inspect_value(path, offset, limit),
                "m_Weight" => (self.m_weight).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_index).inspect_value(path, offset, limit)).map(|v| ("m_Index".to_string(), v)),
            ((self.m_path_hash).inspect_value(path, offset, limit))
                .map(|v| ("m_PathHash".to_string(), v)),
            ((self.m_weight).inspect_value(path, offset, limit))
                .map(|v| ("m_Weight".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
