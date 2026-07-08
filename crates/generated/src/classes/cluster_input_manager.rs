//! 自动生成。请勿手动修改。

use crate::subclasses::cluster_input::ClusterInput;

#[derive(Debug, Clone)]
pub struct ClusterInputManager {
    pub m_inputs: Vec<ClusterInput>,
}

impl crate::Parse for ClusterInputManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_inputs = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { ClusterInput::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(ClusterInputManager {
            m_inputs,
        })
    }
}
