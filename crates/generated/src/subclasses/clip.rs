//! 自动生成。请勿手动修改。

use crate::subclasses::dense_clip::DenseClip;
use crate::subclasses::offset_ptr::OffsetPtr;
use crate::subclasses::streamed_clip::StreamedClip;

#[derive(Debug, Clone)]
pub struct Clip {
    pub m_streamed_clip: StreamedClip,
    pub m_dense_clip: DenseClip,
    pub m_binding: OffsetPtr,
}

impl crate::Parse for Clip {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_streamed_clip = StreamedClip::parse(r, ver)?;
        let m_dense_clip = DenseClip::parse(r, ver)?;
        let m_binding = OffsetPtr::parse(r, ver)?;

        Ok(Clip {
            m_streamed_clip,
            m_dense_clip,
            m_binding,
        })
    }
}
