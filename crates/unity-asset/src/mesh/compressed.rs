//! AssetRipper CompressedMeshExtensions / Packed*VectorExtensions 的移植。
use super::*;
use generated::subclasses::{
    compressed_mesh::CompressedMesh, packed_bit_vector_int32::PackedBitVector_Int32,
    packed_bit_vector_single::PackedBitVector_Single,
};
fn unpack(data: &[u8], count: u32, bits: u8) -> Result<Vec<u32>> {
    check(bits <= 32, "packed bit size 超过 32")?;
    let n = count as usize;
    let total = n
        .checked_mul(bits as usize)
        .ok_or_else(|| invalid("packed 大小溢出"))?;
    check(total.div_ceil(8) <= data.len(), "packed 数据截断")?;
    // 0-bit 常量序列没有输入字节可约束分配，设置明确的资源上限。
    check(n <= 64 * 1024 * 1024, "packed 元素数量超过解码上限")?;
    let mut out = Vec::new();
    out.try_reserve_exact(n)
        .map_err(|_| invalid("packed 分配失败"))?;
    for i in 0..n {
        let mut value = 0u32;
        let mut bit = i * bits as usize;
        let mut consumed = 0usize;
        while consumed < bits as usize {
            let take = (8 - bit % 8).min(bits as usize - consumed);
            value |= (((data[bit / 8] as u32) >> (bit % 8)) & ((1 << take) - 1)) << consumed;
            bit += take;
            consumed += take;
        }
        out.push(value);
    }
    Ok(out)
}
fn ints(p: &PackedBitVector_Int32) -> Result<Vec<u32>> {
    unpack(&p.m_data, p.m_num_items, p.m_bit_size)
}
fn floats(p: &PackedBitVector_Single) -> Result<Vec<f32>> {
    let v = unpack(&p.m_data, p.m_num_items, p.m_bit_size)?;
    let max = if p.m_bit_size == 32 {
        u32::MAX
    } else {
        (1u32 << p.m_bit_size) - 1
    };
    Ok(v.into_iter()
        .map(|x| {
            if max == 0 {
                p.m_start
            } else {
                x as f32 / ((1.0 / p.m_range) * max as f32) + p.m_start
            }
        })
        .collect())
}
fn vectors<const N: usize>(v: &[f32]) -> Result<Vec<[f32; N]>> {
    check(v.len().is_multiple_of(N), "packed 向量分量数不整除")?;
    Ok(v.chunks_exact(N).map(|v| v.try_into().unwrap()).collect())
}
fn normal(x: f32, y: f32, sign: u32) -> [f32; 3] {
    let z2 = 1. - x * x - y * y;
    let (x, y, z) = if z2 >= 0. {
        (x, y, z2.sqrt())
    } else {
        let len = (x * x + y * y).sqrt();
        (x / len, y / len, 0.)
    };
    [x, y, if sign == 0 { -z } else { z }]
}
pub(super) fn decode(c: &CompressedMesh, out: &mut DecodedMesh) -> Result<Option<Vec<u32>>> {
    if out.positions.is_empty() && c.m_vertices.m_num_items > 0 {
        out.positions = vectors(&floats(&c.m_vertices)?)?;
    }
    let n = out.positions.len();
    if c.m_uv.m_num_items > 0 {
        let uv = floats(&c.m_uv)?;
        let info = c.m_uv_info.unwrap_or(0);
        let mut start = 0usize;
        for i in 0..8 {
            let dim = if info != 0 {
                let b = info >> (i * 4) & 15;
                if b & 4 == 0 {
                    continue;
                }
                (b & 3) as usize + 1
            } else if i == 0 || (i == 1 && uv.len() >= n.saturating_mul(4)) {
                2
            } else {
                continue;
            };
            let end = start
                .checked_add(n.checked_mul(dim).ok_or_else(|| invalid("UV 大小溢出"))?)
                .ok_or_else(|| invalid("UV 大小溢出"))?;
            let values = uv.get(start..end).ok_or_else(|| invalid("压缩 UV 截断"))?;
            if out.tex_coords[i].is_none() {
                out.tex_coords[i] = Some(TexCoordSet {
                    dimension: dim as u8,
                    values: values
                        .chunks_exact(dim)
                        .map(|v| {
                            let mut a = [0.; 4];
                            a[..dim].copy_from_slice(v);
                            a
                        })
                        .collect(),
                });
            }
            start = end;
        }
    }
    if out.normals.is_none() && c.m_normals.m_num_items > 0 {
        let xy = vectors::<2>(&floats(&c.m_normals)?)?;
        let signs = ints(&c.m_normal_signs)?;
        check(signs.len() == xy.len(), "法线符号数量不一致")?;
        out.normals = Some(
            xy.into_iter()
                .zip(signs)
                .map(|(p, s)| normal(p[0], p[1], s))
                .collect(),
        );
    }
    if out.tangents.is_none() && c.m_tangents.m_num_items > 0 {
        let xy = vectors::<2>(&floats(&c.m_tangents)?)?;
        let signs = ints(&c.m_tangent_signs)?;
        check(signs.len() == xy.len() * 2, "切线符号数量不一致")?;
        out.tangents = Some(
            xy.into_iter()
                .zip(signs.chunks_exact(2))
                .map(|(p, s)| {
                    let n = normal(p[0], p[1], s[0]);
                    [n[0], n[1], n[2], if s[1] == 0 { -1. } else { 1. }]
                })
                .collect(),
        );
    }
    if out.colors.is_none() {
        if let Some(p) = &c.m_float_colors
            && p.m_num_items > 0
        {
            out.colors = Some(vectors(&floats(p)?)?);
        } else if let Some(p) = &c.m_colors
            && p.m_num_items > 0
        {
            check(p.m_bit_size.is_multiple_of(4), "压缩颜色 bit size 无效")?;
            let count = p
                .m_num_items
                .checked_mul(4)
                .ok_or_else(|| invalid("颜色数量溢出"))?;
            let values = unpack(&p.m_data, count, p.m_bit_size / 4)?;
            out.colors = Some(
                values
                    .chunks_exact(4)
                    .map(|v| {
                        [
                            v[0] as u8 as f32 / 255.,
                            v[1] as u8 as f32 / 255.,
                            v[2] as u8 as f32 / 255.,
                            v[3] as u8 as f32 / 255.,
                        ]
                    })
                    .collect(),
            );
        }
    }
    if let Some(p) = &c.m_bind_poses
        && p.m_num_items > 0
    {
        out.bind_poses = vectors(&floats(p)?)?;
        // 对齐 AssetRipper GetBindPoses，转为与非压缩路径一致的列主序布局。
        for pose in &mut out.bind_poses {
            for row in 0..4 {
                for column in row + 1..4 {
                    pose.swap(row * 4 + column, column * 4 + row);
                }
            }
        }
    }
    if out.skin.is_none() && c.m_weights.m_num_items > 0 {
        let weights = ints(&c.m_weights)?;
        let bones = ints(&c.m_bone_indices)?;
        let mut bone = 0;
        let mut skin = Vec::new();
        let mut current = SkinWeight::default();
        let mut slot = 0;
        let mut sum = 0;
        for weight in weights {
            check(weight <= 31, "压缩骨骼权重超出 31")?;
            current.weights[slot] = weight as f32 / 31.;
            current.indices[slot] = *bones.get(bone).ok_or_else(|| invalid("压缩骨骼索引截断"))?;
            bone += 1;
            slot += 1;
            sum += weight;
            // 对齐 AssetRipper：累计值达到或超过 31 即结束当前顶点。
            if sum >= 31 {
                skin.push(current);
                current = SkinWeight::default();
                slot = 0;
                sum = 0;
            } else if slot == 3 {
                current.weights[3] = (31 - sum) as f32 / 31.;
                current.indices[3] = *bones.get(bone).ok_or_else(|| invalid("第四骨骼索引截断"))?;
                bone += 1;
                skin.push(current);
                current = SkinWeight::default();
                slot = 0;
                sum = 0;
            }
        }
        check(slot == 0 && bone == bones.len(), "压缩骨骼权重未完整结束")?;
        out.skin = Some(skin);
    }
    if c.m_triangles.m_num_items > 0 {
        Ok(Some(ints(&c.m_triangles)?))
    } else {
        Ok(None)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn packed_cross_bytes_and_32_bits() {
        assert_eq!(
            unpack(&[0b1110_0001, 0b0000_0111], 3, 5).unwrap(),
            vec![1, 31, 1]
        );
        assert_eq!(unpack(&[255; 4], 1, 32).unwrap(), vec![u32::MAX]);
        assert!(unpack(&[0], 3, 5).is_err());
        assert!(unpack(&[], 1, 33).is_err());
    }
    #[test]
    fn normal_sign_and_normalization() {
        assert_eq!(normal(0., 0., 0), [0., 0., -1.]);
        let n = normal(1., 1., 1);
        assert!((n[0] - std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-6);
    }
}

#[cfg(test)]
mod mesh_tests {
    use super::*;
    fn pi(v: &[u8]) -> PackedBitVector_Int32 {
        PackedBitVector_Int32 {
            m_num_items: v.len() as u32,
            m_data: v.to_vec(),
            m_bit_size: 8,
        }
    }
    fn pf(v: &[u8], start: f32, range: f32) -> PackedBitVector_Single {
        PackedBitVector_Single {
            m_num_items: v.len() as u32,
            m_data: v.to_vec(),
            m_bit_size: 8,
            m_start: start,
            m_range: range,
        }
    }
    fn compressed() -> CompressedMesh {
        CompressedMesh {
            m_vertices: pf(&[0, 255, 0], 0., 1.),
            m_uv: pf(&[0, 127, 255], 0., 1.),
            m_bind_poses: None,
            m_normals: pf(&[0, 0], 0., 1.),
            m_tangents: pf(&[0, 0], 0., 1.),
            m_weights: pi(&[8, 8, 8]),
            m_normal_signs: pi(&[0]),
            m_tangent_signs: pi(&[1, 0]),
            m_float_colors: Some(pf(&[255, 0, 127, 255], 0., 1.)),
            m_bone_indices: pi(&[0, 1, 2, 3]),
            m_triangles: pi(&[0, 0, 0]),
            m_colors: None,
            m_uv_info: Some(6),
        }
    }
    #[test]
    fn compressed_geometry_uv3_and_implicit_weight() {
        let c = compressed();
        let mut m = DecodedMesh::default();
        assert_eq!(decode(&c, &mut m).unwrap().unwrap(), [0, 0, 0]);
        assert_eq!(m.positions, [[0., 1., 0.]]);
        assert_eq!(m.normals.unwrap(), [[0., 0., -1.]]);
        assert_eq!(m.tangents.unwrap(), [[0., 0., 1., -1.]]);
        let uv = m.tex_coords[0].as_ref().unwrap();
        assert_eq!(uv.dimension, 3);
        assert_eq!(uv.values[0][2], 1.);
        let skin = m.skin.unwrap();
        assert_eq!(skin[0].indices, [0, 1, 2, 3]);
        assert_eq!(skin[0].weights, [8. / 31., 8. / 31., 8. / 31., 7. / 31.]);
    }
    #[test]
    fn compressed_weights_finish_at_or_above_31() {
        for second in [15, 16] {
            let mut c = compressed();
            c.m_vertices = pf(&[0, 255, 0, 255, 0, 255], 0., 1.);
            c.m_uv = pf(&[], 0., 1.);
            c.m_normals = pf(&[], 0., 1.);
            c.m_tangents = pf(&[], 0., 1.);
            c.m_float_colors = None;
            // 第二个顶点检查结束条件是否正确重置权重槽位和骨骼索引游标。
            c.m_weights = pi(&[16, second, 31]);
            c.m_bone_indices = pi(&[2, 3, 4]);
            let mut m = DecodedMesh::default();
            decode(&c, &mut m).unwrap();
            validate(&m).unwrap();
            let skin = m.skin.unwrap();
            assert_eq!(skin.len(), 2);
            assert_eq!(skin[0].weights, [16. / 31., second as f32 / 31., 0., 0.]);
            assert_eq!(skin[0].indices, [2, 3, 0, 0]);
            assert_eq!(skin[1].weights, [1., 0., 0., 0.]);
            assert_eq!(skin[1].indices, [4, 0, 0, 0]);
        }
    }
    #[test]
    fn compressed_weights_still_reject_invalid_values_and_truncation() {
        let mut c = compressed();
        c.m_weights = pi(&[32]);
        c.m_bone_indices = pi(&[0]);
        assert!(decode(&c, &mut DecodedMesh::default()).is_err());
        c.m_weights = pi(&[16, 16]);
        assert!(decode(&c, &mut DecodedMesh::default()).is_err());
    }
    #[test]
    fn resident_attributes_take_precedence_and_corruption_is_error() {
        let mut c = compressed();
        let mut m = DecodedMesh {
            positions: vec![[3., 4., 5.]],
            normals: Some(vec![[1., 0., 0.]]),
            ..Default::default()
        };
        decode(&c, &mut m).unwrap();
        assert_eq!(m.positions, [[3., 4., 5.]]);
        assert_eq!(m.normals.unwrap(), [[1., 0., 0.]]);
        c.m_bone_indices.m_data.clear();
        assert!(decode(&c, &mut DecodedMesh::default()).is_err());
    }
    #[test]
    fn zero_bit_constant_and_zero_range() {
        let mut p = pf(&[], 2., 0.);
        p.m_num_items = 3;
        p.m_bit_size = 0;
        assert_eq!(floats(&p).unwrap(), [2., 2., 2.]);
        let p = pf(&[0, 0], 2., 0.);
        assert_eq!(floats(&p).unwrap(), [2., 2.]);
    }
}
