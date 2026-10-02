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

impl crate::ToValue for AudioRandomContainer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Volume".to_string(), self.m_volume.to_value())),
            Some(("m_Pitch".to_string(), self.m_pitch.to_value())),
            Some((
                "m_AutomaticTriggerTime".to_string(),
                self.m_automatic_trigger_time.to_value(),
            )),
            Some(("m_LoopCount".to_string(), self.m_loop_count.to_value())),
            Some((
                "m_VolumeRandomizationRange".to_string(),
                self.m_volume_randomization_range.to_value(),
            )),
            Some((
                "m_PitchRandomizationRange".to_string(),
                self.m_pitch_randomization_range.to_value(),
            )),
            Some((
                "m_AutomaticTriggerTimeRandomizationRange".to_string(),
                self.m_automatic_trigger_time_randomization_range.to_value(),
            )),
            Some((
                "m_LoopCountRandomizationRange".to_string(),
                self.m_loop_count_randomization_range.to_value(),
            )),
            Some(("m_Elements".to_string(), self.m_elements.to_value())),
            Some((
                "m_AvoidRepeatingLast".to_string(),
                self.m_avoid_repeating_last.to_value(),
            )),
            Some((
                "m_PlaybackMode".to_string(),
                self.m_playback_mode.to_value(),
            )),
            Some(("m_TriggerMode".to_string(), self.m_trigger_mode.to_value())),
            Some((
                "m_AutomaticTriggerMode".to_string(),
                self.m_automatic_trigger_mode.to_value(),
            )),
            Some(("m_LoopMode".to_string(), self.m_loop_mode.to_value())),
            Some((
                "m_VolumeRandomizationEnabled".to_string(),
                self.m_volume_randomization_enabled.to_value(),
            )),
            Some((
                "m_PitchRandomizationEnabled".to_string(),
                self.m_pitch_randomization_enabled.to_value(),
            )),
            Some((
                "m_AutomaticTriggerTimeRandomizationEnabled".to_string(),
                self.m_automatic_trigger_time_randomization_enabled
                    .to_value(),
            )),
            Some((
                "m_LoopCountRandomizationEnabled".to_string(),
                self.m_loop_count_randomization_enabled.to_value(),
            )),
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
                "m_Volume" => (self.m_volume).inspect_value(path, offset, limit),
                "m_Pitch" => (self.m_pitch).inspect_value(path, offset, limit),
                "m_AutomaticTriggerTime" => {
                    (self.m_automatic_trigger_time).inspect_value(path, offset, limit)
                }
                "m_LoopCount" => (self.m_loop_count).inspect_value(path, offset, limit),
                "m_VolumeRandomizationRange" => {
                    (self.m_volume_randomization_range).inspect_value(path, offset, limit)
                }
                "m_PitchRandomizationRange" => {
                    (self.m_pitch_randomization_range).inspect_value(path, offset, limit)
                }
                "m_AutomaticTriggerTimeRandomizationRange" => (self
                    .m_automatic_trigger_time_randomization_range)
                    .inspect_value(path, offset, limit),
                "m_LoopCountRandomizationRange" => {
                    (self.m_loop_count_randomization_range).inspect_value(path, offset, limit)
                }
                "m_Elements" => (self.m_elements).inspect_value(path, offset, limit),
                "m_AvoidRepeatingLast" => {
                    (self.m_avoid_repeating_last).inspect_value(path, offset, limit)
                }
                "m_PlaybackMode" => (self.m_playback_mode).inspect_value(path, offset, limit),
                "m_TriggerMode" => (self.m_trigger_mode).inspect_value(path, offset, limit),
                "m_AutomaticTriggerMode" => {
                    (self.m_automatic_trigger_mode).inspect_value(path, offset, limit)
                }
                "m_LoopMode" => (self.m_loop_mode).inspect_value(path, offset, limit),
                "m_VolumeRandomizationEnabled" => {
                    (self.m_volume_randomization_enabled).inspect_value(path, offset, limit)
                }
                "m_PitchRandomizationEnabled" => {
                    (self.m_pitch_randomization_enabled).inspect_value(path, offset, limit)
                }
                "m_AutomaticTriggerTimeRandomizationEnabled" => (self
                    .m_automatic_trigger_time_randomization_enabled)
                    .inspect_value(path, offset, limit),
                "m_LoopCountRandomizationEnabled" => {
                    (self.m_loop_count_randomization_enabled).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_volume).inspect_value(path, offset, limit))
                .map(|v| ("m_Volume".to_string(), v)),
            ((self.m_pitch).inspect_value(path, offset, limit)).map(|v| ("m_Pitch".to_string(), v)),
            ((self.m_automatic_trigger_time).inspect_value(path, offset, limit))
                .map(|v| ("m_AutomaticTriggerTime".to_string(), v)),
            ((self.m_loop_count).inspect_value(path, offset, limit))
                .map(|v| ("m_LoopCount".to_string(), v)),
            ((self.m_volume_randomization_range).inspect_value(path, offset, limit))
                .map(|v| ("m_VolumeRandomizationRange".to_string(), v)),
            ((self.m_pitch_randomization_range).inspect_value(path, offset, limit))
                .map(|v| ("m_PitchRandomizationRange".to_string(), v)),
            ((self.m_automatic_trigger_time_randomization_range)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_AutomaticTriggerTimeRandomizationRange".to_string(), v)),
            ((self.m_loop_count_randomization_range).inspect_value(path, offset, limit))
                .map(|v| ("m_LoopCountRandomizationRange".to_string(), v)),
            ((self.m_elements).inspect_value(path, offset, limit))
                .map(|v| ("m_Elements".to_string(), v)),
            ((self.m_avoid_repeating_last).inspect_value(path, offset, limit))
                .map(|v| ("m_AvoidRepeatingLast".to_string(), v)),
            ((self.m_playback_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_PlaybackMode".to_string(), v)),
            ((self.m_trigger_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_TriggerMode".to_string(), v)),
            ((self.m_automatic_trigger_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_AutomaticTriggerMode".to_string(), v)),
            ((self.m_loop_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_LoopMode".to_string(), v)),
            ((self.m_volume_randomization_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_VolumeRandomizationEnabled".to_string(), v)),
            ((self.m_pitch_randomization_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_PitchRandomizationEnabled".to_string(), v)),
            ((self.m_automatic_trigger_time_randomization_enabled)
                .inspect_value(path, offset, limit))
            .map(|v| ("m_AutomaticTriggerTimeRandomizationEnabled".to_string(), v)),
            ((self.m_loop_count_randomization_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_LoopCountRandomizationEnabled".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
