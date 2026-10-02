//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GroupConstant {
    pub parent_constant_index: i32,
    pub volume_index: u32,
    pub pitch_index: u32,
    pub mute: bool,
    pub send_index: Option<u32>,
    pub solo: bool,
    pub bypass_effects: bool,
}

impl crate::Parse for GroupConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut parent_constant_index = None;
        let mut volume_index = None;
        let mut pitch_index = None;
        let mut mute = None;
        let mut send_index = None;
        let mut solo = None;
        let mut bypass_effects = None;
        parent_constant_index = Some(rdr.read_i32()?);
        volume_index = Some(rdr.read_u32()?);
        pitch_index = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(2021, 2, 1) {
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            send_index = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 1, 26) {
        } else if *ver >= crate::UnityVersion::new(2021, 1, 0) {
            send_index = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2020, 3, 21) {
        } else if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            send_index = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2019, 4, 32) {
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            send_index = Some(rdr.read_u32()?);
        } else {
        }
        mute = Some(rdr.read_bool()?);
        solo = Some(rdr.read_bool()?);
        bypass_effects = Some(rdr.read_bool()?);
        rdr.align(4)?;

        Ok(GroupConstant {
            parent_constant_index: parent_constant_index.unwrap(),
            volume_index: volume_index.unwrap(),
            pitch_index: pitch_index.unwrap(),
            mute: mute.unwrap(),
            send_index,
            solo: solo.unwrap(),
            bypass_effects: bypass_effects.unwrap(),
        })
    }
}

impl crate::ToValue for GroupConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "parentConstantIndex".to_string(),
                self.parent_constant_index.to_value(),
            )),
            Some(("volumeIndex".to_string(), self.volume_index.to_value())),
            Some(("pitchIndex".to_string(), self.pitch_index.to_value())),
            Some(("mute".to_string(), self.mute.to_value())),
            crate::opt_field("sendIndex", &self.send_index),
            Some(("solo".to_string(), self.solo.to_value())),
            Some(("bypassEffects".to_string(), self.bypass_effects.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "parentConstantIndex" => {
                    (self.parent_constant_index).inspect_value(path, offset, limit)
                }
                "volumeIndex" => (self.volume_index).inspect_value(path, offset, limit),
                "pitchIndex" => (self.pitch_index).inspect_value(path, offset, limit),
                "mute" => (self.mute).inspect_value(path, offset, limit),
                "sendIndex" => (self.send_index).inspect_value(path, offset, limit),
                "solo" => (self.solo).inspect_value(path, offset, limit),
                "bypassEffects" => (self.bypass_effects).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.parent_constant_index).inspect_value(path, offset, limit))
                .map(|v| ("parentConstantIndex".to_string(), v)),
            ((self.volume_index).inspect_value(path, offset, limit))
                .map(|v| ("volumeIndex".to_string(), v)),
            ((self.pitch_index).inspect_value(path, offset, limit))
                .map(|v| ("pitchIndex".to_string(), v)),
            ((self.mute).inspect_value(path, offset, limit)).map(|v| ("mute".to_string(), v)),
            ((self.send_index).inspect_value(path, offset, limit))
                .map(|v| ("sendIndex".to_string(), v)),
            ((self.solo).inspect_value(path, offset, limit)).map(|v| ("solo".to_string(), v)),
            ((self.bypass_effects).inspect_value(path, offset, limit))
                .map(|v| ("bypassEffects".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
