//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AddedGameObject {
    pub target_corresponding_source_object: PPtr,
    pub insert_index: i32,
    pub added_object: PPtr,
}

impl crate::Parse for AddedGameObject {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let target_corresponding_source_object = crate::PPtr::parse(rdr, ver)?;
        let insert_index = rdr.read_i32()?;
        let added_object = crate::PPtr::parse(rdr, ver)?;

        Ok(AddedGameObject {
            target_corresponding_source_object,
            insert_index,
            added_object,
        })
    }
}
