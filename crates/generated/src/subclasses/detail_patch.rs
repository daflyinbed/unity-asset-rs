//! 自动生成。请勿手动修改。

use crate::subclasses::aabb::Aabb;

#[derive(Debug, Clone)]
pub struct DetailPatch {
    pub bounds: Aabb,
    pub layer_indices: Vec<u8>,
    pub number_of_objects: Vec<u8>,
}

impl crate::Parse for DetailPatch {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bounds = Aabb::parse(r, ver)?;
        let layer_indices = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let number_of_objects = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(DetailPatch {
            bounds,
            layer_indices,
            number_of_objects,
        })
    }
}
