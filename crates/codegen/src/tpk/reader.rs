//! 小端字节读取器（带边界检查）。
//!
//! TPK 全程小端，与 Unity AssetBundle header 的大端相反。
//! 字符串编码采用 .NET `BinaryReader.ReadString()` 的 7-bit varint 长度前缀 + UTF-8。

use super::TpkError;

pub struct LeReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> LeReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    fn need(&self, n: usize, reason: &'static str) -> Result<(), TpkError> {
        if self.remaining() < n {
            return Err(TpkError::Truncated {
                reason,
                remaining: self.remaining(),
                needed: n,
            });
        }
        Ok(())
    }

    pub fn take(&mut self, n: usize, reason: &'static str) -> Result<&'a [u8], TpkError> {
        self.need(n, reason)?;
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    pub fn read_u8(&mut self, reason: &'static str) -> Result<u8, TpkError> {
        Ok(self.take(1, reason)?[0])
    }

    pub fn read_i16(&mut self, reason: &'static str) -> Result<i16, TpkError> {
        Ok(i16::from_le_bytes(
            self.take(2, reason)?.try_into().unwrap(),
        ))
    }

    pub fn read_u16(&mut self, reason: &'static str) -> Result<u16, TpkError> {
        Ok(u16::from_le_bytes(
            self.take(2, reason)?.try_into().unwrap(),
        ))
    }

    pub fn read_i32(&mut self, reason: &'static str) -> Result<i32, TpkError> {
        Ok(i32::from_le_bytes(
            self.take(4, reason)?.try_into().unwrap(),
        ))
    }

    pub fn read_u32(&mut self, reason: &'static str) -> Result<u32, TpkError> {
        Ok(u32::from_le_bytes(
            self.take(4, reason)?.try_into().unwrap(),
        ))
    }

    pub fn read_i64(&mut self, reason: &'static str) -> Result<i64, TpkError> {
        Ok(i64::from_le_bytes(
            self.take(8, reason)?.try_into().unwrap(),
        ))
    }

    pub fn read_u64(&mut self, reason: &'static str) -> Result<u64, TpkError> {
        Ok(u64::from_le_bytes(
            self.take(8, reason)?.try_into().unwrap(),
        ))
    }

    /// 读 .NET 7-bit varint 长度前缀 + UTF-8 字符串。
    ///
    /// 参考 `TpkStringBuffer.cs:30-39` + `BinaryReader.ReadString`：
    /// 每字节低 7 位是数据，最高位是 continuation flag。
    pub fn read_7bit_string(&mut self, reason: &'static str) -> Result<String, TpkError> {
        let mut length: usize = 0;
        let mut shift = 0u32;
        loop {
            let b = self.read_u8(reason)?;
            length |= ((b & 0x7F) as usize) << shift;
            if b & 0x80 == 0 {
                break;
            }
            shift += 7;
            if shift >= usize::BITS {
                return Err(TpkError::Truncated {
                    reason: "7-bit 长度前缀过长",
                    remaining: self.remaining(),
                    needed: 0,
                });
            }
        }
        let bytes = self.take(length, reason)?;
        let s = std::str::from_utf8(bytes)?;
        Ok(s.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_7bit_string_short() {
        // "abc" 长度 3，前缀单字节 0x03
        let buf = [0x03, b'a', b'b', b'c'];
        let mut r = LeReader::new(&buf);
        assert_eq!(r.read_7bit_string("test").unwrap(), "abc");
    }

    #[test]
    fn read_7bit_string_long() {
        // 长度 128 = 0x80, 多字节：0x80 0x01 表示 (0 | 1<<7) = 128
        let mut buf = vec![0x80, 0x01];
        buf.extend(std::iter::repeat_n(b'x', 128));
        let mut r = LeReader::new(&buf);
        let s = r.read_7bit_string("test").unwrap();
        assert_eq!(s.len(), 128);
    }
}
