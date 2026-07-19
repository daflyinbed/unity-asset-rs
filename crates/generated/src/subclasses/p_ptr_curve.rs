//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::p_ptr_keyframe::PPtrKeyframe;

/// 版本枚举：PPtrCurveCurveVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum PPtrCurveCurveVariant {
    Default(Vec<PPtrKeyframe>),
    V2017_1(Vec<PPtrKeyframe>),
}

impl crate::Parse for PPtrCurveCurveVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| PPtrKeyframe::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| PPtrKeyframe::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct PPtrCurve {
    pub curve: PPtrCurveCurveVariant,
    pub attribute: String,
    pub path: String,
    pub class_id: i32,
    pub script: PPtr,
    pub flags: Option<i32>,
}

impl crate::Parse for PPtrCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let curve = PPtrCurveCurveVariant::parse(rdr, ver)?;
        let attribute = rdr.read_aligned_string()?;
        let path = rdr.read_aligned_string()?;
        let class_id = rdr.read_i32()?;
        let script = crate::PPtr::parse(rdr, ver)?;
        let flags = if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            Some(rdr.read_i32()?)
        } else {
            None
        };

        Ok(PPtrCurve {
            curve,
            attribute,
            path,
            class_id,
            script,
            flags,
        })
    }
}
