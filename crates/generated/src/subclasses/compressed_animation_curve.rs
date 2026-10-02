//! 自动生成。请勿手动修改。

use crate::subclasses::packed_bit_vector_int32::PackedBitVector_Int32;
use crate::subclasses::packed_bit_vector_quaternionf::PackedBitVector_Quaternionf;
use crate::subclasses::packed_bit_vector_single::PackedBitVector_Single;

#[derive(Debug, Clone)]
pub struct CompressedAnimationCurve {
    pub m_path: String,
    pub m_times: PackedBitVector_Int32,
    pub m_values: PackedBitVector_Quaternionf,
    pub m_slopes: PackedBitVector_Single,
    pub m_pre_infinity: i32,
    pub m_post_infinity: i32,
}

impl crate::Parse for CompressedAnimationCurve {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_path = rdr.read_aligned_string()?;
        let m_times = PackedBitVector_Int32::parse(rdr, ver)?;
        let m_values = PackedBitVector_Quaternionf::parse(rdr, ver)?;
        let m_slopes = PackedBitVector_Single::parse(rdr, ver)?;
        let m_pre_infinity = rdr.read_i32()?;
        let m_post_infinity = rdr.read_i32()?;

        Ok(CompressedAnimationCurve {
            m_path,
            m_times,
            m_values,
            m_slopes,
            m_pre_infinity,
            m_post_infinity,
        })
    }
}

impl crate::ToValue for CompressedAnimationCurve {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Path".to_string(), self.m_path.to_value())),
            Some(("m_Times".to_string(), self.m_times.to_value())),
            Some(("m_Values".to_string(), self.m_values.to_value())),
            Some(("m_Slopes".to_string(), self.m_slopes.to_value())),
            Some(("m_PreInfinity".to_string(), self.m_pre_infinity.to_value())),
            Some((
                "m_PostInfinity".to_string(),
                self.m_post_infinity.to_value(),
            )),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Path" => (self.m_path).inspect_value(path, offset, limit),
                "m_Times" => (self.m_times).inspect_value(path, offset, limit),
                "m_Values" => (self.m_values).inspect_value(path, offset, limit),
                "m_Slopes" => (self.m_slopes).inspect_value(path, offset, limit),
                "m_PreInfinity" => (self.m_pre_infinity).inspect_value(path, offset, limit),
                "m_PostInfinity" => (self.m_post_infinity).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_path).inspect_value(path, offset, limit)).map(|v| ("m_Path".to_string(), v)),
            ((self.m_times).inspect_value(path, offset, limit)).map(|v| ("m_Times".to_string(), v)),
            ((self.m_values).inspect_value(path, offset, limit))
                .map(|v| ("m_Values".to_string(), v)),
            ((self.m_slopes).inspect_value(path, offset, limit))
                .map(|v| ("m_Slopes".to_string(), v)),
            ((self.m_pre_infinity).inspect_value(path, offset, limit))
                .map(|v| ("m_PreInfinity".to_string(), v)),
            ((self.m_post_infinity).inspect_value(path, offset, limit))
                .map(|v| ("m_PostInfinity".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
