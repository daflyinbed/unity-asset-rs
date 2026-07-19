//! 自动生成。请勿手动修改。

use crate::subclasses::cluster_input::ClusterInput;

/// 版本枚举：ClusterInputManagerMInputsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum ClusterInputManagerMInputsVariant {
    Default(Vec<ClusterInput>),
    V2017_1(Vec<ClusterInput>),
}

impl crate::Parse for ClusterInputManagerMInputsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ClusterInput::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ClusterInput::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct ClusterInputManager {
    pub m_inputs: ClusterInputManagerMInputsVariant,
}

impl crate::Parse for ClusterInputManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_inputs = ClusterInputManagerMInputsVariant::parse(rdr, ver)?;

        Ok(ClusterInputManager { m_inputs })
    }
}
