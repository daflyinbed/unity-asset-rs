//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GlTextureSettings {
    pub m_filter_mode: i32,
    pub m_aniso: i32,
    pub m_mip_bias: f32,
    pub m_wrap_mode: Option<i32>,
    pub m_wrap_u: Option<i32>,
    pub m_wrap_v: Option<i32>,
    pub m_wrap_w: Option<i32>,
}

impl crate::Parse for GlTextureSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_filter_mode = None;
        let mut m_aniso = None;
        let mut m_mip_bias = None;
        let mut m_wrap_mode = None;
        let mut m_wrap_u = None;
        let mut m_wrap_v = None;
        let mut m_wrap_w = None;
        m_filter_mode = Some(rdr.read_i32()?);
        m_aniso = Some(rdr.read_i32()?);
        m_mip_bias = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            m_wrap_u = Some(rdr.read_i32()?);
            m_wrap_v = Some(rdr.read_i32()?);
            m_wrap_w = Some(rdr.read_i32()?);
        } else {
            m_wrap_mode = Some(rdr.read_i32()?);
        }

        Ok(GlTextureSettings {
            m_filter_mode: m_filter_mode.unwrap(),
            m_aniso: m_aniso.unwrap(),
            m_mip_bias: m_mip_bias.unwrap(),
            m_wrap_mode,
            m_wrap_u,
            m_wrap_v,
            m_wrap_w,
        })
    }
}

impl crate::ToValue for GlTextureSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_FilterMode".to_string(), self.m_filter_mode.to_value())),
            Some(("m_Aniso".to_string(), self.m_aniso.to_value())),
            Some(("m_MipBias".to_string(), self.m_mip_bias.to_value())),
            crate::opt_field("m_WrapMode", &self.m_wrap_mode),
            crate::opt_field("m_WrapU", &self.m_wrap_u),
            crate::opt_field("m_WrapV", &self.m_wrap_v),
            crate::opt_field("m_WrapW", &self.m_wrap_w),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_FilterMode" => (self.m_filter_mode).inspect_value(path, offset, limit),
                "m_Aniso" => (self.m_aniso).inspect_value(path, offset, limit),
                "m_MipBias" => (self.m_mip_bias).inspect_value(path, offset, limit),
                "m_WrapMode" => (self.m_wrap_mode).inspect_value(path, offset, limit),
                "m_WrapU" => (self.m_wrap_u).inspect_value(path, offset, limit),
                "m_WrapV" => (self.m_wrap_v).inspect_value(path, offset, limit),
                "m_WrapW" => (self.m_wrap_w).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_filter_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_FilterMode".to_string(), v)),
            ((self.m_aniso).inspect_value(path, offset, limit)).map(|v| ("m_Aniso".to_string(), v)),
            ((self.m_mip_bias).inspect_value(path, offset, limit))
                .map(|v| ("m_MipBias".to_string(), v)),
            ((self.m_wrap_mode).inspect_value(path, offset, limit))
                .map(|v| ("m_WrapMode".to_string(), v)),
            ((self.m_wrap_u).inspect_value(path, offset, limit))
                .map(|v| ("m_WrapU".to_string(), v)),
            ((self.m_wrap_v).inspect_value(path, offset, limit))
                .map(|v| ("m_WrapV".to_string(), v)),
            ((self.m_wrap_w).inspect_value(path, offset, limit))
                .map(|v| ("m_WrapW".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
