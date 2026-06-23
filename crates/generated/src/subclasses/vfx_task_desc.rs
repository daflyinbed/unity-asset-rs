//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::vfx_mapping::VfxMapping;

#[derive(Debug, Clone)]
pub struct VfxTaskDesc {
    pub r#type: i32,
    pub buffers: Vec<VfxMapping>,
    pub values: Vec<VfxMapping>,
    pub params: Vec<VfxMapping>,
    pub processor: PPtr,
}

impl crate::Parse for VfxTaskDesc {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r#type = r.read_i32()?;
        let buffers = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxMapping::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let values = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxMapping::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let params = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { VfxMapping::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let processor = crate::PPtr::parse(r, ver)?;

        Ok(VfxTaskDesc {
            r#type,
            buffers,
            values,
            params,
            processor,
        })
    }
}
