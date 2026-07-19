//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SnapshotConstant {
    pub name_hash: u32,
    pub values: Vec<f32>,
    pub transition_types: Vec<u32>,
    pub transition_indices: Vec<u32>,
}

impl crate::Parse for SnapshotConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name_hash = rdr.read_u32()?;
        let values = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_f32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let transition_types = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let transition_indices = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(SnapshotConstant {
            name_hash,
            values,
            transition_types,
            transition_indices,
        })
    }
}
