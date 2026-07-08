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
        rdr.align(4)?;
        let m_allow_filter_list = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { D3D12DeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_deny_filter_list = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { D3D12DeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let m_graphics_jobs_filter_list = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { D3D12GraphicsJobsDeviceFilterData::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;

        Ok(D3D12DeviceFilterLists {
            m_name,
            m_allow_filter_list,
            m_deny_filter_list,
            m_graphics_jobs_filter_list,
        })
    }
}
