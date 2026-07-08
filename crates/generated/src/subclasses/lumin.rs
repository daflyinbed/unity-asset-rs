//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Lumin {
    pub depth_format: i32,
    pub frame_timing: i32,
    pub enable_gl_cache: bool,
    pub gl_cache_max_blob_size: u32,
    pub gl_cache_max_file_size: u32,
}

impl crate::Parse for Lumin {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let depth_format = rdr.read_i32()?;
        let frame_timing = rdr.read_i32()?;
        let enable_gl_cache = rdr.read_bool()?;
        rdr.align(4)?;
        let gl_cache_max_blob_size = rdr.read_u32()?;
        let gl_cache_max_file_size = rdr.read_u32()?;
        rdr.align(4)?;

        Ok(Lumin {
            depth_format,
            frame_timing,
            enable_gl_cache,
            gl_cache_max_blob_size,
            gl_cache_max_file_size,
        })
    }
}
