//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MuscleClipInfo {
    pub m_start_time: f32,
    pub m_stop_time: f32,
    pub m_orientation_offset_y: f32,
    pub m_level: f32,
    pub m_cycle_offset: f32,
    pub m_loop_blend: bool,
    pub m_loop_blend_orientation: bool,
    pub m_loop_blend_position_y: bool,
    pub m_loop_blend_position_xz: bool,
    pub m_keep_original_orientation: bool,
    pub m_keep_original_position_y: bool,
    pub m_keep_original_position_xz: bool,
    pub m_height_from_feet: bool,
    pub m_mirror: bool,
    pub m_keep_additional_bones_animation: bool,
}

impl crate::Parse for MuscleClipInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_start_time = rdr.read_f32()?;
        let m_stop_time = rdr.read_f32()?;
        let m_orientation_offset_y = rdr.read_f32()?;
        let m_level = rdr.read_f32()?;
        let m_cycle_offset = rdr.read_f32()?;
        let m_loop_blend = rdr.read_bool()?;
        let m_loop_blend_orientation = rdr.read_bool()?;
        let m_loop_blend_position_y = rdr.read_bool()?;
        let m_loop_blend_position_xz = rdr.read_bool()?;
        let m_keep_original_orientation = rdr.read_bool()?;
        let m_keep_original_position_y = rdr.read_bool()?;
        let m_keep_original_position_xz = rdr.read_bool()?;
        let m_height_from_feet = rdr.read_bool()?;
        let m_mirror = rdr.read_bool()?;
        let m_keep_additional_bones_animation = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(MuscleClipInfo {
            m_start_time,
            m_stop_time,
            m_orientation_offset_y,
            m_level,
            m_cycle_offset,
            m_loop_blend,
            m_loop_blend_orientation,
            m_loop_blend_position_y,
            m_loop_blend_position_xz,
            m_keep_original_orientation,
            m_keep_original_position_y,
            m_keep_original_position_xz,
            m_height_from_feet,
            m_mirror,
            m_keep_additional_bones_animation,
        })
    }
}

impl crate::ToValue for MuscleClipInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_StartTime".to_string(), self.m_start_time.to_value())),
            Some(("m_StopTime".to_string(), self.m_stop_time.to_value())),
            Some((
                "m_OrientationOffsetY".to_string(),
                self.m_orientation_offset_y.to_value(),
            )),
            Some(("m_Level".to_string(), self.m_level.to_value())),
            Some(("m_CycleOffset".to_string(), self.m_cycle_offset.to_value())),
            Some(("m_LoopBlend".to_string(), self.m_loop_blend.to_value())),
            Some((
                "m_LoopBlendOrientation".to_string(),
                self.m_loop_blend_orientation.to_value(),
            )),
            Some((
                "m_LoopBlendPositionY".to_string(),
                self.m_loop_blend_position_y.to_value(),
            )),
            Some((
                "m_LoopBlendPositionXZ".to_string(),
                self.m_loop_blend_position_xz.to_value(),
            )),
            Some((
                "m_KeepOriginalOrientation".to_string(),
                self.m_keep_original_orientation.to_value(),
            )),
            Some((
                "m_KeepOriginalPositionY".to_string(),
                self.m_keep_original_position_y.to_value(),
            )),
            Some((
                "m_KeepOriginalPositionXZ".to_string(),
                self.m_keep_original_position_xz.to_value(),
            )),
            Some((
                "m_HeightFromFeet".to_string(),
                self.m_height_from_feet.to_value(),
            )),
            Some(("m_Mirror".to_string(), self.m_mirror.to_value())),
            Some((
                "m_KeepAdditionalBonesAnimation".to_string(),
                self.m_keep_additional_bones_animation.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_StartTime" => (self.m_start_time).inspect_value(path, offset, limit),
                "m_StopTime" => (self.m_stop_time).inspect_value(path, offset, limit),
                "m_OrientationOffsetY" => {
                    (self.m_orientation_offset_y).inspect_value(path, offset, limit)
                }
                "m_Level" => (self.m_level).inspect_value(path, offset, limit),
                "m_CycleOffset" => (self.m_cycle_offset).inspect_value(path, offset, limit),
                "m_LoopBlend" => (self.m_loop_blend).inspect_value(path, offset, limit),
                "m_LoopBlendOrientation" => {
                    (self.m_loop_blend_orientation).inspect_value(path, offset, limit)
                }
                "m_LoopBlendPositionY" => {
                    (self.m_loop_blend_position_y).inspect_value(path, offset, limit)
                }
                "m_LoopBlendPositionXZ" => {
                    (self.m_loop_blend_position_xz).inspect_value(path, offset, limit)
                }
                "m_KeepOriginalOrientation" => {
                    (self.m_keep_original_orientation).inspect_value(path, offset, limit)
                }
                "m_KeepOriginalPositionY" => {
                    (self.m_keep_original_position_y).inspect_value(path, offset, limit)
                }
                "m_KeepOriginalPositionXZ" => {
                    (self.m_keep_original_position_xz).inspect_value(path, offset, limit)
                }
                "m_HeightFromFeet" => (self.m_height_from_feet).inspect_value(path, offset, limit),
                "m_Mirror" => (self.m_mirror).inspect_value(path, offset, limit),
                "m_KeepAdditionalBonesAnimation" => {
                    (self.m_keep_additional_bones_animation).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_start_time).inspect_value(path, offset, limit))
                .map(|v| ("m_StartTime".to_string(), v)),
            ((self.m_stop_time).inspect_value(path, offset, limit))
                .map(|v| ("m_StopTime".to_string(), v)),
            ((self.m_orientation_offset_y).inspect_value(path, offset, limit))
                .map(|v| ("m_OrientationOffsetY".to_string(), v)),
            ((self.m_level).inspect_value(path, offset, limit)).map(|v| ("m_Level".to_string(), v)),
            ((self.m_cycle_offset).inspect_value(path, offset, limit))
                .map(|v| ("m_CycleOffset".to_string(), v)),
            ((self.m_loop_blend).inspect_value(path, offset, limit))
                .map(|v| ("m_LoopBlend".to_string(), v)),
            ((self.m_loop_blend_orientation).inspect_value(path, offset, limit))
                .map(|v| ("m_LoopBlendOrientation".to_string(), v)),
            ((self.m_loop_blend_position_y).inspect_value(path, offset, limit))
                .map(|v| ("m_LoopBlendPositionY".to_string(), v)),
            ((self.m_loop_blend_position_xz).inspect_value(path, offset, limit))
                .map(|v| ("m_LoopBlendPositionXZ".to_string(), v)),
            ((self.m_keep_original_orientation).inspect_value(path, offset, limit))
                .map(|v| ("m_KeepOriginalOrientation".to_string(), v)),
            ((self.m_keep_original_position_y).inspect_value(path, offset, limit))
                .map(|v| ("m_KeepOriginalPositionY".to_string(), v)),
            ((self.m_keep_original_position_xz).inspect_value(path, offset, limit))
                .map(|v| ("m_KeepOriginalPositionXZ".to_string(), v)),
            ((self.m_height_from_feet).inspect_value(path, offset, limit))
                .map(|v| ("m_HeightFromFeet".to_string(), v)),
            ((self.m_mirror).inspect_value(path, offset, limit))
                .map(|v| ("m_Mirror".to_string(), v)),
            ((self.m_keep_additional_bones_animation).inspect_value(path, offset, limit))
                .map(|v| ("m_KeepAdditionalBonesAnimation".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
