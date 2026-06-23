//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AudioClip {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_format: i32,
    pub m_type: i32,
    pub m_3d: bool,
    pub m_use_hardware: bool,
    pub m_stream: i32,
    pub m_audio_data: Vec<u8>,
}

impl crate::Parse for AudioClip {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_format = r.read_i32()?;
        let m_type = r.read_i32()?;
        let m_3d = r.read_bool()?;
        let m_use_hardware = r.read_bool()?;
        r.align(4)?;
        let m_stream = r.read_i32()?;
        let m_audio_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(AudioClip {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_format,
            m_type,
            m_3d,
            m_use_hardware,
            m_stream,
            m_audio_data,
        })
    }
}
