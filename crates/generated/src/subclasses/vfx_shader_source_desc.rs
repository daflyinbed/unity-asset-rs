//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxShaderSourceDesc {
    pub compute: bool,
    pub name: String,
    pub source: String,
}

impl crate::Parse for VfxShaderSourceDesc {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let compute = r.read_bool()?;
        r.align(4)?;
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let source = r.read_aligned_string()?;
        r.align(4)?;

        Ok(VfxShaderSourceDesc {
            compute,
            name,
            source,
        })
    }
}
