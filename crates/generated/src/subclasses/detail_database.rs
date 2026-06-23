//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba::ColorRgba;
use crate::subclasses::detail_patch::DetailPatch;
use crate::subclasses::detail_prototype::DetailPrototype;
use crate::subclasses::tree_instance::TreeInstance;
use crate::subclasses::tree_prototype::TreePrototype;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct DetailDatabase {
    pub m_patches: Vec<DetailPatch>,
    pub m_detail_prototypes: Vec<DetailPrototype>,
    pub m_patch_count: i32,
    pub m_patch_samples: i32,
    pub m_random_rotations: Vec<Vector3f>,
    pub waving_grass_tint: ColorRgba,
    pub m_waving_grass_strength: f32,
    pub m_waving_grass_amount: f32,
    pub m_waving_grass_speed: f32,
    pub m_tree_instances: Vec<TreeInstance>,
    pub m_tree_prototypes: Vec<TreePrototype>,
    pub m_preload_texture_atlas_data: Vec<PPtr>,
}

impl crate::Parse for DetailDatabase {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_patches = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { DetailPatch::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_detail_prototypes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { DetailPrototype::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_patch_count = r.read_i32()?;
        let m_patch_samples = r.read_i32()?;
        let m_random_rotations = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Vector3f::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let waving_grass_tint = ColorRgba::parse(r, ver)?;
        let m_waving_grass_strength = r.read_f32()?;
        let m_waving_grass_amount = r.read_f32()?;
        let m_waving_grass_speed = r.read_f32()?;
        let m_tree_instances = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TreeInstance::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_tree_prototypes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { TreePrototype::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_preload_texture_atlas_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(DetailDatabase {
            m_patches,
            m_detail_prototypes,
            m_patch_count,
            m_patch_samples,
            m_random_rotations,
            waving_grass_tint,
            m_waving_grass_strength,
            m_waving_grass_amount,
            m_waving_grass_speed,
            m_tree_instances,
            m_tree_prototypes,
            m_preload_texture_atlas_data,
        })
    }
}
