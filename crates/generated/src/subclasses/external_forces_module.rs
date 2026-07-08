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
            rdr.align(4)?;
            influence_filter = Some(rdr.read_i32()?);
            influence_mask = Some(BitField::parse(rdr, ver)?);
            influence_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            multiplier = Some(rdr.read_f32()?);
            influence_filter = Some(rdr.read_i32()?);
            influence_mask = Some(BitField::parse(rdr, ver)?);
            influence_list = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
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
