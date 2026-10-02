//! 自动生成。请勿手动修改。

use crate::subclasses::channel_info::ChannelInfo;

#[derive(Debug, Clone)]
pub struct VertexLayoutInfo {
    pub vertex_channels_info: Vec<ChannelInfo>,
    pub vertex_strides: Vec<u8>,
    pub vertex_stream_count: i32,
}

impl crate::Parse for VertexLayoutInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let vertex_channels_info = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| ChannelInfo::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let vertex_strides = ({
            let n = rdr.read_i32()? as usize;
            (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let vertex_stream_count = rdr.read_i32()?;

        Ok(VertexLayoutInfo {
            vertex_channels_info,
            vertex_strides,
            vertex_stream_count,
        })
    }
}

impl crate::ToValue for VertexLayoutInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "vertexChannelsInfo".to_string(),
                self.vertex_channels_info.to_value(),
            )),
            Some(("vertexStrides".to_string(), self.vertex_strides.to_value())),
            Some((
                "vertexStreamCount".to_string(),
                self.vertex_stream_count.to_value(),
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
                "vertexChannelsInfo" => {
                    (self.vertex_channels_info).inspect_value(path, offset, limit)
                }
                "vertexStrides" => (self.vertex_strides).inspect_value(path, offset, limit),
                "vertexStreamCount" => {
                    (self.vertex_stream_count).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.vertex_channels_info).inspect_value(path, offset, limit))
                .map(|v| ("vertexChannelsInfo".to_string(), v)),
            ((self.vertex_strides).inspect_value(path, offset, limit))
                .map(|v| ("vertexStrides".to_string(), v)),
            ((self.vertex_stream_count).inspect_value(path, offset, limit))
                .map(|v| ("vertexStreamCount".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
