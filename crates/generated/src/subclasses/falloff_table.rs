//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct FalloffTable {
    pub m_table_0: f32,
    pub m_table_1: f32,
    pub m_table_2: f32,
    pub m_table_3: f32,
    pub m_table_4: f32,
    pub m_table_5: f32,
    pub m_table_6: f32,
    pub m_table_7: f32,
    pub m_table_8: f32,
    pub m_table_9: f32,
    pub m_table_10: f32,
    pub m_table_11: f32,
    pub m_table_12: f32,
}

impl crate::Parse for FalloffTable {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_table_0 = r.read_f32()?;
        let m_table_1 = r.read_f32()?;
        let m_table_2 = r.read_f32()?;
        let m_table_3 = r.read_f32()?;
        let m_table_4 = r.read_f32()?;
        let m_table_5 = r.read_f32()?;
        let m_table_6 = r.read_f32()?;
        let m_table_7 = r.read_f32()?;
        let m_table_8 = r.read_f32()?;
        let m_table_9 = r.read_f32()?;
        let m_table_10 = r.read_f32()?;
        let m_table_11 = r.read_f32()?;
        let m_table_12 = r.read_f32()?;

        Ok(FalloffTable {
            m_table_0,
            m_table_1,
            m_table_2,
            m_table_3,
            m_table_4,
            m_table_5,
            m_table_6,
            m_table_7,
            m_table_8,
            m_table_9,
            m_table_10,
            m_table_11,
            m_table_12,
        })
    }
}
