//! 自动生成。请勿手动修改。

use crate::subclasses::class_info::ClassInfo;
use crate::subclasses::class_method_info::ClassMethodInfo;


/// 版本枚举：RuntimeInitializeOnLoadManagerMAssemblyNamesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RuntimeInitializeOnLoadManagerMAssemblyNamesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for RuntimeInitializeOnLoadManagerMAssemblyNamesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：RuntimeInitializeOnLoadManagerMNamespaceNamesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RuntimeInitializeOnLoadManagerMNamespaceNamesVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for RuntimeInitializeOnLoadManagerMNamespaceNamesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：RuntimeInitializeOnLoadManagerMClassInfosVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RuntimeInitializeOnLoadManagerMClassInfosVariant {
    Default(Vec<ClassInfo>),
    V2017_1(Vec<ClassInfo>),
}

impl crate::Parse for RuntimeInitializeOnLoadManagerMClassInfosVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClassInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClassInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：RuntimeInitializeOnLoadManagerMClassMethodInfosVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RuntimeInitializeOnLoadManagerMClassMethodInfosVariant {
    Default(Vec<ClassMethodInfo>),
    V2017_1(Vec<ClassMethodInfo>),
}

impl crate::Parse for RuntimeInitializeOnLoadManagerMClassMethodInfosVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClassMethodInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClassMethodInfo::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            Ok(Self::Default(v))
        }
    }
}

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

/// 版本枚举：RuntimeInitializeOnLoadManagerMAfterMethodExecutionOrdersVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RuntimeInitializeOnLoadManagerMAfterMethodExecutionOrdersVariant {
    Default(Vec<i32>),
    V2017_1(Vec<i32>),
}

impl crate::Parse for RuntimeInitializeOnLoadManagerMAfterMethodExecutionOrdersVariant {
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
    pub m_assembly_names: Option<RuntimeInitializeOnLoadManagerMAssemblyNamesVariant>,
    pub m_namespace_names: Option<RuntimeInitializeOnLoadManagerMNamespaceNamesVariant>,
    pub m_class_infos: Option<RuntimeInitializeOnLoadManagerMClassInfosVariant>,
    pub m_class_method_infos: Option<RuntimeInitializeOnLoadManagerMClassMethodInfosVariant>,
    pub m_unity_method_execution_orders: Option<Vec<i32>>,
    pub m_before_unity_method_execution_orders: Option<RuntimeInitializeOnLoadManagerMBeforeUnityMethodExecutionOrdersVariant>,
    pub m_method_execution_orders: Option<Vec<i32>>,
    pub m_after_unity_method_execution_orders: Option<RuntimeInitializeOnLoadManagerMAfterUnityMethodExecutionOrdersVariant>,
    pub m_before_method_execution_orders: Option<RuntimeInitializeOnLoadManagerMBeforeMethodExecutionOrdersVariant>,
    pub m_after_method_execution_orders: Option<RuntimeInitializeOnLoadManagerMAfterMethodExecutionOrdersVariant>,
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
        let mut m_assembly_names = None;
        let mut m_namespace_names = None;
        let mut m_class_infos = None;
        let mut m_class_method_infos = None;
        let mut m_unity_method_execution_orders = None;
        let mut m_before_unity_method_execution_orders = None;
        let mut m_method_execution_orders = None;
        let mut m_after_unity_method_execution_orders = None;
        let mut m_before_method_execution_orders = None;
        let mut m_after_method_execution_orders = None;
        let mut m_after_assemblies_loaded_unity_method_execution_orders = None;
        let mut m_after_assemblies_loaded_method_execution_orders = None;
        let mut m_before_splash_screen_unity_method_execution_orders = None;
        let mut m_before_splash_screen_method_execution_orders = None;
        let mut m_subsystem_registration_unity_method_execution_orders = None;
        let mut m_subsystem_registration_method_execution_orders = None;
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            m_assembly_names = Some(RuntimeInitializeOnLoadManagerMAssemblyNamesVariant::parse(rdr, ver)?);
            m_namespace_names = Some(RuntimeInitializeOnLoadManagerMNamespaceNamesVariant::parse(rdr, ver)?);
            m_class_infos = Some(RuntimeInitializeOnLoadManagerMClassInfosVariant::parse(rdr, ver)?);
            m_class_method_infos = Some(RuntimeInitializeOnLoadManagerMClassMethodInfosVariant::parse(rdr, ver)?);
            m_before_unity_method_execution_orders = Some(RuntimeInitializeOnLoadManagerMBeforeUnityMethodExecutionOrdersVariant::parse(rdr, ver)?);
            m_after_unity_method_execution_orders = Some(RuntimeInitializeOnLoadManagerMAfterUnityMethodExecutionOrdersVariant::parse(rdr, ver)?);
            m_before_method_execution_orders = Some(RuntimeInitializeOnLoadManagerMBeforeMethodExecutionOrdersVariant::parse(rdr, ver)?);
            m_after_method_execution_orders = Some(RuntimeInitializeOnLoadManagerMAfterMethodExecutionOrdersVariant::parse(rdr, ver)?);
            m_after_assemblies_loaded_unity_method_execution_orders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_after_assemblies_loaded_method_execution_orders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_before_splash_screen_unity_method_execution_orders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_before_splash_screen_method_execution_orders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_subsystem_registration_unity_method_execution_orders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_subsystem_registration_method_execution_orders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            m_assembly_names = Some(RuntimeInitializeOnLoadManagerMAssemblyNamesVariant::parse(rdr, ver)?);
            m_namespace_names = Some(RuntimeInitializeOnLoadManagerMNamespaceNamesVariant::parse(rdr, ver)?);
            m_class_infos = Some(RuntimeInitializeOnLoadManagerMClassInfosVariant::parse(rdr, ver)?);
            m_class_method_infos = Some(RuntimeInitializeOnLoadManagerMClassMethodInfosVariant::parse(rdr, ver)?);
            m_before_unity_method_execution_orders = Some(RuntimeInitializeOnLoadManagerMBeforeUnityMethodExecutionOrdersVariant::parse(rdr, ver)?);
            m_after_unity_method_execution_orders = Some(RuntimeInitializeOnLoadManagerMAfterUnityMethodExecutionOrdersVariant::parse(rdr, ver)?);
            m_before_method_execution_orders = Some(RuntimeInitializeOnLoadManagerMBeforeMethodExecutionOrdersVariant::parse(rdr, ver)?);
            m_after_method_execution_orders = Some(RuntimeInitializeOnLoadManagerMAfterMethodExecutionOrdersVariant::parse(rdr, ver)?);
            m_after_assemblies_loaded_unity_method_execution_orders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_after_assemblies_loaded_method_execution_orders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_before_splash_screen_unity_method_execution_orders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            m_before_splash_screen_method_execution_orders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(5, 2, 0) {
            m_assembly_names = Some(RuntimeInitializeOnLoadManagerMAssemblyNamesVariant::parse(rdr, ver)?);
            m_namespace_names = Some(RuntimeInitializeOnLoadManagerMNamespaceNamesVariant::parse(rdr, ver)?);
            m_class_infos = Some(RuntimeInitializeOnLoadManagerMClassInfosVariant::parse(rdr, ver)?);
            m_class_method_infos = Some(RuntimeInitializeOnLoadManagerMClassMethodInfosVariant::parse(rdr, ver)?);
            m_before_unity_method_execution_orders = Some(RuntimeInitializeOnLoadManagerMBeforeUnityMethodExecutionOrdersVariant::parse(rdr, ver)?);
            m_after_unity_method_execution_orders = Some(RuntimeInitializeOnLoadManagerMAfterUnityMethodExecutionOrdersVariant::parse(rdr, ver)?);
            m_before_method_execution_orders = Some(RuntimeInitializeOnLoadManagerMBeforeMethodExecutionOrdersVariant::parse(rdr, ver)?);
            m_after_method_execution_orders = Some(RuntimeInitializeOnLoadManagerMAfterMethodExecutionOrdersVariant::parse(rdr, ver)?);
        } else {
            m_assembly_names = Some(RuntimeInitializeOnLoadManagerMAssemblyNamesVariant::parse(rdr, ver)?);
            m_namespace_names = Some(RuntimeInitializeOnLoadManagerMNamespaceNamesVariant::parse(rdr, ver)?);
            m_class_infos = Some(RuntimeInitializeOnLoadManagerMClassInfosVariant::parse(rdr, ver)?);
            m_class_method_infos = Some(RuntimeInitializeOnLoadManagerMClassMethodInfosVariant::parse(rdr, ver)?);
            m_unity_method_execution_orders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
            m_method_execution_orders = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_i32() }).collect::<Result<Vec<_>, _>>() } )?);
        }

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
