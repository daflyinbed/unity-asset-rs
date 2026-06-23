//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Expression {
    pub op: i32,
    pub value_index: i32,
    pub data_0: i32,
    pub data_1: i32,
    pub data_2: i32,
    pub data_3: i32,
}

impl crate::Parse for Expression {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let op = r.read_i32()?;
        let value_index = r.read_i32()?;
        let data_0 = r.read_i32()?;
        let data_1 = r.read_i32()?;
        let data_2 = r.read_i32()?;
        let data_3 = r.read_i32()?;

        Ok(Expression {
            op,
            value_index,
            data_0,
            data_1,
            data_2,
            data_3,
        })
    }
}
