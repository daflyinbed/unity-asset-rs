//! 自动生成。请勿手动修改。

/// 版本枚举：ClassInfoMClassNameVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClassInfoMClassNameVariant {
    Default(String),
    V2019_4_29(String),
    V2020_1(String),
}

impl crate::Parse for ClassInfoMClassNameVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Ok(Self::V2020_1(v))
        } else if *ver >= crate::UnityVersion::new(2019, 4, 29) {
            let v = rdr.read_aligned_string()?;
            Ok(Self::V2019_4_29(v))
        } else {
            let v = rdr.read_aligned_string()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ClassInfoMClassNameVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_4_29(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_4_29(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub m_assembly_name_index: i32,
    pub m_namespace_index: i32,
    pub m_class_name: ClassInfoMClassNameVariant,
    pub m_num_of_methods: i32,
    pub m_namespace_name: Option<String>,
    pub m_method_index: i32,
    pub m_is_unity_class: bool,
}

impl crate::Parse for ClassInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_assembly_name_index = None;
        let mut m_namespace_index = None;
        let mut m_class_name = None;
        let mut m_num_of_methods = None;
        let mut m_namespace_name = None;
        let mut m_method_index = None;
        let mut m_is_unity_class = None;
        m_assembly_name_index = Some(rdr.read_i32()?);
        m_namespace_index = Some(rdr.read_i32()?);
        rdr.align(4)?;
        m_class_name = Some(ClassInfoMClassNameVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2019, 4, 29) {
            m_namespace_name = Some(rdr.read_aligned_string()?);
            rdr.align(4)?;
        } else {
        }
        m_num_of_methods = Some(rdr.read_i32()?);
        m_method_index = Some(rdr.read_i32()?);
        rdr.align(4)?;
        m_is_unity_class = Some(rdr.read_bool()?);
        rdr.align(4)?;

        Ok(ClassInfo {
            m_assembly_name_index: m_assembly_name_index.unwrap(),
            m_namespace_index: m_namespace_index.unwrap(),
            m_class_name: m_class_name.unwrap(),
            m_num_of_methods: m_num_of_methods.unwrap(),
            m_namespace_name,
            m_method_index: m_method_index.unwrap(),
            m_is_unity_class: m_is_unity_class.unwrap(),
        })
    }
}

impl crate::ToValue for ClassInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_AssemblyNameIndex".to_string(),
                self.m_assembly_name_index.to_value(),
            )),
            Some((
                "m_NamespaceIndex".to_string(),
                self.m_namespace_index.to_value(),
            )),
            Some(("m_ClassName".to_string(), self.m_class_name.to_value())),
            Some((
                "m_NumOfMethods".to_string(),
                self.m_num_of_methods.to_value(),
            )),
            crate::opt_field("m_NamespaceName", &self.m_namespace_name),
            Some(("m_MethodIndex".to_string(), self.m_method_index.to_value())),
            Some((
                "m_IsUnityClass".to_string(),
                self.m_is_unity_class.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_AssemblyNameIndex" => {
                    (self.m_assembly_name_index).inspect_value(path, offset, limit)
                }
                "m_NamespaceIndex" => (self.m_namespace_index).inspect_value(path, offset, limit),
                "m_ClassName" => (self.m_class_name).inspect_value(path, offset, limit),
                "m_NumOfMethods" => (self.m_num_of_methods).inspect_value(path, offset, limit),
                "m_NamespaceName" => (self.m_namespace_name).inspect_value(path, offset, limit),
                "m_MethodIndex" => (self.m_method_index).inspect_value(path, offset, limit),
                "m_IsUnityClass" => (self.m_is_unity_class).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_assembly_name_index).inspect_value(path, offset, limit))
                .map(|v| ("m_AssemblyNameIndex".to_string(), v)),
            ((self.m_namespace_index).inspect_value(path, offset, limit))
                .map(|v| ("m_NamespaceIndex".to_string(), v)),
            ((self.m_class_name).inspect_value(path, offset, limit))
                .map(|v| ("m_ClassName".to_string(), v)),
            ((self.m_num_of_methods).inspect_value(path, offset, limit))
                .map(|v| ("m_NumOfMethods".to_string(), v)),
            ((self.m_namespace_name).inspect_value(path, offset, limit))
                .map(|v| ("m_NamespaceName".to_string(), v)),
            ((self.m_method_index).inspect_value(path, offset, limit))
                .map(|v| ("m_MethodIndex".to_string(), v)),
            ((self.m_is_unity_class).inspect_value(path, offset, limit))
                .map(|v| ("m_IsUnityClass".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
