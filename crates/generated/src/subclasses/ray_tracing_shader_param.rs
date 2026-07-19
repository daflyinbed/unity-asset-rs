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
