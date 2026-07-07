//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AudioReverbFilter {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_dry_level: f32,
    pub m_room: f32,
    pub m_room_hf: f32,
    pub m_room_rolloff: Option<f32>,
    pub m_decay_time: f32,
    pub m_decay_hf_ratio: f32,
    pub m_reflections_level: f32,
    pub m_reverb_level: f32,
    pub m_reverb_delay: f32,
    pub m_diffusion: f32,
    pub m_density: f32,
    pub m_hf_reference: f32,
    pub m_room_lf: f32,
    pub m_lf_reference: f32,
    pub m_reflections_delay: f32,
    pub m_reverb_preset: i32,
}

impl crate::Parse for AudioReverbFilter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_dry_level = r.read_f32()?;
        let m_room = r.read_f32()?;
        let m_room_hf = r.read_f32()?;
        let m_room_rolloff = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(r.read_f32()?)
        } else {
            None
        };
        let m_decay_time = r.read_f32()?;
        let m_decay_hf_ratio = r.read_f32()?;
        let m_reflections_level = r.read_f32()?;
        let m_reverb_level = r.read_f32()?;
        let m_reverb_delay = r.read_f32()?;
        let m_diffusion = r.read_f32()?;
        let m_density = r.read_f32()?;
        let m_hf_reference = r.read_f32()?;
        let m_room_lf = r.read_f32()?;
        let m_lf_reference = r.read_f32()?;
        let m_reflections_delay = r.read_f32()?;
        let m_reverb_preset = r.read_i32()?;

        Ok(AudioReverbFilter {
            m_game_object,
            m_enabled,
            m_dry_level,
            m_room,
            m_room_hf,
            m_room_rolloff,
            m_decay_time,
            m_decay_hf_ratio,
            m_reflections_level,
            m_reverb_level,
            m_reverb_delay,
            m_diffusion,
            m_density,
            m_hf_reference,
            m_room_lf,
            m_lf_reference,
            m_reflections_delay,
            m_reverb_preset,
        })
    }
}
