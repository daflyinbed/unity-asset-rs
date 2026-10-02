//! 自动生成。请勿手动修改。

use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct SkeletonPose {
    pub m_x: Vec<xform>,
}

impl crate::Parse for SkeletonPose {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_x = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| xform::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(SkeletonPose { m_x })
    }
}

impl crate::ToValue for SkeletonPose {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some(("m_X".to_string(), self.m_x.to_value()))]
            .into_iter()
            .flatten()
            .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_X" => (self.m_x).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> =
            vec![((self.m_x).inspect_value(path, offset, limit)).map(|v| ("m_X".to_string(), v))]
                .into_iter()
                .flatten()
                .collect();
        Some(crate::Value::Struct(fields))
    }
}
