//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Output {
    pub preview_data: Vec<f32>,
    pub imported_type: i32,
}

impl crate::Parse for Output {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let preview_data = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_f32() }).collect::<Result<Vec<_>, _>>() } )?;
        let imported_type = r.read_i32()?;

        Ok(Output {
            preview_data,
            imported_type,
        })
    }
}
