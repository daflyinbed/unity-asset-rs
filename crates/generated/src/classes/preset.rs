//! 自动生成。请勿手动修改。

use crate::subclasses::preset_type::PresetType;
use crate::subclasses::property_modification::PropertyModification;

#[derive(Debug, Clone)]
pub struct Preset {
    pub m_name: String,
    pub m_target_type: PresetType,
    pub m_properties: Vec<PropertyModification>,
    pub m_excluded_properties: Option<Vec<String>>,
    pub m_coupled_type: Option<PresetType>,
    pub m_coupled_properties: Option<Vec<PropertyModification>>,
}

impl crate::Parse for Preset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_target_type = PresetType::parse(rdr, ver)?;
        let m_properties = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| PropertyModification::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_excluded_properties = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_coupled_type = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            Some(PresetType::parse(rdr, ver)?)
        } else {
            None
        };
        let m_coupled_properties = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| PropertyModification::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(Preset {
            m_name,
            m_target_type,
            m_properties,
            m_excluded_properties,
            m_coupled_type,
            m_coupled_properties,
        })
    }
}

impl crate::ToValue for Preset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_TargetType".to_string(), self.m_target_type.to_value())),
            Some(("m_Properties".to_string(), self.m_properties.to_value())),
            crate::opt_field("m_ExcludedProperties", &self.m_excluded_properties),
            crate::opt_field("m_CoupledType", &self.m_coupled_type),
            crate::opt_field("m_CoupledProperties", &self.m_coupled_properties),
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
                "m_TargetType" => (self.m_target_type).inspect_value(path, offset, limit),
                "m_Properties" => (self.m_properties).inspect_value(path, offset, limit),
                "m_ExcludedProperties" => {
                    (self.m_excluded_properties).inspect_value(path, offset, limit)
                }
                "m_CoupledType" => (self.m_coupled_type).inspect_value(path, offset, limit),
                "m_CoupledProperties" => {
                    (self.m_coupled_properties).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_target_type).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetType".to_string(), v)),
            ((self.m_properties).inspect_value(path, offset, limit))
                .map(|v| ("m_Properties".to_string(), v)),
            ((self.m_excluded_properties).inspect_value(path, offset, limit))
                .map(|v| ("m_ExcludedProperties".to_string(), v)),
            ((self.m_coupled_type).inspect_value(path, offset, limit))
                .map(|v| ("m_CoupledType".to_string(), v)),
            ((self.m_coupled_properties).inspect_value(path, offset, limit))
                .map(|v| ("m_CoupledProperties".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
