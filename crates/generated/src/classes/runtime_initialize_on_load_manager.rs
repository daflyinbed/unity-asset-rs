//! 自动生成。请勿手动修改。

use crate::subclasses::class_info::ClassInfo;
use crate::subclasses::class_method_info::ClassMethodInfo;

#[derive(Debug, Clone)]
pub struct RuntimeInitializeOnLoadManager {
    pub m_assembly_names: Option<Vec<String>>,
    pub m_namespace_names: Option<Vec<String>>,
    pub m_class_infos: Option<Vec<ClassInfo>>,
    pub m_class_method_infos: Option<Vec<ClassMethodInfo>>,
    pub m_unity_method_execution_orders: Option<Vec<i32>>,
    pub m_before_unity_method_execution_orders: Option<Vec<i32>>,
    pub m_method_execution_orders: Option<Vec<i32>>,
    pub m_after_unity_method_execution_orders: Option<Vec<i32>>,
    pub m_before_method_execution_orders: Option<Vec<i32>>,
    pub m_after_method_execution_orders: Option<Vec<i32>>,
    pub m_after_assemblies_loaded_unity_method_execution_orders: Option<Vec<i32>>,
    pub m_after_assemblies_loaded_method_execution_orders: Option<Vec<i32>>,
    pub m_before_splash_screen_unity_method_execution_orders: Option<Vec<i32>>,
    pub m_before_splash_screen_method_execution_orders: Option<Vec<i32>>,
    pub m_subsystem_registration_unity_method_execution_orders: Option<Vec<i32>>,
    pub m_subsystem_registration_method_execution_orders: Option<Vec<i32>>,
}

impl crate::Parse for RuntimeInitializeOnLoadManager {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_assembly_names = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_namespace_names = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_class_infos = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ClassInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_class_method_infos = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ClassMethodInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_unity_method_execution_orders = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_before_unity_method_execution_orders = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_method_execution_orders = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_after_unity_method_execution_orders = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_before_method_execution_orders = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_after_method_execution_orders = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_after_assemblies_loaded_unity_method_execution_orders = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_after_assemblies_loaded_method_execution_orders = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_before_splash_screen_unity_method_execution_orders = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_before_splash_screen_method_execution_orders = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_subsystem_registration_unity_method_execution_orders = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_subsystem_registration_method_execution_orders = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(RuntimeInitializeOnLoadManager {
            m_assembly_names,
            m_namespace_names,
            m_class_infos,
            m_class_method_infos,
            m_unity_method_execution_orders,
            m_before_unity_method_execution_orders,
            m_method_execution_orders,
            m_after_unity_method_execution_orders,
            m_before_method_execution_orders,
            m_after_method_execution_orders,
            m_after_assemblies_loaded_unity_method_execution_orders,
            m_after_assemblies_loaded_method_execution_orders,
            m_before_splash_screen_unity_method_execution_orders,
            m_before_splash_screen_method_execution_orders,
            m_subsystem_registration_unity_method_execution_orders,
            m_subsystem_registration_method_execution_orders,
        })
    }
}
