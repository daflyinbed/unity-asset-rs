//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ImportLog_ImportLogEntry {
    pub message: String,
    pub mode: i32,
    pub line: i32,
    pub file: String,
    pub object: PPtr,
}

impl crate::Parse for ImportLog_ImportLogEntry {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let message = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let mode = rdr.read_i32()?;
        let line = rdr.read_i32()?;
        let file = rdr.read_aligned_string()?;
        rdr.align(4)?;
        let object = crate::PPtr::parse(rdr, ver)?;

        Ok(ImportLog_ImportLogEntry {
            message,
            mode,
            line,
            file,
            object,
        })
    }
}
