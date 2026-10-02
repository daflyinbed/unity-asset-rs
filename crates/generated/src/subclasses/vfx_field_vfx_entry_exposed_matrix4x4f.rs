//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_entry_exposed_matrix4x4f::VfxEntryExposed_Matrix4x4f;

#[derive(Debug, Clone)]
pub struct VfxField_VfxEntryExposed_Matrix4x4f {
    pub m_array: Vec<VfxEntryExposed_Matrix4x4f>,
}

impl crate::Parse for VfxField_VfxEntryExposed_Matrix4x4f {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| VfxEntryExposed_Matrix4x4f::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(VfxField_VfxEntryExposed_Matrix4x4f { m_array })
    }
}

impl crate::ToValue for VfxField_VfxEntryExposed_Matrix4x4f {
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
