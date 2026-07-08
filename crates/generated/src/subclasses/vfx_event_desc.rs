//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxEventDesc {
    pub name: String,
    pub play_systems: Vec<u32>,
    pub stop_systems: Vec<u32>,
    pub init_systems: Option<Vec<u32>>,
}

impl crate::Parse for VfxEventDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let play_systems = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let stop_systems = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let init_systems = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(VfxEventDesc {
            name,
            play_systems,
            stop_systems,
            init_systems,
        })
    }
}
