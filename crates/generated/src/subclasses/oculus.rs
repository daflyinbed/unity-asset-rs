//! 自动生成。请勿手动修改。

/// 版本枚举：OculusDashSupportVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum OculusDashSupportVariant {
    Default(bool),
    V2018_4_9(bool),
    V2019_1(bool),
    V2019_2(bool),
}

impl crate::Parse for OculusDashSupportVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V2019_2(v))
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2019_1(v))
        } else if *ver >= crate::UnityVersion::new(2018, 4, 9) {
            let v = rdr.read_bool()?;
            Ok(Self::V2018_4_9(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for OculusDashSupportVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2018_4_9(v) => v.to_value(),
            Self::V2019_1(v) => v.to_value(),
            Self::V2019_2(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2018_4_9(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_1(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_2(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：OculusLowOverheadModeVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum OculusLowOverheadModeVariant {
    Default(bool),
    V2019_2(bool),
    V2019_2_3(bool),
}

impl crate::Parse for OculusLowOverheadModeVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2019, 2, 3) {
            let v = rdr.read_bool()?;
            Ok(Self::V2019_2_3(v))
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::V2019_2(v))
        } else {
            let v = rdr.read_bool()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for OculusLowOverheadModeVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2019_2(v) => v.to_value(),
            Self::V2019_2_3(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_2(v) => (v).inspect_value(path, offset, limit),
            Self::V2019_2_3(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Oculus {
    pub shared_depth_buffer: bool,
    pub dash_support: OculusDashSupportVariant,
    pub low_overhead_mode: Option<OculusLowOverheadModeVariant>,
    pub protected_context: Option<bool>,
    pub v2_signing: Option<bool>,
}

impl crate::Parse for Oculus {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut shared_depth_buffer = None;
        let mut dash_support = None;
        let mut low_overhead_mode = None;
        let mut protected_context = None;
        let mut v2_signing = None;
        shared_depth_buffer = Some(rdr.read_bool()?);
        dash_support = Some(OculusDashSupportVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2019, 2, 3) {
            low_overhead_mode = Some(OculusLowOverheadModeVariant::parse(rdr, ver)?);
            protected_context = Some(rdr.read_bool()?);
            v2_signing = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            low_overhead_mode = Some(OculusLowOverheadModeVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(2019, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2018, 4, 9) {
            low_overhead_mode = Some(OculusLowOverheadModeVariant::parse(rdr, ver)?);
            protected_context = Some(rdr.read_bool()?);
            v2_signing = Some(rdr.read_bool()?);
            rdr.align(4)?;
        } else {
        }

        Ok(Oculus {
            shared_depth_buffer: shared_depth_buffer.unwrap(),
            dash_support: dash_support.unwrap(),
            low_overhead_mode,
            protected_context,
            v2_signing,
        })
    }
}

impl crate::ToValue for Oculus {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "sharedDepthBuffer".to_string(),
                self.shared_depth_buffer.to_value(),
            )),
            Some(("dashSupport".to_string(), self.dash_support.to_value())),
            crate::opt_field("lowOverheadMode", &self.low_overhead_mode),
            crate::opt_field("protectedContext", &self.protected_context),
            crate::opt_field("v2Signing", &self.v2_signing),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "sharedDepthBuffer" => {
                    (self.shared_depth_buffer).inspect_value(path, offset, limit)
                }
                "dashSupport" => (self.dash_support).inspect_value(path, offset, limit),
                "lowOverheadMode" => (self.low_overhead_mode).inspect_value(path, offset, limit),
                "protectedContext" => (self.protected_context).inspect_value(path, offset, limit),
                "v2Signing" => (self.v2_signing).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.shared_depth_buffer).inspect_value(path, offset, limit))
                .map(|v| ("sharedDepthBuffer".to_string(), v)),
            ((self.dash_support).inspect_value(path, offset, limit))
                .map(|v| ("dashSupport".to_string(), v)),
            ((self.low_overhead_mode).inspect_value(path, offset, limit))
                .map(|v| ("lowOverheadMode".to_string(), v)),
            ((self.protected_context).inspect_value(path, offset, limit))
                .map(|v| ("protectedContext".to_string(), v)),
            ((self.v2_signing).inspect_value(path, offset, limit))
                .map(|v| ("v2Signing".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
