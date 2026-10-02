//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::multi_mode_parameter_mesh_spawn::MultiModeParameter_MeshSpawn;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：ShapeModuleRadiusVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ShapeModuleRadiusVariant {
    Default(f32),
    V5_6(Box<MultiModeParameter_MeshSpawn>),
}

impl crate::Parse for ShapeModuleRadiusVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({ Ok(Box::new(MultiModeParameter_MeshSpawn::parse(rdr, ver)?)) })?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_f32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ShapeModuleRadiusVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ShapeModuleArcVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ShapeModuleArcVariant {
    Default(f32),
    V5_6(Box<MultiModeParameter_MeshSpawn>),
}

impl crate::Parse for ShapeModuleArcVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = ({ Ok(Box::new(MultiModeParameter_MeshSpawn::parse(rdr, ver)?)) })?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_f32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ShapeModuleArcVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ShapeModule {
    pub enabled: bool,
    pub r#type: i32,
    pub radius: ShapeModuleRadiusVariant,
    pub angle: f32,
    pub length: Option<f32>,
    pub box_x: Option<f32>,
    pub box_thickness: Option<Vector3f>,
    pub box_y: Option<f32>,
    pub radius_thickness: Option<f32>,
    pub box_z: Option<f32>,
    pub donut_radius: Option<f32>,
    pub placement_mode: i32,
    pub m_position: Option<Vector3f>,
    pub m_mesh: PPtr,
    pub arc: Option<ShapeModuleArcVariant>,
    pub m_rotation: Option<Vector3f>,
    pub random_direction: Option<bool>,
    pub m_scale: Option<Vector3f>,
    pub m_mesh_renderer: Option<PPtr>,
    pub m_mesh_material_index: Option<i32>,
    pub m_skinned_mesh_renderer: Option<PPtr>,
    pub m_mesh_normal_offset: Option<f32>,
    pub m_mesh_spawn: Option<MultiModeParameter_MeshSpawn>,
    pub m_use_mesh_material_index: Option<bool>,
    pub m_mesh_scale: Option<f32>,
    pub m_use_mesh_colors: Option<bool>,
    pub m_sprite: Option<PPtr>,
    pub m_sprite_renderer: Option<PPtr>,
    pub align_to_direction: Option<bool>,
    pub random_direction_amount: Option<f32>,
    pub m_texture: Option<PPtr>,
    pub spherical_direction_amount: Option<f32>,
    pub m_texture_clip_channel: Option<i32>,
    pub random_position_amount: Option<f32>,
    pub m_texture_clip_threshold: Option<f32>,
    pub m_texture_uv_channel: Option<i32>,
    pub m_texture_color_affects_particles: Option<bool>,
    pub m_texture_alpha_affects_particles: Option<bool>,
    pub m_texture_bilinear_filtering: Option<bool>,
}

impl crate::Parse for ShapeModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut r#type = None;
        let mut radius = None;
        let mut angle = None;
        let mut length = None;
        let mut box_x = None;
        let mut box_thickness = None;
        let mut box_y = None;
        let mut radius_thickness = None;
        let mut box_z = None;
        let mut donut_radius = None;
        let mut placement_mode = None;
        let mut m_position = None;
        let mut m_mesh = None;
        let mut arc = None;
        let mut m_rotation = None;
        let mut random_direction = None;
        let mut m_scale = None;
        let mut m_mesh_renderer = None;
        let mut m_mesh_material_index = None;
        let mut m_skinned_mesh_renderer = None;
        let mut m_mesh_normal_offset = None;
        let mut m_mesh_spawn = None;
        let mut m_use_mesh_material_index = None;
        let mut m_mesh_scale = None;
        let mut m_use_mesh_colors = None;
        let mut m_sprite = None;
        let mut m_sprite_renderer = None;
        let mut align_to_direction = None;
        let mut random_direction_amount = None;
        let mut m_texture = None;
        let mut spherical_direction_amount = None;
        let mut m_texture_clip_channel = None;
        let mut random_position_amount = None;
        let mut m_texture_clip_threshold = None;
        let mut m_texture_uv_channel = None;
        let mut m_texture_color_affects_particles = None;
        let mut m_texture_alpha_affects_particles = None;
        let mut m_texture_bilinear_filtering = None;
        enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        r#type = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
        } else {
            radius = Some(ShapeModuleRadiusVariant::parse(rdr, ver)?);
        }
        angle = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            length = Some(rdr.read_f32()?);
            box_thickness = Some(Vector3f::parse(rdr, ver)?);
            radius_thickness = Some(rdr.read_f32()?);
            donut_radius = Some(rdr.read_f32()?);
            m_position = Some(Vector3f::parse(rdr, ver)?);
            m_rotation = Some(Vector3f::parse(rdr, ver)?);
            m_scale = Some(Vector3f::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 0) {
            length = Some(rdr.read_f32()?);
            box_x = Some(rdr.read_f32()?);
            box_y = Some(rdr.read_f32()?);
            box_z = Some(rdr.read_f32()?);
            radius = Some(ShapeModuleRadiusVariant::parse(rdr, ver)?);
            arc = Some(ShapeModuleArcVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            length = Some(rdr.read_f32()?);
            box_x = Some(rdr.read_f32()?);
            box_y = Some(rdr.read_f32()?);
            box_z = Some(rdr.read_f32()?);
            arc = Some(ShapeModuleArcVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            length = Some(rdr.read_f32()?);
            box_x = Some(rdr.read_f32()?);
            box_y = Some(rdr.read_f32()?);
            box_z = Some(rdr.read_f32()?);
        } else {
            box_x = Some(rdr.read_f32()?);
            box_y = Some(rdr.read_f32()?);
            box_z = Some(rdr.read_f32()?);
        }
        placement_mode = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_mesh_material_index = Some(rdr.read_i32()?);
            m_mesh_normal_offset = Some(rdr.read_f32()?);
            m_mesh_spawn = Some(MultiModeParameter_MeshSpawn::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_mesh_material_index = Some(rdr.read_i32()?);
            m_mesh_normal_offset = Some(rdr.read_f32()?);
        } else {
        }
        m_mesh = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            m_mesh_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_skinned_mesh_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite = Some(crate::PPtr::parse(rdr, ver)?);
            m_sprite_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_use_mesh_material_index = Some(rdr.read_bool()?);
            m_use_mesh_colors = Some(rdr.read_bool()?);
            align_to_direction = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_texture = Some(crate::PPtr::parse(rdr, ver)?);
            m_texture_clip_channel = Some(rdr.read_i32()?);
            m_texture_clip_threshold = Some(rdr.read_f32()?);
            m_texture_uv_channel = Some(rdr.read_i32()?);
            m_texture_color_affects_particles = Some(rdr.read_bool()?);
            m_texture_alpha_affects_particles = Some(rdr.read_bool()?);
            m_texture_bilinear_filtering = Some(rdr.read_bool()?);
            rdr.align(4)?;
            random_direction_amount = Some(rdr.read_f32()?);
            spherical_direction_amount = Some(rdr.read_f32()?);
            random_position_amount = Some(rdr.read_f32()?);
            radius = Some(ShapeModuleRadiusVariant::parse(rdr, ver)?);
            arc = Some(ShapeModuleArcVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_mesh_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_skinned_mesh_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_use_mesh_material_index = Some(rdr.read_bool()?);
            m_use_mesh_colors = Some(rdr.read_bool()?);
            align_to_direction = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_texture = Some(crate::PPtr::parse(rdr, ver)?);
            m_texture_clip_channel = Some(rdr.read_i32()?);
            m_texture_clip_threshold = Some(rdr.read_f32()?);
            m_texture_uv_channel = Some(rdr.read_i32()?);
            m_texture_color_affects_particles = Some(rdr.read_bool()?);
            m_texture_alpha_affects_particles = Some(rdr.read_bool()?);
            m_texture_bilinear_filtering = Some(rdr.read_bool()?);
            rdr.align(4)?;
            random_direction_amount = Some(rdr.read_f32()?);
            spherical_direction_amount = Some(rdr.read_f32()?);
            random_position_amount = Some(rdr.read_f32()?);
            radius = Some(ShapeModuleRadiusVariant::parse(rdr, ver)?);
            arc = Some(ShapeModuleArcVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_mesh_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_skinned_mesh_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_use_mesh_material_index = Some(rdr.read_bool()?);
            m_use_mesh_colors = Some(rdr.read_bool()?);
            align_to_direction = Some(rdr.read_bool()?);
            rdr.align(4)?;
            random_direction_amount = Some(rdr.read_f32()?);
            spherical_direction_amount = Some(rdr.read_f32()?);
            random_position_amount = Some(rdr.read_f32()?);
            radius = Some(ShapeModuleRadiusVariant::parse(rdr, ver)?);
            arc = Some(ShapeModuleArcVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_mesh_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_skinned_mesh_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh_material_index = Some(rdr.read_i32()?);
            m_mesh_normal_offset = Some(rdr.read_f32()?);
            m_use_mesh_material_index = Some(rdr.read_bool()?);
            m_use_mesh_colors = Some(rdr.read_bool()?);
            align_to_direction = Some(rdr.read_bool()?);
            rdr.align(4)?;
            random_direction_amount = Some(rdr.read_f32()?);
            spherical_direction_amount = Some(rdr.read_f32()?);
            random_position_amount = Some(rdr.read_f32()?);
            radius = Some(ShapeModuleRadiusVariant::parse(rdr, ver)?);
            arc = Some(ShapeModuleArcVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_mesh_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_skinned_mesh_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh_material_index = Some(rdr.read_i32()?);
            m_mesh_normal_offset = Some(rdr.read_f32()?);
            m_mesh_scale = Some(rdr.read_f32()?);
            m_use_mesh_material_index = Some(rdr.read_bool()?);
            m_use_mesh_colors = Some(rdr.read_bool()?);
            align_to_direction = Some(rdr.read_bool()?);
            rdr.align(4)?;
            random_direction_amount = Some(rdr.read_f32()?);
            spherical_direction_amount = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_mesh_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_skinned_mesh_renderer = Some(crate::PPtr::parse(rdr, ver)?);
            m_mesh_material_index = Some(rdr.read_i32()?);
            m_mesh_normal_offset = Some(rdr.read_f32()?);
            m_use_mesh_material_index = Some(rdr.read_bool()?);
            m_use_mesh_colors = Some(rdr.read_bool()?);
            random_direction = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            random_direction = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }

        Ok(ShapeModule {
            enabled: enabled.unwrap(),
            r#type: r#type.unwrap(),
            radius: radius.unwrap(),
            angle: angle.unwrap(),
            length,
            box_x,
            box_thickness,
            box_y,
            radius_thickness,
            box_z,
            donut_radius,
            placement_mode: placement_mode.unwrap(),
            m_position,
            m_mesh: m_mesh.unwrap(),
            arc,
            m_rotation,
            random_direction,
            m_scale,
            m_mesh_renderer,
            m_mesh_material_index,
            m_skinned_mesh_renderer,
            m_mesh_normal_offset,
            m_mesh_spawn,
            m_use_mesh_material_index,
            m_mesh_scale,
            m_use_mesh_colors,
            m_sprite,
            m_sprite_renderer,
            align_to_direction,
            random_direction_amount,
            m_texture,
            spherical_direction_amount,
            m_texture_clip_channel,
            random_position_amount,
            m_texture_clip_threshold,
            m_texture_uv_channel,
            m_texture_color_affects_particles,
            m_texture_alpha_affects_particles,
            m_texture_bilinear_filtering,
        })
    }
}

impl crate::ToValue for ShapeModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("type".to_string(), self.r#type.to_value())),
            Some(("radius".to_string(), self.radius.to_value())),
            Some(("angle".to_string(), self.angle.to_value())),
            crate::opt_field("length", &self.length),
            crate::opt_field("boxX", &self.box_x),
            crate::opt_field("boxThickness", &self.box_thickness),
            crate::opt_field("boxY", &self.box_y),
            crate::opt_field("radiusThickness", &self.radius_thickness),
            crate::opt_field("boxZ", &self.box_z),
            crate::opt_field("donutRadius", &self.donut_radius),
            Some(("placementMode".to_string(), self.placement_mode.to_value())),
            crate::opt_field("m_Position", &self.m_position),
            Some(("m_Mesh".to_string(), self.m_mesh.to_value())),
            crate::opt_field("arc", &self.arc),
            crate::opt_field("m_Rotation", &self.m_rotation),
            crate::opt_field("randomDirection", &self.random_direction),
            crate::opt_field("m_Scale", &self.m_scale),
            crate::opt_field("m_MeshRenderer", &self.m_mesh_renderer),
            crate::opt_field("m_MeshMaterialIndex", &self.m_mesh_material_index),
            crate::opt_field("m_SkinnedMeshRenderer", &self.m_skinned_mesh_renderer),
            crate::opt_field("m_MeshNormalOffset", &self.m_mesh_normal_offset),
            crate::opt_field("m_MeshSpawn", &self.m_mesh_spawn),
            crate::opt_field("m_UseMeshMaterialIndex", &self.m_use_mesh_material_index),
            crate::opt_field("m_MeshScale", &self.m_mesh_scale),
            crate::opt_field("m_UseMeshColors", &self.m_use_mesh_colors),
            crate::opt_field("m_Sprite", &self.m_sprite),
            crate::opt_field("m_SpriteRenderer", &self.m_sprite_renderer),
            crate::opt_field("alignToDirection", &self.align_to_direction),
            crate::opt_field("randomDirectionAmount", &self.random_direction_amount),
            crate::opt_field("m_Texture", &self.m_texture),
            crate::opt_field("sphericalDirectionAmount", &self.spherical_direction_amount),
            crate::opt_field("m_TextureClipChannel", &self.m_texture_clip_channel),
            crate::opt_field("randomPositionAmount", &self.random_position_amount),
            crate::opt_field("m_TextureClipThreshold", &self.m_texture_clip_threshold),
            crate::opt_field("m_TextureUVChannel", &self.m_texture_uv_channel),
            crate::opt_field(
                "m_TextureColorAffectsParticles",
                &self.m_texture_color_affects_particles,
            ),
            crate::opt_field(
                "m_TextureAlphaAffectsParticles",
                &self.m_texture_alpha_affects_particles,
            ),
            crate::opt_field(
                "m_TextureBilinearFiltering",
                &self.m_texture_bilinear_filtering,
            ),
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
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "radius" => (self.radius).inspect_value(path, offset, limit),
                "angle" => (self.angle).inspect_value(path, offset, limit),
                "length" => (self.length).inspect_value(path, offset, limit),
                "boxX" => (self.box_x).inspect_value(path, offset, limit),
                "boxThickness" => (self.box_thickness).inspect_value(path, offset, limit),
                "boxY" => (self.box_y).inspect_value(path, offset, limit),
                "radiusThickness" => (self.radius_thickness).inspect_value(path, offset, limit),
                "boxZ" => (self.box_z).inspect_value(path, offset, limit),
                "donutRadius" => (self.donut_radius).inspect_value(path, offset, limit),
                "placementMode" => (self.placement_mode).inspect_value(path, offset, limit),
                "m_Position" => (self.m_position).inspect_value(path, offset, limit),
                "m_Mesh" => (self.m_mesh).inspect_value(path, offset, limit),
                "arc" => (self.arc).inspect_value(path, offset, limit),
                "m_Rotation" => (self.m_rotation).inspect_value(path, offset, limit),
                "randomDirection" => (self.random_direction).inspect_value(path, offset, limit),
                "m_Scale" => (self.m_scale).inspect_value(path, offset, limit),
                "m_MeshRenderer" => (self.m_mesh_renderer).inspect_value(path, offset, limit),
                "m_MeshMaterialIndex" => {
                    (self.m_mesh_material_index).inspect_value(path, offset, limit)
                }
                "m_SkinnedMeshRenderer" => {
                    (self.m_skinned_mesh_renderer).inspect_value(path, offset, limit)
                }
                "m_MeshNormalOffset" => {
                    (self.m_mesh_normal_offset).inspect_value(path, offset, limit)
                }
                "m_MeshSpawn" => (self.m_mesh_spawn).inspect_value(path, offset, limit),
                "m_UseMeshMaterialIndex" => {
                    (self.m_use_mesh_material_index).inspect_value(path, offset, limit)
                }
                "m_MeshScale" => (self.m_mesh_scale).inspect_value(path, offset, limit),
                "m_UseMeshColors" => (self.m_use_mesh_colors).inspect_value(path, offset, limit),
                "m_Sprite" => (self.m_sprite).inspect_value(path, offset, limit),
                "m_SpriteRenderer" => (self.m_sprite_renderer).inspect_value(path, offset, limit),
                "alignToDirection" => (self.align_to_direction).inspect_value(path, offset, limit),
                "randomDirectionAmount" => {
                    (self.random_direction_amount).inspect_value(path, offset, limit)
                }
                "m_Texture" => (self.m_texture).inspect_value(path, offset, limit),
                "sphericalDirectionAmount" => {
                    (self.spherical_direction_amount).inspect_value(path, offset, limit)
                }
                "m_TextureClipChannel" => {
                    (self.m_texture_clip_channel).inspect_value(path, offset, limit)
                }
                "randomPositionAmount" => {
                    (self.random_position_amount).inspect_value(path, offset, limit)
                }
                "m_TextureClipThreshold" => {
                    (self.m_texture_clip_threshold).inspect_value(path, offset, limit)
                }
                "m_TextureUVChannel" => {
                    (self.m_texture_uv_channel).inspect_value(path, offset, limit)
                }
                "m_TextureColorAffectsParticles" => {
                    (self.m_texture_color_affects_particles).inspect_value(path, offset, limit)
                }
                "m_TextureAlphaAffectsParticles" => {
                    (self.m_texture_alpha_affects_particles).inspect_value(path, offset, limit)
                }
                "m_TextureBilinearFiltering" => {
                    (self.m_texture_bilinear_filtering).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.radius).inspect_value(path, offset, limit)).map(|v| ("radius".to_string(), v)),
            ((self.angle).inspect_value(path, offset, limit)).map(|v| ("angle".to_string(), v)),
            ((self.length).inspect_value(path, offset, limit)).map(|v| ("length".to_string(), v)),
            ((self.box_x).inspect_value(path, offset, limit)).map(|v| ("boxX".to_string(), v)),
            ((self.box_thickness).inspect_value(path, offset, limit))
                .map(|v| ("boxThickness".to_string(), v)),
            ((self.box_y).inspect_value(path, offset, limit)).map(|v| ("boxY".to_string(), v)),
            ((self.radius_thickness).inspect_value(path, offset, limit))
                .map(|v| ("radiusThickness".to_string(), v)),
            ((self.box_z).inspect_value(path, offset, limit)).map(|v| ("boxZ".to_string(), v)),
            ((self.donut_radius).inspect_value(path, offset, limit))
                .map(|v| ("donutRadius".to_string(), v)),
            ((self.placement_mode).inspect_value(path, offset, limit))
                .map(|v| ("placementMode".to_string(), v)),
            ((self.m_position).inspect_value(path, offset, limit))
                .map(|v| ("m_Position".to_string(), v)),
            ((self.m_mesh).inspect_value(path, offset, limit)).map(|v| ("m_Mesh".to_string(), v)),
            ((self.arc).inspect_value(path, offset, limit)).map(|v| ("arc".to_string(), v)),
            ((self.m_rotation).inspect_value(path, offset, limit))
                .map(|v| ("m_Rotation".to_string(), v)),
            ((self.random_direction).inspect_value(path, offset, limit))
                .map(|v| ("randomDirection".to_string(), v)),
            ((self.m_scale).inspect_value(path, offset, limit)).map(|v| ("m_Scale".to_string(), v)),
            ((self.m_mesh_renderer).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshRenderer".to_string(), v)),
            ((self.m_mesh_material_index).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshMaterialIndex".to_string(), v)),
            ((self.m_skinned_mesh_renderer).inspect_value(path, offset, limit))
                .map(|v| ("m_SkinnedMeshRenderer".to_string(), v)),
            ((self.m_mesh_normal_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshNormalOffset".to_string(), v)),
            ((self.m_mesh_spawn).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshSpawn".to_string(), v)),
            ((self.m_use_mesh_material_index).inspect_value(path, offset, limit))
                .map(|v| ("m_UseMeshMaterialIndex".to_string(), v)),
            ((self.m_mesh_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_MeshScale".to_string(), v)),
            ((self.m_use_mesh_colors).inspect_value(path, offset, limit))
                .map(|v| ("m_UseMeshColors".to_string(), v)),
            ((self.m_sprite).inspect_value(path, offset, limit))
                .map(|v| ("m_Sprite".to_string(), v)),
            ((self.m_sprite_renderer).inspect_value(path, offset, limit))
                .map(|v| ("m_SpriteRenderer".to_string(), v)),
            ((self.align_to_direction).inspect_value(path, offset, limit))
                .map(|v| ("alignToDirection".to_string(), v)),
            ((self.random_direction_amount).inspect_value(path, offset, limit))
                .map(|v| ("randomDirectionAmount".to_string(), v)),
            ((self.m_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_Texture".to_string(), v)),
            ((self.spherical_direction_amount).inspect_value(path, offset, limit))
                .map(|v| ("sphericalDirectionAmount".to_string(), v)),
            ((self.m_texture_clip_channel).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureClipChannel".to_string(), v)),
            ((self.random_position_amount).inspect_value(path, offset, limit))
                .map(|v| ("randomPositionAmount".to_string(), v)),
            ((self.m_texture_clip_threshold).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureClipThreshold".to_string(), v)),
            ((self.m_texture_uv_channel).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureUVChannel".to_string(), v)),
            ((self.m_texture_color_affects_particles).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureColorAffectsParticles".to_string(), v)),
            ((self.m_texture_alpha_affects_particles).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureAlphaAffectsParticles".to_string(), v)),
            ((self.m_texture_bilinear_filtering).inspect_value(path, offset, limit))
                .map(|v| ("m_TextureBilinearFiltering".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
