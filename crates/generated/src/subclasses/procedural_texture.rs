//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::gl_texture_settings::GlTextureSettings;
use crate::subclasses::texture_parameters::TextureParameters;

#[derive(Debug, Clone)]
pub struct ProceduralTexture {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_substance_material: PPtr,
    pub m_substance_texture_uid: u64,
    pub r#type: i32,
    pub alpha_source: i32,
    pub format: i32,
    pub m_texture_parameters: TextureParameters,
    pub m_texture_settings: GlTextureSettings,
    pub m_baked_data: Vec<u8>,
    pub m_baked_parameters: TextureParameters,
    pub m_lightmap_format: i32,
}

impl crate::Parse for ProceduralTexture {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_substance_material = crate::PPtr::parse(r, ver)?;
        let m_substance_texture_uid = r.read_u64()?;
        let r#type = r.read_i32()?;
        let alpha_source = r.read_i32()?;
        let format = r.read_i32()?;
        let m_texture_parameters = TextureParameters::parse(r, ver)?;
        let m_texture_settings = GlTextureSettings::parse(r, ver)?;
        let m_baked_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_baked_parameters = TextureParameters::parse(r, ver)?;
        let m_lightmap_format = r.read_i32()?;

        Ok(ProceduralTexture {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_substance_material,
            m_substance_texture_uid,
            r#type,
            alpha_source,
            format,
            m_texture_parameters,
            m_texture_settings,
            m_baked_data,
            m_baked_parameters,
            m_lightmap_format,
        })
    }
}
