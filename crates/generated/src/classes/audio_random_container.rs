//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct AudioRandomContainer {
    pub m_name: String,
    pub m_volume: f32,
    pub m_pitch: f32,
    pub m_automatic_trigger_time: f32,
    pub m_loop_count: i32,
    pub m_volume_randomization_range: Vector2f,
    pub m_pitch_randomization_range: Vector2f,
    pub m_automatic_trigger_time_randomization_range: Vector2f,
    pub m_loop_count_randomization_range: Vector2f,
    pub m_elements: Vec<PPtr>,
    pub m_avoid_repeating_last: i32,
    pub m_playback_mode: i32,
    pub m_trigger_mode: i32,
    pub m_automatic_trigger_mode: i32,
    pub m_loop_mode: i32,
    pub m_volume_randomization_enabled: bool,
    pub m_pitch_randomization_enabled: bool,
    pub m_automatic_trigger_time_randomization_enabled: bool,
    pub m_loop_count_randomization_enabled: bool,
}

impl crate::Parse for AudioRandomContainer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_volume = rdr.read_f32()?;
        let m_pitch = rdr.read_f32()?;
        let m_automatic_trigger_time = rdr.read_f32()?;
        let m_loop_count = rdr.read_i32()?;
        let m_volume_randomization_range = Vector2f::parse(rdr, ver)?;
        let m_pitch_randomization_range = Vector2f::parse(rdr, ver)?;
        let m_automatic_trigger_time_randomization_range = Vector2f::parse(rdr, ver)?;
        let m_loop_count_randomization_range = Vector2f::parse(rdr, ver)?;
        let m_elements = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_avoid_repeating_last = rdr.read_i32()?;
        let m_playback_mode = rdr.read_i32()?;
        let m_trigger_mode = rdr.read_i32()?;
        let m_automatic_trigger_mode = rdr.read_i32()?;
        let m_loop_mode = rdr.read_i32()?;
        let m_volume_randomization_enabled = rdr.read_bool()?;
        let m_pitch_randomization_enabled = rdr.read_bool()?;
        let m_automatic_trigger_time_randomization_enabled = rdr.read_bool()?;
        let m_loop_count_randomization_enabled = rdr.read_bool()?;

        Ok(AudioRandomContainer {
            m_name,
            m_volume,
            m_pitch,
            m_automatic_trigger_time,
            m_loop_count,
            m_volume_randomization_range,
            m_pitch_randomization_range,
            m_automatic_trigger_time_randomization_range,
            m_loop_count_randomization_range,
            m_elements,
            m_avoid_repeating_last,
            m_playback_mode,
            m_trigger_mode,
            m_automatic_trigger_mode,
            m_loop_mode,
            m_volume_randomization_enabled,
            m_pitch_randomization_enabled,
            m_automatic_trigger_time_randomization_enabled,
            m_loop_count_randomization_enabled,
        })
    }
}
