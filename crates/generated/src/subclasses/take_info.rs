//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TakeInfo {
    pub name: String,
    pub default_clip_name: String,
    pub start_time: f32,
    pub internal_id: Option<i64>,
    pub stop_time: f32,
    pub bake_start_time: f32,
    pub bake_stop_time: f32,
    pub sample_rate: f32,
    pub clip: PPtr,
}

impl crate::Parse for TakeInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut default_clip_name = None;
        let mut start_time = None;
        let mut internal_id = None;
        let mut stop_time = None;
        let mut bake_start_time = None;
        let mut bake_stop_time = None;
        let mut sample_rate = None;
        let mut clip = None;
        name = Some(rdr.read_aligned_string()?);
        default_clip_name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            internal_id = Some(rdr.read_i64()?);
        } else {
        }
        start_time = Some(rdr.read_f32()?);
        stop_time = Some(rdr.read_f32()?);
        bake_start_time = Some(rdr.read_f32()?);
        bake_stop_time = Some(rdr.read_f32()?);
        sample_rate = Some(rdr.read_f32()?);
        clip = Some(crate::PPtr::parse(rdr, ver)?);

        Ok(TakeInfo {
            name: name.unwrap(),
            default_clip_name: default_clip_name.unwrap(),
            start_time: start_time.unwrap(),
            internal_id,
            stop_time: stop_time.unwrap(),
            bake_start_time: bake_start_time.unwrap(),
            bake_stop_time: bake_stop_time.unwrap(),
            sample_rate: sample_rate.unwrap(),
            clip: clip.unwrap(),
        })
    }
}

impl crate::ToValue for TakeInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some((
                "defaultClipName".to_string(),
                self.default_clip_name.to_value(),
            )),
            Some(("startTime".to_string(), self.start_time.to_value())),
            crate::opt_field("internalID", &self.internal_id),
            Some(("stopTime".to_string(), self.stop_time.to_value())),
            Some(("bakeStartTime".to_string(), self.bake_start_time.to_value())),
            Some(("bakeStopTime".to_string(), self.bake_stop_time.to_value())),
            Some(("sampleRate".to_string(), self.sample_rate.to_value())),
            Some(("clip".to_string(), self.clip.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "defaultClipName" => (self.default_clip_name).inspect_value(path, offset, limit),
                "startTime" => (self.start_time).inspect_value(path, offset, limit),
                "internalID" => (self.internal_id).inspect_value(path, offset, limit),
                "stopTime" => (self.stop_time).inspect_value(path, offset, limit),
                "bakeStartTime" => (self.bake_start_time).inspect_value(path, offset, limit),
                "bakeStopTime" => (self.bake_stop_time).inspect_value(path, offset, limit),
                "sampleRate" => (self.sample_rate).inspect_value(path, offset, limit),
                "clip" => (self.clip).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.default_clip_name).inspect_value(path, offset, limit))
                .map(|v| ("defaultClipName".to_string(), v)),
            ((self.start_time).inspect_value(path, offset, limit))
                .map(|v| ("startTime".to_string(), v)),
            ((self.internal_id).inspect_value(path, offset, limit))
                .map(|v| ("internalID".to_string(), v)),
            ((self.stop_time).inspect_value(path, offset, limit))
                .map(|v| ("stopTime".to_string(), v)),
            ((self.bake_start_time).inspect_value(path, offset, limit))
                .map(|v| ("bakeStartTime".to_string(), v)),
            ((self.bake_stop_time).inspect_value(path, offset, limit))
                .map(|v| ("bakeStopTime".to_string(), v)),
            ((self.sample_rate).inspect_value(path, offset, limit))
                .map(|v| ("sampleRate".to_string(), v)),
            ((self.clip).inspect_value(path, offset, limit)).map(|v| ("clip".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
