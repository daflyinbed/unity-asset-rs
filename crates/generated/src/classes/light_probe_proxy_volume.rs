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
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_bounding_box_mode = r.read_i32()?;
        let m_resolution_x = r.read_u32()?;
        let m_resolution_y = r.read_u32()?;
        let m_resolution_z = r.read_u32()?;
        let m_resolution_probes_per_unit = r.read_f32()?;
        let m_bounding_box_size = Vector3f::parse(r, ver)?;
        let m_bounding_box_origin = Vector3f::parse(r, ver)?;
        let m_resolution_mode = r.read_i32()?;
        let m_probe_position_mode = r.read_i32()?;
        let m_refresh_mode = r.read_i32()?;
        let m_quality_mode = if *ver >= crate::UnityVersion::new(2018, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_data_format = if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            Some(r.read_i32()?)
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
