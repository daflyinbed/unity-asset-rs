//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LodSelectionCurve {
    pub m_lod_slope: f32,
    pub m_lod_bias: f32,
}

impl crate::Parse for LodSelectionCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_lod_slope = rdr.read_f32()?;
        let m_lod_bias = rdr.read_f32()?;
        rdr.align(4)?;

        Ok(LodSelectionCurve {
            m_lod_slope,
            m_lod_bias,
        })
    }
}
