//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_entry_expression_value_gradient::VfxEntryExpressionValue_Gradient;

#[derive(Debug, Clone)]
pub struct VfxField_VfxEntryExpressionValue_Gradient {
    pub m_array: Vec<VfxEntryExpressionValue_Gradient>,
}

impl crate::Parse for VfxField_VfxEntryExpressionValue_Gradient {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| VfxEntryExpressionValue_Gradient::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(VfxField_VfxEntryExpressionValue_Gradient { m_array })
    }
}

impl crate::ToValue for VfxField_VfxEntryExpressionValue_Gradient {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> =
            [Some(("m_Array".to_string(), self.m_array.to_value()))]
                .into_iter()
                .flatten()
                .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Array" => (self.m_array).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_array).inspect_value(path, offset, limit)).map(|v| ("m_Array".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
