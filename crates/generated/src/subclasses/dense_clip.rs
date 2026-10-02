//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct DenseClip {
    pub m_frame_count: i32,
    pub m_curve_count: u32,
    pub m_sample_rate: f32,
    pub m_begin_time: f32,
    pub m_sample_array: Vec<f32>,
}

impl crate::Parse for DenseClip {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_frame_count = rdr.read_i32()?;
        let m_curve_count = rdr.read_u32()?;
        let m_sample_rate = rdr.read_f32()?;
        let m_begin_time = rdr.read_f32()?;
        let m_sample_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_f32())
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(DenseClip {
            m_frame_count,
            m_curve_count,
            m_sample_rate,
            m_begin_time,
            m_sample_array,
        })
    }
}

impl crate::ToValue for DenseClip {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_FrameCount".to_string(), self.m_frame_count.to_value())),
            Some(("m_CurveCount".to_string(), self.m_curve_count.to_value())),
            Some(("m_SampleRate".to_string(), self.m_sample_rate.to_value())),
            Some(("m_BeginTime".to_string(), self.m_begin_time.to_value())),
            Some(("m_SampleArray".to_string(), self.m_sample_array.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_FrameCount" => (self.m_frame_count).inspect_value(path, offset, limit),
                "m_CurveCount" => (self.m_curve_count).inspect_value(path, offset, limit),
                "m_SampleRate" => (self.m_sample_rate).inspect_value(path, offset, limit),
                "m_BeginTime" => (self.m_begin_time).inspect_value(path, offset, limit),
                "m_SampleArray" => (self.m_sample_array).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_frame_count).inspect_value(path, offset, limit))
                .map(|v| ("m_FrameCount".to_string(), v)),
            ((self.m_curve_count).inspect_value(path, offset, limit))
                .map(|v| ("m_CurveCount".to_string(), v)),
            ((self.m_sample_rate).inspect_value(path, offset, limit))
                .map(|v| ("m_SampleRate".to_string(), v)),
            ((self.m_begin_time).inspect_value(path, offset, limit))
                .map(|v| ("m_BeginTime".to_string(), v)),
            ((self.m_sample_array).inspect_value(path, offset, limit))
                .map(|v| ("m_SampleArray".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
