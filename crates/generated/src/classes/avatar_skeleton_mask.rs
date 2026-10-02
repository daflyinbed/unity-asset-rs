//! 自动生成。请勿手动修改。

use crate::subclasses::avatar_skeleton_mask_element::AvatarSkeletonMaskElement;

#[derive(Debug, Clone)]
pub struct AvatarSkeletonMask {
    pub m_name: String,
    pub elements: Vec<AvatarSkeletonMaskElement>,
}

impl crate::Parse for AvatarSkeletonMask {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let elements = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| AvatarSkeletonMaskElement::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(AvatarSkeletonMask { m_name, elements })
    }
}

impl crate::ToValue for AvatarSkeletonMask {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("elements".to_string(), self.elements.to_value())),
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
                "elements" => (self.elements).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.elements).inspect_value(path, offset, limit))
                .map(|v| ("elements".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
