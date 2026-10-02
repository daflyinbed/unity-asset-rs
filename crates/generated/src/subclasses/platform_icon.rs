//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct PlatformIcon {
    pub m_textures: Vec<PPtr>,
    pub m_width: i32,
    pub m_height: i32,
    pub m_kind: i32,
    pub m_sub_kind: String,
}

impl crate::Parse for PlatformIcon {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_textures = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_width = rdr.read_i32()?;
        let m_height = rdr.read_i32()?;
        let m_kind = rdr.read_i32()?;
        let m_sub_kind = rdr.read_aligned_string()?;

        Ok(PlatformIcon {
            m_textures,
            m_width,
            m_height,
            m_kind,
            m_sub_kind,
        })
    }
}

impl crate::ToValue for PlatformIcon {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Textures".to_string(), self.m_textures.to_value())),
            Some(("m_Width".to_string(), self.m_width.to_value())),
            Some(("m_Height".to_string(), self.m_height.to_value())),
            Some(("m_Kind".to_string(), self.m_kind.to_value())),
            Some(("m_SubKind".to_string(), self.m_sub_kind.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Textures" => (self.m_textures).inspect_value(path, offset, limit),
                "m_Width" => (self.m_width).inspect_value(path, offset, limit),
                "m_Height" => (self.m_height).inspect_value(path, offset, limit),
                "m_Kind" => (self.m_kind).inspect_value(path, offset, limit),
                "m_SubKind" => (self.m_sub_kind).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_textures).inspect_value(path, offset, limit))
                .map(|v| ("m_Textures".to_string(), v)),
            ((self.m_width).inspect_value(path, offset, limit)).map(|v| ("m_Width".to_string(), v)),
            ((self.m_height).inspect_value(path, offset, limit))
                .map(|v| ("m_Height".to_string(), v)),
            ((self.m_kind).inspect_value(path, offset, limit)).map(|v| ("m_Kind".to_string(), v)),
            ((self.m_sub_kind).inspect_value(path, offset, limit))
                .map(|v| ("m_SubKind".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
