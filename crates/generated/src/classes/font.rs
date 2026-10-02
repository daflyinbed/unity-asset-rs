//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::character_info::CharacterInfo;

/// 版本枚举：FontMTextureVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FontMTextureVariant {
    Default(PPtr),
    V5_5(PPtr),
}

impl crate::Parse for FontMTextureVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            let v = crate::PPtr::parse(rdr, ver)?;
            rdr.align(4)?;
            Ok(Self::V5_5(v))
        } else {
            let v = crate::PPtr::parse(rdr, ver)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for FontMTextureVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：FontMCharacterRectsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FontMCharacterRectsVariant {
    Default(Vec<CharacterInfo>),
    V2017_1(Vec<CharacterInfo>),
}

impl crate::Parse for FontMCharacterRectsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| CharacterInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| CharacterInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for FontMCharacterRectsVariant {
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

/// 版本枚举：FontMFontNamesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum FontMFontNamesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for FontMFontNamesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for FontMFontNamesVariant {
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
    pub m_texture: FontMTextureVariant,
    pub m_character_spacing: Option<i32>,
    pub m_character_padding: Option<i32>,
    pub m_per_character_kerning: Option<Vec<(i32, f32)>>,
    pub m_convert_case: i32,
    pub m_character_rects: FontMCharacterRectsVariant,
    pub m_kerning_values: Vec<((u16, u16), f32)>,
    pub m_pixel_scale: Option<f32>,
    pub m_grid_font: Option<bool>,
    pub m_font_data: Vec<u8>,
    pub m_ascent: f32,
    pub m_default_style: u32,
    pub m_descent: Option<f32>,
    pub m_font_names: FontMFontNamesVariant,
    pub m_fallback_fonts: Option<Vec<PPtr>>,
    pub m_font_rendering_mode: Option<i32>,
    pub m_use_legacy_bounds_calculation: Option<bool>,
    pub m_should_round_advance_value: Option<bool>,
}

impl crate::Parse for Font {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
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
        m_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_ascii_start_offset = Some(rdr.read_i32()?);
            m_tracking = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_ascii_start_offset = Some(rdr.read_i32()?);
            m_kerning = Some(rdr.read_f32()?);
        } else {
            m_ascii_start_offset = Some(rdr.read_i32()?);
            m_font_count_x = Some(rdr.read_i32()?);
            m_font_count_y = Some(rdr.read_i32()?);
            m_kerning = Some(rdr.read_f32()?);
        }
        m_line_spacing = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_character_spacing = Some(rdr.read_i32()?);
            m_character_padding = Some(rdr.read_i32()?);
            m_convert_case = Some(rdr.read_i32()?);
        } else {
            m_per_character_kerning = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| {
                            ({
                                let a = rdr.read_i32()?;
                                let b = rdr.read_f32()?;
                                Ok((a, b))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            m_convert_case = Some(rdr.read_i32()?);
        }
        m_default_material = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_font_size = Some(rdr.read_f32()?);
        } else {
            m_character_rects = Some(FontMCharacterRectsVariant::parse(rdr, ver)?);
        }
        m_texture = Some(FontMTextureVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            m_ascii_start_offset = Some(rdr.read_i32()?);
            m_tracking = Some(rdr.read_f32()?);
            m_character_spacing = Some(rdr.read_i32()?);
            m_character_padding = Some(rdr.read_i32()?);
            m_convert_case = Some(rdr.read_i32()?);
            m_character_rects = Some(FontMCharacterRectsVariant::parse(rdr, ver)?);
        } else {
        }
        m_kerning_values = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| {
                        ({
                            let a = ({
                                let a = rdr.read_u16()?;
                                let b = rdr.read_u16()?;
                                Ok((a, b))
                            })?;
                            let b = rdr.read_f32()?;
                            Ok((a, b))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_pixel_scale = Some(rdr.read_f32()?);
            rdr.align(4)?;
        } else {
            m_grid_font = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        m_font_data = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
        } else {
            m_font_size = Some(rdr.read_f32()?);
        }
        m_ascent = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_descent = Some(rdr.read_f32()?);
        } else {
        }
        m_default_style = Some(rdr.read_u32()?);
        m_font_names = Some(FontMFontNamesVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
            m_fallback_fonts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_use_legacy_bounds_calculation = Some(rdr.read_bool()?);
            m_should_round_advance_value = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 2) {
            m_fallback_fonts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_use_legacy_bounds_calculation = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_fallback_fonts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(5, 6, 5) {
            m_fallback_fonts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
            m_use_legacy_bounds_calculation = Some(rdr.read_bool()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_fallback_fonts = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
            m_font_rendering_mode = Some(rdr.read_i32()?);
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

impl crate::ToValue for Font {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_AsciiStartOffset".to_string(),
                self.m_ascii_start_offset.to_value(),
            )),
            Some(("m_LineSpacing".to_string(), self.m_line_spacing.to_value())),
            crate::opt_field("m_FontCountX", &self.m_font_count_x),
            crate::opt_field("m_Kerning", &self.m_kerning),
            Some((
                "m_DefaultMaterial".to_string(),
                self.m_default_material.to_value(),
            )),
            crate::opt_field("m_Tracking", &self.m_tracking),
            crate::opt_field("m_FontCountY", &self.m_font_count_y),
            Some(("m_FontSize".to_string(), self.m_font_size.to_value())),
            Some(("m_Texture".to_string(), self.m_texture.to_value())),
            crate::opt_field("m_CharacterSpacing", &self.m_character_spacing),
            crate::opt_field("m_CharacterPadding", &self.m_character_padding),
            crate::opt_field("m_PerCharacterKerning", &self.m_per_character_kerning),
            Some(("m_ConvertCase".to_string(), self.m_convert_case.to_value())),
            Some((
                "m_CharacterRects".to_string(),
                self.m_character_rects.to_value(),
            )),
            Some((
                "m_KerningValues".to_string(),
                self.m_kerning_values.to_value(),
            )),
            crate::opt_field("m_PixelScale", &self.m_pixel_scale),
            crate::opt_field("m_GridFont", &self.m_grid_font),
            Some(("m_FontData".to_string(), self.m_font_data.to_value())),
            Some(("m_Ascent".to_string(), self.m_ascent.to_value())),
            Some((
                "m_DefaultStyle".to_string(),
                self.m_default_style.to_value(),
            )),
            crate::opt_field("m_Descent", &self.m_descent),
            Some(("m_FontNames".to_string(), self.m_font_names.to_value())),
            crate::opt_field("m_FallbackFonts", &self.m_fallback_fonts),
            crate::opt_field("m_FontRenderingMode", &self.m_font_rendering_mode),
            crate::opt_field(
                "m_UseLegacyBoundsCalculation",
                &self.m_use_legacy_bounds_calculation,
            ),
            crate::opt_field(
                "m_ShouldRoundAdvanceValue",
                &self.m_should_round_advance_value,
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
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_AsciiStartOffset" => {
                    (self.m_ascii_start_offset).inspect_value(path, offset, limit)
                }
                "m_LineSpacing" => (self.m_line_spacing).inspect_value(path, offset, limit),
                "m_FontCountX" => (self.m_font_count_x).inspect_value(path, offset, limit),
                "m_Kerning" => (self.m_kerning).inspect_value(path, offset, limit),
                "m_DefaultMaterial" => (self.m_default_material).inspect_value(path, offset, limit),
                "m_Tracking" => (self.m_tracking).inspect_value(path, offset, limit),
                "m_FontCountY" => (self.m_font_count_y).inspect_value(path, offset, limit),
                "m_FontSize" => (self.m_font_size).inspect_value(path, offset, limit),
                "m_Texture" => (self.m_texture).inspect_value(path, offset, limit),
                "m_CharacterSpacing" => {
                    (self.m_character_spacing).inspect_value(path, offset, limit)
                }
                "m_CharacterPadding" => {
                    (self.m_character_padding).inspect_value(path, offset, limit)
                }
                "m_PerCharacterKerning" => {
                    (self.m_per_character_kerning).inspect_value(path, offset, limit)
                }
                "m_ConvertCase" => (self.m_convert_case).inspect_value(path, offset, limit),
                "m_CharacterRects" => (self.m_character_rects).inspect_value(path, offset, limit),
                "m_KerningValues" => (self.m_kerning_values).inspect_value(path, offset, limit),
                "m_PixelScale" => (self.m_pixel_scale).inspect_value(path, offset, limit),
                "m_GridFont" => (self.m_grid_font).inspect_value(path, offset, limit),
                "m_FontData" => (self.m_font_data).inspect_value(path, offset, limit),
                "m_Ascent" => (self.m_ascent).inspect_value(path, offset, limit),
                "m_DefaultStyle" => (self.m_default_style).inspect_value(path, offset, limit),
                "m_Descent" => (self.m_descent).inspect_value(path, offset, limit),
                "m_FontNames" => (self.m_font_names).inspect_value(path, offset, limit),
                "m_FallbackFonts" => (self.m_fallback_fonts).inspect_value(path, offset, limit),
                "m_FontRenderingMode" => {
                    (self.m_font_rendering_mode).inspect_value(path, offset, limit)
                }
                "m_UseLegacyBoundsCalculation" => {
                    (self.m_use_legacy_bounds_calculation).inspect_value(path, offset, limit)
                }
                "m_ShouldRoundAdvanceValue" => {
                    (self.m_should_round_advance_value).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_ascii_start_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_AsciiStartOffset".to_string(), v)),
            ((self.m_line_spacing).inspect_value(path, offset, limit))
                .map(|v| ("m_LineSpacing".to_string(), v)),
            ((self.m_font_count_x).inspect_value(path, offset, limit))
                .map(|v| ("m_FontCountX".to_string(), v)),
            ((self.m_kerning).inspect_value(path, offset, limit))
                .map(|v| ("m_Kerning".to_string(), v)),
            ((self.m_default_material).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultMaterial".to_string(), v)),
            ((self.m_tracking).inspect_value(path, offset, limit))
                .map(|v| ("m_Tracking".to_string(), v)),
            ((self.m_font_count_y).inspect_value(path, offset, limit))
                .map(|v| ("m_FontCountY".to_string(), v)),
            ((self.m_font_size).inspect_value(path, offset, limit))
                .map(|v| ("m_FontSize".to_string(), v)),
            ((self.m_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_Texture".to_string(), v)),
            ((self.m_character_spacing).inspect_value(path, offset, limit))
                .map(|v| ("m_CharacterSpacing".to_string(), v)),
            ((self.m_character_padding).inspect_value(path, offset, limit))
                .map(|v| ("m_CharacterPadding".to_string(), v)),
            ((self.m_per_character_kerning).inspect_value(path, offset, limit))
                .map(|v| ("m_PerCharacterKerning".to_string(), v)),
            ((self.m_convert_case).inspect_value(path, offset, limit))
                .map(|v| ("m_ConvertCase".to_string(), v)),
            ((self.m_character_rects).inspect_value(path, offset, limit))
                .map(|v| ("m_CharacterRects".to_string(), v)),
            ((self.m_kerning_values).inspect_value(path, offset, limit))
                .map(|v| ("m_KerningValues".to_string(), v)),
            ((self.m_pixel_scale).inspect_value(path, offset, limit))
                .map(|v| ("m_PixelScale".to_string(), v)),
            ((self.m_grid_font).inspect_value(path, offset, limit))
                .map(|v| ("m_GridFont".to_string(), v)),
            ((self.m_font_data).inspect_value(path, offset, limit))
                .map(|v| ("m_FontData".to_string(), v)),
            ((self.m_ascent).inspect_value(path, offset, limit))
                .map(|v| ("m_Ascent".to_string(), v)),
            ((self.m_default_style).inspect_value(path, offset, limit))
                .map(|v| ("m_DefaultStyle".to_string(), v)),
            ((self.m_descent).inspect_value(path, offset, limit))
                .map(|v| ("m_Descent".to_string(), v)),
            ((self.m_font_names).inspect_value(path, offset, limit))
                .map(|v| ("m_FontNames".to_string(), v)),
            ((self.m_fallback_fonts).inspect_value(path, offset, limit))
                .map(|v| ("m_FallbackFonts".to_string(), v)),
            ((self.m_font_rendering_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_FontRenderingMode".to_string(), v)),
            ((self.m_use_legacy_bounds_calculation).inspect_value(path, offset, limit))
                .map(|v| ("m_UseLegacyBoundsCalculation".to_string(), v)),
            ((self.m_should_round_advance_value).inspect_value(path, offset, limit))
                .map(|v| ("m_ShouldRoundAdvanceValue".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
