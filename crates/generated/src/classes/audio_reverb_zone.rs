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
