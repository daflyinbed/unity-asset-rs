//! 自动生成。请勿手动修改。

/// 版本枚举：PreviewDataMPreviewDataVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PreviewDataMPreviewDataVariant {
    Default(Vec<f32>),
    V2017_1(Vec<f32>),
}

impl crate::Parse for PreviewDataMPreviewDataVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_f32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for PreviewDataMPreviewDataVariant {
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
#[derive(Debug, Clone)]
pub struct PreviewData {
    pub m_preview_data: PreviewDataMPreviewDataVariant,
    pub m_orig_size: i32,
    pub m_comp_size: i32,
}

impl crate::Parse for PreviewData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_preview_data = PreviewDataMPreviewDataVariant::parse(rdr, ver)?;
        let m_orig_size = rdr.read_i32()?;
        let m_comp_size = rdr.read_i32()?;

        Ok(PreviewData {
            m_preview_data,
            m_orig_size,
            m_comp_size,
        })
    }
}

impl crate::ToValue for PreviewData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_PreviewData".to_string(), self.m_preview_data.to_value())),
            Some(("m_OrigSize".to_string(), self.m_orig_size.to_value())),
            Some(("m_CompSize".to_string(), self.m_comp_size.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_PreviewData" => (self.m_preview_data).inspect_value(path, offset, limit),
                "m_OrigSize" => (self.m_orig_size).inspect_value(path, offset, limit),
                "m_CompSize" => (self.m_comp_size).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_preview_data).inspect_value(path, offset, limit))
                .map(|v| ("m_PreviewData".to_string(), v)),
            ((self.m_orig_size).inspect_value(path, offset, limit))
                .map(|v| ("m_OrigSize".to_string(), v)),
            ((self.m_comp_size).inspect_value(path, offset, limit))
                .map(|v| ("m_CompSize".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
