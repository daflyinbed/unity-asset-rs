//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AudioReverbZone {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
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
    pub m_room_rolloff_factor: f32,
    pub m_diffusion: f32,
    pub m_density: f32,
    pub m_lf_reference: f32,
    pub m_room_lf: i32,
}

impl crate::Parse for AudioReverbZone {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_min_distance = r.read_f32()?;
        let m_max_distance = r.read_f32()?;
        let m_reverb_preset = r.read_i32()?;
        let m_room = r.read_i32()?;
        let m_room_hf = r.read_i32()?;
        let m_decay_time = r.read_f32()?;
        let m_decay_hf_ratio = r.read_f32()?;
        let m_reflections = r.read_i32()?;
        let m_reflections_delay = r.read_f32()?;
        let m_reverb = r.read_i32()?;
        let m_reverb_delay = r.read_f32()?;
        let m_hf_reference = r.read_f32()?;
        let m_room_rolloff_factor = r.read_f32()?;
        let m_diffusion = r.read_f32()?;
        let m_density = r.read_f32()?;
        let m_lf_reference = r.read_f32()?;
        let m_room_lf = r.read_i32()?;

        Ok(AudioReverbZone {
            m_object_hide_flags,
            m_extension_ptr,
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
