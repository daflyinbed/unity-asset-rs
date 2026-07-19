//! 自动生成。请勿手动修改。

use crate::subclasses::channel::Channel;

/// 版本枚举：NewAnimationTrackMCurvesVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum NewAnimationTrackMCurvesVariant {
    Default(Vec<Channel>),
    V2017_1(Vec<Channel>),
}

impl crate::Parse for NewAnimationTrackMCurvesVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Channel::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Channel::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct NewAnimationTrack {
    pub m_name: String,
    pub m_curves: NewAnimationTrackMCurvesVariant,
    pub m_class_id: i32,
}

impl crate::Parse for NewAnimationTrack {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_curves = NewAnimationTrackMCurvesVariant::parse(rdr, ver)?;
        let m_class_id = rdr.read_i32()?;

        Ok(NewAnimationTrack {
            m_name,
            m_curves,
            m_class_id,
        })
    }
}
