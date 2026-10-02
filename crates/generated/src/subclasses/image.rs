//! 自动生成。请勿手动修改。

/// 版本枚举：ImageMFormatVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ImageMFormatVariant {
    Default(u32),
    V5_0(i32),
}

impl crate::Parse for ImageMFormatVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_0(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for ImageMFormatVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_0(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_0(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Image {
    pub m_format: ImageMFormatVariant,
    pub m_width: i32,
    pub m_height: i32,
    pub m_row_bytes: i32,
    pub image_data: Vec<u8>,
}

impl crate::Parse for Image {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_format = ImageMFormatVariant::parse(rdr, ver)?;
        let m_width = rdr.read_i32()?;
        let m_height = rdr.read_i32()?;
        let m_row_bytes = rdr.read_i32()?;
        let image_data = ({
            let n = rdr.read_i32()? as usize;
            rdr.read_bytes(n)
        })?;
        rdr.align(4)?;

        Ok(Image {
            m_format,
            m_width,
            m_height,
            m_row_bytes,
            image_data,
        })
    }
}

impl crate::ToValue for Image {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Format".to_string(), self.m_format.to_value())),
            Some(("m_Width".to_string(), self.m_width.to_value())),
            Some(("m_Height".to_string(), self.m_height.to_value())),
            Some(("m_RowBytes".to_string(), self.m_row_bytes.to_value())),
            Some((
                "image data".to_string(),
                crate::Value::Bytes(self.image_data.clone()),
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
                "m_Format" => (self.m_format).inspect_value(path, offset, limit),
                "m_Width" => (self.m_width).inspect_value(path, offset, limit),
                "m_Height" => (self.m_height).inspect_value(path, offset, limit),
                "m_RowBytes" => (self.m_row_bytes).inspect_value(path, offset, limit),
                "image data" => crate::inspect_bytes(&(self.image_data), path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_format).inspect_value(path, offset, limit))
                .map(|v| ("m_Format".to_string(), v)),
            ((self.m_width).inspect_value(path, offset, limit)).map(|v| ("m_Width".to_string(), v)),
            ((self.m_height).inspect_value(path, offset, limit))
                .map(|v| ("m_Height".to_string(), v)),
            ((self.m_row_bytes).inspect_value(path, offset, limit))
                .map(|v| ("m_RowBytes".to_string(), v)),
            (crate::inspect_bytes(&(self.image_data), path, offset, limit))
                .map(|v| ("image data".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
