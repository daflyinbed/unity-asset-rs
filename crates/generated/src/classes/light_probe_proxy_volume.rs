//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct LightProbeProxyVolume {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_bounding_box_mode: i32,
    pub m_resolution_x: u32,
    pub m_resolution_y: u32,
    pub m_resolution_z: u32,
    pub m_resolution_probes_per_unit: f32,
    pub m_bounding_box_size: Vector3f,
    pub m_bounding_box_origin: Vector3f,
    pub m_resolution_mode: i32,
    pub m_probe_position_mode: i32,
    pub m_refresh_mode: i32,
    pub m_quality_mode: Option<i32>,
    pub m_data_format: Option<i32>,
}

impl crate::Parse for LightProbeProxyVolume {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_bounding_box_mode = rdr.read_i32()?;
        let m_resolution_x = rdr.read_u32()?;
        let m_resolution_y = rdr.read_u32()?;
        let m_resolution_z = rdr.read_u32()?;
        let m_resolution_probes_per_unit = rdr.read_f32()?;
        let m_bounding_box_size = Vector3f::parse(rdr, ver)?;
        let m_bounding_box_origin = Vector3f::parse(rdr, ver)?;
        let m_resolution_mode = rdr.read_i32()?;
        let m_probe_position_mode = rdr.read_i32()?;
        let m_refresh_mode = rdr.read_i32()?;
        let m_quality_mode = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };
        let m_data_format = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(LightProbeProxyVolume {
            m_game_object,
            m_enabled,
            m_bounding_box_mode,
            m_resolution_x,
            m_resolution_y,
            m_resolution_z,
            m_resolution_probes_per_unit,
            m_bounding_box_size,
            m_bounding_box_origin,
            m_resolution_mode,
            m_probe_position_mode,
            m_refresh_mode,
            m_quality_mode,
            m_data_format,
        })
    }
}

impl crate::ToValue for LightProbeProxyVolume {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some((
                "m_BoundingBoxMode".to_string(),
                self.m_bounding_box_mode.to_value(),
            )),
            Some(("m_ResolutionX".to_string(), self.m_resolution_x.to_value())),
            Some(("m_ResolutionY".to_string(), self.m_resolution_y.to_value())),
            Some(("m_ResolutionZ".to_string(), self.m_resolution_z.to_value())),
            Some((
                "m_ResolutionProbesPerUnit".to_string(),
                self.m_resolution_probes_per_unit.to_value(),
            )),
            Some((
                "m_BoundingBoxSize".to_string(),
                self.m_bounding_box_size.to_value(),
            )),
            Some((
                "m_BoundingBoxOrigin".to_string(),
                self.m_bounding_box_origin.to_value(),
            )),
            Some((
                "m_ResolutionMode".to_string(),
                self.m_resolution_mode.to_value(),
            )),
            Some((
                "m_ProbePositionMode".to_string(),
                self.m_probe_position_mode.to_value(),
            )),
            Some(("m_RefreshMode".to_string(), self.m_refresh_mode.to_value())),
            crate::opt_field("m_QualityMode", &self.m_quality_mode),
            crate::opt_field("m_DataFormat", &self.m_data_format),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_BoundingBoxMode" => {
                    (self.m_bounding_box_mode).inspect_value(path, offset, limit)
                }
                "m_ResolutionX" => (self.m_resolution_x).inspect_value(path, offset, limit),
                "m_ResolutionY" => (self.m_resolution_y).inspect_value(path, offset, limit),
                "m_ResolutionZ" => (self.m_resolution_z).inspect_value(path, offset, limit),
                "m_ResolutionProbesPerUnit" => {
                    (self.m_resolution_probes_per_unit).inspect_value(path, offset, limit)
                }
                "m_BoundingBoxSize" => {
                    (self.m_bounding_box_size).inspect_value(path, offset, limit)
                }
                "m_BoundingBoxOrigin" => {
                    (self.m_bounding_box_origin).inspect_value(path, offset, limit)
                }
                "m_ResolutionMode" => (self.m_resolution_mode).inspect_value(path, offset, limit),
                "m_ProbePositionMode" => {
                    (self.m_probe_position_mode).inspect_value(path, offset, limit)
                }
                "m_RefreshMode" => (self.m_refresh_mode).inspect_value(path, offset, limit),
                "m_QualityMode" => (self.m_quality_mode).inspect_value(path, offset, limit),
                "m_DataFormat" => (self.m_data_format).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_bounding_box_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_BoundingBoxMode".to_string(), v)),
            ((self.m_resolution_x).inspect_value(path, offset, limit))
                .map(|v| ("m_ResolutionX".to_string(), v)),
            ((self.m_resolution_y).inspect_value(path, offset, limit))
                .map(|v| ("m_ResolutionY".to_string(), v)),
            ((self.m_resolution_z).inspect_value(path, offset, limit))
                .map(|v| ("m_ResolutionZ".to_string(), v)),
            ((self.m_resolution_probes_per_unit).inspect_value(path, offset, limit))
                .map(|v| ("m_ResolutionProbesPerUnit".to_string(), v)),
            ((self.m_bounding_box_size).inspect_value(path, offset, limit))
                .map(|v| ("m_BoundingBoxSize".to_string(), v)),
            ((self.m_bounding_box_origin).inspect_value(path, offset, limit))
                .map(|v| ("m_BoundingBoxOrigin".to_string(), v)),
            ((self.m_resolution_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_ResolutionMode".to_string(), v)),
            ((self.m_probe_position_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_ProbePositionMode".to_string(), v)),
            ((self.m_refresh_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_RefreshMode".to_string(), v)),
            ((self.m_quality_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_QualityMode".to_string(), v)),
            ((self.m_data_format).inspect_value(path, offset, limit))
                .map(|v| ("m_DataFormat".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
