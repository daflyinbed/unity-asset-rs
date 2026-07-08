//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_field::VfxField;

#[derive(Debug, Clone)]
pub struct VfxPropertySheetSerializedBase {
    pub m_float: VfxField,
    pub m_vector2f: VfxField,
    pub m_vector3f: VfxField,
    pub m_vector4f: VfxField,
    pub m_uint: VfxField,
    pub m_int: VfxField,
    pub m_matrix4x4f: VfxField,
    pub m_animation_curve: VfxField,
    pub m_gradient: VfxField,
    pub m_named_object: VfxField,
    pub m_bool: VfxField,
}

impl crate::Parse for VfxPropertySheetSerializedBase {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_float = VfxField::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_vector2f = VfxField::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_vector3f = VfxField::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_vector4f = VfxField::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_uint = VfxField::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_int = VfxField::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_matrix4x4f = VfxField::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_animation_curve = VfxField::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_gradient = VfxField::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_named_object = VfxField::parse(rdr, ver)?;
        rdr.align(4)?;
        let m_bool = VfxField::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(VfxPropertySheetSerializedBase {
            m_float,
            m_vector2f,
            m_vector3f,
            m_vector4f,
            m_uint,
            m_int,
            m_matrix4x4f,
            m_animation_curve,
            m_gradient,
            m_named_object,
            m_bool,
        })
    }
}
