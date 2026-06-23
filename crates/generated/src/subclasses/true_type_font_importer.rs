//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::md_four::MdFour;

#[derive(Debug, Clone)]
pub struct TrueTypeFontImporter {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_file_id_to_recycle_name: Vec<(i32, String)>,
    pub m_old_hash_identity: MdFour,
    pub m_new_hash_identity: MdFour,
    pub m_font_size: i32,
    pub m_force_texture_case: i32,
    pub m_render_mode: i32,
    pub m_style: i32,
    pub m_include_font_data: bool,
    pub m_use2x_behaviour: bool,
    pub m_font_names: Vec<String>,
}

impl crate::Parse for TrueTypeFontImporter {
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
        let m_font_size = r.read_i32()?;
        let m_force_texture_case = r.read_i32()?;
        let m_render_mode = r.read_i32()?;
        let m_style = r.read_i32()?;
        let m_include_font_data = r.read_bool()?;
        let m_use2x_behaviour = r.read_bool()?;
        r.align(4)?;
        let m_font_names = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(TrueTypeFontImporter {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_file_id_to_recycle_name,
            m_old_hash_identity,
            m_new_hash_identity,
            m_font_size,
            m_force_texture_case,
            m_render_mode,
            m_style,
            m_include_font_data,
            m_use2x_behaviour,
            m_font_names,
        })
    }
}
