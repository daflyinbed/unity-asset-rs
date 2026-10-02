//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GfxStencilState {
    pub stencil_enable: u8,
    pub read_mask: u8,
    pub write_mask: u8,
    pub padding: u8,
    pub stencil_func_front: u8,
    pub stencil_pass_op_front: u8,
    pub stencil_fail_op_front: u8,
    pub stencil_z_fail_op_front: u8,
    pub stencil_func_back: u8,
    pub stencil_pass_op_back: u8,
    pub stencil_fail_op_back: u8,
    pub stencil_z_fail_op_back: u8,
}

impl crate::Parse for GfxStencilState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let stencil_enable = rdr.read_u8()?;
        let read_mask = rdr.read_u8()?;
        let write_mask = rdr.read_u8()?;
        let padding = rdr.read_u8()?;
        let stencil_func_front = rdr.read_u8()?;
        let stencil_pass_op_front = rdr.read_u8()?;
        let stencil_fail_op_front = rdr.read_u8()?;
        let stencil_z_fail_op_front = rdr.read_u8()?;
        let stencil_func_back = rdr.read_u8()?;
        let stencil_pass_op_back = rdr.read_u8()?;
        let stencil_fail_op_back = rdr.read_u8()?;
        let stencil_z_fail_op_back = rdr.read_u8()?;

        Ok(GfxStencilState {
            stencil_enable,
            read_mask,
            write_mask,
            padding,
            stencil_func_front,
            stencil_pass_op_front,
            stencil_fail_op_front,
            stencil_z_fail_op_front,
            stencil_func_back,
            stencil_pass_op_back,
            stencil_fail_op_back,
            stencil_z_fail_op_back,
        })
    }
}

impl crate::ToValue for GfxStencilState {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("stencilEnable".to_string(), self.stencil_enable.to_value())),
            Some(("readMask".to_string(), self.read_mask.to_value())),
            Some(("writeMask".to_string(), self.write_mask.to_value())),
            Some(("padding".to_string(), self.padding.to_value())),
            Some((
                "stencilFuncFront".to_string(),
                self.stencil_func_front.to_value(),
            )),
            Some((
                "stencilPassOpFront".to_string(),
                self.stencil_pass_op_front.to_value(),
            )),
            Some((
                "stencilFailOpFront".to_string(),
                self.stencil_fail_op_front.to_value(),
            )),
            Some((
                "stencilZFailOpFront".to_string(),
                self.stencil_z_fail_op_front.to_value(),
            )),
            Some((
                "stencilFuncBack".to_string(),
                self.stencil_func_back.to_value(),
            )),
            Some((
                "stencilPassOpBack".to_string(),
                self.stencil_pass_op_back.to_value(),
            )),
            Some((
                "stencilFailOpBack".to_string(),
                self.stencil_fail_op_back.to_value(),
            )),
            Some((
                "stencilZFailOpBack".to_string(),
                self.stencil_z_fail_op_back.to_value(),
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
                "stencilEnable" => (self.stencil_enable).inspect_value(path, offset, limit),
                "readMask" => (self.read_mask).inspect_value(path, offset, limit),
                "writeMask" => (self.write_mask).inspect_value(path, offset, limit),
                "padding" => (self.padding).inspect_value(path, offset, limit),
                "stencilFuncFront" => (self.stencil_func_front).inspect_value(path, offset, limit),
                "stencilPassOpFront" => {
                    (self.stencil_pass_op_front).inspect_value(path, offset, limit)
                }
                "stencilFailOpFront" => {
                    (self.stencil_fail_op_front).inspect_value(path, offset, limit)
                }
                "stencilZFailOpFront" => {
                    (self.stencil_z_fail_op_front).inspect_value(path, offset, limit)
                }
                "stencilFuncBack" => (self.stencil_func_back).inspect_value(path, offset, limit),
                "stencilPassOpBack" => {
                    (self.stencil_pass_op_back).inspect_value(path, offset, limit)
                }
                "stencilFailOpBack" => {
                    (self.stencil_fail_op_back).inspect_value(path, offset, limit)
                }
                "stencilZFailOpBack" => {
                    (self.stencil_z_fail_op_back).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.stencil_enable).inspect_value(path, offset, limit))
                .map(|v| ("stencilEnable".to_string(), v)),
            ((self.read_mask).inspect_value(path, offset, limit))
                .map(|v| ("readMask".to_string(), v)),
            ((self.write_mask).inspect_value(path, offset, limit))
                .map(|v| ("writeMask".to_string(), v)),
            ((self.padding).inspect_value(path, offset, limit)).map(|v| ("padding".to_string(), v)),
            ((self.stencil_func_front).inspect_value(path, offset, limit))
                .map(|v| ("stencilFuncFront".to_string(), v)),
            ((self.stencil_pass_op_front).inspect_value(path, offset, limit))
                .map(|v| ("stencilPassOpFront".to_string(), v)),
            ((self.stencil_fail_op_front).inspect_value(path, offset, limit))
                .map(|v| ("stencilFailOpFront".to_string(), v)),
            ((self.stencil_z_fail_op_front).inspect_value(path, offset, limit))
                .map(|v| ("stencilZFailOpFront".to_string(), v)),
            ((self.stencil_func_back).inspect_value(path, offset, limit))
                .map(|v| ("stencilFuncBack".to_string(), v)),
            ((self.stencil_pass_op_back).inspect_value(path, offset, limit))
                .map(|v| ("stencilPassOpBack".to_string(), v)),
            ((self.stencil_fail_op_back).inspect_value(path, offset, limit))
                .map(|v| ("stencilFailOpBack".to_string(), v)),
            ((self.stencil_z_fail_op_back).inspect_value(path, offset, limit))
                .map(|v| ("stencilZFailOpBack".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
