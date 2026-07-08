//! 自动生成。请勿手动修改。

use crate::subclasses::aabb::Aabb;

#[derive(Debug, Clone)]
pub struct DetailPatch {
    pub bounds: Option<Aabb>,
    pub layer_indices: Vec<u8>,
    pub number_of_objects: Option<Vec<u8>>,
    pub coverage: Option<Vec<u8>>,
}

impl crate::Parse for DetailPatch {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bounds = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(Aabb::parse(rdr, ver)?)
        } else {
            None
        };
        let layer_indices = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let number_of_objects = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let coverage = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(DetailPatch {
            bounds,
            layer_indices,
            number_of_objects,
            coverage,
        })
    }
}
