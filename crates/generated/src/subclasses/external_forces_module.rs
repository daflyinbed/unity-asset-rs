//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::bit_field::BitField;
use crate::subclasses::min_max_curve::MinMaxCurve;

#[derive(Debug, Clone)]
pub struct ExternalForcesModule {
    pub enabled: bool,
    pub multiplier: Option<f32>,
    pub multiplier_curve: Option<MinMaxCurve>,
    pub influence_filter: Option<i32>,
    pub influence_mask: Option<BitField>,
    pub influence_list: Option<Vec<PPtr>>,
}

impl crate::Parse for ExternalForcesModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut multiplier = None;
        let mut multiplier_curve = None;
        let mut influence_filter = None;
        let mut influence_mask = None;
        let mut influence_list = None;
        enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            multiplier_curve = Some(MinMaxCurve::parse(rdr, ver)?);
            influence_filter = Some(rdr.read_i32()?);
            influence_mask = Some(BitField::parse(rdr, ver)?);
            influence_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            multiplier = Some(rdr.read_f32()?);
            influence_filter = Some(rdr.read_i32()?);
            influence_mask = Some(BitField::parse(rdr, ver)?);
            influence_list = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
            multiplier = Some(rdr.read_f32()?);
        }

        Ok(ExternalForcesModule {
            enabled: enabled.unwrap(),
            multiplier,
            multiplier_curve,
            influence_filter,
            influence_mask,
            influence_list,
        })
    }
}

impl crate::ToValue for ExternalForcesModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            crate::opt_field("multiplier", &self.multiplier),
            crate::opt_field("multiplierCurve", &self.multiplier_curve),
            crate::opt_field("influenceFilter", &self.influence_filter),
            crate::opt_field("influenceMask", &self.influence_mask),
            crate::opt_field("influenceList", &self.influence_list),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enabled" => (self.enabled).inspect_value(path, offset, limit),
                "multiplier" => (self.multiplier).inspect_value(path, offset, limit),
                "multiplierCurve" => (self.multiplier_curve).inspect_value(path, offset, limit),
                "influenceFilter" => (self.influence_filter).inspect_value(path, offset, limit),
                "influenceMask" => (self.influence_mask).inspect_value(path, offset, limit),
                "influenceList" => (self.influence_list).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.multiplier).inspect_value(path, offset, limit))
                .map(|v| ("multiplier".to_string(), v)),
            ((self.multiplier_curve).inspect_value(path, offset, limit))
                .map(|v| ("multiplierCurve".to_string(), v)),
            ((self.influence_filter).inspect_value(path, offset, limit))
                .map(|v| ("influenceFilter".to_string(), v)),
            ((self.influence_mask).inspect_value(path, offset, limit))
                .map(|v| ("influenceMask".to_string(), v)),
            ((self.influence_list).inspect_value(path, offset, limit))
                .map(|v| ("influenceList".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
