//! 自动生成。请勿手动修改。

use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct ProceduralMaterialInformation {
    pub m_offset: Vector2f,
    pub m_scale: Vector2f,
    pub m_generated_at_loading: Option<i32>,
    pub m_generate_all_outputs: Option<i32>,
    pub m_generate_mipmaps: Option<bool>,
    pub m_animation_update_rate: Option<i32>,
}

impl crate::Parse for ProceduralMaterialInformation {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_offset = None;
        let mut m_scale = None;
        let mut m_generated_at_loading = None;
        let mut m_generate_all_outputs = None;
        let mut m_generate_mipmaps = None;
        let mut m_animation_update_rate = None;
        m_offset = Some(Vector2f::parse(rdr, ver)?);
        m_scale = Some(Vector2f::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(4, 5, 0) {
            m_generate_mipmaps = Some(rdr.read_bool()?);
            rdr.align(4)?;
            m_generate_all_outputs = Some(rdr.read_i32()?);
            m_animation_update_rate = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_generate_all_outputs = Some(rdr.read_i32()?);
            m_animation_update_rate = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            m_generated_at_loading = Some(rdr.read_i32()?);
            m_generate_all_outputs = Some(rdr.read_i32()?);
            m_animation_update_rate = Some(rdr.read_i32()?);
        } else {
            m_generated_at_loading = Some(rdr.read_i32()?);
        }

        Ok(ProceduralMaterialInformation {
            m_offset: m_offset.unwrap(),
            m_scale: m_scale.unwrap(),
            m_generated_at_loading,
            m_generate_all_outputs,
            m_generate_mipmaps,
            m_animation_update_rate,
        })
    }
}

impl crate::ToValue for ProceduralMaterialInformation {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Offset".to_string(), self.m_offset.to_value())),
            Some(("m_Scale".to_string(), self.m_scale.to_value())),
            crate::opt_field("m_GeneratedAtLoading", &self.m_generated_at_loading),
            crate::opt_field("m_GenerateAllOutputs", &self.m_generate_all_outputs),
            crate::opt_field("m_GenerateMipmaps", &self.m_generate_mipmaps),
            crate::opt_field("m_AnimationUpdateRate", &self.m_animation_update_rate),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Offset" => (self.m_offset).inspect_value(path, offset, limit),
                "m_Scale" => (self.m_scale).inspect_value(path, offset, limit),
                "m_GeneratedAtLoading" => {
                    (self.m_generated_at_loading).inspect_value(path, offset, limit)
                }
                "m_GenerateAllOutputs" => {
                    (self.m_generate_all_outputs).inspect_value(path, offset, limit)
                }
                "m_GenerateMipmaps" => (self.m_generate_mipmaps).inspect_value(path, offset, limit),
                "m_AnimationUpdateRate" => {
                    (self.m_animation_update_rate).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_Offset".to_string(), v)),
            ((self.m_scale).inspect_value(path, offset, limit)).map(|v| ("m_Scale".to_string(), v)),
            ((self.m_generated_at_loading).inspect_value(path, offset, limit))
                .map(|v| ("m_GeneratedAtLoading".to_string(), v)),
            ((self.m_generate_all_outputs).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateAllOutputs".to_string(), v)),
            ((self.m_generate_mipmaps).inspect_value(path, offset, limit))
                .map(|v| ("m_GenerateMipmaps".to_string(), v)),
            ((self.m_animation_update_rate).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimationUpdateRate".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
