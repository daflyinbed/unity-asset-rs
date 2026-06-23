//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::md_four::MdFour;

#[derive(Debug, Clone)]
pub struct AudioImporter {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_file_id_to_recycle_name: Vec<(i32, String)>,
    pub m_old_hash_identity: MdFour,
    pub m_new_hash_identity: MdFour,
    pub m_format: i32,
    pub m_quality: f32,
    pub m_stream: i32,
    pub m_preview_data_length: u32,
    pub audio_preview_data: Vec<u8>,
    pub m_3d: bool,
    pub m_force_to_mono: bool,
    pub m_use_hardware: bool,
    pub m_loopable: bool,
}

impl crate::Parse for AudioImporter {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_file_id_to_recycle_name = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_old_hash_identity = MdFour::parse(r, ver)?;
        r.align(4)?;
        let m_new_hash_identity = MdFour::parse(r, ver)?;
        r.align(4)?;
        let m_format = r.read_i32()?;
        let m_quality = r.read_f32()?;
        let m_stream = r.read_i32()?;
        let m_preview_data_length = r.read_u32()?;
        let audio_preview_data = ( { let n = r.read_i32()? as usize; r.read_bytes(n) } )?;
        r.align(4)?;
        let m_3d = r.read_bool()?;
        let m_force_to_mono = r.read_bool()?;
        let m_use_hardware = r.read_bool()?;
        let m_loopable = r.read_bool()?;

        Ok(AudioImporter {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_file_id_to_recycle_name,
            m_old_hash_identity,
            m_new_hash_identity,
            m_format,
            m_quality,
            m_stream,
            m_preview_data_length,
            audio_preview_data,
            m_3d,
            m_force_to_mono,
            m_use_hardware,
            m_loopable,
        })
    }
}
