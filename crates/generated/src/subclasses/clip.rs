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
