//! 自动生成。请勿手动修改。

use crate::subclasses::d3_d12_device_filter_data::D3D12DeviceFilterData;

#[derive(Debug, Clone)]
pub struct D3D12GraphicsJobsDeviceFilterData {
    pub preferred_mode: u32,
    pub filter: D3D12DeviceFilterData,
}

impl crate::Parse for D3D12GraphicsJobsDeviceFilterData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let preferred_mode = rdr.read_u32()?;
        let filter = D3D12DeviceFilterData::parse(rdr, ver)?;
        rdr.align(4)?;

        Ok(D3D12GraphicsJobsDeviceFilterData {
            preferred_mode,
            filter,
        })
    }
}
