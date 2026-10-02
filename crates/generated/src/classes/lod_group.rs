//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::lod::Lod;
use crate::subclasses::vector3f::Vector3f;

/// 版本枚举：LodGroupMLoDsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LodGroupMLoDsVariant {
    Default(Vec<Lod>),
    V2017_1(Vec<Lod>),
}

impl crate::Parse for LodGroupMLoDsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Lod::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Lod::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for LodGroupMLoDsVariant {
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

/// 版本枚举：LodGroupMEnabledVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LodGroupMEnabledVariant {
    Default(bool),
    V5_0(bool),
}

impl crate::Parse for LodGroupMEnabledVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for LodGroupMEnabledVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：LodGroupMAnimateCrossFadingVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum LodGroupMAnimateCrossFadingVariant {
    Default(bool),
    V2018_3(bool),
}

impl crate::Parse for LodGroupMAnimateCrossFadingVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for LodGroupMAnimateCrossFadingVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct LodGroup {
    pub m_game_object: PPtr,
    pub m_local_reference_point: Vector3f,
    pub m_size: f32,
    pub m_screen_relative_transition_height: Option<f32>,
    pub m_lo_ds: LodGroupMLoDsVariant,
    pub m_fade_mode: Option<i32>,
    pub m_enabled: LodGroupMEnabledVariant,
    pub m_animate_cross_fading: Option<LodGroupMAnimateCrossFadingVariant>,
    pub m_last_lod_is_billboard: Option<bool>,
    pub m_global_illumination_lod: Option<i32>,
}

impl crate::Parse for LodGroup {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_local_reference_point = None;
        let mut m_size = None;
        let mut m_screen_relative_transition_height = None;
        let mut m_lo_ds = None;
        let mut m_fade_mode = None;
        let mut m_enabled = None;
        let mut m_animate_cross_fading = None;
        let mut m_last_lod_is_billboard = None;
        let mut m_global_illumination_lod = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_local_reference_point = Some(Vector3f::parse(rdr, ver)?);
        m_size = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_fade_mode = Some(rdr.read_i32()?);
            m_animate_cross_fading = Some(LodGroupMAnimateCrossFadingVariant::parse(rdr, ver)?);
            m_last_lod_is_billboard = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 1, 0) {
            m_fade_mode = Some(rdr.read_i32()?);
            m_animate_cross_fading = Some(LodGroupMAnimateCrossFadingVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
        } else {
            m_screen_relative_transition_height = Some(rdr.read_f32()?);
        }
        m_lo_ds = Some(LodGroupMLoDsVariant::parse(rdr, ver)?);
        m_enabled = Some(LodGroupMEnabledVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            m_global_illumination_lod = Some(rdr.read_i32()?);
        } else {
        }

        Ok(LodGroup {
            m_game_object: m_game_object.unwrap(),
            m_local_reference_point: m_local_reference_point.unwrap(),
            m_size: m_size.unwrap(),
            m_screen_relative_transition_height,
            m_lo_ds: m_lo_ds.unwrap(),
            m_fade_mode,
            m_enabled: m_enabled.unwrap(),
            m_animate_cross_fading,
            m_last_lod_is_billboard,
            m_global_illumination_lod,
        })
    }
}

impl crate::ToValue for LodGroup {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some((
                "m_LocalReferencePoint".to_string(),
                self.m_local_reference_point.to_value(),
            )),
            Some(("m_Size".to_string(), self.m_size.to_value())),
            crate::opt_field(
                "m_ScreenRelativeTransitionHeight",
                &self.m_screen_relative_transition_height,
            ),
            Some(("m_LODs".to_string(), self.m_lo_ds.to_value())),
            crate::opt_field("m_FadeMode", &self.m_fade_mode),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            crate::opt_field("m_AnimateCrossFading", &self.m_animate_cross_fading),
            crate::opt_field("m_LastLODIsBillboard", &self.m_last_lod_is_billboard),
            crate::opt_field("m_GlobalIlluminationLOD", &self.m_global_illumination_lod),
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
                "m_LocalReferencePoint" => {
                    (self.m_local_reference_point).inspect_value(path, offset, limit)
                }
                "m_Size" => (self.m_size).inspect_value(path, offset, limit),
                "m_ScreenRelativeTransitionHeight" => {
                    (self.m_screen_relative_transition_height).inspect_value(path, offset, limit)
                }
                "m_LODs" => (self.m_lo_ds).inspect_value(path, offset, limit),
                "m_FadeMode" => (self.m_fade_mode).inspect_value(path, offset, limit),
                "m_Enabled" => (self.m_enabled).inspect_value(path, offset, limit),
                "m_AnimateCrossFading" => {
                    (self.m_animate_cross_fading).inspect_value(path, offset, limit)
                }
                "m_LastLODIsBillboard" => {
                    (self.m_last_lod_is_billboard).inspect_value(path, offset, limit)
                }
                "m_GlobalIlluminationLOD" => {
                    (self.m_global_illumination_lod).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_local_reference_point).inspect_value(path, offset, limit))
                .map(|v| ("m_LocalReferencePoint".to_string(), v)),
            ((self.m_size).inspect_value(path, offset, limit)).map(|v| ("m_Size".to_string(), v)),
            ((self.m_screen_relative_transition_height).inspect_value(path, offset, limit))
                .map(|v| ("m_ScreenRelativeTransitionHeight".to_string(), v)),
            ((self.m_lo_ds).inspect_value(path, offset, limit)).map(|v| ("m_LODs".to_string(), v)),
            ((self.m_fade_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_FadeMode".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_animate_cross_fading).inspect_value(path, offset, limit))
                .map(|v| ("m_AnimateCrossFading".to_string(), v)),
            ((self.m_last_lod_is_billboard).inspect_value(path, offset, limit))
                .map(|v| ("m_LastLODIsBillboard".to_string(), v)),
            ((self.m_global_illumination_lod).inspect_value(path, offset, limit))
                .map(|v| ("m_GlobalIlluminationLOD".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
