//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct VideoPlayer {
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
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_video_clip = None;
        let mut m_target_camera_alpha = None;
        let mut m_target_camera = None;
        let mut m_target_camera3_d_layout = None;
        let mut m_target_texture = None;
        let mut m_target_material_renderer = None;
        let mut m_time_reference = None;
        let mut m_render_mode = None;
        let mut m_target_material_property = None;
        let mut m_aspect_ratio = None;
        let mut m_data_source = None;
        let mut m_playback_speed = None;
        let mut m_audio_output_mode = None;
        let mut m_target_audio_sources = None;
        let mut m_direct_audio_volumes = None;
        let mut m_time_update_mode = None;
        let mut m_url = None;
        let mut m_target_material_name = None;
        let mut m_enabled_audio_tracks = None;
        let mut m_direct_audio_mutes = None;
        let mut m_controlled_audio_track_count = None;
        let mut m_play_on_awake = None;
        let mut m_skip_on_drop = None;
        let mut m_looping = None;
        let mut m_wait_for_first_frame = None;
        let mut m_frame_ready_event_enabled = None;
        let mut m_video_shaders = None;
            m_game_object = Some(crate::PPtr::parse(r, ver)?);
            m_enabled = Some(r.read_u8()?);
            r.align(4)?;
            m_video_clip = Some(crate::PPtr::parse(r, ver)?);
            m_target_camera_alpha = Some(r.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_target_camera3_d_layout = Some(r.read_i32()?);
        } else {
        }
            m_target_camera = Some(crate::PPtr::parse(r, ver)?);
            m_target_texture = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_time_reference = Some(r.read_i32()?);
        } else {
        }
            m_target_material_renderer = Some(crate::PPtr::parse(r, ver)?);
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_target_material_property = Some(r.read_aligned_string()?);
            r.align(4)?;
        } else {
        }
            m_render_mode = Some(r.read_i32()?);
            m_aspect_ratio = Some(r.read_i32()?);
            m_data_source = Some(r.read_i32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_time_update_mode = Some(r.read_i32()?);
        } else {
        }
            m_playback_speed = Some(r.read_f32()?);
            m_audio_output_mode = Some(r.read_i32()?);
            m_target_audio_sources = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_direct_audio_volumes = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_url = Some(r.read_aligned_string()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
        } else {
            m_target_material_name = Some(r.read_aligned_string()?);
            r.align(4)?;
            m_target_material_property = Some(r.read_aligned_string()?);
            r.align(4)?;
        }
            m_enabled_audio_tracks = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_direct_audio_mutes = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_bool() }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
            m_controlled_audio_track_count = Some(r.read_u16()?);
            m_play_on_awake = Some(r.read_bool()?);
            m_skip_on_drop = Some(r.read_bool()?);
            m_looping = Some(r.read_bool()?);
            m_wait_for_first_frame = Some(r.read_bool()?);
            m_frame_ready_event_enabled = Some(r.read_bool()?);
            r.align(4)?;
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_video_shaders = Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            r.align(4)?;
        } else {
        }

        Ok(VideoPlayer {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_video_clip: m_video_clip.unwrap(),
            m_target_camera_alpha: m_target_camera_alpha.unwrap(),
            m_target_camera: m_target_camera.unwrap(),
            m_target_camera3_d_layout,
            m_target_texture: m_target_texture.unwrap(),
            m_target_material_renderer: m_target_material_renderer.unwrap(),
            m_time_reference,
            m_render_mode: m_render_mode.unwrap(),
            m_target_material_property: m_target_material_property.unwrap(),
            m_aspect_ratio: m_aspect_ratio.unwrap(),
            m_data_source: m_data_source.unwrap(),
            m_playback_speed: m_playback_speed.unwrap(),
            m_audio_output_mode: m_audio_output_mode.unwrap(),
            m_target_audio_sources: m_target_audio_sources.unwrap(),
            m_direct_audio_volumes: m_direct_audio_volumes.unwrap(),
            m_time_update_mode,
            m_url: m_url.unwrap(),
            m_target_material_name,
            m_enabled_audio_tracks: m_enabled_audio_tracks.unwrap(),
            m_direct_audio_mutes: m_direct_audio_mutes.unwrap(),
            m_controlled_audio_track_count: m_controlled_audio_track_count.unwrap(),
            m_play_on_awake: m_play_on_awake.unwrap(),
            m_skip_on_drop: m_skip_on_drop.unwrap(),
            m_looping: m_looping.unwrap(),
            m_wait_for_first_frame: m_wait_for_first_frame.unwrap(),
            m_frame_ready_event_enabled: m_frame_ready_event_enabled.unwrap(),
            m_video_shaders,
        })
    }
}
