//! 自动生成。请勿手动修改。

use crate::subclasses::substance_enum_item::SubstanceEnumItem;
use crate::subclasses::substance_value::SubstanceValue;


/// 版本枚举：SubstanceInputComponentLabelsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SubstanceInputComponentLabelsVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for SubstanceInputComponentLabelsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SubstanceInputEnumValuesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SubstanceInputEnumValuesVariant {
    Default(Vec<SubstanceEnumItem>),
    V2017_1(Vec<SubstanceEnumItem>),
}

impl crate::Parse for SubstanceInputEnumValuesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SubstanceEnumItem::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SubstanceEnumItem::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubstanceInput {
    pub name: String,
    pub r#type: i32,
    pub group: Option<String>,
    pub label: Option<String>,
    pub value: SubstanceValue,
    pub internal_type: i32,
    pub visible_if: Option<String>,
    pub internal_index: u32,
    pub minimum: f32,
    pub maximum: f32,
    pub internal_identifier: Option<u32>,
    pub step: f32,
    pub flags: u32,
    pub component_labels: Option<SubstanceInputComponentLabelsVariant>,
    pub altered_textures_uid: Vec<u32>,
    pub enum_values: SubstanceInputEnumValuesVariant,
}

impl crate::Parse for SubstanceInput {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut r#type = None;
        let mut group = None;
        let mut label = None;
        let mut value = None;
        let mut internal_type = None;
        let mut visible_if = None;
        let mut internal_index = None;
        let mut minimum = None;
        let mut maximum = None;
        let mut internal_identifier = None;
        let mut step = None;
        let mut flags = None;
        let mut component_labels = None;
        let mut altered_textures_uid = None;
        let mut enum_values = None;
            name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(5, 2, 0) {
            label = Some(rdr.read_aligned_string()?);
            group = Some(rdr.read_aligned_string()?);
            visible_if = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(4, 1, 0) {
            label = Some(rdr.read_aligned_string()?);
            group = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            group = Some(rdr.read_aligned_string()?);
        } else {
        }
            r#type = Some(rdr.read_i32()?);
            value = Some(SubstanceValue::parse(rdr, ver)?);
            internal_type = Some(rdr.read_i32()?);
            internal_index = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            internal_identifier = Some(rdr.read_u32()?);
            component_labels = Some(SubstanceInputComponentLabelsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            internal_identifier = Some(rdr.read_u32()?);
        } else {
        }
            minimum = Some(rdr.read_f32()?);
            maximum = Some(rdr.read_f32()?);
            step = Some(rdr.read_f32()?);
            flags = Some(rdr.read_u32()?);
            altered_textures_uid = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
            enum_values = Some(SubstanceInputEnumValuesVariant::parse(rdr, ver)?);

        Ok(SubstanceInput {
            name: name.unwrap(),
            r#type: r#type.unwrap(),
            group,
            label,
            value: value.unwrap(),
            internal_type: internal_type.unwrap(),
            visible_if,
            internal_index: internal_index.unwrap(),
            minimum: minimum.unwrap(),
            maximum: maximum.unwrap(),
            internal_identifier,
            step: step.unwrap(),
            flags: flags.unwrap(),
            component_labels,
            altered_textures_uid: altered_textures_uid.unwrap(),
            enum_values: enum_values.unwrap(),
        })
    }
}
