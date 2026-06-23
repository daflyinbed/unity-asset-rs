//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::substance_input::SubstanceInput;
use crate::subclasses::unity_property_sheet::UnityPropertySheet;

#[derive(Debug, Clone)]
pub struct ProceduralMaterial {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_shader: PPtr,
    pub m_saved_properties: UnityPropertySheet,
    pub m_substance_package: PPtr,
    pub m_maximum_size: i32,
    pub m_textures: Vec<PPtr>,
    pub m_inputs: Vec<SubstanceInput>,
    pub m_flags: u32,
    pub m_cache_size: i32,
}

impl crate::Parse for ProceduralMaterial {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_shader = crate::PPtr::parse(r, ver)?;
        let m_saved_properties = UnityPropertySheet::parse(r, ver)?;
        r.align(4)?;
        let m_substance_package = crate::PPtr::parse(r, ver)?;
        let m_maximum_size = r.read_i32()?;
        let m_textures = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_inputs = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { SubstanceInput::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_flags = r.read_u32()?;
        let m_cache_size = r.read_i32()?;

        Ok(ProceduralMaterial {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_shader,
            m_saved_properties,
            m_substance_package,
            m_maximum_size,
            m_textures,
            m_inputs,
            m_flags,
            m_cache_size,
        })
    }
}
