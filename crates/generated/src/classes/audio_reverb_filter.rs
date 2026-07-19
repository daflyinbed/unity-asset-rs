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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_dry_level = None;
        let mut m_room = None;
        let mut m_room_hf = None;
        let mut m_room_rolloff = None;
        let mut m_decay_time = None;
        let mut m_decay_hf_ratio = None;
        let mut m_reflections_level = None;
        let mut m_reverb_level = None;
        let mut m_reverb_delay = None;
        let mut m_diffusion = None;
        let mut m_density = None;
        let mut m_hf_reference = None;
        let mut m_room_lf = None;
        let mut m_lf_reference = None;
        let mut m_reflections_delay = None;
        let mut m_reverb_preset = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_dry_level = Some(rdr.read_f32()?);
        m_room = Some(rdr.read_f32()?);
        m_room_hf = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
        } else {
            m_room_rolloff = Some(rdr.read_f32()?);
        }
        m_decay_time = Some(rdr.read_f32()?);
        m_decay_hf_ratio = Some(rdr.read_f32()?);
        m_reflections_level = Some(rdr.read_f32()?);
        m_reverb_level = Some(rdr.read_f32()?);
        m_reverb_delay = Some(rdr.read_f32()?);
        m_diffusion = Some(rdr.read_f32()?);
        m_density = Some(rdr.read_f32()?);
        m_hf_reference = Some(rdr.read_f32()?);
        m_room_lf = Some(rdr.read_f32()?);
        m_lf_reference = Some(rdr.read_f32()?);
        m_reflections_delay = Some(rdr.read_f32()?);
        m_reverb_preset = Some(rdr.read_i32()?);

        Ok(AudioReverbFilter {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_dry_level: m_dry_level.unwrap(),
            m_room: m_room.unwrap(),
            m_room_hf: m_room_hf.unwrap(),
            m_room_rolloff,
            m_decay_time: m_decay_time.unwrap(),
            m_decay_hf_ratio: m_decay_hf_ratio.unwrap(),
            m_reflections_level: m_reflections_level.unwrap(),
            m_reverb_level: m_reverb_level.unwrap(),
            m_reverb_delay: m_reverb_delay.unwrap(),
            m_diffusion: m_diffusion.unwrap(),
            m_density: m_density.unwrap(),
            m_hf_reference: m_hf_reference.unwrap(),
            m_room_lf: m_room_lf.unwrap(),
            m_lf_reference: m_lf_reference.unwrap(),
            m_reflections_delay: m_reflections_delay.unwrap(),
            m_reverb_preset: m_reverb_preset.unwrap(),
        })
    }
}
