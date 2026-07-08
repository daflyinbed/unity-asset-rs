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
        let enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let multiplier = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let multiplier_curve = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = MinMaxCurve::parse(rdr, ver)?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let influence_filter = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let influence_mask = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            Some(BitField::parse(rdr, ver)?)
        } else {
            None
        };
        let influence_list = if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(ExternalForcesModule {
            enabled,
            multiplier,
            multiplier_curve,
            influence_filter,
            influence_mask,
            influence_list,
        })
    }
}
