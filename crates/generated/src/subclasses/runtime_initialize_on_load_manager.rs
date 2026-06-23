//! 自动生成。请勿手动修改。

use crate::subclasses::class_info::ClassInfo;
use crate::subclasses::class_method_info::ClassMethodInfo;

#[derive(Debug, Clone)]
pub struct RuntimeInitializeOnLoadManager {
    pub m_object_hide_flags: u32,
    pub m_assembly_names: Vec<String>,
    pub m_namespace_names: Vec<String>,
    pub m_class_infos: Vec<ClassInfo>,
    pub m_class_method_infos: Vec<ClassMethodInfo>,
    pub m_unity_method_execution_orders: Vec<i32>,
    pub m_method_execution_orders: Vec<i32>,
    pub m_full_class_names: Vec<String>,
}

impl crate::Parse for RuntimeInitializeOnLoadManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_assembly_names = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_namespace_names = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_class_infos = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ClassInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_class_method_infos = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ClassMethodInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_unity_method_execution_orders = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        let m_method_execution_orders = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_full_class_names = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(RuntimeInitializeOnLoadManager {
            m_object_hide_flags,
            m_assembly_names,
            m_namespace_names,
            m_class_infos,
            m_class_method_infos,
            m_unity_method_execution_orders,
            m_method_execution_orders,
            m_full_class_names,
        })
    }
}
