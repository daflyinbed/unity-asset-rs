//! 自动生成。请勿手动修改。

use crate::subclasses::d3_d12_device_filter_data::D3D12DeviceFilterData;
use crate::subclasses::d3_d12_graphics_jobs_device_filter_data::D3D12GraphicsJobsDeviceFilterData;

#[derive(Debug, Clone)]
pub struct D3D12DeviceFilterLists {
    pub m_name: String,
    pub m_allow_filter_list: Vec<D3D12DeviceFilterData>,
    pub m_deny_filter_list: Vec<D3D12DeviceFilterData>,
    pub m_graphics_jobs_filter_list: Vec<D3D12GraphicsJobsDeviceFilterData>,
}

impl crate::Parse for D3D12DeviceFilterLists {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_allow_filter_list = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| D3D12DeviceFilterData::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_deny_filter_list = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| D3D12DeviceFilterData::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_graphics_jobs_filter_list = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| D3D12GraphicsJobsDeviceFilterData::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(D3D12DeviceFilterLists {
            m_name,
            m_allow_filter_list,
            m_deny_filter_list,
            m_graphics_jobs_filter_list,
        })
    }
}

impl crate::ToValue for D3D12DeviceFilterLists {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_AllowFilterList".to_string(),
                self.m_allow_filter_list.to_value(),
            )),
            Some((
                "m_DenyFilterList".to_string(),
                self.m_deny_filter_list.to_value(),
            )),
            Some((
                "m_GraphicsJobsFilterList".to_string(),
                self.m_graphics_jobs_filter_list.to_value(),
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
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_AllowFilterList" => {
                    (self.m_allow_filter_list).inspect_value(path, offset, limit)
                }
                "m_DenyFilterList" => (self.m_deny_filter_list).inspect_value(path, offset, limit),
                "m_GraphicsJobsFilterList" => {
                    (self.m_graphics_jobs_filter_list).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_allow_filter_list).inspect_value(path, offset, limit))
                .map(|v| ("m_AllowFilterList".to_string(), v)),
            ((self.m_deny_filter_list).inspect_value(path, offset, limit))
                .map(|v| ("m_DenyFilterList".to_string(), v)),
            ((self.m_graphics_jobs_filter_list).inspect_value(path, offset, limit))
                .map(|v| ("m_GraphicsJobsFilterList".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
