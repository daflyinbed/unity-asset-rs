//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vfx_mapping::VfxMapping;
use crate::subclasses::vfx_mapping_temporary::VfxMappingTemporary;

#[derive(Debug, Clone)]
pub struct VfxTaskDesc {
    pub r#type: i32,
    pub buffers: Vec<VfxMapping>,
    pub values: Vec<VfxMapping>,
    pub temporary_buffers: Option<Vec<VfxMappingTemporary>>,
    pub params: Vec<VfxMapping>,
    pub processor: PPtr,
    pub instance_split_index: Option<u32>,
}

impl crate::Parse for VfxTaskDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut r#type = None;
        let mut buffers = None;
        let mut values = None;
        let mut temporary_buffers = None;
        let mut params = None;
        let mut processor = None;
        let mut instance_split_index = None;
        r#type = Some(rdr.read_i32()?);
        buffers = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxMapping::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2019, 2, 0) {
            temporary_buffers = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| VfxMappingTemporary::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }
        values = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxMapping::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        params = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| VfxMapping::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        processor = Some(crate::PPtr::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(2023, 1, 17) {
            instance_split_index = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2023, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2022, 3, 6) {
            instance_split_index = Some(rdr.read_u32()?);
        } else {
        }

        Ok(VfxTaskDesc {
            r#type: r#type.unwrap(),
            buffers: buffers.unwrap(),
            values: values.unwrap(),
            temporary_buffers,
            params: params.unwrap(),
            processor: processor.unwrap(),
            instance_split_index,
        })
    }
}
