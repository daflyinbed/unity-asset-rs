//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::character_info::CharacterInfo;

#[derive(Debug, Clone)]
pub struct Font {
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
        let mut m_name = None;
        let mut m_ascii_start_offset = None;
        let mut m_line_spacing = None;
        let mut m_font_count_x = None;
        let mut m_kerning = None;
        let mut m_default_material = None;
        let mut m_tracking = None;
        let mut m_font_count_y = None;
        let mut m_font_size = None;
        let mut m_texture = None;
        let mut m_character_spacing = None;
        let mut m_character_padding = None;
        let mut m_per_character_kerning = None;
        let mut m_convert_case = None;
        let mut m_character_rects = None;
        let mut m_kerning_values = None;
        let mut m_pixel_scale = None;
        let mut m_grid_font = None;
        let mut m_font_data = None;
        let mut m_ascent = None;
        let mut m_default_style = None;
        let mut m_descent = None;
        let mut m_font_names = None;
        let mut m_fallback_fonts = None;
        let mut m_font_rendering_mode = None;
        let mut m_use_legacy_bounds_calculation = None;
        let mut m_should_round_advance_value = None;
            m_name = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_ascii_start_offset = Some(r.read_i32()?);
            m_tracking = Some(r.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_ascii_start_offset = Some(r.read_i32()?);
            m_kerning = Some(r.read_f32()?);
        } else {
            m_ascii_start_offset = Some(r.read_i32()?);
            m_font_count_x = Some(r.read_i32()?);
            m_font_count_y = Some(r.read_i32()?);
            m_kerning = Some(r.read_f32()?);
        }
            m_line_spacing = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_character_spacing = Some(r.read_i32()?);
            m_character_padding = Some(r.read_i32()?);
            m_convert_case = Some(r.read_i32()?);
        } else {
            m_per_character_kerning = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_f32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
            m_convert_case = Some(r.read_i32()?);
        }
            m_default_material = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_font_size = Some(r.read_f32()?);
        } else {
            m_character_rects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { CharacterInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        }
            m_texture = Some(crate::PPtr::parse(r, ver)?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_ascii_start_offset = Some(r.read_i32()?);
            m_tracking = Some(r.read_f32()?);
            m_character_spacing = Some(r.read_i32()?);
            m_character_padding = Some(r.read_i32()?);
            m_convert_case = Some(r.read_i32()?);
            m_character_rects = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { CharacterInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else {
        }
            m_kerning_values = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = r.read_u16()?; let b = r.read_u16()?; Ok((a, b)) } )?; let b = r.read_f32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_pixel_scale = Some(r.read_f32()?);
            r.align(4)?;
        } else {
            m_grid_font = Some(r.read_bool()?);
            r.align(4)?;
        }
            m_font_data = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else {
            m_font_size = Some(r.read_f32()?);
        }
            m_ascent = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_descent = Some(r.read_f32()?);
        } else {
        }
            m_default_style = Some(r.read_u32()?);
            m_font_names = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_fallback_fonts = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_font_rendering_mode = Some(r.read_i32()?);
            m_use_legacy_bounds_calculation = Some(r.read_bool()?);
            m_should_round_advance_value = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_fallback_fonts = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_font_rendering_mode = Some(r.read_i32()?);
            m_use_legacy_bounds_calculation = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_fallback_fonts = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_font_rendering_mode = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_fallback_fonts = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_font_rendering_mode = Some(r.read_i32()?);
            m_use_legacy_bounds_calculation = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 2, 0) {
            m_fallback_fonts = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_font_rendering_mode = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 2) {
            m_fallback_fonts = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_font_rendering_mode = Some(r.read_i32()?);
            m_use_legacy_bounds_calculation = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_fallback_fonts = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_font_rendering_mode = Some(r.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 5) {
            m_fallback_fonts = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_font_rendering_mode = Some(r.read_i32()?);
            m_use_legacy_bounds_calculation = Some(r.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_fallback_fonts = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_font_rendering_mode = Some(r.read_i32()?);
        } else {
        }

        Ok(Font {
            m_name: m_name.unwrap(),
            m_ascii_start_offset: m_ascii_start_offset.unwrap(),
            m_line_spacing: m_line_spacing.unwrap(),
            m_font_count_x,
            m_kerning,
            m_default_material: m_default_material.unwrap(),
            m_tracking,
            m_font_count_y,
            m_font_size: m_font_size.unwrap(),
            m_texture: m_texture.unwrap(),
            m_character_spacing,
            m_character_padding,
            m_per_character_kerning,
            m_convert_case: m_convert_case.unwrap(),
            m_character_rects: m_character_rects.unwrap(),
            m_kerning_values: m_kerning_values.unwrap(),
            m_pixel_scale,
            m_grid_font,
            m_font_data: m_font_data.unwrap(),
            m_ascent: m_ascent.unwrap(),
            m_default_style: m_default_style.unwrap(),
            m_descent,
            m_font_names: m_font_names.unwrap(),
            m_fallback_fonts,
            m_font_rendering_mode,
            m_use_legacy_bounds_calculation,
            m_should_round_advance_value,
        })
    }
}
