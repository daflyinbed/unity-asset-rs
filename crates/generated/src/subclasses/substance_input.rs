//! 自动生成。请勿手动修改。

use crate::subclasses::substance_enum_item::SubstanceEnumItem;
use crate::subclasses::substance_value::SubstanceValue;

#[derive(Debug, Clone)]
pub struct SubstanceInput {
    pub name: String,
    pub r#type: i32,
    pub value: SubstanceValue,
    pub internal_type: i32,
    pub internal_index: u32,
    pub minimum: f32,
    pub maximum: f32,
    pub step: f32,
    pub flags: u32,
    pub altered_textures_uid: Vec<u32>,
    pub enum_values: Vec<SubstanceEnumItem>,
}

impl crate::Parse for SubstanceInput {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let r#type = r.read_i32()?;
        let value = SubstanceValue::parse(r, ver)?;
        let internal_type = r.read_i32()?;
        let internal_index = r.read_u32()?;
        let minimum = r.read_f32()?;
        let maximum = r.read_f32()?;
        let step = r.read_f32()?;
        let flags = r.read_u32()?;
        let altered_textures_uid = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let enum_values = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SubstanceEnumItem::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(SubstanceInput {
            name,
            r#type,
            value,
            internal_type,
            internal_index,
            minimum,
            maximum,
            step,
            flags,
            altered_textures_uid,
            enum_values,
        })
    }
}
