//! 自动生成。请勿手动修改。

use crate::PPtr;

/// 版本枚举：VideoPlayerMTargetAudioSourcesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VideoPlayerMTargetAudioSourcesVariant {
    Default(Vec<PPtr>),
    V2017_1(Vec<PPtr>),
}

impl crate::Parse for VideoPlayerMTargetAudioSourcesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| crate::PPtr::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VideoPlayerMTargetAudioSourcesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VideoPlayerMDirectAudioVolumesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VideoPlayerMDirectAudioVolumesVariant {
    Default(Vec<f32>),
    V2017_1(Vec<f32>),
}

impl crate::Parse for VideoPlayerMDirectAudioVolumesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VideoPlayerMDirectAudioVolumesVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VideoPlayerMFrameReadyEventEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VideoPlayerMFrameReadyEventEnabledVariant {
    Default(bool),
    V2020_1(bool),
}

impl crate::Parse for VideoPlayerMFrameReadyEventEnabledVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VideoPlayerMFrameReadyEventEnabledVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
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
    pub m_target_material_property: String,
    pub m_render_mode: i32,
    pub m_aspect_ratio: i32,
    pub m_data_source: i32,
    pub m_playback_speed: f32,
    pub m_audio_output_mode: i32,
    pub m_target_audio_sources: VideoPlayerMTargetAudioSourcesVariant,
    pub m_time_update_mode: Option<i32>,
    pub m_direct_audio_volumes: VideoPlayerMDirectAudioVolumesVariant,
    pub m_url: String,
    pub m_enabled_audio_tracks: Vec<bool>,
    pub m_direct_audio_mutes: Vec<bool>,
    pub m_controlled_audio_track_count: u16,
    pub m_play_on_awake: bool,
    pub m_skip_on_drop: bool,
    pub m_looping: bool,
    pub m_wait_for_first_frame: bool,
    pub m_frame_ready_event_enabled: VideoPlayerMFrameReadyEventEnabledVariant,
    pub m_video_shaders: Option<Vec<PPtr>>,
}

impl crate::Parse for VideoPlayer {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
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
        let mut m_target_material_property = None;
        let mut m_render_mode = None;
        let mut m_aspect_ratio = None;
        let mut m_data_source = None;
        let mut m_playback_speed = None;
        let mut m_audio_output_mode = None;
        let mut m_target_audio_sources = None;
        let mut m_time_update_mode = None;
        let mut m_direct_audio_volumes = None;
        let mut m_url = None;
        let mut m_enabled_audio_tracks = None;
        let mut m_direct_audio_mutes = None;
        let mut m_controlled_audio_track_count = None;
        let mut m_play_on_awake = None;
        let mut m_skip_on_drop = None;
        let mut m_looping = None;
        let mut m_wait_for_first_frame = None;
        let mut m_frame_ready_event_enabled = None;
        let mut m_video_shaders = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_video_clip = Some(crate::PPtr::parse(rdr, ver)?);
        m_target_camera_alpha = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            m_target_camera3_d_layout = Some(rdr.read_i32()?);
        } else {
        }
        m_target_camera = Some(crate::PPtr::parse(rdr, ver)?);
        m_target_texture = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_time_reference = Some(rdr.read_i32()?);
        } else {
        }
        m_target_material_renderer = Some(crate::PPtr::parse(rdr, ver)?);
        m_target_material_property = Some(rdr.read_aligned_string()?);
        m_render_mode = Some(rdr.read_i32()?);
        m_aspect_ratio = Some(rdr.read_i32()?);
        m_data_source = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            m_time_update_mode = Some(rdr.read_i32()?);
        } else {
        }
        m_playback_speed = Some(rdr.read_f32()?);
        m_audio_output_mode = Some(rdr.read_i32()?);
        m_target_audio_sources = Some(VideoPlayerMTargetAudioSourcesVariant::parse(rdr, ver)?);
        m_direct_audio_volumes = Some(VideoPlayerMDirectAudioVolumesVariant::parse(rdr, ver)?);
        m_url = Some(rdr.read_aligned_string()?);
        m_enabled_audio_tracks = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_bool())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_direct_audio_mutes = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_bool())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_controlled_audio_track_count = Some(rdr.read_u16()?);
        m_play_on_awake = Some(rdr.read_bool()?);
        m_skip_on_drop = Some(rdr.read_bool()?);
        m_looping = Some(rdr.read_bool()?);
        m_wait_for_first_frame = Some(rdr.read_bool()?);
        m_frame_ready_event_enabled =
            Some(VideoPlayerMFrameReadyEventEnabledVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            m_video_shaders = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
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
            m_target_material_property: m_target_material_property.unwrap(),
            m_render_mode: m_render_mode.unwrap(),
            m_aspect_ratio: m_aspect_ratio.unwrap(),
            m_data_source: m_data_source.unwrap(),
            m_playback_speed: m_playback_speed.unwrap(),
            m_audio_output_mode: m_audio_output_mode.unwrap(),
            m_target_audio_sources: m_target_audio_sources.unwrap(),
            m_time_update_mode,
            m_direct_audio_volumes: m_direct_audio_volumes.unwrap(),
            m_url: m_url.unwrap(),
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

impl crate::ToValue for VideoPlayer {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_VideoClip".to_string(), self.m_video_clip.to_value())),
            Some((
                "m_TargetCameraAlpha".to_string(),
                self.m_target_camera_alpha.to_value(),
            )),
            Some((
                "m_TargetCamera".to_string(),
                self.m_target_camera.to_value(),
            )),
            crate::opt_field("m_TargetCamera3DLayout", &self.m_target_camera3_d_layout),
            Some((
                "m_TargetTexture".to_string(),
                self.m_target_texture.to_value(),
            )),
            Some((
                "m_TargetMaterialRenderer".to_string(),
                self.m_target_material_renderer.to_value(),
            )),
            crate::opt_field("m_TimeReference", &self.m_time_reference),
            Some((
                "m_TargetMaterialProperty".to_string(),
                self.m_target_material_property.to_value(),
            )),
            Some(("m_RenderMode".to_string(), self.m_render_mode.to_value())),
            Some(("m_AspectRatio".to_string(), self.m_aspect_ratio.to_value())),
            Some(("m_DataSource".to_string(), self.m_data_source.to_value())),
            Some((
                "m_PlaybackSpeed".to_string(),
                self.m_playback_speed.to_value(),
            )),
            Some((
                "m_AudioOutputMode".to_string(),
                self.m_audio_output_mode.to_value(),
            )),
            Some((
                "m_TargetAudioSources".to_string(),
                self.m_target_audio_sources.to_value(),
            )),
            crate::opt_field("m_TimeUpdateMode", &self.m_time_update_mode),
            Some((
                "m_DirectAudioVolumes".to_string(),
                self.m_direct_audio_volumes.to_value(),
            )),
            Some(("m_Url".to_string(), self.m_url.to_value())),
            Some((
                "m_EnabledAudioTracks".to_string(),
                self.m_enabled_audio_tracks.to_value(),
            )),
            Some((
                "m_DirectAudioMutes".to_string(),
                self.m_direct_audio_mutes.to_value(),
            )),
            Some((
                "m_ControlledAudioTrackCount".to_string(),
                self.m_controlled_audio_track_count.to_value(),
            )),
            Some(("m_PlayOnAwake".to_string(), self.m_play_on_awake.to_value())),
            Some(("m_SkipOnDrop".to_string(), self.m_skip_on_drop.to_value())),
            Some(("m_Looping".to_string(), self.m_looping.to_value())),
            Some((
                "m_WaitForFirstFrame".to_string(),
                self.m_wait_for_first_frame.to_value(),
            )),
            Some((
                "m_FrameReadyEventEnabled".to_string(),
                self.m_frame_ready_event_enabled.to_value(),
            )),
            crate::opt_field("m_VideoShaders", &self.m_video_shaders),
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
                "m_VideoClip" => (self.m_video_clip).inspect_value(path, offset, limit),
                "m_TargetCameraAlpha" => {
                    (self.m_target_camera_alpha).inspect_value(path, offset, limit)
                }
                "m_TargetCamera" => (self.m_target_camera).inspect_value(path, offset, limit),
                "m_TargetCamera3DLayout" => {
                    (self.m_target_camera3_d_layout).inspect_value(path, offset, limit)
                }
                "m_TargetTexture" => (self.m_target_texture).inspect_value(path, offset, limit),
                "m_TargetMaterialRenderer" => {
                    (self.m_target_material_renderer).inspect_value(path, offset, limit)
                }
                "m_TimeReference" => (self.m_time_reference).inspect_value(path, offset, limit),
                "m_TargetMaterialProperty" => {
                    (self.m_target_material_property).inspect_value(path, offset, limit)
                }
                "m_RenderMode" => (self.m_render_mode).inspect_value(path, offset, limit),
                "m_AspectRatio" => (self.m_aspect_ratio).inspect_value(path, offset, limit),
                "m_DataSource" => (self.m_data_source).inspect_value(path, offset, limit),
                "m_PlaybackSpeed" => (self.m_playback_speed).inspect_value(path, offset, limit),
                "m_AudioOutputMode" => {
                    (self.m_audio_output_mode).inspect_value(path, offset, limit)
                }
                "m_TargetAudioSources" => {
                    (self.m_target_audio_sources).inspect_value(path, offset, limit)
                }
                "m_TimeUpdateMode" => (self.m_time_update_mode).inspect_value(path, offset, limit),
                "m_DirectAudioVolumes" => {
                    (self.m_direct_audio_volumes).inspect_value(path, offset, limit)
                }
                "m_Url" => (self.m_url).inspect_value(path, offset, limit),
                "m_EnabledAudioTracks" => {
                    (self.m_enabled_audio_tracks).inspect_value(path, offset, limit)
                }
                "m_DirectAudioMutes" => {
                    (self.m_direct_audio_mutes).inspect_value(path, offset, limit)
                }
                "m_ControlledAudioTrackCount" => {
                    (self.m_controlled_audio_track_count).inspect_value(path, offset, limit)
                }
                "m_PlayOnAwake" => (self.m_play_on_awake).inspect_value(path, offset, limit),
                "m_SkipOnDrop" => (self.m_skip_on_drop).inspect_value(path, offset, limit),
                "m_Looping" => (self.m_looping).inspect_value(path, offset, limit),
                "m_WaitForFirstFrame" => {
                    (self.m_wait_for_first_frame).inspect_value(path, offset, limit)
                }
                "m_FrameReadyEventEnabled" => {
                    (self.m_frame_ready_event_enabled).inspect_value(path, offset, limit)
                }
                "m_VideoShaders" => (self.m_video_shaders).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_video_clip).inspect_value(path, offset, limit))
                .map(|v| ("m_VideoClip".to_string(), v)),
            ((self.m_target_camera_alpha).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetCameraAlpha".to_string(), v)),
            ((self.m_target_camera).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetCamera".to_string(), v)),
            ((self.m_target_camera3_d_layout).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetCamera3DLayout".to_string(), v)),
            ((self.m_target_texture).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetTexture".to_string(), v)),
            ((self.m_target_material_renderer).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetMaterialRenderer".to_string(), v)),
            ((self.m_time_reference).inspect_value(path, offset, limit))
                .map(|v| ("m_TimeReference".to_string(), v)),
            ((self.m_target_material_property).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetMaterialProperty".to_string(), v)),
            ((self.m_render_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_RenderMode".to_string(), v)),
            ((self.m_aspect_ratio).inspect_value(path, offset, limit))
                .map(|v| ("m_AspectRatio".to_string(), v)),
            ((self.m_data_source).inspect_value(path, offset, limit))
                .map(|v| ("m_DataSource".to_string(), v)),
            ((self.m_playback_speed).inspect_value(path, offset, limit))
                .map(|v| ("m_PlaybackSpeed".to_string(), v)),
            ((self.m_audio_output_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_AudioOutputMode".to_string(), v)),
            ((self.m_target_audio_sources).inspect_value(path, offset, limit))
                .map(|v| ("m_TargetAudioSources".to_string(), v)),
            ((self.m_time_update_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_TimeUpdateMode".to_string(), v)),
            ((self.m_direct_audio_volumes).inspect_value(path, offset, limit))
                .map(|v| ("m_DirectAudioVolumes".to_string(), v)),
            ((self.m_url).inspect_value(path, offset, limit)).map(|v| ("m_Url".to_string(), v)),
            ((self.m_enabled_audio_tracks).inspect_value(path, offset, limit))
                .map(|v| ("m_EnabledAudioTracks".to_string(), v)),
            ((self.m_direct_audio_mutes).inspect_value(path, offset, limit))
                .map(|v| ("m_DirectAudioMutes".to_string(), v)),
            ((self.m_controlled_audio_track_count).inspect_value(path, offset, limit))
                .map(|v| ("m_ControlledAudioTrackCount".to_string(), v)),
            ((self.m_play_on_awake).inspect_value(path, offset, limit))
                .map(|v| ("m_PlayOnAwake".to_string(), v)),
            ((self.m_skip_on_drop).inspect_value(path, offset, limit))
                .map(|v| ("m_SkipOnDrop".to_string(), v)),
            ((self.m_looping).inspect_value(path, offset, limit))
                .map(|v| ("m_Looping".to_string(), v)),
            ((self.m_wait_for_first_frame).inspect_value(path, offset, limit))
                .map(|v| ("m_WaitForFirstFrame".to_string(), v)),
            ((self.m_frame_ready_event_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_FrameReadyEventEnabled".to_string(), v)),
            ((self.m_video_shaders).inspect_value(path, offset, limit))
                .map(|v| ("m_VideoShaders".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
