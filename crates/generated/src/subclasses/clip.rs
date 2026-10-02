//! 自动生成。请勿手动修改。

use crate::subclasses::constant_clip::ConstantClip;
use crate::subclasses::dense_clip::DenseClip;
use crate::subclasses::offset_ptr::OffsetPtr;
use crate::subclasses::streamed_clip::StreamedClip;

#[derive(Debug, Clone)]
pub struct Clip {
    pub m_streamed_clip: StreamedClip,
    pub m_dense_clip: DenseClip,
    pub m_binding: Option<OffsetPtr>,
    pub m_constant_clip: Option<ConstantClip>,
}

impl crate::Parse for Clip {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_streamed_clip = None;
        let mut m_dense_clip = None;
        let mut m_binding = None;
        let mut m_constant_clip = None;
        m_streamed_clip = Some(StreamedClip::parse(rdr, ver)?);
        m_dense_clip = Some(DenseClip::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2018, 3, 0) {
            m_constant_clip = Some(ConstantClip::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 3, 0) {
            m_constant_clip = Some(ConstantClip::parse(rdr, ver)?);
            m_binding = Some(OffsetPtr::parse(rdr, ver)?);
        } else {
            m_binding = Some(OffsetPtr::parse(rdr, ver)?);
        }

        Ok(Clip {
            m_streamed_clip: m_streamed_clip.unwrap(),
            m_dense_clip: m_dense_clip.unwrap(),
            m_binding,
            m_constant_clip,
        })
    }
}

impl crate::ToValue for Clip {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_StreamedClip".to_string(),
                self.m_streamed_clip.to_value(),
            )),
            Some(("m_DenseClip".to_string(), self.m_dense_clip.to_value())),
            crate::opt_field("m_Binding", &self.m_binding),
            crate::opt_field("m_ConstantClip", &self.m_constant_clip),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_StreamedClip" => (self.m_streamed_clip).inspect_value(path, offset, limit),
                "m_DenseClip" => (self.m_dense_clip).inspect_value(path, offset, limit),
                "m_Binding" => (self.m_binding).inspect_value(path, offset, limit),
                "m_ConstantClip" => (self.m_constant_clip).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_streamed_clip).inspect_value(path, offset, limit))
                .map(|v| ("m_StreamedClip".to_string(), v)),
            ((self.m_dense_clip).inspect_value(path, offset, limit))
                .map(|v| ("m_DenseClip".to_string(), v)),
            ((self.m_binding).inspect_value(path, offset, limit))
                .map(|v| ("m_Binding".to_string(), v)),
            ((self.m_constant_clip).inspect_value(path, offset, limit))
                .map(|v| ("m_ConstantClip".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
