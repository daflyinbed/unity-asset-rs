//! 通用带边界检查的游标读取器，支持运行时切换字节序。
//!
//! - `bundle.rs` 用 `Endian::Big`（UnityFS 在磁盘上是 大端）。
//! - `SerializedFile` 读到 endian 字节后切换。
//! - TypeTree 解释器继承 SerializedFile 的 endian。
//! - 通过 `impl EndianReader for Cursor` 桥接 generated crate 的 `Parse` trait。

use crate::error::{DecompressSnafu, Result, TruncatedSnafu};
pub use unity_core::Endian;

/// 可切换字节序的带边界检查游标读取器。
pub struct Cursor<'a> {
    buf: &'a [u8],
    pos: usize,
    endian: Endian,
}

impl<'a> Cursor<'a> {
    pub fn new(buf: &'a [u8], endian: Endian) -> Self {
        Self {
            buf,
            pos: 0,
            endian,
        }
    }

    pub fn endian(&self) -> Endian {
        self.endian
    }

    /// 切换字节序（读 SerializedFile 的 endian 字节后调用）。
    pub fn set_endian(&mut self, endian: Endian) {
        self.endian = endian;
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    /// 将游标跳到 `pos`（不做边界检查；调用方负责）。
    pub fn seek(&mut self, pos: usize) {
        self.pos = pos;
    }

    fn need(&self, n: usize, what: &str) -> Result<()> {
        if self.remaining() < n {
            return Err(TruncatedSnafu {
                reason: what.to_string(),
            }
            .build());
        }
        Ok(())
    }

    /// 取 `n` 字节切片并前进游标。
    pub fn take(&mut self, n: usize, what: &str) -> Result<&'a [u8]> {
        self.need(n, what)?;
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    // ── 单字节（无字节序）──

    pub fn read_u8(&mut self) -> Result<u8> {
        Ok(self.take(1, "u8")?[0])
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    // ── 多字节整数（按 endian）──

    pub fn read_u16(&mut self, what: &str) -> Result<u16> {
        let b: [u8; 2] = self.take(2, what)?.try_into().unwrap();
        Ok(match self.endian {
            Endian::Big => u16::from_be_bytes(b),
            Endian::Little => u16::from_le_bytes(b),
        })
    }

    pub fn read_i16(&mut self, what: &str) -> Result<i16> {
        Ok(self.read_u16(what)? as i16)
    }

    pub fn read_u32(&mut self, what: &str) -> Result<u32> {
        let b: [u8; 4] = self.take(4, what)?.try_into().unwrap();
        Ok(match self.endian {
            Endian::Big => u32::from_be_bytes(b),
            Endian::Little => u32::from_le_bytes(b),
        })
    }

    pub fn read_i32(&mut self, what: &str) -> Result<i32> {
        Ok(self.read_u32(what)? as i32)
    }

    pub fn read_u64(&mut self, what: &str) -> Result<u64> {
        let b: [u8; 8] = self.take(8, what)?.try_into().unwrap();
        Ok(match self.endian {
            Endian::Big => u64::from_be_bytes(b),
            Endian::Little => u64::from_le_bytes(b),
        })
    }

    pub fn read_i64(&mut self, what: &str) -> Result<i64> {
        let b: [u8; 8] = self.take(8, what)?.try_into().unwrap();
        Ok(match self.endian {
            Endian::Big => i64::from_be_bytes(b),
            Endian::Little => i64::from_le_bytes(b),
        })
    }

    // ── 浮点（位模式按 endian）──

    pub fn read_f32(&mut self, what: &str) -> Result<f32> {
        Ok(f32::from_bits(self.read_u32(what)?))
    }

    pub fn read_f64(&mut self, what: &str) -> Result<f64> {
        Ok(f64::from_bits(self.read_u64(what)?))
    }

    // ── 字节块 / 字符串 ──

    pub fn read_bytes(&mut self, n: usize, what: &str) -> Result<&'a [u8]> {
        self.take(n, what)
    }

    /// 读到第一个 0x00 为止（不含终止符）。
    pub fn read_cstr(&mut self, what: &str) -> Result<String> {
        let start = self.pos;
        let end = self.buf[start..]
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| TruncatedSnafu {
                reason: what.to_string(),
            }
            .build())?;
        let s = std::str::from_utf8(&self.buf[start..start + end])
            .map_err(|e| DecompressSnafu {
                reason: format!("非 UTF-8 字符串: {e}"),
            }
            .build())?
            .to_owned();
        self.pos = start + end + 1;
        Ok(s)
    }

    /// 对齐字符串：i32 长度 + bytes + 对齐到 4。
    pub fn read_aligned_string(&mut self, what: &str) -> Result<String> {
        let n = self.read_i32(what)? as usize;
        let b = self.take(n, what)?;
        let s = std::str::from_utf8(b)
            .map_err(|e| DecompressSnafu {
                reason: format!("非 UTF-8 字符串: {e}"),
            }
            .build())?
            .to_owned();
        self.align(4);
        Ok(s)
    }

    /// 把游标对齐到 `alignment` 的倍数。
    pub fn align(&mut self, alignment: usize) {
        let r = self.pos % alignment;
        if r != 0 {
            self.pos += alignment - r;
        }
    }
}

// ────────────────────────────────────────────────────────────────────
// EndianReader impl：桥接 generated crate 的 Parse trait
// ────────────────────────────────────────────────────────────────────

impl unity_core::EndianReader for Cursor<'_> {
    type Error = crate::error::Error;

    fn read_bool(&mut self) -> Result<bool, Self::Error> {
        self.read_u8().map(|b| b != 0)
    }

    fn read_i8(&mut self) -> Result<i8, Self::Error> {
        self.read_i8()
    }

    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        self.read_u8()
    }

    fn read_i16(&mut self) -> Result<i16, Self::Error> {
        self.read_i16("typed")
    }

    fn read_u16(&mut self) -> Result<u16, Self::Error> {
        self.read_u16("typed")
    }

    fn read_i32(&mut self) -> Result<i32, Self::Error> {
        self.read_i32("typed")
    }

    fn read_u32(&mut self) -> Result<u32, Self::Error> {
        self.read_u32("typed")
    }

    fn read_i64(&mut self) -> Result<i64, Self::Error> {
        self.read_i64("typed")
    }

    fn read_u64(&mut self) -> Result<u64, Self::Error> {
        self.read_u64("typed")
    }

    fn read_f32(&mut self) -> Result<f32, Self::Error> {
        self.read_f32("typed")
    }

    fn read_f64(&mut self) -> Result<f64, Self::Error> {
        self.read_f64("typed")
    }

    fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, Self::Error> {
        Ok(self.read_bytes(n, "typed")?.to_vec())
    }

    fn read_aligned_string(&mut self) -> Result<String, Self::Error> {
        self.read_aligned_string("typed")
    }

    fn align(&mut self, n: usize) -> Result<(), Self::Error> {
        Cursor::align(self, n);
        Ok(())
    }

    fn make_error(&self, msg: &str) -> Self::Error {
        crate::error::Error::TypedParse {
            reason: msg.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endian_switch() {
        let buf = [0x01u8, 0x02, 0x03, 0x04];
        let mut r = Cursor::new(&buf, Endian::Big);
        assert_eq!(r.read_u32("t").unwrap(), 0x01020304);
        r.seek(0);
        r.set_endian(Endian::Little);
        assert_eq!(r.read_u32("t").unwrap(), 0x04030201);
    }

    #[test]
    fn cstr_and_align() {
        let mut buf = vec![b'a', b'b', b'\0'];
        buf.extend_from_slice(&[0u8; 3]); // 填充到 4 对齐
        let mut r = Cursor::new(&buf, Endian::Big);
        assert_eq!(r.read_cstr("cstr").unwrap(), "ab");
        assert_eq!(r.pos(), 3);
        r.align(4);
        assert_eq!(r.pos(), 4);
    }
}
