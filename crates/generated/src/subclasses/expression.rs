//! 自动生成。请勿手动修改。

/// 版本枚举：ExpressionOpVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ExpressionOpVariant {
    Default(i32),
    V6000_5(u8),
}

impl crate::Parse for ExpressionOpVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = rdr.read_u8()?;
            Ok(Self::V6000_5(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ExpressionOpVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：ExpressionValueIndexVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ExpressionValueIndexVariant {
    Default(i32),
    V6000_5(u16),
}

impl crate::Parse for ExpressionValueIndexVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            let v = rdr.read_u16()?;
            Ok(Self::V6000_5(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ExpressionValueIndexVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_5(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_5(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Expression {
    pub op: ExpressionOpVariant,
    pub value_index: ExpressionValueIndexVariant,
    pub data_type: Option<u8>,
    pub data_0: i32,
    pub data_size: Option<u8>,
    pub data_1: i32,
    pub scalar_switch_case: Option<u8>,
    pub data_2: i32,
    pub jmp_code: Option<u16>,
    pub data_3: i32,
}

impl crate::Parse for Expression {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut op = None;
        let mut value_index = None;
        let mut data_type = None;
        let mut data_0 = None;
        let mut data_size = None;
        let mut data_1 = None;
        let mut scalar_switch_case = None;
        let mut data_2 = None;
        let mut jmp_code = None;
        let mut data_3 = None;
        op = Some(ExpressionOpVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 5, 0) {
            data_type = Some(rdr.read_u8()?);
            data_size = Some(rdr.read_u8()?);
            scalar_switch_case = Some(rdr.read_u8()?);
            jmp_code = Some(rdr.read_u16()?);
        } else {
        }
        value_index = Some(ExpressionValueIndexVariant::parse(rdr, ver)?);
        data_0 = Some(rdr.read_i32()?);
        data_1 = Some(rdr.read_i32()?);
        data_2 = Some(rdr.read_i32()?);
        data_3 = Some(rdr.read_i32()?);

        Ok(Expression {
            op: op.unwrap(),
            value_index: value_index.unwrap(),
            data_type,
            data_0: data_0.unwrap(),
            data_size,
            data_1: data_1.unwrap(),
            scalar_switch_case,
            data_2: data_2.unwrap(),
            jmp_code,
            data_3: data_3.unwrap(),
        })
    }
}

impl crate::ToValue for Expression {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("op".to_string(), self.op.to_value())),
            Some(("valueIndex".to_string(), self.value_index.to_value())),
            crate::opt_field("dataType", &self.data_type),
            Some(("data[0]".to_string(), self.data_0.to_value())),
            crate::opt_field("dataSize", &self.data_size),
            Some(("data[1]".to_string(), self.data_1.to_value())),
            crate::opt_field("scalarSwitchCase", &self.scalar_switch_case),
            Some(("data[2]".to_string(), self.data_2.to_value())),
            crate::opt_field("jmpCode", &self.jmp_code),
            Some(("data[3]".to_string(), self.data_3.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "op" => (self.op).inspect_value(path, offset, limit),
                "valueIndex" => (self.value_index).inspect_value(path, offset, limit),
                "dataType" => (self.data_type).inspect_value(path, offset, limit),
                "data[0]" => (self.data_0).inspect_value(path, offset, limit),
                "dataSize" => (self.data_size).inspect_value(path, offset, limit),
                "data[1]" => (self.data_1).inspect_value(path, offset, limit),
                "scalarSwitchCase" => (self.scalar_switch_case).inspect_value(path, offset, limit),
                "data[2]" => (self.data_2).inspect_value(path, offset, limit),
                "jmpCode" => (self.jmp_code).inspect_value(path, offset, limit),
                "data[3]" => (self.data_3).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.op).inspect_value(path, offset, limit)).map(|v| ("op".to_string(), v)),
            ((self.value_index).inspect_value(path, offset, limit))
                .map(|v| ("valueIndex".to_string(), v)),
            ((self.data_type).inspect_value(path, offset, limit))
                .map(|v| ("dataType".to_string(), v)),
            ((self.data_0).inspect_value(path, offset, limit)).map(|v| ("data[0]".to_string(), v)),
            ((self.data_size).inspect_value(path, offset, limit))
                .map(|v| ("dataSize".to_string(), v)),
            ((self.data_1).inspect_value(path, offset, limit)).map(|v| ("data[1]".to_string(), v)),
            ((self.scalar_switch_case).inspect_value(path, offset, limit))
                .map(|v| ("scalarSwitchCase".to_string(), v)),
            ((self.data_2).inspect_value(path, offset, limit)).map(|v| ("data[2]".to_string(), v)),
            ((self.jmp_code).inspect_value(path, offset, limit))
                .map(|v| ("jmpCode".to_string(), v)),
            ((self.data_3).inspect_value(path, offset, limit)).map(|v| ("data[3]".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
