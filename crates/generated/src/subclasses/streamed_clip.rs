//! 自动生成。请勿手动修改。


/// 版本枚举：StreamedClipCurveCountVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum StreamedClipCurveCountVariant {
    Default(u32),
    V2022_3_19(u16),
    V2023_1(u32),
    V2023_2_8(u16),
}

impl crate::Parse for StreamedClipCurveCountVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2023, 2, 8) {
            let v = rdr.read_u16()?;
            Ok(Self::V2023_2_8(v))
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
            let v = rdr.read_u32()?;
            Ok(Self::V2023_1(v))
        } else if *ver >= crate::UnityVersion::new(2022, 3, 19) {
            let v = rdr.read_u16()?;
            Ok(Self::V2022_3_19(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct StreamedClip {
    pub data: Vec<u32>,
    pub curve_count: StreamedClipCurveCountVariant,
    pub discrete_curve_count: Option<u16>,
}

impl crate::Parse for StreamedClip {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut data = None;
        let mut curve_count = None;
        let mut discrete_curve_count = None;
            data = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
            curve_count = Some(StreamedClipCurveCountVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 2, 8) {
            discrete_curve_count = Some(rdr.read_u16()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2022, 3, 19) {
            discrete_curve_count = Some(rdr.read_u16()?);
        } else {
        }

        Ok(StreamedClip {
            data: data.unwrap(),
            curve_count: curve_count.unwrap(),
            discrete_curve_count,
        })
    }
}
