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
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_min_distance = rdr.read_f32()?;
        let m_max_distance = rdr.read_f32()?;
        let m_reverb_preset = rdr.read_i32()?;
        let m_room = rdr.read_i32()?;
        let m_room_hf = rdr.read_i32()?;
        let m_decay_time = rdr.read_f32()?;
        let m_decay_hf_ratio = rdr.read_f32()?;
        let m_reflections = rdr.read_i32()?;
        let m_reflections_delay = rdr.read_f32()?;
        let m_reverb = rdr.read_i32()?;
        let m_reverb_delay = rdr.read_f32()?;
        let m_hf_reference = rdr.read_f32()?;
        let m_room_rolloff_factor = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let m_diffusion = rdr.read_f32()?;
        let m_density = rdr.read_f32()?;
        let m_lf_reference = rdr.read_f32()?;
        let m_room_lf = rdr.read_i32()?;

        Ok(AudioReverbZone {
            m_game_object,
            m_enabled,
            m_min_distance,
            m_max_distance,
            m_reverb_preset,
            m_room,
            m_room_hf,
            m_decay_time,
            m_decay_hf_ratio,
            m_reflections,
            m_reflections_delay,
            m_reverb,
            m_reverb_delay,
            m_hf_reference,
            m_room_rolloff_factor,
            m_diffusion,
            m_density,
            m_lf_reference,
            m_room_lf,
        })
    }
}
