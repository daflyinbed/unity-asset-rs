use crate::UnityVersion;

/// 字节序。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Little,
    Big,
}

/// 字节序 reader 抽象。
///
/// 生成代码通过此 trait 读字节，实际实现由主 crate 的 `Cursor` 提供
/// （`impl EndianReader for Cursor`）。
pub trait EndianReader {
    type Error;
    fn read_bool(&mut self) -> Result<bool, Self::Error>;
    fn read_i8(&mut self) -> Result<i8, Self::Error>;
    fn read_u8(&mut self) -> Result<u8, Self::Error>;
    fn read_i16(&mut self) -> Result<i16, Self::Error>;
    fn read_u16(&mut self) -> Result<u16, Self::Error>;
    fn read_i32(&mut self) -> Result<i32, Self::Error>;
    fn read_u32(&mut self) -> Result<u32, Self::Error>;
    fn read_i64(&mut self) -> Result<i64, Self::Error>;
    fn read_u64(&mut self) -> Result<u64, Self::Error>;
    fn read_f32(&mut self) -> Result<f32, Self::Error>;
    fn read_f64(&mut self) -> Result<f64, Self::Error>;
    fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, Self::Error>;
    fn read_aligned_string(&mut self) -> Result<String, Self::Error>;
    fn align(&mut self, n: usize) -> Result<(), Self::Error>;

    /// 从消息字符串构造错误。用于生成代码在不支持的 class_id 时返回 Err 而非 panic。
    fn make_error(&self, msg: &str) -> Self::Error;
}

/// 解析 trait。每个生成的 struct 实现它。
pub trait Parse: Sized {
    fn parse<R: EndianReader>(r: &mut R, ver: &UnityVersion) -> Result<Self, R::Error>;
}
