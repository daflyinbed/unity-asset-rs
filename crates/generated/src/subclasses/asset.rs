//! 自动生成。请勿手动修改。

use crate::subclasses::asset_labels::AssetLabels;
use crate::subclasses::guid::Guid;
use crate::subclasses::library_representation::LibraryRepresentation;
use crate::subclasses::md_four::MdFour;

#[derive(Debug, Clone)]
pub struct Asset {
    pub modification_date_0: u32,
    pub modification_date_1: u32,
    pub meta_modification_date_0: u32,
    pub meta_modification_date_1: u32,
    pub main_representation: LibraryRepresentation,
    pub representations: Vec<LibraryRepresentation>,
    pub children: Vec<Guid>,
    pub parent: Guid,
    pub digest: MdFour,
    pub r#type: i32,
    pub labels: AssetLabels,
}

impl crate::Parse for Asset {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let modification_date_0 = r.read_u32()?;
        let modification_date_1 = r.read_u32()?;
        let meta_modification_date_0 = r.read_u32()?;
        let meta_modification_date_1 = r.read_u32()?;
        let main_representation = LibraryRepresentation::parse(r, ver)?;
        r.align(4)?;
        let representations = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { LibraryRepresentation::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let children = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Guid::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let parent = Guid::parse(r, ver)?;
        let digest = MdFour::parse(r, ver)?;
        r.align(4)?;
        let r#type = r.read_i32()?;
        let labels = AssetLabels::parse(r, ver)?;
        r.align(4)?;

        Ok(Asset {
            modification_date_0,
            modification_date_1,
            meta_modification_date_0,
            meta_modification_date_1,
            main_representation,
            representations,
            children,
            parent,
            digest,
            r#type,
            labels,
        })
    }
}
