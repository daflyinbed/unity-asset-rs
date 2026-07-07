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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_target_type = PresetType::parse(r, ver)?;
        r.align(4)?;
        let m_properties = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { PropertyModification::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_excluded_properties = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_coupled_type = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = PresetType::parse(r, ver)?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_coupled_properties = if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { PropertyModification::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
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
