//! 自动生成。请勿手动修改。

use crate::subclasses::min_max_curve::MinMaxCurve;
use crate::subclasses::min_max_gradient::MinMaxGradient;

#[derive(Debug, Clone)]
pub struct CustomDataModule {
    pub enabled: bool,
    pub mode0: i32,
    pub vector_component_count0: i32,
    pub color0: MinMaxGradient,
    pub vector0_0: MinMaxCurve,
    pub vector0_1: MinMaxCurve,
    pub vector0_2: MinMaxCurve,
    pub vector0_3: MinMaxCurve,
    pub mode1: i32,
    pub vector_component_count1: i32,
    pub color1: MinMaxGradient,
    pub vector1_0: MinMaxCurve,
    pub vector1_1: MinMaxCurve,
    pub vector1_2: MinMaxCurve,
    pub vector1_3: MinMaxCurve,
}

impl crate::Parse for CustomDataModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let enabled = rdr.read_bool()?;
        rdr.align(4)?;
        let mode0 = rdr.read_i32()?;
        let vector_component_count0 = rdr.read_i32()?;
        let color0 = MinMaxGradient::parse(rdr, ver)?;
        let vector0_0 = MinMaxCurve::parse(rdr, ver)?;
        let vector0_1 = MinMaxCurve::parse(rdr, ver)?;
        let vector0_2 = MinMaxCurve::parse(rdr, ver)?;
        let vector0_3 = MinMaxCurve::parse(rdr, ver)?;
        let mode1 = rdr.read_i32()?;
        let vector_component_count1 = rdr.read_i32()?;
        let color1 = MinMaxGradient::parse(rdr, ver)?;
        let vector1_0 = MinMaxCurve::parse(rdr, ver)?;
        let vector1_1 = MinMaxCurve::parse(rdr, ver)?;
        let vector1_2 = MinMaxCurve::parse(rdr, ver)?;
        let vector1_3 = MinMaxCurve::parse(rdr, ver)?;

        Ok(CustomDataModule {
            enabled,
            mode0,
            vector_component_count0,
            color0,
            vector0_0,
            vector0_1,
            vector0_2,
            vector0_3,
            mode1,
            vector_component_count1,
            color1,
            vector1_0,
            vector1_1,
            vector1_2,
            vector1_3,
        })
    }
}

impl crate::ToValue for CustomDataModule {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("enabled".to_string(), self.enabled.to_value())),
            Some(("mode0".to_string(), self.mode0.to_value())),
            Some((
                "vectorComponentCount0".to_string(),
                self.vector_component_count0.to_value(),
            )),
            Some(("color0".to_string(), self.color0.to_value())),
            Some(("vector0_0".to_string(), self.vector0_0.to_value())),
            Some(("vector0_1".to_string(), self.vector0_1.to_value())),
            Some(("vector0_2".to_string(), self.vector0_2.to_value())),
            Some(("vector0_3".to_string(), self.vector0_3.to_value())),
            Some(("mode1".to_string(), self.mode1.to_value())),
            Some((
                "vectorComponentCount1".to_string(),
                self.vector_component_count1.to_value(),
            )),
            Some(("color1".to_string(), self.color1.to_value())),
            Some(("vector1_0".to_string(), self.vector1_0.to_value())),
            Some(("vector1_1".to_string(), self.vector1_1.to_value())),
            Some(("vector1_2".to_string(), self.vector1_2.to_value())),
            Some(("vector1_3".to_string(), self.vector1_3.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "enabled" => (self.enabled).inspect_value(path, offset, limit),
                "mode0" => (self.mode0).inspect_value(path, offset, limit),
                "vectorComponentCount0" => {
                    (self.vector_component_count0).inspect_value(path, offset, limit)
                }
                "color0" => (self.color0).inspect_value(path, offset, limit),
                "vector0_0" => (self.vector0_0).inspect_value(path, offset, limit),
                "vector0_1" => (self.vector0_1).inspect_value(path, offset, limit),
                "vector0_2" => (self.vector0_2).inspect_value(path, offset, limit),
                "vector0_3" => (self.vector0_3).inspect_value(path, offset, limit),
                "mode1" => (self.mode1).inspect_value(path, offset, limit),
                "vectorComponentCount1" => {
                    (self.vector_component_count1).inspect_value(path, offset, limit)
                }
                "color1" => (self.color1).inspect_value(path, offset, limit),
                "vector1_0" => (self.vector1_0).inspect_value(path, offset, limit),
                "vector1_1" => (self.vector1_1).inspect_value(path, offset, limit),
                "vector1_2" => (self.vector1_2).inspect_value(path, offset, limit),
                "vector1_3" => (self.vector1_3).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.enabled).inspect_value(path, offset, limit)).map(|v| ("enabled".to_string(), v)),
            ((self.mode0).inspect_value(path, offset, limit)).map(|v| ("mode0".to_string(), v)),
            ((self.vector_component_count0).inspect_value(path, offset, limit))
                .map(|v| ("vectorComponentCount0".to_string(), v)),
            ((self.color0).inspect_value(path, offset, limit)).map(|v| ("color0".to_string(), v)),
            ((self.vector0_0).inspect_value(path, offset, limit))
                .map(|v| ("vector0_0".to_string(), v)),
            ((self.vector0_1).inspect_value(path, offset, limit))
                .map(|v| ("vector0_1".to_string(), v)),
            ((self.vector0_2).inspect_value(path, offset, limit))
                .map(|v| ("vector0_2".to_string(), v)),
            ((self.vector0_3).inspect_value(path, offset, limit))
                .map(|v| ("vector0_3".to_string(), v)),
            ((self.mode1).inspect_value(path, offset, limit)).map(|v| ("mode1".to_string(), v)),
            ((self.vector_component_count1).inspect_value(path, offset, limit))
                .map(|v| ("vectorComponentCount1".to_string(), v)),
            ((self.color1).inspect_value(path, offset, limit)).map(|v| ("color1".to_string(), v)),
            ((self.vector1_0).inspect_value(path, offset, limit))
                .map(|v| ("vector1_0".to_string(), v)),
            ((self.vector1_1).inspect_value(path, offset, limit))
                .map(|v| ("vector1_1".to_string(), v)),
            ((self.vector1_2).inspect_value(path, offset, limit))
                .map(|v| ("vector1_2".to_string(), v)),
            ((self.vector1_3).inspect_value(path, offset, limit))
                .map(|v| ("vector1_3".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
