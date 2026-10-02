//! 自动生成。请勿手动修改。

use crate::subclasses::motion_neighbor_list::MotionNeighborList;
use crate::subclasses::vector2f::Vector2f;

#[derive(Debug, Clone)]
pub struct Blend2dDataConstant {
    pub m_child_position_array: Vec<Vector2f>,
    pub m_child_magnitude_array: Vec<f32>,
    pub m_child_pair_vector_array: Vec<Vector2f>,
    pub m_child_pair_avg_mag_inv_array: Vec<f32>,
    pub m_child_neighbor_list_array: Vec<MotionNeighborList>,
}

impl crate::Parse for Blend2dDataConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_child_position_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Vector2f::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_child_magnitude_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_f32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_child_pair_vector_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Vector2f::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_child_pair_avg_mag_inv_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_f32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_child_neighbor_list_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| MotionNeighborList::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(Blend2dDataConstant {
            m_child_position_array,
            m_child_magnitude_array,
            m_child_pair_vector_array,
            m_child_pair_avg_mag_inv_array,
            m_child_neighbor_list_array,
        })
    }
}

impl crate::ToValue for Blend2dDataConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_ChildPositionArray".to_string(),
                self.m_child_position_array.to_value(),
            )),
            Some((
                "m_ChildMagnitudeArray".to_string(),
                self.m_child_magnitude_array.to_value(),
            )),
            Some((
                "m_ChildPairVectorArray".to_string(),
                self.m_child_pair_vector_array.to_value(),
            )),
            Some((
                "m_ChildPairAvgMagInvArray".to_string(),
                self.m_child_pair_avg_mag_inv_array.to_value(),
            )),
            Some((
                "m_ChildNeighborListArray".to_string(),
                self.m_child_neighbor_list_array.to_value(),
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
                "m_ChildPositionArray" => {
                    (self.m_child_position_array).inspect_value(path, offset, limit)
                }
                "m_ChildMagnitudeArray" => {
                    (self.m_child_magnitude_array).inspect_value(path, offset, limit)
                }
                "m_ChildPairVectorArray" => {
                    (self.m_child_pair_vector_array).inspect_value(path, offset, limit)
                }
                "m_ChildPairAvgMagInvArray" => {
                    (self.m_child_pair_avg_mag_inv_array).inspect_value(path, offset, limit)
                }
                "m_ChildNeighborListArray" => {
                    (self.m_child_neighbor_list_array).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_child_position_array).inspect_value(path, offset, limit))
                .map(|v| ("m_ChildPositionArray".to_string(), v)),
            ((self.m_child_magnitude_array).inspect_value(path, offset, limit))
                .map(|v| ("m_ChildMagnitudeArray".to_string(), v)),
            ((self.m_child_pair_vector_array).inspect_value(path, offset, limit))
                .map(|v| ("m_ChildPairVectorArray".to_string(), v)),
            ((self.m_child_pair_avg_mag_inv_array).inspect_value(path, offset, limit))
                .map(|v| ("m_ChildPairAvgMagInvArray".to_string(), v)),
            ((self.m_child_neighbor_list_array).inspect_value(path, offset, limit))
                .map(|v| ("m_ChildNeighborListArray".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
