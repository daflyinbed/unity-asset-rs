//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::character_info::CharacterInfo;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct Font {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_ascii_start_offset: i32,
    pub m_line_spacing: f32,
    pub m_font_count_x: Option<i32>,
    pub m_kerning: Option<f32>,
    pub m_default_material: PPtr,
    pub m_tracking: Option<f32>,
    pub m_font_count_y: Option<i32>,
    pub m_font_size: f32,
    pub m_texture: PPtr,
    pub m_character_spacing: Option<i32>,
    pub m_character_padding: Option<i32>,
    pub m_per_character_kerning: Option<Vec<(i32, f32)>>,
    pub m_convert_case: i32,
    pub m_character_rects: Vec<CharacterInfo>,
    pub m_kerning_values: Vec<((u16, u16), f32)>,
    pub m_pixel_scale: Option<f32>,
    pub m_grid_font: Option<bool>,
    pub m_font_data: Vec<u8>,
    pub m_ascent: f32,
    pub m_default_style: u32,
    pub m_descent: Option<f32>,
    pub m_font_names: Vec<String>,
    pub m_fallback_fonts: Option<Vec<PPtr>>,
    pub m_font_rendering_mode: Option<i32>,
    pub m_use_legacy_bounds_calculation: Option<bool>,
    pub m_should_round_advance_value: Option<bool>,
}

impl crate::Parse for Font {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_ascii_start_offset = r.read_i32()?;
        let m_line_spacing = r.read_f32()?;
        let m_font_count_x = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_kerning = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_default_material = crate::PPtr::parse(r, ver)?;
        let m_tracking = if *ver >= crate::UnityVersion::new(5, 3, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_font_count_y = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_font_size = r.read_f32()?;
        let m_texture = crate::PPtr::parse(r, ver)?;
        let m_character_spacing = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_character_padding = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_per_character_kerning = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_f32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_convert_case = r.read_i32()?;
        let m_character_rects = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { CharacterInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_kerning_values = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = r.read_u16()?; let b = r.read_u16()?; Ok((a, b)) } )?; let b = r.read_f32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_pixel_scale = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = r.read_f32()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_grid_font = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_font_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_ascent = r.read_f32()?;
        let m_default_style = r.read_u32()?;
        let m_descent = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_font_names = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_fallback_fonts = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_font_rendering_mode = if *ver >= crate::UnityVersion::new(4, 0, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_use_legacy_bounds_calculation = if *ver >= crate::UnityVersion::new(5, 6, 5) {
            Some(r.read_bool()?)
        } else {
            None
        };
        let m_should_round_advance_value = if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            Some(r.read_bool()?)
        } else {
            None
        };

        Ok(Font {
            parent,
            m_name,
            m_ascii_start_offset,
            m_line_spacing,
            m_font_count_x,
            m_kerning,
            m_default_material,
            m_tracking,
            m_font_count_y,
            m_font_size,
            m_texture,
            m_character_spacing,
            m_character_padding,
            m_per_character_kerning,
            m_convert_case,
            m_character_rects,
            m_kerning_values,
            m_pixel_scale,
            m_grid_font,
            m_font_data,
            m_ascent,
            m_default_style,
            m_descent,
            m_font_names,
            m_fallback_fonts,
            m_font_rendering_mode,
            m_use_legacy_bounds_calculation,
            m_should_round_advance_value,
        })
    }
}
