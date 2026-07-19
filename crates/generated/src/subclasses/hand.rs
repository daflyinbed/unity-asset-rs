//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Hand {
    pub m_hand_bone_index: Vec<i32>,
}

impl crate::Parse for Hand {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_hand_bone_index = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i32())
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(Hand { m_hand_bone_index })
    }
}
