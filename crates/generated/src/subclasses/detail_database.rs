//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgb_af::ColorRgbAf;
use crate::subclasses::detail_patch::DetailPatch;
use crate::subclasses::detail_prototype::DetailPrototype;
use crate::subclasses::tree_instance::TreeInstance;
use crate::subclasses::tree_prototype::TreePrototype;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：DetailDatabaseMPatchesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum DetailDatabaseMPatchesVariant {
    Default(Vec<DetailPatch>),
    V2017_1(Vec<DetailPatch>),
}

impl crate::Parse for DetailDatabaseMPatchesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| DetailPatch::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| DetailPatch::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for DetailDatabaseMPatchesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：DetailDatabaseMDetailPrototypesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum DetailDatabaseMDetailPrototypesVariant {
    Default(Vec<DetailPrototype>),
    V2017_1(Vec<DetailPrototype>),
}

impl crate::Parse for DetailDatabaseMDetailPrototypesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| DetailPrototype::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| DetailPrototype::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for DetailDatabaseMDetailPrototypesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：DetailDatabaseMRandomRotationsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum DetailDatabaseMRandomRotationsVariant {
    Default(Vec<Vector3f>),
    V2017_1(Vec<Vector3f>),
}

impl crate::Parse for DetailDatabaseMRandomRotationsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for DetailDatabaseMRandomRotationsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：DetailDatabaseMTreeInstancesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum DetailDatabaseMTreeInstancesVariant {
    Default(Vec<TreeInstance>),
    V2017_1(Vec<TreeInstance>),
}

impl crate::Parse for DetailDatabaseMTreeInstancesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TreeInstance::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TreeInstance::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for DetailDatabaseMTreeInstancesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：DetailDatabaseMTreePrototypesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum DetailDatabaseMTreePrototypesVariant {
    Default(Vec<TreePrototype>),
    V2017_1(Vec<TreePrototype>),
}

impl crate::Parse for DetailDatabaseMTreePrototypesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TreePrototype::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| TreePrototype::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for DetailDatabaseMTreePrototypesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：DetailDatabaseMPreloadTextureAtlasDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum DetailDatabaseMPreloadTextureAtlasDataVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for DetailDatabaseMPreloadTextureAtlasDataVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for DetailDatabaseMPreloadTextureAtlasDataVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct DetailDatabase {
    pub m_patches: DetailDatabaseMPatchesVariant,
    pub m_detail_prototypes: DetailDatabaseMDetailPrototypesVariant,
    pub m_patch_count: i32,
    pub m_patch_samples: i32,
    pub m_random_rotations: Option<DetailDatabaseMRandomRotationsVariant>,
    pub waving_grass_tint: ColorRgbAf,
    pub m_waving_grass_strength: f32,
    pub m_waving_grass_amount: f32,
    pub m_waving_grass_speed: f32,
    pub m_tree_instances: DetailDatabaseMTreeInstancesVariant,
    pub m_detail_billboard_shader: Option<PPtr>,
    pub m_detail_scatter_mode: Option<i32>,
    pub m_tree_prototypes: DetailDatabaseMTreePrototypesVariant,
    pub m_detail_mesh_lit_shader: Option<PPtr>,
    pub m_preload_texture_atlas_data: DetailDatabaseMPreloadTextureAtlasDataVariant,
    pub m_detail_mesh_grass_shader: Option<PPtr>,
    pub m_default_shaders_0: Option<PPtr>,
    pub m_default_shaders_1: Option<PPtr>,
    pub m_default_shaders_2: Option<PPtr>,
}

impl crate::Parse for DetailDatabase {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_patches = None;
        let mut m_detail_prototypes = None;
        let mut m_patch_count = None;
        let mut m_patch_samples = None;
        let mut m_random_rotations = None;
        let mut waving_grass_tint = None;
        let mut m_waving_grass_strength = None;
        let mut m_waving_grass_amount = None;
        let mut m_waving_grass_speed = None;
        let mut m_tree_instances = None;
        let mut m_detail_billboard_shader = None;
        let mut m_detail_scatter_mode = None;
        let mut m_tree_prototypes = None;
        let mut m_detail_mesh_lit_shader = None;
        let mut m_preload_texture_atlas_data = None;
        let mut m_detail_mesh_grass_shader = None;
        let mut m_default_shaders_0 = None;
        let mut m_default_shaders_1 = None;
        let mut m_default_shaders_2 = None;
        m_patches = Some(DetailDatabaseMPatchesVariant::parse(rdr, ver)?);
        m_detail_prototypes = Some(DetailDatabaseMDetailPrototypesVariant::parse(rdr, ver)?);
        m_patch_count = Some(rdr.read_i32()?);
        m_patch_samples = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
        } else {
            m_random_rotations = Some(DetailDatabaseMRandomRotationsVariant::parse(rdr, ver)?);
        }
        waving_grass_tint = Some(ColorRgbAf::parse(rdr, ver)?);
        m_waving_grass_strength = Some(rdr.read_f32()?);
        m_waving_grass_amount = Some(rdr.read_f32()?);
        m_waving_grass_speed = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_detail_scatter_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_detail_billboard_shader = Some(crate::PPtr::parse(rdr, ver)?);
            m_detail_mesh_lit_shader = Some(crate::PPtr::parse(rdr, ver)?);
            m_detail_mesh_grass_shader = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }
        m_tree_instances = Some(DetailDatabaseMTreeInstancesVariant::parse(rdr, ver)?);
        m_tree_prototypes = Some(DetailDatabaseMTreePrototypesVariant::parse(rdr, ver)?);
        m_preload_texture_atlas_data = Some(DetailDatabaseMPreloadTextureAtlasDataVariant::parse(
            rdr, ver,
        )?);
        if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            m_default_shaders_0 = Some(crate::PPtr::parse(rdr, ver)?);
            m_default_shaders_1 = Some(crate::PPtr::parse(rdr, ver)?);
            m_default_shaders_2 = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
        }

        Ok(DetailDatabase {
            m_patches: m_patches.unwrap(),
            m_detail_prototypes: m_detail_prototypes.unwrap(),
            m_patch_count: m_patch_count.unwrap(),
            m_patch_samples: m_patch_samples.unwrap(),
            m_random_rotations,
            waving_grass_tint: waving_grass_tint.unwrap(),
            m_waving_grass_strength: m_waving_grass_strength.unwrap(),
            m_waving_grass_amount: m_waving_grass_amount.unwrap(),
            m_waving_grass_speed: m_waving_grass_speed.unwrap(),
            m_tree_instances: m_tree_instances.unwrap(),
            m_detail_billboard_shader,
            m_detail_scatter_mode,
            m_tree_prototypes: m_tree_prototypes.unwrap(),
            m_detail_mesh_lit_shader,
            m_preload_texture_atlas_data: m_preload_texture_atlas_data.unwrap(),
            m_detail_mesh_grass_shader,
            m_default_shaders_0,
            m_default_shaders_1,
            m_default_shaders_2,
        })
    }
}

impl crate::ToValue for DetailDatabase {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Patches".to_string(), self.m_patches.to_value())),
            Some((
                "m_DetailPrototypes".to_string(),
                self.m_detail_prototypes.to_value(),
            )),
            Some(("m_PatchCount".to_string(), self.m_patch_count.to_value())),
            Some((
                "m_PatchSamples".to_string(),
                self.m_patch_samples.to_value(),
            )),
            crate::opt_field("m_RandomRotations", &self.m_random_rotations),
            Some((
                "WavingGrassTint".to_string(),
                self.waving_grass_tint.to_value(),
            )),
            Some((
                "m_WavingGrassStrength".to_string(),
                self.m_waving_grass_strength.to_value(),
            )),
            Some((
                "m_WavingGrassAmount".to_string(),
                self.m_waving_grass_amount.to_value(),
            )),
            Some((
                "m_WavingGrassSpeed".to_string(),
                self.m_waving_grass_speed.to_value(),
            )),
            Some((
                "m_TreeInstances".to_string(),
                self.m_tree_instances.to_value(),
            )),
            crate::opt_field("m_DetailBillboardShader", &self.m_detail_billboard_shader),
            crate::opt_field("m_DetailScatterMode", &self.m_detail_scatter_mode),
            Some((
                "m_TreePrototypes".to_string(),
                self.m_tree_prototypes.to_value(),
            )),
            crate::opt_field("m_DetailMeshLitShader", &self.m_detail_mesh_lit_shader),
            Some((
                "m_PreloadTextureAtlasData".to_string(),
                self.m_preload_texture_atlas_data.to_value(),
            )),
            crate::opt_field("m_DetailMeshGrassShader", &self.m_detail_mesh_grass_shader),
            crate::opt_field("m_DefaultShaders[0]", &self.m_default_shaders_0),
            crate::opt_field("m_DefaultShaders[1]", &self.m_default_shaders_1),
            crate::opt_field("m_DefaultShaders[2]", &self.m_default_shaders_2),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Patches" => (self.m_patches).inspect_value(path, offset, limit),
                "m_DetailPrototypes" => {
                    (self.m_detail_prototypes).inspect_value(path, offset, limit)
                }
                "m_PatchCount" => (self.m_patch_count).inspect_value(path, offset, limit),
                "m_PatchSamples" => (self.m_patch_samples).inspect_value(path, offset, limit),
                "m_RandomRotations" => (self.m_random_rotations).inspect_value(path, offset, limit),
                "WavingGrassTint" => (self.waving_grass_tint).inspect_value(path, offset, limit),
                "m_WavingGrassStrength" => {
                    (self.m_waving_grass_strength).inspect_value(path, offset, limit)
                }
                "m_WavingGrassAmount" => {
                    (self.m_waving_grass_amount).inspect_value(path, offset, limit)
                }
                "m_WavingGrassSpeed" => {
                    (self.m_waving_grass_speed).inspect_value(path, offset, limit)
                }
                "m_TreeInstances" => (self.m_tree_instances).inspect_value(path, offset, limit),
                "m_DetailBillboardShader" => {
                    (self.m_detail_billboard_shader).inspect_value(path, offset, limit)
                }
                "m_DetailScatterMode" => {
                    (self.m_detail_scatter_mode).inspect_value(path, offset, limit)
                }
                "m_TreePrototypes" => (self.m_tree_prototypes).inspect_value(path, offset, limit),
                "m_DetailMeshLitShader" => {
                    (self.m_detail_mesh_lit_shader).inspect_value(path, offset, limit)
                }
                "m_PreloadTextureAtlasData" => {
                    (self.m_preload_texture_atlas_data).inspect_value(path, offset, limit)
                }
                "m_DetailMeshGrassShader" => {
                    (self.m_detail_mesh_grass_shader).inspect_value(path, offset, limit)
                }
                "m_DefaultShaders[0]" => {
                    (self.m_default_shaders_0).inspect_value(path, offset, limit)
                }
                "m_DefaultShaders[1]" => {
                    (self.m_default_shaders_1).inspect_value(path, offset, limit)
                }
                "m_DefaultShaders[2]" => {
                    (self.m_default_shaders_2).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_patches).inspect_value(path, offset, limit))
                .map(|v| ("m_Patches".to_string(), v)),
            ((self.m_detail_prototypes).inspect_value(path, offset, limit))
                .map(|v| ("m_DetailPrototypes".to_string(), v)),
            ((self.m_patch_count).inspect_value(path, offset, limit))
                .map(|v| ("m_PatchCount".to_string(), v)),
            ((self.m_patch_samples).inspect_value(path, offset, limit))
                .map(|v| ("m_PatchSamples".to_string(), v)),
            ((self.m_random_rotations).inspect_value(path, offset, limit))
                .map(|v| ("m_RandomRotations".to_string(), v)),
            ((self.waving_grass_tint).inspect_value(path, offset, limit))
                .map(|v| ("WavingGrassTint".to_string(), v)),
            ((self.m_waving_grass_strength).inspect_value(path, offset, limit))
                .map(|v| ("m_WavingGrassStrength".to_string(), v)),
            ((self.m_waving_grass_amount).inspect_value(path, offset, limit))
                .map(|v| ("m_WavingGrassAmount".to_string(), v)),
            ((self.m_waving_grass_speed).inspect_value(path, offset, limit))
                .map(|v| ("m_WavingGrassSpeed".to_string(), v)),
            ((self.m_tree_instances).inspect_value(path, offset, limit))
                .map(|v| ("m_TreeInstances".to_string(), v)),
            ((self.m_detail_billboard_shader).inspect_value(path, offset, limit))
                .map(|v| ("m_DetailBillboardShader".to_string(), v)),
            ((self.m_detail_scatter_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_DetailScatterMode".to_string(), v)),
            ((self.m_tree_prototypes).inspect_value(path, offset, limit))
                .map(|v| ("m_TreePrototypes".to_string(), v)),
            ((self.m_detail_mesh_lit_shader).inspect_value(path, offset, limit))
                .map(|v| ("m_DetailMeshLitShader".to_string(), v)),
            ((self.m_preload_texture_atlas_data).inspect_value(path, offset, limit))
                .map(|v| ("m_PreloadTextureAtlasData".to_string(), v)),
            ((self.m_detail_mesh_grass_shader).inspect_value(path, offset, limit))
                .map(|v| ("m_DetailMeshGrassShader".to_string(), v)),
            ((self.m_default_shaders_0).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultShaders[0]".to_string(), v)),
            ((self.m_default_shaders_1).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultShaders[1]".to_string(), v)),
            ((self.m_default_shaders_2).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultShaders[2]".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
