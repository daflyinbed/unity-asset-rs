//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_layout_element_desc::VfxLayoutElementDesc;

#[derive(Debug, Clone)]
pub struct VfxgpuBufferDesc {
    pub r#type: Option<i32>,
    pub target: Option<i32>,
    pub size: u32,
    pub layout: Vec<VfxLayoutElementDesc>,
    pub capacity: u32,
    pub stride: u32,
    pub mode: Option<i32>,
    pub debug_name: Option<String>,
}

impl crate::Parse for VfxgpuBufferDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut r#type = None;
        let mut target = None;
        let mut size = None;
        let mut layout = None;
        let mut capacity = None;
        let mut stride = None;
        let mut mode = None;
        let mut debug_name = None;
        if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            target = Some(rdr.read_i32()?);
        } else {
            r#type = Some(rdr.read_i32()?);
        }
        size = Some(rdr.read_u32()?);
        layout = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxLayoutElementDesc::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        capacity = Some(rdr.read_u32()?);
        stride = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(6000, 6, 0) {
            mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 4, 4) {
            mode = Some(rdr.read_i32()?);
            debug_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            mode = Some(rdr.read_i32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 3, 14) {
            mode = Some(rdr.read_i32()?);
            debug_name = Some(rdr.read_aligned_string()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 0) {
            mode = Some(rdr.read_i32()?);
        } else {
        }

        Ok(VfxgpuBufferDesc {
            r#type,
            target,
            size: size.unwrap(),
            layout: layout.unwrap(),
            capacity: capacity.unwrap(),
            stride: stride.unwrap(),
            mode,
            debug_name,
        })
    }
}

impl crate::ToValue for VfxgpuBufferDesc {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("type", &self.r#type),
            crate::opt_field("target", &self.target),
            Some(("size".to_string(), self.size.to_value())),
            Some(("layout".to_string(), self.layout.to_value())),
            Some(("capacity".to_string(), self.capacity.to_value())),
            Some(("stride".to_string(), self.stride.to_value())),
            crate::opt_field("mode", &self.mode),
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
                "type" => (self.r#type).inspect_value(path, offset, limit),
                "target" => (self.target).inspect_value(path, offset, limit),
                "size" => (self.size).inspect_value(path, offset, limit),
                "layout" => (self.layout).inspect_value(path, offset, limit),
                "capacity" => (self.capacity).inspect_value(path, offset, limit),
                "stride" => (self.stride).inspect_value(path, offset, limit),
                "mode" => (self.mode).inspect_value(path, offset, limit),
                "debugName" => (self.debug_name).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.r#type).inspect_value(path, offset, limit)).map(|v| ("type".to_string(), v)),
            ((self.target).inspect_value(path, offset, limit)).map(|v| ("target".to_string(), v)),
            ((self.size).inspect_value(path, offset, limit)).map(|v| ("size".to_string(), v)),
            ((self.layout).inspect_value(path, offset, limit)).map(|v| ("layout".to_string(), v)),
            ((self.capacity).inspect_value(path, offset, limit))
                .map(|v| ("capacity".to_string(), v)),
            ((self.stride).inspect_value(path, offset, limit)).map(|v| ("stride".to_string(), v)),
            ((self.mode).inspect_value(path, offset, limit)).map(|v| ("mode".to_string(), v)),
            ((self.debug_name).inspect_value(path, offset, limit))
                .map(|v| ("debugName".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
