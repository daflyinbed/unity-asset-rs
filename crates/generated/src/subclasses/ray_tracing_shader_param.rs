//! 自动生成。请勿手动修改。

/// 版本枚举：RayTracingShaderParamOffsetVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RayTracingShaderParamOffsetVariant {
    Default(i32),
    V2020_1(u32),
}

impl crate::Parse for RayTracingShaderParamOffsetVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u32()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for RayTracingShaderParamOffsetVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：RayTracingShaderParamArraySizeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RayTracingShaderParamArraySizeVariant {
    Default(i32),
    V2020_1(u32),
}

impl crate::Parse for RayTracingShaderParamArraySizeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u32()?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for RayTracingShaderParamArraySizeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：RayTracingShaderParamRowCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RayTracingShaderParamRowCountVariant {
    Default(i32),
    V2020_1(u8),
}

impl crate::Parse for RayTracingShaderParamRowCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for RayTracingShaderParamRowCountVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：RayTracingShaderParamColCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum RayTracingShaderParamColCountVariant {
    Default(i32),
    V2020_1(u8),
}

impl crate::Parse for RayTracingShaderParamColCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            let v = rdr.read_u8()?;
            rdr.align(4)?;
            Ok(Self::V2020_1(v))
        } else {
            let v = rdr.read_i32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for RayTracingShaderParamColCountVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2020_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2020_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct RayTracingShaderParam {
    pub name: String,
    pub r#type: Option<i32>,
    pub data_type: Option<i32>,
    pub offset: RayTracingShaderParamOffsetVariant,
    pub data_size: Option<u32>,
    pub array_size: RayTracingShaderParamArraySizeVariant,
    pub row_count: RayTracingShaderParamRowCountVariant,
    pub property_sheet_type: Option<i32>,
    pub col_count: RayTracingShaderParamColCountVariant,
}

impl crate::Parse for RayTracingShaderParam {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut name = None;
        let mut r#type = None;
        let mut data_type = None;
        let mut offset = None;
        let mut data_size = None;
        let mut array_size = None;
        let mut row_count = None;
        let mut property_sheet_type = None;
        let mut col_count = None;
        name = Some(rdr.read_aligned_string()?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            data_type = Some(rdr.read_i32()?);
            data_size = Some(rdr.read_u32()?);
        } else {
            r#type = Some(rdr.read_i32()?);
        }
        offset = Some(RayTracingShaderParamOffsetVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2020, 1, 0) {
            property_sheet_type = Some(rdr.read_i32()?);
        } else {
        }
        array_size = Some(RayTracingShaderParamArraySizeVariant::parse(rdr, ver)?);
        row_count = Some(RayTracingShaderParamRowCountVariant::parse(rdr, ver)?);
        col_count = Some(RayTracingShaderParamColCountVariant::parse(rdr, ver)?);

        Ok(RayTracingShaderParam {
            name: name.unwrap(),
            r#type,
            data_type,
            offset: offset.unwrap(),
            data_size,
            array_size: array_size.unwrap(),
            row_count: row_count.unwrap(),
            property_sheet_type,
            col_count: col_count.unwrap(),
        })
    }
}

impl crate::ToValue for RayTracingShaderParam {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            crate::opt_field("type", &self.r#type),
            crate::opt_field("dataType", &self.data_type),
            Some(("offset".to_string(), self.offset.to_value())),
            crate::opt_field("dataSize", &self.data_size),
            Some(("arraySize".to_string(), self.array_size.to_value())),
            Some(("rowCount".to_string(), self.row_count.to_value())),
            crate::opt_field("propertySheetType", &self.property_sheet_type),
            Some(("colCount".to_string(), self.col_count.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "dataType" => (self.data_type).inspect_value(path, offset, limit),
                "offset" => (self.offset).inspect_value(path, offset, limit),
                "dataSize" => (self.data_size).inspect_value(path, offset, limit),
                "arraySize" => (self.array_size).inspect_value(path, offset, limit),
                "rowCount" => (self.row_count).inspect_value(path, offset, limit),
                "propertySheetType" => {
                    (self.property_sheet_type).inspect_value(path, offset, limit)
                }
                "colCount" => (self.col_count).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.data_type).inspect_value(path, offset, limit))
                .map(|v| ("dataType".to_string(), v)),
            ((self.offset).inspect_value(path, offset, limit)).map(|v| ("offset".to_string(), v)),
            ((self.data_size).inspect_value(path, offset, limit))
                .map(|v| ("dataSize".to_string(), v)),
            ((self.array_size).inspect_value(path, offset, limit))
                .map(|v| ("arraySize".to_string(), v)),
            ((self.row_count).inspect_value(path, offset, limit))
                .map(|v| ("rowCount".to_string(), v)),
            ((self.property_sheet_type).inspect_value(path, offset, limit))
                .map(|v| ("propertySheetType".to_string(), v)),
            ((self.col_count).inspect_value(path, offset, limit))
                .map(|v| ("colCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
