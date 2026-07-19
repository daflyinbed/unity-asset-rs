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
