//! 自动生成。请勿手动修改。

use crate::subclasses::vfxgpu_buffer_desc::VfxgpuBufferDesc;

#[derive(Debug, Clone)]
pub struct VfxTemporaryGpuBufferDesc {
    pub desc: VfxgpuBufferDesc,
    pub frame_count: u32,
}

impl crate::Parse for VfxTemporaryGpuBufferDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let desc = VfxgpuBufferDesc::parse(rdr, ver)?;
        let frame_count = rdr.read_u32()?;

        Ok(VfxTemporaryGpuBufferDesc { desc, frame_count })
    }
}

impl crate::ToValue for VfxTemporaryGpuBufferDesc {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("desc".to_string(), self.desc.to_value())),
            Some(("frameCount".to_string(), self.frame_count.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "desc" => (self.desc).inspect_value(path, offset, limit),
                "frameCount" => (self.frame_count).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.desc).inspect_value(path, offset, limit)).map(|v| ("desc".to_string(), v)),
            ((self.frame_count).inspect_value(path, offset, limit))
                .map(|v| ("frameCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
