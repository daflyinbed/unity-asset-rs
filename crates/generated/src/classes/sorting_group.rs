//! 自动生成。请勿手动修改。

use crate::PPtr;


/// 版本枚举：SortingGroupMSortingOrderVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SortingGroupMSortingOrderVariant {
    Default(i16),
    V2022_1(i16),
}

impl crate::Parse for SortingGroupMSortingOrderVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            let v = rdr.read_i16()?;
            Ok(Self::V2022_1(v))
        } else {
            let v = rdr.read_i16()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}

/// 版本枚举：SortingGroupMSortAtRootVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum SortingGroupMSortAtRootVariant {
    Default(bool),
    V6000_3(bool),
}

impl crate::Parse for SortingGroupMSortAtRootVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_3(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct SortingGroup {
    pub m_game_object: PPtr,
    pub m_enabled: u8,
    pub m_sorting_layer_id: i32,
    pub m_sorting_layer: i16,
    pub m_sorting_order: SortingGroupMSortingOrderVariant,
    pub m_sort_at_root: Option<SortingGroupMSortAtRootVariant>,
    pub m_sort3_d_as2_d: Option<bool>,
}

impl crate::Parse for SortingGroup {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let m_enabled = rdr.read_u8()?;
        rdr.align(4)?;
        let m_sorting_layer_id = rdr.read_i32()?;
        let m_sorting_layer = rdr.read_i16()?;
        let m_sorting_order = SortingGroupMSortingOrderVariant::parse(rdr, ver)?;
        let m_sort_at_root = if *ver >= crate::UnityVersion::new(2022, 1, 0) {
            Some(SortingGroupMSortAtRootVariant::parse(rdr, ver)?)
        } else {
            None
        };
        let m_sort3_d_as2_d = if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(SortingGroup {
            m_game_object,
            m_enabled,
            m_sorting_layer_id,
            m_sorting_layer,
            m_sorting_order,
            m_sort_at_root,
            m_sort3_d_as2_d,
        })
    }
}
