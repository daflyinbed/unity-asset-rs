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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SubstanceInputComponentLabelsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubstanceEnumItem::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SubstanceEnumItem::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for SubstanceInputEnumValuesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
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
        altered_textures_uid = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
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

impl crate::ToValue for SubstanceInput {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("type".to_string(), self.r#type.to_value())),
            crate::opt_field("group", &self.group),
            crate::opt_field("label", &self.label),
            Some(("value".to_string(), self.value.to_value())),
            Some(("internalType".to_string(), self.internal_type.to_value())),
            crate::opt_field("visibleIf", &self.visible_if),
            Some(("internalIndex".to_string(), self.internal_index.to_value())),
            Some(("minimum".to_string(), self.minimum.to_value())),
            Some(("maximum".to_string(), self.maximum.to_value())),
            crate::opt_field("internalIdentifier", &self.internal_identifier),
            Some(("step".to_string(), self.step.to_value())),
            Some(("flags".to_string(), self.flags.to_value())),
            crate::opt_field("componentLabels", &self.component_labels),
            Some((
                "alteredTexturesUID".to_string(),
                self.altered_textures_uid.to_value(),
            )),
            Some(("enumValues".to_string(), self.enum_values.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "group" => (self.group).inspect_value(path, offset, limit),
                "label" => (self.label).inspect_value(path, offset, limit),
                "value" => (self.value).inspect_value(path, offset, limit),
                "internalType" => (self.internal_type).inspect_value(path, offset, limit),
                "visibleIf" => (self.visible_if).inspect_value(path, offset, limit),
                "internalIndex" => (self.internal_index).inspect_value(path, offset, limit),
                "minimum" => (self.minimum).inspect_value(path, offset, limit),
                "maximum" => (self.maximum).inspect_value(path, offset, limit),
                "internalIdentifier" => {
                    (self.internal_identifier).inspect_value(path, offset, limit)
                }
                "step" => (self.step).inspect_value(path, offset, limit),
                "flags" => (self.flags).inspect_value(path, offset, limit),
                "componentLabels" => (self.component_labels).inspect_value(path, offset, limit),
                "alteredTexturesUID" => {
                    (self.altered_textures_uid).inspect_value(path, offset, limit)
                }
                "enumValues" => (self.enum_values).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.group).inspect_value(path, offset, limit)).map(|v| ("group".to_string(), v)),
            ((self.label).inspect_value(path, offset, limit)).map(|v| ("label".to_string(), v)),
            ((self.value).inspect_value(path, offset, limit)).map(|v| ("value".to_string(), v)),
            ((self.internal_type).inspect_value(path, offset, limit))
                .map(|v| ("internalType".to_string(), v)),
            ((self.visible_if).inspect_value(path, offset, limit))
                .map(|v| ("visibleIf".to_string(), v)),
            ((self.internal_index).inspect_value(path, offset, limit))
                .map(|v| ("internalIndex".to_string(), v)),
            ((self.minimum).inspect_value(path, offset, limit)).map(|v| ("minimum".to_string(), v)),
            ((self.maximum).inspect_value(path, offset, limit)).map(|v| ("maximum".to_string(), v)),
            ((self.internal_identifier).inspect_value(path, offset, limit))
                .map(|v| ("internalIdentifier".to_string(), v)),
            ((self.step).inspect_value(path, offset, limit)).map(|v| ("step".to_string(), v)),
            ((self.flags).inspect_value(path, offset, limit)).map(|v| ("flags".to_string(), v)),
            ((self.component_labels).inspect_value(path, offset, limit))
                .map(|v| ("componentLabels".to_string(), v)),
            ((self.altered_textures_uid).inspect_value(path, offset, limit))
                .map(|v| ("alteredTexturesUID".to_string(), v)),
            ((self.enum_values).inspect_value(path, offset, limit))
                .map(|v| ("enumValues".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
