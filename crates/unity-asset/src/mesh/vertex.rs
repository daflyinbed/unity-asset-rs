//! AssetRipper VertexDataBlob / MeshHelper 的 Rust 移植，见 NOTICE.md。
use super::*;
use generated::subclasses::{
    channel_info::ChannelInfo,
    stream_info::{StreamInfo, StreamInfoStrideVariant},
    vertex_data::{VertexData, VertexDataMChannelsVariant},
};

pub(super) fn uint(b: &[u8], e: Endian) -> u32 {
    match e {
        Endian::Little => b
            .iter()
            .enumerate()
            .fold(0, |v, (i, b)| v | ((*b as u32) << (8 * i))),
        Endian::Big => b.iter().fold(0, |v, b| (v << 8) | *b as u32),
    }
}
fn format(f: u8, v: UnityVersion) -> Result<u8> {
    let f = if v < UnityVersion::new(2017, 0, 0) {
        match f {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 6,
            4 => 10,
            _ => return Err(invalid("未知旧版顶点格式")),
        }
    } else if v < UnityVersion::new(2019, 0, 0) {
        if f <= 2 { f } else { f - 1 }
    } else {
        f
    };
    check(f <= 11, "未知顶点格式")?;
    Ok(f)
}
fn size(f: u8) -> usize {
    match f {
        0 | 10 | 11 => 4,
        1 | 4 | 5 | 8 | 9 => 2,
        _ => 1,
    }
}
fn scalar(b: &[u8], f: u8, e: Endian) -> f32 {
    let v = uint(b, e);
    match f {
        0 => f32::from_bits(v),
        1 => half::f16::from_bits(v as u16).to_f32(),
        2 => v as f32 / 255.,
        3 => (v as i8 as f32 / 127.).max(-1.),
        4 => v as f32 / 65535.,
        5 => (v as i16 as f32 / 32767.).max(-1.),
        6 | 8 | 10 => v as f32,
        7 => v as i8 as f32,
        9 => v as i16 as f32,
        11 => v as i32 as f32,
        _ => unreachable!(),
    }
}
#[derive(Clone, Copy, Default)]
struct Stream {
    mask: u32,
    offset: usize,
    stride: usize,
}
impl From<&StreamInfo> for Stream {
    fn from(s: &StreamInfo) -> Self {
        Self {
            mask: s.channel_mask,
            offset: s.offset as usize,
            stride: match s.stride {
                StreamInfoStrideVariant::Default(v) => v as usize,
                StreamInfoStrideVariant::V4_0(v) => v as usize,
            },
        }
    }
}
fn dimension(c: &ChannelInfo, i: usize, v: UnityVersion) -> usize {
    if c.dimension != 0 && v < UnityVersion::new(2018, 0, 0) && i == 2 && c.format == 2 {
        4
    } else {
        (c.dimension & 15) as usize
    }
}
fn semantic(i: usize, v: UnityVersion) -> usize {
    if v >= UnityVersion::new(2018, 0, 0) {
        i
    } else {
        match i {
            0 | 1 => i,
            2 => 3,
            3 => 4,
            4 => 5,
            5 if v < UnityVersion::new(5, 0, 0) => 2,
            5 => 6,
            6 => 7,
            7 => 2,
            _ => 99,
        }
    }
}
pub(super) fn decode(
    vd: &VertexData,
    data: &[u8],
    ver: UnityVersion,
    endian: Endian,
    out: &mut DecodedMesh,
) -> Result<()> {
    if data.is_empty() {
        return Ok(());
    }
    let mut streams: Vec<Stream> = if let Some(s) = &vd.m_streams {
        s.iter().map(Stream::from).collect()
    } else {
        [
            &vd.m_streams_0,
            &vd.m_streams_1,
            &vd.m_streams_2,
            &vd.m_streams_3,
        ]
        .into_iter()
        .flatten()
        .map(Stream::from)
        .collect()
    };
    let channels = if let Some(
        VertexDataMChannelsVariant::Default(c) | VertexDataMChannelsVariant::V2017_1(c),
    ) = &vd.m_channels
    {
        c.clone()
    } else {
        // 旧版仅存 stream mask；按语义槽填充，保留缺失通道位置。
        let mut c: Vec<ChannelInfo> = (0..6)
            .map(|_| ChannelInfo {
                stream: 0,
                offset: 0,
                format: 0,
                dimension: 0,
            })
            .collect();
        for (s, st) in streams.iter().enumerate() {
            let mut offset = 0usize;
            for (i, ch) in c.iter_mut().enumerate() {
                if st.mask & (1 << i) != 0 {
                    let (dim, fmt) = match i {
                        0 | 1 => (3, 0),
                        2 => (4, 2),
                        3 | 4 => (2, 0),
                        _ => (4, 0),
                    };
                    *ch = ChannelInfo {
                        stream: s as u8,
                        offset: u8::try_from(offset)
                            .map_err(|_| invalid("旧 stream offset 溢出"))?,
                        format: fmt,
                        dimension: dim,
                    };
                    offset += dim as usize * size(format(fmt, ver)?);
                }
            }
        }
        c
    };
    check(channels.len() <= 32, "channel mask 最多 32 个通道")?;
    if streams.is_empty() {
        let count = channels
            .iter()
            .map(|c| c.stream as usize + 1)
            .max()
            .unwrap_or(0);
        streams.resize(count, Stream::default());
        for (i, c) in channels.iter().enumerate() {
            let d = dimension(c, i, ver);
            check(d <= 4, "顶点通道维数超过 4")?;
            if d == 0 {
                continue;
            }
            let s = &mut streams[c.stream as usize];
            s.mask |= 1 << i;
            s.stride += d * size(format(c.format, ver)?);
        }
        let mut offset = 0usize;
        for s in &mut streams {
            s.offset = offset;
            offset = offset
                .checked_add(
                    (vd.m_vertex_count as usize)
                        .checked_mul(s.stride)
                        .ok_or_else(|| invalid("stream 大小溢出"))?,
                )
                .and_then(|v| v.checked_add(15))
                .ok_or_else(|| invalid("stream offset 溢出"))?
                & !15;
        }
    }
    let n = vd.m_vertex_count as usize;
    for (i, c) in channels.iter().enumerate() {
        let d = dimension(c, i, ver);
        if d == 0 {
            continue;
        }
        check(d <= 4, "顶点通道维数超过 4")?;
        let st = streams
            .get(c.stream as usize)
            .ok_or_else(|| invalid("顶点 stream 越界"))?;
        if st.mask & (1 << i) == 0 {
            continue;
        }
        let f = format(c.format, ver)?;
        let bytes = size(f);
        let width = d * bytes;
        check(
            c.offset as usize + width <= st.stride,
            "channel 超出 stream stride",
        )?;
        let start = st
            .offset
            .checked_add(c.offset as usize)
            .ok_or_else(|| invalid("channel offset 溢出"))?;
        if n > 0 {
            let end = (n - 1)
                .checked_mul(st.stride)
                .and_then(|v| v.checked_add(start))
                .and_then(|v| v.checked_add(width))
                .ok_or_else(|| invalid("顶点范围溢出"))?;
            check(end <= data.len(), "顶点 buffer 截断")?;
        }
        let sem = semantic(i, ver);
        if sem == 13 {
            check(f >= 6, "骨骼索引必须使用整数格式")?;
            let skin = out
                .skin
                .get_or_insert_with(|| vec![SkinWeight::default(); n]);
            check(skin.len() == n, "skin 长度不一致")?;
            for (v, s) in skin.iter_mut().enumerate() {
                for k in 0..d {
                    let p = start + v * st.stride + k * bytes;
                    let raw = uint(&data[p..p + bytes], endian);
                    check(
                        !matches!(f, 7 | 9 | 11) || scalar(&data[p..p + bytes], f, endian) >= 0.,
                        "负骨骼索引",
                    )?;
                    s.indices[k] = raw;
                }
            }
            continue;
        }
        let mut values = Vec::with_capacity(n);
        for v in 0..n {
            let mut a = [0.; 4];
            for (k, x) in a[..d].iter_mut().enumerate() {
                let p = start + v * st.stride + k * bytes;
                *x = scalar(&data[p..p + bytes], f, endian);
            }
            values.push(a);
        }
        match sem {
            0 => out.positions = values.iter().map(|v| [v[0], v[1], v[2]]).collect(),
            1 => out.normals = Some(values.iter().map(|v| [v[0], v[1], v[2]]).collect()),
            2 => out.tangents = Some(values),
            3 => out.colors = Some(values),
            4..=11 => {
                out.tex_coords[sem - 4] = Some(TexCoordSet {
                    dimension: d as u8,
                    values,
                })
            }
            12 => {
                let skin = out
                    .skin
                    .get_or_insert_with(|| vec![SkinWeight::default(); n]);
                check(skin.len() == n, "skin 长度不一致")?;
                for (s, w) in skin.iter_mut().zip(values) {
                    s.weights = w;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn vd(channels: Vec<ChannelInfo>, data: Vec<u8>) -> VertexData {
        VertexData {
            m_current_channels: None,
            m_vertex_count: 1,
            m_channels: Some(VertexDataMChannelsVariant::V2017_1(channels)),
            m_streams_0: None,
            m_streams_1: None,
            m_streams_2: None,
            m_streams_3: None,
            m_streams: None,
            m_data: data,
        }
    }
    #[test]
    fn multistream_big_endian_half_and_snorm() {
        let mut data = vec![0; 20];
        data[..4].copy_from_slice(&1f32.to_be_bytes());
        data[4..8].copy_from_slice(&2f32.to_be_bytes());
        data[8..12].copy_from_slice(&3f32.to_be_bytes());
        data[16..18].copy_from_slice(&0x3800u16.to_be_bytes());
        data[18..20].copy_from_slice(&0xbc00u16.to_be_bytes());
        let empty = || ChannelInfo {
            stream: 0,
            offset: 0,
            format: 0,
            dimension: 0,
        };
        let mut channels = vec![empty(), empty(), empty(), empty(), empty()];
        channels[0].dimension = 3;
        channels[4] = ChannelInfo {
            stream: 1,
            offset: 0,
            format: 1,
            dimension: 2,
        };
        let v = vd(channels, data);
        let mut m = DecodedMesh::default();
        decode(
            &v,
            &v.m_data,
            UnityVersion::new(2020, 3, 0),
            Endian::Big,
            &mut m,
        )
        .unwrap();
        assert_eq!(m.positions, [[1., 2., 3.]]);
        assert_eq!(
            m.tex_coords[0].as_ref().unwrap().values,
            [[0.5, -1., 0., 0.]]
        );
        assert_eq!(scalar(&[128], 3, Endian::Little), -1.);
        assert_eq!(scalar(&[0, 128], 5, Endian::Little), -1.);
        assert!(
            decode(
                &v,
                &v.m_data[..19],
                UnityVersion::new(2020, 3, 0),
                Endian::Big,
                &mut m
            )
            .is_err()
        );
    }
    #[test]
    fn legacy_stream_preserves_missing_channel_slots() {
        let mut v = vd(
            vec![],
            [1f32, 2., 3., 0.25, 0.75]
                .into_iter()
                .flat_map(f32::to_le_bytes)
                .collect(),
        );
        v.m_channels = None;
        v.m_streams = Some(vec![StreamInfo {
            channel_mask: 9,
            offset: 0,
            stride: StreamInfoStrideVariant::Default(20),
            align: None,
            divider_op: None,
            frequency: None,
        }]);
        let mut m = DecodedMesh::default();
        decode(
            &v,
            &v.m_data,
            UnityVersion::new(3, 5, 0),
            Endian::Little,
            &mut m,
        )
        .unwrap();
        assert!(m.normals.is_none());
        assert_eq!(
            m.tex_coords[0].as_ref().unwrap().values,
            [[0.25, 0.75, 0., 0.]]
        );
    }
    #[test]
    fn format_versions() {
        assert_eq!(format(3, UnityVersion::new(5, 0, 0)).unwrap(), 6);
        assert_eq!(format(3, UnityVersion::new(2017, 0, 0)).unwrap(), 2);
        assert_eq!(format(3, UnityVersion::new(2019, 0, 0)).unwrap(), 3);
    }
}
