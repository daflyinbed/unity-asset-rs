//! 自动生成。请勿手动修改。

/// 版本枚举：StreamedClipCurveCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StreamedClipCurveCountVariant {
    Default(u32),
    V2022_3_19(u16),
    V2023_1(u32),
    V2023_2_8(u16),
}

impl crate::Parse for StreamedClipCurveCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 8) {
            let v = rdr.read_u16()?;
            Ok(Self::V2023_2_8(v))
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_u32()?;
            Ok(Self::V2023_1(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 19) {
            let v = rdr.read_u16()?;
            Ok(Self::V2022_3_19(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for StreamedClipCurveCountVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2022_3_19(v) => v.to_value(),
            Self::V2023_1(v) => v.to_value(),
            Self::V2023_2_8(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2022_3_19(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2023_2_8(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct StreamedClip {
    pub data: Vec<u32>,
    pub curve_count: StreamedClipCurveCountVariant,
    pub discrete_curve_count: Option<u16>,
}

impl crate::Parse for StreamedClip {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut data = None;
        let mut curve_count = None;
        let mut discrete_curve_count = None;
        data = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        curve_count = Some(StreamedClipCurveCountVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 8) {
            discrete_curve_count = Some(rdr.read_u16()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2022, 3, 19) {
            discrete_curve_count = Some(rdr.read_u16()?);
        } else {
        }

        Ok(StreamedClip {
            data: data.unwrap(),
            curve_count: curve_count.unwrap(),
            discrete_curve_count,
        })
    }
}

impl crate::ToValue for StreamedClip {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("data".to_string(), self.data.to_value())),
            Some(("curveCount".to_string(), self.curve_count.to_value())),
            crate::opt_field("discreteCurveCount", &self.discrete_curve_count),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "data" => (self.data).inspect_value(path, offset, limit),
                "curveCount" => (self.curve_count).inspect_value(path, offset, limit),
                "discreteCurveCount" => {
                    (self.discrete_curve_count).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.data).inspect_value(path, offset, limit)).map(|v| ("data".to_string(), v)),
            ((self.curve_count).inspect_value(path, offset, limit))
                .map(|v| ("curveCount".to_string(), v)),
            ((self.discrete_curve_count).inspect_value(path, offset, limit))
                .map(|v| ("discreteCurveCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
