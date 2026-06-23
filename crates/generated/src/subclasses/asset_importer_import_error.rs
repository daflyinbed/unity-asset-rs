//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AssetImporter_ImportError {
    pub error: String,
    pub mode: i32,
    pub line: i32,
    pub file: String,
    pub object: PPtr,
}

impl crate::Parse for AssetImporter_ImportError {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let error = r.read_aligned_string()?;
        r.align(4)?;
        let mode = r.read_i32()?;
        let line = r.read_i32()?;
        let file = r.read_aligned_string()?;
        r.align(4)?;
        let object = crate::PPtr::parse(r, ver)?;

        Ok(AssetImporter_ImportError {
            error,
            mode,
            line,
            file,
            object,
        })
    }
}
