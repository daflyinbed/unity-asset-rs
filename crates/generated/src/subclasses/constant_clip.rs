//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ConstantClip {
    pub data: Vec<f32>,
}

impl crate::Parse for ConstantClip {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let data = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_f32())
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(ConstantClip { data })
    }
}
