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
