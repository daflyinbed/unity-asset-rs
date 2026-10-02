//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct iOsDeviceRequirementForResourceVariant {
    pub variant_tag_name: String,
    pub variations: Vec<Vec<(String, String)>>,
}

impl crate::Parse for iOsDeviceRequirementForResourceVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let variant_tag_name = rdr.read_aligned_string()?;
        let variations = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let n = rdr.read_i32()? as usize;
                        (0..n)
                            .map(|_| {
                                ({
                                    let a = rdr.read_aligned_string()?;
                                    let b = rdr.read_aligned_string()?;
                                    Ok((a, b))
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(iOsDeviceRequirementForResourceVariant {
            variant_tag_name,
            variations,
        })
    }
}

impl crate::ToValue for iOsDeviceRequirementForResourceVariant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "variantTagName".to_string(),
                self.variant_tag_name.to_value(),
            )),
            Some(("variations".to_string(), self.variations.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "variantTagName" => (self.variant_tag_name).inspect_value(path, offset, limit),
                "variations" => (self.variations).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.variant_tag_name).inspect_value(path, offset, limit))
                .map(|v| ("variantTagName".to_string(), v)),
            ((self.variations).inspect_value(path, offset, limit))
                .map(|v| ("variations".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
