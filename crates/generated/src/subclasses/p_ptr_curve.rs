//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::p_ptr_keyframe::PPtrKeyframe;

#[derive(Debug, Clone)]
pub struct PPtrCurve {
    pub curve: Vec<PPtrKeyframe>,
    pub attribute: String,
    pub path: String,
    pub class_id: i32,
    pub script: PPtr,
}

impl crate::Parse for PPtrCurve {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let curve = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { PPtrKeyframe::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let attribute = r.read_aligned_string()?;
        r.align(4)?;
        let path = r.read_aligned_string()?;
        r.align(4)?;
        let class_id = r.read_i32()?;
        let script = crate::PPtr::parse(r, ver)?;

        Ok(PPtrCurve {
            curve,
            attribute,
            path,
            class_id,
            script,
        })
    }
}
