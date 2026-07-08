//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TreePrototype {
    pub prefab: PPtr,
    pub bend_factor: f32,
    pub nav_mesh_lod: Option<i32>,
}

impl crate::Parse for TreePrototype {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let prefab = crate::PPtr::parse(rdr, ver)?;
        let bend_factor = rdr.read_f32()?;
        let nav_mesh_lod = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(TreePrototype {
            prefab,
            bend_factor,
            nav_mesh_lod,
        })
    }
}
