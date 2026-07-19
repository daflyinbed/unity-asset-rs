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
