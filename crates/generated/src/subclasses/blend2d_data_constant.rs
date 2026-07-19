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
