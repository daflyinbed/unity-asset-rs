//! 自动生成。请勿手动修改。

use crate::subclasses::channel_info::ChannelInfo;
use crate::subclasses::stream_info::StreamInfo;

/// 版本枚举：VertexDataMCurrentChannelsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VertexDataMCurrentChannelsVariant {
    Default(u32),
    V5_6(i32),
}

impl crate::Parse for VertexDataMCurrentChannelsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            let v = rdr.read_i32()?;
            Ok(Self::V5_6(v))
        } else {
            let v = rdr.read_u32()?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VertexDataMCurrentChannelsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V5_6(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V5_6(v) => (v).inspect_value(path, offset, limit),
        }
    }
}

/// 版本枚举：VertexDataMChannelsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum VertexDataMChannelsVariant {
    Default(Vec<ChannelInfo>),
    V2017_1(Vec<ChannelInfo>),
}

impl crate::Parse for VertexDataMChannelsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ChannelInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| ChannelInfo::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}

impl crate::ToValue for VertexDataMChannelsVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V2017_1(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V2017_1(v) => (v).inspect_value(path, offset, limit),
        }
    }
}
#[derive(Debug, Clone)]
pub struct VertexData {
    pub m_current_channels: Option<VertexDataMCurrentChannelsVariant>,
    pub m_vertex_count: u32,
    pub m_channels: Option<VertexDataMChannelsVariant>,
    pub m_streams_0: Option<StreamInfo>,
    pub m_data: Vec<u8>,
    pub m_streams_1: Option<StreamInfo>,
    pub m_streams: Option<Vec<StreamInfo>>,
    pub m_streams_2: Option<StreamInfo>,
    pub m_streams_3: Option<StreamInfo>,
}

impl crate::Parse for VertexData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_current_channels = None;
        let mut m_vertex_count = None;
        let mut m_channels = None;
        let mut m_streams_0 = None;
        let mut m_data = None;
        let mut m_streams_1 = None;
        let mut m_streams = None;
        let mut m_streams_2 = None;
        let mut m_streams_3 = None;
        if *ver >= crate::UnityVersion::new(2018, 1, 0) {
        } else {
            m_current_channels = Some(VertexDataMCurrentChannelsVariant::parse(rdr, ver)?);
        }
        m_vertex_count = Some(rdr.read_u32()?);
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_channels = Some(VertexDataMChannelsVariant::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(4, 0, 0) {
            m_channels = Some(VertexDataMChannelsVariant::parse(rdr, ver)?);
            m_streams = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| StreamInfo::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
            m_streams_0 = Some(StreamInfo::parse(rdr, ver)?);
            m_streams_1 = Some(StreamInfo::parse(rdr, ver)?);
            m_streams_2 = Some(StreamInfo::parse(rdr, ver)?);
            m_streams_3 = Some(StreamInfo::parse(rdr, ver)?);
        }
        m_data = Some(
            ({
                let n = rdr.read_i32()? as usize;
                rdr.read_bytes(n)
            })?,
        );
        rdr.align(4)?;

        Ok(VertexData {
            m_current_channels,
            m_vertex_count: m_vertex_count.unwrap(),
            m_channels,
            m_streams_0,
            m_data: m_data.unwrap(),
            m_streams_1,
            m_streams,
            m_streams_2,
            m_streams_3,
        })
    }
}

impl crate::ToValue for VertexData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_CurrentChannels", &self.m_current_channels),
            Some(("m_VertexCount".to_string(), self.m_vertex_count.to_value())),
            crate::opt_field("m_Channels", &self.m_channels),
            crate::opt_field("m_Streams[0]", &self.m_streams_0),
            Some((
                "m_Data".to_string(),
                crate::Value::Bytes(self.m_data.clone()),
            )),
            crate::opt_field("m_Streams[1]", &self.m_streams_1),
            crate::opt_field("m_Streams", &self.m_streams),
            crate::opt_field("m_Streams[2]", &self.m_streams_2),
            crate::opt_field("m_Streams[3]", &self.m_streams_3),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_CurrentChannels" => (self.m_current_channels).inspect_value(path, offset, limit),
                "m_VertexCount" => (self.m_vertex_count).inspect_value(path, offset, limit),
                "m_Channels" => (self.m_channels).inspect_value(path, offset, limit),
                "m_Streams[0]" => (self.m_streams_0).inspect_value(path, offset, limit),
                "m_Data" => crate::inspect_bytes(&(self.m_data), path, offset, limit),
                "m_Streams[1]" => (self.m_streams_1).inspect_value(path, offset, limit),
                "m_Streams" => (self.m_streams).inspect_value(path, offset, limit),
                "m_Streams[2]" => (self.m_streams_2).inspect_value(path, offset, limit),
                "m_Streams[3]" => (self.m_streams_3).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_current_channels).inspect_value(path, offset, limit))
                .map(|v| ("m_CurrentChannels".to_string(), v)),
            ((self.m_vertex_count).inspect_value(path, offset, limit))
                .map(|v| ("m_VertexCount".to_string(), v)),
            ((self.m_channels).inspect_value(path, offset, limit))
                .map(|v| ("m_Channels".to_string(), v)),
            ((self.m_streams_0).inspect_value(path, offset, limit))
                .map(|v| ("m_Streams[0]".to_string(), v)),
            (crate::inspect_bytes(&(self.m_data), path, offset, limit))
                .map(|v| ("m_Data".to_string(), v)),
            ((self.m_streams_1).inspect_value(path, offset, limit))
                .map(|v| ("m_Streams[1]".to_string(), v)),
            ((self.m_streams).inspect_value(path, offset, limit))
                .map(|v| ("m_Streams".to_string(), v)),
            ((self.m_streams_2).inspect_value(path, offset, limit))
                .map(|v| ("m_Streams[2]".to_string(), v)),
            ((self.m_streams_3).inspect_value(path, offset, limit))
                .map(|v| ("m_Streams[3]".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
