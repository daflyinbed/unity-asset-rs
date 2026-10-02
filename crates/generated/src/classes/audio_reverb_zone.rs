//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AudioReverbZone {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_min_distance: f32,
    pub m_max_distance: f32,
    pub m_reverb_preset: i32,
    pub m_room: i32,
    pub m_room_hf: i32,
    pub m_decay_time: f32,
    pub m_decay_hf_ratio: f32,
    pub m_reflections: i32,
    pub m_reflections_delay: f32,
    pub m_reverb: i32,
    pub m_reverb_delay: f32,
    pub m_hf_reference: f32,
    pub m_room_rolloff_factor: Option<f32>,
    pub m_diffusion: f32,
    pub m_density: f32,
    pub m_lf_reference: f32,
    pub m_room_lf: i32,
}

impl crate::Parse for AudioReverbZone {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_min_distance = None;
        let mut m_max_distance = None;
        let mut m_reverb_preset = None;
        let mut m_room = None;
        let mut m_room_hf = None;
        let mut m_decay_time = None;
        let mut m_decay_hf_ratio = None;
        let mut m_reflections = None;
        let mut m_reflections_delay = None;
        let mut m_reverb = None;
        let mut m_reverb_delay = None;
        let mut m_hf_reference = None;
        let mut m_room_rolloff_factor = None;
        let mut m_diffusion = None;
        let mut m_density = None;
        let mut m_lf_reference = None;
        let mut m_room_lf = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_min_distance = Some(rdr.read_f32()?);
        m_max_distance = Some(rdr.read_f32()?);
        m_reverb_preset = Some(rdr.read_i32()?);
        m_room = Some(rdr.read_i32()?);
        m_room_hf = Some(rdr.read_i32()?);
        m_decay_time = Some(rdr.read_f32()?);
        m_decay_hf_ratio = Some(rdr.read_f32()?);
        m_reflections = Some(rdr.read_i32()?);
        m_reflections_delay = Some(rdr.read_f32()?);
        m_reverb = Some(rdr.read_i32()?);
        m_reverb_delay = Some(rdr.read_f32()?);
        m_hf_reference = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
        } else {
            m_room_rolloff_factor = Some(rdr.read_f32()?);
        }
        m_diffusion = Some(rdr.read_f32()?);
        m_density = Some(rdr.read_f32()?);
        m_lf_reference = Some(rdr.read_f32()?);
        m_room_lf = Some(rdr.read_i32()?);

        Ok(AudioReverbZone {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_min_distance: m_min_distance.unwrap(),
            m_max_distance: m_max_distance.unwrap(),
            m_reverb_preset: m_reverb_preset.unwrap(),
            m_room: m_room.unwrap(),
            m_room_hf: m_room_hf.unwrap(),
            m_decay_time: m_decay_time.unwrap(),
            m_decay_hf_ratio: m_decay_hf_ratio.unwrap(),
            m_reflections: m_reflections.unwrap(),
            m_reflections_delay: m_reflections_delay.unwrap(),
            m_reverb: m_reverb.unwrap(),
            m_reverb_delay: m_reverb_delay.unwrap(),
            m_hf_reference: m_hf_reference.unwrap(),
            m_room_rolloff_factor,
            m_diffusion: m_diffusion.unwrap(),
            m_density: m_density.unwrap(),
            m_lf_reference: m_lf_reference.unwrap(),
            m_room_lf: m_room_lf.unwrap(),
        })
    }
}

impl crate::ToValue for AudioReverbZone {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_MinDistance".to_string(), self.m_min_distance.to_value())),
            Some(("m_MaxDistance".to_string(), self.m_max_distance.to_value())),
            Some((
                "m_ReverbPreset".to_string(),
                self.m_reverb_preset.to_value(),
            )),
            Some(("m_Room".to_string(), self.m_room.to_value())),
            Some(("m_RoomHF".to_string(), self.m_room_hf.to_value())),
            Some(("m_DecayTime".to_string(), self.m_decay_time.to_value())),
            Some((
                "m_DecayHFRatio".to_string(),
                self.m_decay_hf_ratio.to_value(),
            )),
            Some(("m_Reflections".to_string(), self.m_reflections.to_value())),
            Some((
                "m_ReflectionsDelay".to_string(),
                self.m_reflections_delay.to_value(),
            )),
            Some(("m_Reverb".to_string(), self.m_reverb.to_value())),
            Some(("m_ReverbDelay".to_string(), self.m_reverb_delay.to_value())),
            Some(("m_HFReference".to_string(), self.m_hf_reference.to_value())),
            crate::opt_field("m_RoomRolloffFactor", &self.m_room_rolloff_factor),
            Some(("m_Diffusion".to_string(), self.m_diffusion.to_value())),
            Some(("m_Density".to_string(), self.m_density.to_value())),
            Some(("m_LFReference".to_string(), self.m_lf_reference.to_value())),
            Some(("m_RoomLF".to_string(), self.m_room_lf.to_value())),
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
                "m_MinDistance" => (self.m_min_distance).inspect_value(path, offset, limit),
                "m_MaxDistance" => (self.m_max_distance).inspect_value(path, offset, limit),
                "m_ReverbPreset" => (self.m_reverb_preset).inspect_value(path, offset, limit),
                "m_Room" => (self.m_room).inspect_value(path, offset, limit),
                "m_RoomHF" => (self.m_room_hf).inspect_value(path, offset, limit),
                "m_DecayTime" => (self.m_decay_time).inspect_value(path, offset, limit),
                "m_DecayHFRatio" => (self.m_decay_hf_ratio).inspect_value(path, offset, limit),
                "m_Reflections" => (self.m_reflections).inspect_value(path, offset, limit),
                "m_ReflectionsDelay" => {
                    (self.m_reflections_delay).inspect_value(path, offset, limit)
                }
                "m_Reverb" => (self.m_reverb).inspect_value(path, offset, limit),
                "m_ReverbDelay" => (self.m_reverb_delay).inspect_value(path, offset, limit),
                "m_HFReference" => (self.m_hf_reference).inspect_value(path, offset, limit),
                "m_RoomRolloffFactor" => {
                    (self.m_room_rolloff_factor).inspect_value(path, offset, limit)
                }
                "m_Diffusion" => (self.m_diffusion).inspect_value(path, offset, limit),
                "m_Density" => (self.m_density).inspect_value(path, offset, limit),
                "m_LFReference" => (self.m_lf_reference).inspect_value(path, offset, limit),
                "m_RoomLF" => (self.m_room_lf).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_min_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_MinDistance".to_string(), v)),
            ((self.m_max_distance).inspect_value(path, offset, limit))
                .map(|v| ("m_MaxDistance".to_string(), v)),
            ((self.m_reverb_preset).inspect_value(path, offset, limit))
                .map(|v| ("m_ReverbPreset".to_string(), v)),
            ((self.m_room).inspect_value(path, offset, limit)).map(|v| ("m_Room".to_string(), v)),
            ((self.m_room_hf).inspect_value(path, offset, limit))
                .map(|v| ("m_RoomHF".to_string(), v)),
            ((self.m_decay_time).inspect_value(path, offset, limit))
                .map(|v| ("m_DecayTime".to_string(), v)),
            ((self.m_decay_hf_ratio).inspect_value(path, offset, limit))
                .map(|v| ("m_DecayHFRatio".to_string(), v)),
            ((self.m_reflections).inspect_value(path, offset, limit))
                .map(|v| ("m_Reflections".to_string(), v)),
            ((self.m_reflections_delay).inspect_value(path, offset, limit))
                .map(|v| ("m_ReflectionsDelay".to_string(), v)),
            ((self.m_reverb).inspect_value(path, offset, limit))
                .map(|v| ("m_Reverb".to_string(), v)),
            ((self.m_reverb_delay).inspect_value(path, offset, limit))
                .map(|v| ("m_ReverbDelay".to_string(), v)),
            ((self.m_hf_reference).inspect_value(path, offset, limit))
                .map(|v| ("m_HFReference".to_string(), v)),
            ((self.m_room_rolloff_factor).inspect_value(path, offset, limit))
                .map(|v| ("m_RoomRolloffFactor".to_string(), v)),
            ((self.m_diffusion).inspect_value(path, offset, limit))
                .map(|v| ("m_Diffusion".to_string(), v)),
            ((self.m_density).inspect_value(path, offset, limit))
                .map(|v| ("m_Density".to_string(), v)),
            ((self.m_lf_reference).inspect_value(path, offset, limit))
                .map(|v| ("m_LFReference".to_string(), v)),
            ((self.m_room_lf).inspect_value(path, offset, limit))
                .map(|v| ("m_RoomLF".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
