//! 自动生成。请勿手动修改。

use crate::subclasses::variant_info::VariantInfo;

#[derive(Debug, Clone)]
pub struct ShaderInfo {
    pub variants: Vec<VariantInfo>,
}

impl crate::Parse for ShaderInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let variants = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| VariantInfo::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(ShaderInfo { variants })
    }
}

impl crate::ToValue for ShaderInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("variants".to_string(), self.variants.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "variants" => (self.variants).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.variants).inspect_value(path, offset, limit))
                .map(|v| ("variants".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
