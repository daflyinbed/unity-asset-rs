//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::character_info::CharacterInfo;

#[derive(Debug, Clone)]
pub struct Font {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_ascii_start_offset: i32,
    pub m_font_count_x: i32,
    pub m_font_count_y: i32,
    pub m_kerning: f32,
    pub m_line_spacing: f32,
    pub m_per_character_kerning: Vec<(i32, f32)>,
    pub m_convert_case: i32,
    pub m_default_material: PPtr,
    pub m_character_rects: Vec<CharacterInfo>,
    pub m_texture: PPtr,
    pub m_kerning_values: Vec<((u16, u16), f32)>,
    pub m_grid_font: bool,
    pub m_font_data: Vec<u8>,
    pub m_font_size: f32,
    pub m_ascent: f32,
    pub m_default_style: u32,
    pub m_font_names: Vec<String>,
}

impl crate::Parse for Font {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_ascii_start_offset = r.read_i32()?;
        let m_font_count_x = r.read_i32()?;
        let m_font_count_y = r.read_i32()?;
        let m_kerning = r.read_f32()?;
        let m_line_spacing = r.read_f32()?;
        let m_per_character_kerning = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_f32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_convert_case = r.read_i32()?;
        let m_default_material = crate::PPtr::parse(r, ver)?;
        let m_character_rects = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { CharacterInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_texture = crate::PPtr::parse(r, ver)?;
        let m_kerning_values = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = ( { let a = r.read_u16()?; let b = r.read_u16()?; Ok((a, b)) } )?; let b = r.read_f32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_grid_font = r.read_bool()?;
        r.align(4)?;
        let m_font_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_font_size = r.read_f32()?;
        let m_ascent = r.read_f32()?;
        let m_default_style = r.read_u32()?;
        let m_font_names = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(Font {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_ascii_start_offset,
            m_font_count_x,
            m_font_count_y,
            m_kerning,
            m_line_spacing,
            m_per_character_kerning,
            m_convert_case,
            m_default_material,
            m_character_rects,
            m_texture,
            m_kerning_values,
            m_grid_font,
            m_font_data,
            m_font_size,
            m_ascent,
            m_default_style,
            m_font_names,
        })
    }
}
