//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct Hash128 {
    pub bytes_0: u8,
    pub bytes_1: u8,
    pub bytes_2: u8,
    pub bytes_3: u8,
    pub bytes_4: u8,
    pub bytes_5: u8,
    pub bytes_6: u8,
    pub bytes_7: u8,
    pub bytes_8: u8,
    pub bytes_9: u8,
    pub bytes_10: u8,
    pub bytes_11: u8,
    pub bytes_12: u8,
    pub bytes_13: u8,
    pub bytes_14: u8,
    pub bytes_15: u8,
}

impl crate::Parse for Hash128 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let bytes_0 = rdr.read_u8()?;
        let bytes_1 = rdr.read_u8()?;
        let bytes_2 = rdr.read_u8()?;
        let bytes_3 = rdr.read_u8()?;
        let bytes_4 = rdr.read_u8()?;
        let bytes_5 = rdr.read_u8()?;
        let bytes_6 = rdr.read_u8()?;
        let bytes_7 = rdr.read_u8()?;
        let bytes_8 = rdr.read_u8()?;
        let bytes_9 = rdr.read_u8()?;
        let bytes_10 = rdr.read_u8()?;
        let bytes_11 = rdr.read_u8()?;
        let bytes_12 = rdr.read_u8()?;
        let bytes_13 = rdr.read_u8()?;
        let bytes_14 = rdr.read_u8()?;
        let bytes_15 = rdr.read_u8()?;

        Ok(Hash128 {
            bytes_0,
            bytes_1,
            bytes_2,
            bytes_3,
            bytes_4,
            bytes_5,
            bytes_6,
            bytes_7,
            bytes_8,
            bytes_9,
            bytes_10,
            bytes_11,
            bytes_12,
            bytes_13,
            bytes_14,
            bytes_15,
        })
    }
}
