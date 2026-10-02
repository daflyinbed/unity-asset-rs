//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct EffectConstant {
    pub r#type: i32,
    pub group_constant_index: u32,
    pub send_target_effect_index: u32,
    pub wet_mix_level_index: u32,
    pub prev_effect_index: u32,
    pub bypass: bool,
    pub parameter_indices: Vec<u32>,
}

impl crate::Parse for EffectConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = rdr.read_i32()?;
        let group_constant_index = rdr.read_u32()?;
        let send_target_effect_index = rdr.read_u32()?;
        let wet_mix_level_index = rdr.read_u32()?;
        let prev_effect_index = rdr.read_u32()?;
        let bypass = rdr.read_bool()?;
        rdr.align(4)?;
        let parameter_indices = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(EffectConstant {
            r#type,
            group_constant_index,
            send_target_effect_index,
            wet_mix_level_index,
            prev_effect_index,
            bypass,
            parameter_indices,
        })
    }
}

impl crate::ToValue for EffectConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("type".to_string(), self.r#type.to_value())),
            Some((
                "groupConstantIndex".to_string(),
                self.group_constant_index.to_value(),
            )),
            Some((
                "sendTargetEffectIndex".to_string(),
                self.send_target_effect_index.to_value(),
            )),
            Some((
                "wetMixLevelIndex".to_string(),
                self.wet_mix_level_index.to_value(),
            )),
            Some((
                "prevEffectIndex".to_string(),
                self.prev_effect_index.to_value(),
            )),
            Some(("bypass".to_string(), self.bypass.to_value())),
            Some((
                "parameterIndices".to_string(),
                self.parameter_indices.to_value(),
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
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "groupConstantIndex" => {
                    (self.group_constant_index).inspect_value(path, offset, limit)
                }
                "sendTargetEffectIndex" => {
                    (self.send_target_effect_index).inspect_value(path, offset, limit)
                }
                "wetMixLevelIndex" => (self.wet_mix_level_index).inspect_value(path, offset, limit),
                "prevEffectIndex" => (self.prev_effect_index).inspect_value(path, offset, limit),
                "bypass" => (self.bypass).inspect_value(path, offset, limit),
                "parameterIndices" => (self.parameter_indices).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.group_constant_index).inspect_value(path, offset, limit))
                .map(|v| ("groupConstantIndex".to_string(), v)),
            ((self.send_target_effect_index).inspect_value(path, offset, limit))
                .map(|v| ("sendTargetEffectIndex".to_string(), v)),
            ((self.wet_mix_level_index).inspect_value(path, offset, limit))
                .map(|v| ("wetMixLevelIndex".to_string(), v)),
            ((self.prev_effect_index).inspect_value(path, offset, limit))
                .map(|v| ("prevEffectIndex".to_string(), v)),
            ((self.bypass).inspect_value(path, offset, limit)).map(|v| ("bypass".to_string(), v)),
            ((self.parameter_indices).inspect_value(path, offset, limit))
                .map(|v| ("parameterIndices".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
