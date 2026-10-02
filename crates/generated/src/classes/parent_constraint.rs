//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::constraint_source::ConstraintSource;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ParentConstraint {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_weight: f32,
    pub m_translation_at_rest: Vector3f,
    pub m_rotation_at_rest: Vector3f,
    pub m_translation_offsets: Vec<Vector3f>,
    pub m_rotation_offsets: Vec<Vector3f>,
    pub m_affect_translation_x: bool,
    pub m_affect_translation_y: bool,
    pub m_affect_translation_z: bool,
    pub m_affect_rotation_x: bool,
    pub m_affect_rotation_y: bool,
    pub m_affect_rotation_z: bool,
    pub m_is_contraint_active: Option<bool>,
    pub m_active: Option<bool>,
    pub m_sources: Vec<ConstraintSource>,
}

impl crate::Parse for ParentConstraint {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_weight = None;
        let mut m_translation_at_rest = None;
        let mut m_rotation_at_rest = None;
        let mut m_translation_offsets = None;
        let mut m_rotation_offsets = None;
        let mut m_affect_translation_x = None;
        let mut m_affect_translation_y = None;
        let mut m_affect_translation_z = None;
        let mut m_affect_rotation_x = None;
        let mut m_affect_rotation_y = None;
        let mut m_affect_rotation_z = None;
        let mut m_is_contraint_active = None;
        let mut m_active = None;
        let mut m_sources = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_weight = Some(rdr.read_f32()?);
        m_translation_at_rest = Some(Vector3f::parse(rdr, ver)?);
        m_rotation_at_rest = Some(Vector3f::parse(rdr, ver)?);
        m_translation_offsets = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_rotation_offsets = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Vector3f::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        m_affect_translation_x = Some(rdr.read_bool()?);
        m_affect_translation_y = Some(rdr.read_bool()?);
        m_affect_translation_z = Some(rdr.read_bool()?);
        m_affect_rotation_x = Some(rdr.read_bool()?);
        m_affect_rotation_y = Some(rdr.read_bool()?);
        m_affect_rotation_z = Some(rdr.read_bool()?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            m_active = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
            m_is_contraint_active = Some(rdr.read_bool()?);
            rdr.align(4)?;
        }
        m_sources = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ConstraintSource::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;

        Ok(ParentConstraint {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_weight: m_weight.unwrap(),
            m_translation_at_rest: m_translation_at_rest.unwrap(),
            m_rotation_at_rest: m_rotation_at_rest.unwrap(),
            m_translation_offsets: m_translation_offsets.unwrap(),
            m_rotation_offsets: m_rotation_offsets.unwrap(),
            m_affect_translation_x: m_affect_translation_x.unwrap(),
            m_affect_translation_y: m_affect_translation_y.unwrap(),
            m_affect_translation_z: m_affect_translation_z.unwrap(),
            m_affect_rotation_x: m_affect_rotation_x.unwrap(),
            m_affect_rotation_y: m_affect_rotation_y.unwrap(),
            m_affect_rotation_z: m_affect_rotation_z.unwrap(),
            m_is_contraint_active,
            m_active,
            m_sources: m_sources.unwrap(),
        })
    }
}

impl crate::ToValue for ParentConstraint {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_Weight".to_string(), self.m_weight.to_value())),
            Some((
                "m_TranslationAtRest".to_string(),
                self.m_translation_at_rest.to_value(),
            )),
            Some((
                "m_RotationAtRest".to_string(),
                self.m_rotation_at_rest.to_value(),
            )),
            Some((
                "m_TranslationOffsets".to_string(),
                self.m_translation_offsets.to_value(),
            )),
            Some((
                "m_RotationOffsets".to_string(),
                self.m_rotation_offsets.to_value(),
            )),
            Some((
                "m_AffectTranslationX".to_string(),
                self.m_affect_translation_x.to_value(),
            )),
            Some((
                "m_AffectTranslationY".to_string(),
                self.m_affect_translation_y.to_value(),
            )),
            Some((
                "m_AffectTranslationZ".to_string(),
                self.m_affect_translation_z.to_value(),
            )),
            Some((
                "m_AffectRotationX".to_string(),
                self.m_affect_rotation_x.to_value(),
            )),
            Some((
                "m_AffectRotationY".to_string(),
                self.m_affect_rotation_y.to_value(),
            )),
            Some((
                "m_AffectRotationZ".to_string(),
                self.m_affect_rotation_z.to_value(),
            )),
            crate::opt_field("m_IsContraintActive", &self.m_is_contraint_active),
            crate::opt_field("m_Active", &self.m_active),
            Some(("m_Sources".to_string(), self.m_sources.to_value())),
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
                "m_Weight" => (self.m_weight).inspect_value(path, offset, limit),
                "m_TranslationAtRest" => {
                    (self.m_translation_at_rest).inspect_value(path, offset, limit)
                }
                "m_RotationAtRest" => (self.m_rotation_at_rest).inspect_value(path, offset, limit),
                "m_TranslationOffsets" => {
                    (self.m_translation_offsets).inspect_value(path, offset, limit)
                }
                "m_RotationOffsets" => (self.m_rotation_offsets).inspect_value(path, offset, limit),
                "m_AffectTranslationX" => {
                    (self.m_affect_translation_x).inspect_value(path, offset, limit)
                }
                "m_AffectTranslationY" => {
                    (self.m_affect_translation_y).inspect_value(path, offset, limit)
                }
                "m_AffectTranslationZ" => {
                    (self.m_affect_translation_z).inspect_value(path, offset, limit)
                }
                "m_AffectRotationX" => {
                    (self.m_affect_rotation_x).inspect_value(path, offset, limit)
                }
                "m_AffectRotationY" => {
                    (self.m_affect_rotation_y).inspect_value(path, offset, limit)
                }
                "m_AffectRotationZ" => {
                    (self.m_affect_rotation_z).inspect_value(path, offset, limit)
                }
                "m_IsContraintActive" => {
                    (self.m_is_contraint_active).inspect_value(path, offset, limit)
                }
                "m_Active" => (self.m_active).inspect_value(path, offset, limit),
                "m_Sources" => (self.m_sources).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_weight).inspect_value(path, offset, limit))
                .map(|v| ("m_Weight".to_string(), v)),
            ((self.m_translation_at_rest).inspect_value(path, offset, limit))
                .map(|v| ("m_TranslationAtRest".to_string(), v)),
            ((self.m_rotation_at_rest).inspect_value(path, offset, limit))
                .map(|v| ("m_RotationAtRest".to_string(), v)),
            ((self.m_translation_offsets).inspect_value(path, offset, limit))
                .map(|v| ("m_TranslationOffsets".to_string(), v)),
            ((self.m_rotation_offsets).inspect_value(path, offset, limit))
                .map(|v| ("m_RotationOffsets".to_string(), v)),
            ((self.m_affect_translation_x).inspect_value(path, offset, limit))
                .map(|v| ("m_AffectTranslationX".to_string(), v)),
            ((self.m_affect_translation_y).inspect_value(path, offset, limit))
                .map(|v| ("m_AffectTranslationY".to_string(), v)),
            ((self.m_affect_translation_z).inspect_value(path, offset, limit))
                .map(|v| ("m_AffectTranslationZ".to_string(), v)),
            ((self.m_affect_rotation_x).inspect_value(path, offset, limit))
                .map(|v| ("m_AffectRotationX".to_string(), v)),
            ((self.m_affect_rotation_y).inspect_value(path, offset, limit))
                .map(|v| ("m_AffectRotationY".to_string(), v)),
            ((self.m_affect_rotation_z).inspect_value(path, offset, limit))
                .map(|v| ("m_AffectRotationZ".to_string(), v)),
            ((self.m_is_contraint_active).inspect_value(path, offset, limit))
                .map(|v| ("m_IsContraintActive".to_string(), v)),
            ((self.m_active).inspect_value(path, offset, limit))
                .map(|v| ("m_Active".to_string(), v)),
            ((self.m_sources).inspect_value(path, offset, limit))
                .map(|v| ("m_Sources".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
