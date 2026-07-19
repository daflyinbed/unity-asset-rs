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
        let mut bounds = None;
        let mut layer_indices = None;
        let mut number_of_objects = None;
        let mut coverage = None;
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
        } else {
            bounds = Some(Aabb::parse(rdr, ver)?);
        }
        layer_indices = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            coverage = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
            number_of_objects = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        }

        Ok(DetailPatch {
            bounds,
            layer_indices: layer_indices.unwrap(),
            number_of_objects,
            coverage,
        })
    }
}
