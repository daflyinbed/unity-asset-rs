//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AudioChorusFilter {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_dry_mix: f32,
    pub m_wet_mix1: f32,
    pub m_wet_mix2: f32,
    pub m_wet_mix3: f32,
    pub m_delay: f32,
    pub m_rate: f32,
    pub m_depth: f32,
    pub m_feed_back: Option<f32>,
}

impl crate::Parse for AudioChorusFilter {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_game_object = None;
        let mut m_enabled = None;
        let mut m_dry_mix = None;
        let mut m_wet_mix1 = None;
        let mut m_wet_mix2 = None;
        let mut m_wet_mix3 = None;
        let mut m_delay = None;
        let mut m_rate = None;
        let mut m_depth = None;
        let mut m_feed_back = None;
        m_game_object = Some(crate::PPtr::parse(rdr, ver)?);
        m_enabled = Some(rdr.read_u8()?);
        rdr.align(4)?;
        m_dry_mix = Some(rdr.read_f32()?);
        m_wet_mix1 = Some(rdr.read_f32()?);
        m_wet_mix2 = Some(rdr.read_f32()?);
        m_wet_mix3 = Some(rdr.read_f32()?);
        m_delay = Some(rdr.read_f32()?);
        m_rate = Some(rdr.read_f32()?);
        m_depth = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(4, 2, 0) {
        } else {
            m_feed_back = Some(rdr.read_f32()?);
        }

        Ok(AudioChorusFilter {
            m_game_object: m_game_object.unwrap(),
            m_enabled: m_enabled.unwrap(),
            m_dry_mix: m_dry_mix.unwrap(),
            m_wet_mix1: m_wet_mix1.unwrap(),
            m_wet_mix2: m_wet_mix2.unwrap(),
            m_wet_mix3: m_wet_mix3.unwrap(),
            m_delay: m_delay.unwrap(),
            m_rate: m_rate.unwrap(),
            m_depth: m_depth.unwrap(),
            m_feed_back,
        })
    }
}

impl crate::ToValue for AudioChorusFilter {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some(("m_Enabled".to_string(), self.m_enabled.to_value())),
            Some(("m_DryMix".to_string(), self.m_dry_mix.to_value())),
            Some(("m_WetMix1".to_string(), self.m_wet_mix1.to_value())),
            Some(("m_WetMix2".to_string(), self.m_wet_mix2.to_value())),
            Some(("m_WetMix3".to_string(), self.m_wet_mix3.to_value())),
            Some(("m_Delay".to_string(), self.m_delay.to_value())),
            Some(("m_Rate".to_string(), self.m_rate.to_value())),
            Some(("m_Depth".to_string(), self.m_depth.to_value())),
            crate::opt_field("m_FeedBack", &self.m_feed_back),
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
                "m_DryMix" => (self.m_dry_mix).inspect_value(path, offset, limit),
                "m_WetMix1" => (self.m_wet_mix1).inspect_value(path, offset, limit),
                "m_WetMix2" => (self.m_wet_mix2).inspect_value(path, offset, limit),
                "m_WetMix3" => (self.m_wet_mix3).inspect_value(path, offset, limit),
                "m_Delay" => (self.m_delay).inspect_value(path, offset, limit),
                "m_Rate" => (self.m_rate).inspect_value(path, offset, limit),
                "m_Depth" => (self.m_depth).inspect_value(path, offset, limit),
                "m_FeedBack" => (self.m_feed_back).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.m_enabled).inspect_value(path, offset, limit))
                .map(|v| ("m_Enabled".to_string(), v)),
            ((self.m_dry_mix).inspect_value(path, offset, limit))
                .map(|v| ("m_DryMix".to_string(), v)),
            ((self.m_wet_mix1).inspect_value(path, offset, limit))
                .map(|v| ("m_WetMix1".to_string(), v)),
            ((self.m_wet_mix2).inspect_value(path, offset, limit))
                .map(|v| ("m_WetMix2".to_string(), v)),
            ((self.m_wet_mix3).inspect_value(path, offset, limit))
                .map(|v| ("m_WetMix3".to_string(), v)),
            ((self.m_delay).inspect_value(path, offset, limit)).map(|v| ("m_Delay".to_string(), v)),
            ((self.m_rate).inspect_value(path, offset, limit)).map(|v| ("m_Rate".to_string(), v)),
            ((self.m_depth).inspect_value(path, offset, limit)).map(|v| ("m_Depth".to_string(), v)),
            ((self.m_feed_back).inspect_value(path, offset, limit))
                .map(|v| ("m_FeedBack".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
