//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct Scene {
    pub m_object_hide_flags: u32,
    pub m_pvs_data: Vec<u8>,
    pub m_pvs_objects_array: Vec<PPtr>,
    pub m_view_cell_size: f32,
}

impl crate::Parse for Scene {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_pvs_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_pvs_objects_array = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_view_cell_size = r.read_f32()?;

        Ok(Scene {
            m_object_hide_flags,
            m_pvs_data,
            m_pvs_objects_array,
            m_view_cell_size,
        })
    }
}
