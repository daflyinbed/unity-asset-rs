//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_layout_element_desc::VfxLayoutElementDesc;
use crate::subclasses::vfxcpu_buffer_data::VfxcpuBufferData;

#[derive(Debug, Clone)]
pub struct VfxcpuBufferDesc {
    pub capacity: u32,
    pub stride: u32,
    pub layout: Vec<VfxLayoutElementDesc>,
    pub initial_data: VfxcpuBufferData,
    pub debug_name: Option<String>,
}

impl crate::Parse for VfxcpuBufferDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut capacity = None;
        let mut stride = None;
        let mut layout = None;
        let mut initial_data = None;
        let mut debug_name = None;
        capacity = Some(rdr.read_u32()?);
        stride = Some(rdr.read_u32()?);
        layout = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxLayoutElementDesc::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        initial_data = Some(VfxcpuBufferData::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
        } else if *ver >= crate::UnityVersion::new(6000, 4, 4) {
            debug_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(6000, 4, 0) {
        } else if *ver >= crate::UnityVersion::new(6000, 3, 14) {
            debug_name = Some(rdr.read_aligned_string()?);
        } else {
        }

        Ok(VfxcpuBufferDesc {
            capacity: capacity.unwrap(),
            stride: stride.unwrap(),
            layout: layout.unwrap(),
            initial_data: initial_data.unwrap(),
            debug_name,
        })
    }
}

impl crate::ToValue for VfxcpuBufferDesc {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("capacity".to_string(), self.capacity.to_value())),
            Some(("stride".to_string(), self.stride.to_value())),
            Some(("layout".to_string(), self.layout.to_value())),
            Some(("initialData".to_string(), self.initial_data.to_value())),
            crate::opt_field("debugName", &self.debug_name),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "capacity" => (self.capacity).inspect_value(path, offset, limit),
                "stride" => (self.stride).inspect_value(path, offset, limit),
                "layout" => (self.layout).inspect_value(path, offset, limit),
                "initialData" => (self.initial_data).inspect_value(path, offset, limit),
                "debugName" => (self.debug_name).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.capacity).inspect_value(path, offset, limit))
                .map(|v| ("capacity".to_string(), v)),
            ((self.stride).inspect_value(path, offset, limit)).map(|v| ("stride".to_string(), v)),
            ((self.layout).inspect_value(path, offset, limit)).map(|v| ("layout".to_string(), v)),
            ((self.initial_data).inspect_value(path, offset, limit))
                .map(|v| ("initialData".to_string(), v)),
            ((self.debug_name).inspect_value(path, offset, limit))
                .map(|v| ("debugName".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
