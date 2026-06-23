//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxEventDesc {
    pub name: String,
    pub play_systems: Vec<u32>,
    pub stop_systems: Vec<u32>,
}

impl crate::Parse for VfxEventDesc {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = r.read_aligned_string()?;
        r.align(4)?;
        let play_systems = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let stop_systems = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(VfxEventDesc {
            name,
            play_systems,
            stop_systems,
        })
    }
}
