//! 自动生成。请勿手动修改。

use crate::PPtr;
use super::behaviour::Behaviour;

#[derive(Debug, Clone)]
pub struct VideoPlayer {
    pub parent: Behaviour,
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_video_clip: PPtr,
    pub m_target_camera_alpha: f32,
    pub m_target_camera: PPtr,
    pub m_target_camera3_d_layout: Option<i32>,
    pub m_target_texture: PPtr,
    pub m_target_material_renderer: PPtr,
    pub m_time_reference: Option<i32>,
    pub m_render_mode: i32,
    pub m_target_material_property: String,
    pub m_aspect_ratio: i32,
    pub m_data_source: i32,
    pub m_playback_speed: f32,
    pub m_audio_output_mode: i32,
    pub m_target_audio_sources: Vec<PPtr>,
    pub m_direct_audio_volumes: Vec<f32>,
    pub m_time_update_mode: Option<i32>,
    pub m_url: String,
    pub m_target_material_name: Option<String>,
    pub m_enabled_audio_tracks: Vec<bool>,
    pub m_direct_audio_mutes: Vec<bool>,
    pub m_controlled_audio_track_count: u16,
    pub m_play_on_awake: bool,
    pub m_skip_on_drop: bool,
    pub m_looping: bool,
    pub m_wait_for_first_frame: bool,
    pub m_frame_ready_event_enabled: bool,
    pub m_video_shaders: Option<Vec<PPtr>>,
}

impl crate::Parse for VideoPlayer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = Behaviour::parse(r, ver)?;
        let m_game_object = crate::PPtr::parse(r, ver)?;
        let m_enabled = r.read_u8()?;
        r.align(4)?;
        let m_video_clip = crate::PPtr::parse(r, ver)?;
        let m_target_camera_alpha = r.read_f32()?;
        let m_target_camera = crate::PPtr::parse(r, ver)?;
        let m_target_camera3_d_layout = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_target_texture = crate::PPtr::parse(r, ver)?;
        let m_target_material_renderer = crate::PPtr::parse(r, ver)?;
        let m_time_reference = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_render_mode = r.read_i32()?;
        let m_target_material_property = r.read_aligned_string()?;
        r.align(4)?;
        let m_aspect_ratio = r.read_i32()?;
        let m_data_source = r.read_i32()?;
        let m_playback_speed = r.read_f32()?;
        let m_audio_output_mode = r.read_i32()?;
        let m_target_audio_sources = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_direct_audio_volumes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_time_update_mode = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_url = r.read_aligned_string()?;
        r.align(4)?;
        let m_target_material_name = if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_enabled_audio_tracks = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_direct_audio_mutes = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_controlled_audio_track_count = r.read_u16()?;
        let m_play_on_awake = r.read_bool()?;
        let m_skip_on_drop = r.read_bool()?;
        let m_looping = r.read_bool()?;
        let m_wait_for_first_frame = r.read_bool()?;
        let m_frame_ready_event_enabled = r.read_bool()?;
        let m_video_shaders = if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(VideoPlayer {
            parent,
            m_game_object,
            m_enabled,
            m_video_clip,
            m_target_camera_alpha,
            m_target_camera,
            m_target_camera3_d_layout,
            m_target_texture,
            m_target_material_renderer,
            m_time_reference,
            m_render_mode,
            m_target_material_property,
            m_aspect_ratio,
            m_data_source,
            m_playback_speed,
            m_audio_output_mode,
            m_target_audio_sources,
            m_direct_audio_volumes,
            m_time_update_mode,
            m_url,
            m_target_material_name,
            m_enabled_audio_tracks,
            m_direct_audio_mutes,
            m_controlled_audio_track_count,
            m_play_on_awake,
            m_skip_on_drop,
            m_looping,
            m_wait_for_first_frame,
            m_frame_ready_event_enabled,
            m_video_shaders,
        })
    }
}
