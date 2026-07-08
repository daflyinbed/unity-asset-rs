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
            let v = ( { Ok(Box::new(MultiModeParameter_MeshSpawn::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_f32()?;
            Ok(Self::Default(v))
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
            let v = ( { Ok(Box::new(MultiModeParameter_MeshSpawn::parse(rdr, ver)?)) } )?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_f32()?;
            Ok(Self::Default(v))
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
