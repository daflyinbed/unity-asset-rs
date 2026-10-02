//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_mapping::VfxMapping;

#[derive(Debug, Clone)]
pub struct VfxMappingTemporary {
    pub mapping: VfxMapping,
    pub past_frame_index: u32,
    pub per_camera_buffer: bool,
}

impl crate::Parse for VfxMappingTemporary {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mapping = VfxMapping::parse(rdr, ver)?;
        let past_frame_index = rdr.read_u32()?;
        let per_camera_buffer = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(VfxMappingTemporary {
            mapping,
            past_frame_index,
            per_camera_buffer,
        })
    }
}

impl crate::ToValue for VfxMappingTemporary {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("mapping".to_string(), self.mapping.to_value())),
            Some((
                "pastFrameIndex".to_string(),
                self.past_frame_index.to_value(),
            )),
            Some((
                "perCameraBuffer".to_string(),
                self.per_camera_buffer.to_value(),
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
                "mapping" => (self.mapping).inspect_value(path, offset, limit),
                "pastFrameIndex" => (self.past_frame_index).inspect_value(path, offset, limit),
                "perCameraBuffer" => (self.per_camera_buffer).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.mapping).inspect_value(path, offset, limit)).map(|v| ("mapping".to_string(), v)),
            ((self.past_frame_index).inspect_value(path, offset, limit))
                .map(|v| ("pastFrameIndex".to_string(), v)),
            ((self.per_camera_buffer).inspect_value(path, offset, limit))
                .map(|v| ("perCameraBuffer".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
