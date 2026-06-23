//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ConstantClip {
    pub data: Vec<f32>,
}

impl crate::Parse for ConstantClip {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;

        Ok(ConstantClip {
            data,
        })
    }
}
