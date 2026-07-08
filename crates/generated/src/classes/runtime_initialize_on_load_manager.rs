//! 自动生成。请勿手动修改。

use crate::subclasses::class_info::ClassInfo;
use crate::subclasses::class_method_info::ClassMethodInfo;


/// 版本枚举：RuntimeInitializeOnLoadManagerMBeforeUnityMethodExecutionOrdersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RuntimeInitializeOnLoadManagerMBeforeUnityMethodExecutionOrdersVariant {
    Default(Vec<i32>),
    V2017_1(Vec<i32>),
}

impl crate::Parse for RuntimeInitializeOnLoadManagerMBeforeUnityMethodExecutionOrdersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：RuntimeInitializeOnLoadManagerMAfterUnityMethodExecutionOrdersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RuntimeInitializeOnLoadManagerMAfterUnityMethodExecutionOrdersVariant {
    Default(Vec<i32>),
    V2017_1(Vec<i32>),
}

impl crate::Parse for RuntimeInitializeOnLoadManagerMAfterUnityMethodExecutionOrdersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：RuntimeInitializeOnLoadManagerMBeforeMethodExecutionOrdersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RuntimeInitializeOnLoadManagerMBeforeMethodExecutionOrdersVariant {
    Default(Vec<i32>),
    V2017_1(Vec<i32>),
}

impl crate::Parse for RuntimeInitializeOnLoadManagerMBeforeMethodExecutionOrdersVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct RuntimeInitializeOnLoadManager {
    pub m_assembly_names: Option<Vec<String>>,
    pub m_namespace_names: Option<Vec<String>>,
    pub m_class_infos: Option<Vec<ClassInfo>>,
    pub m_class_method_infos: Option<Vec<ClassMethodInfo>>,
    pub m_unity_method_execution_orders: Option<Vec<i32>>,
    pub m_before_unity_method_execution_orders: Option<RuntimeInitializeOnLoadManagerMBeforeUnityMethodExecutionOrdersVariant>,
    pub m_method_execution_orders: Option<Vec<i32>>,
    pub m_after_unity_method_execution_orders: Option<RuntimeInitializeOnLoadManagerMAfterUnityMethodExecutionOrdersVariant>,
    pub m_before_method_execution_orders: Option<RuntimeInitializeOnLoadManagerMBeforeMethodExecutionOrdersVariant>,
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
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_assembly_names = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_namespace_names = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_class_infos = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClassInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_class_method_infos = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClassMethodInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_unity_method_execution_orders = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_before_unity_method_execution_orders = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(RuntimeInitializeOnLoadManagerMBeforeUnityMethodExecutionOrdersVariant::parse(rdr, ver)?)
        } else {
            None
        };
        let m_method_execution_orders = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_after_unity_method_execution_orders = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(RuntimeInitializeOnLoadManagerMAfterUnityMethodExecutionOrdersVariant::parse(rdr, ver)?)
        } else {
            None
        };
        let m_before_method_execution_orders = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            Some(RuntimeInitializeOnLoadManagerMBeforeMethodExecutionOrdersVariant::parse(rdr, ver)?)
        } else {
            None
        };
        let m_after_method_execution_orders = if *ver >= crate::UnityVersion::new(5, 2, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_after_assemblies_loaded_unity_method_execution_orders = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_after_assemblies_loaded_method_execution_orders = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_before_splash_screen_unity_method_execution_orders = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_before_splash_screen_method_execution_orders = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_subsystem_registration_unity_method_execution_orders = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_subsystem_registration_method_execution_orders = if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
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
