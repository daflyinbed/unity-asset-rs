//! 自动生成。请勿手动修改。

use crate::subclasses::matrix4x4f::Matrix4x4f;

#[derive(Debug, Clone)]
pub struct ShadowSettings {
    pub m_type: i32,
    pub m_resolution: i32,
    pub m_strength: f32,
    pub m_custom_resolution: Option<i32>,
    pub m_bias: f32,
    pub m_softness: Option<f32>,
    pub m_normal_bias: Option<f32>,
    pub m_softness_fade: Option<f32>,
    pub m_near_plane: Option<f32>,
    pub m_culling_matrix_override: Option<Matrix4x4f>,
    pub m_use_culling_matrix_override: Option<bool>,
}

impl crate::Parse for ShadowSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_type = None;
        let mut m_resolution = None;
        let mut m_strength = None;
        let mut m_custom_resolution = None;
        let mut m_bias = None;
        let mut m_softness = None;
        let mut m_normal_bias = None;
        let mut m_softness_fade = None;
        let mut m_near_plane = None;
        let mut m_culling_matrix_override = None;
        let mut m_use_culling_matrix_override = None;
        m_type = Some(rdr.read_i32()?);
        m_resolution = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(5, 4, 0) {
            m_custom_resolution = Some(rdr.read_i32()?);
        } else {
        }
        m_strength = Some(rdr.read_f32()?);
        m_bias = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_normal_bias = Some(rdr.read_f32()?);
            m_near_plane = Some(rdr.read_f32()?);
            m_culling_matrix_override = Some(Matrix4x4f::parse(rdr, ver)?);
            m_use_culling_matrix_override = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 3, 0) {
            m_normal_bias = Some(rdr.read_f32()?);
            m_near_plane = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_normal_bias = Some(rdr.read_f32()?);
        } else {
            m_softness = Some(rdr.read_f32()?);
            m_softness_fade = Some(rdr.read_f32()?);
        }

        Ok(ShadowSettings {
            m_type: m_type.unwrap(),
            m_resolution: m_resolution.unwrap(),
            m_strength: m_strength.unwrap(),
            m_custom_resolution,
            m_bias: m_bias.unwrap(),
            m_softness,
            m_normal_bias,
            m_softness_fade,
            m_near_plane,
            m_culling_matrix_override,
            m_use_culling_matrix_override,
        })
    }
}

impl crate::ToValue for ShadowSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Type".to_string(), self.m_type.to_value())),
            Some(("m_Resolution".to_string(), self.m_resolution.to_value())),
            Some(("m_Strength".to_string(), self.m_strength.to_value())),
            crate::opt_field("m_CustomResolution", &self.m_custom_resolution),
            Some(("m_Bias".to_string(), self.m_bias.to_value())),
            crate::opt_field("m_Softness", &self.m_softness),
            crate::opt_field("m_NormalBias", &self.m_normal_bias),
            crate::opt_field("m_SoftnessFade", &self.m_softness_fade),
            crate::opt_field("m_NearPlane", &self.m_near_plane),
            crate::opt_field("m_CullingMatrixOverride", &self.m_culling_matrix_override),
            crate::opt_field(
                "m_UseCullingMatrixOverride",
                &self.m_use_culling_matrix_override,
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
                "m_Type" => (self.m_type).inspect_value(path, offset, limit),
                "m_Resolution" => (self.m_resolution).inspect_value(path, offset, limit),
                "m_Strength" => (self.m_strength).inspect_value(path, offset, limit),
                "m_CustomResolution" => {
                    (self.m_custom_resolution).inspect_value(path, offset, limit)
                }
                "m_Bias" => (self.m_bias).inspect_value(path, offset, limit),
                "m_Softness" => (self.m_softness).inspect_value(path, offset, limit),
                "m_NormalBias" => (self.m_normal_bias).inspect_value(path, offset, limit),
                "m_SoftnessFade" => (self.m_softness_fade).inspect_value(path, offset, limit),
                "m_NearPlane" => (self.m_near_plane).inspect_value(path, offset, limit),
                "m_CullingMatrixOverride" => {
                    (self.m_culling_matrix_override).inspect_value(path, offset, limit)
                }
                "m_UseCullingMatrixOverride" => {
                    (self.m_use_culling_matrix_override).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_type).inspect_value(path, offset, limit)).map(|v| ("m_Type".to_string(), v)),
            ((self.m_resolution).inspect_value(path, offset, limit))
                .map(|v| ("m_Resolution".to_string(), v)),
            ((self.m_strength).inspect_value(path, offset, limit))
                .map(|v| ("m_Strength".to_string(), v)),
            ((self.m_custom_resolution).inspect_value(path, offset, limit))
                .map(|v| ("m_CustomResolution".to_string(), v)),
            ((self.m_bias).inspect_value(path, offset, limit)).map(|v| ("m_Bias".to_string(), v)),
            ((self.m_softness).inspect_value(path, offset, limit))
                .map(|v| ("m_Softness".to_string(), v)),
            ((self.m_normal_bias).inspect_value(path, offset, limit))
                .map(|v| ("m_NormalBias".to_string(), v)),
            ((self.m_softness_fade).inspect_value(path, offset, limit))
                .map(|v| ("m_SoftnessFade".to_string(), v)),
            ((self.m_near_plane).inspect_value(path, offset, limit))
                .map(|v| ("m_NearPlane".to_string(), v)),
            ((self.m_culling_matrix_override).inspect_value(path, offset, limit))
                .map(|v| ("m_CullingMatrixOverride".to_string(), v)),
            ((self.m_use_culling_matrix_override).inspect_value(path, offset, limit))
                .map(|v| ("m_UseCullingMatrixOverride".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
