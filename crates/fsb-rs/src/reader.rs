//! A small bounds-checked cursor over a byte slice.
//!
//! Stand-in for C#'s `BinaryReader`. Unlike `BinaryReader`, reads are little-endian
//! by default (matching the FSB5 format) with explicit big-endian helpers for the
//! few fields that need them (e.g. GCADPCM DSP coefficients).

use crate::error::FsbError;

/// Sequential reader over an immutable byte slice. Tracks a position and reports
/// [`FsbError::UnexpectedEof`] on any read past the end.
pub(crate) struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// The full underlying buffer (not affected by the cursor position).
    pub(crate) fn full(&self) -> &'a [u8] {
        self.buf
    }

    pub(crate) fn position(&self) -> usize {
        self.pos
    }

    #[allow(dead_code)]
    pub(crate) fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.pos)
    }

    #[allow(dead_code)]
    pub(crate) fn seek(&mut self, pos: usize) -> Result<(), FsbError> {
        if pos > self.buf.len() {
            return Err(FsbError::UnexpectedEof);
        }
        self.pos = pos;
        Ok(())
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], FsbError> {
        let end = self
            .pos
            .checked_add(n)
            .filter(|&e| e <= self.buf.len())
            .ok_or(FsbError::UnexpectedEof)?;
        let out = &self.buf[self.pos..end];
        self.pos = end;
        Ok(out)
    }

    pub(crate) fn read_bytes(&mut self, n: usize) -> Result<&'a [u8], FsbError> {
        self.take(n)
    }

    /// Skip `n` bytes without copying them.
    pub(crate) fn skip(&mut self, n: usize) -> Result<(), FsbError> {
        self.take(n).map(|_| ())
    }

    pub(crate) fn read_u8(&mut self) -> Result<u8, FsbError> {
        Ok(self.take(1)?[0])
    }

    #[allow(dead_code)]
    pub(crate) fn read_u16_le(&mut self) -> Result<u16, FsbError> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    pub(crate) fn read_u32_le(&mut self) -> Result<u32, FsbError> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    pub(crate) fn read_u64_le(&mut self) -> Result<u64, FsbError> {
        let b = self.take(8)?;
        Ok(u64::from_le_bytes([
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
        ]))
    }

    /// Big-endian `i16`, used for GCADPCM DSP coefficients (see `DspCoefficientsBlockData`).
    pub(crate) fn read_i16_be(&mut self) -> Result<i16, FsbError> {
        let b = self.take(2)?;
        Ok(i16::from_be_bytes([b[0], b[1]]))
    }

    /// Read up to `max` bytes, stopping after the first `0x00`. The returned slice
    /// excludes the terminator. Used for the sample name table.
    #[allow(dead_code)]
    pub(crate) fn read_null_terminated(&mut self, max: usize) -> Result<&'a [u8], FsbError> {
        let end = self.buf[self.pos..]
            .iter()
            .take(max)
            .position(|&b| b == 0)
            .map(|p| self.pos + p)
            .unwrap_or_else(|| self.buf.len().min(self.pos + max));
        let out = self.take(end - self.pos)?;
        // Consume the terminator if present (but not past `max`).
        if self.pos < self.buf.len() && self.buf[self.pos] == 0 && out.len() < max {
            self.pos += 1;
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_le_values() {
        let mut r = Reader::new(&[0x01, 0x00, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
        assert_eq!(r.read_u16_le().unwrap(), 1);
        // Little-endian read of [0x02,0x03,0x04,0x05] = 0x05040302.
        assert_eq!(r.read_u32_le().unwrap(), 0x05040302);
        assert_eq!(r.position(), 6);
        assert!(r.read_u32_le().is_err()); // only 2 bytes left
    }

    #[test]
    fn reads_be_i16() {
        let mut r = Reader::new(&[0x12, 0x34]);
        assert_eq!(r.read_i16_be().unwrap(), 0x1234);
    }

    #[test]
    fn null_terminated_strips_terminator() {
        let mut r = Reader::new(b"hello\x00world");
        let s = r.read_null_terminated(16).unwrap();
        assert_eq!(s, b"hello");
        assert_eq!(r.position(), 6); // consumed the 0x00
    }
}
