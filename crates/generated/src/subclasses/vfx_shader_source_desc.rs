//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxShaderSourceDesc {
    pub compute: bool,
    pub name: String,
    pub source: String,
}

impl crate::Parse for VfxShaderSourceDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let compute = rdr.read_bool()?;
        rdr.align(4)?;
        let name = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let source = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(VfxShaderSourceDesc {
            compute,
            name,
            source,
        })
    }
}
