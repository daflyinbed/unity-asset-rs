//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::asset_bundle_script_info::AssetBundleScriptInfo;
use crate::subclasses::asset_info::AssetInfo;
use super::named_object::NamedObject;

#[derive(Debug, Clone)]
pub struct AssetBundle {
    pub parent: NamedObject,
    pub m_name: String,
    pub m_preload_table: Vec<PPtr>,
    pub m_container: Vec<(String, AssetInfo)>,
    pub m_main_asset: AssetInfo,
    pub m_script_compatibility: Option<Vec<AssetBundleScriptInfo>>,
    pub m_runtime_compatibility: Option<u32>,
    pub m_class_version_map: Option<Vec<(i32, i32)>>,
    pub m_class_compatibility: Option<Vec<(i32, u32)>>,
    pub m_asset_bundle_name: Option<String>,
    pub m_dependencies: Option<Vec<String>>,
    pub m_is_streamed_scene_asset_bundle: Option<bool>,
    pub m_path_flags: Option<i32>,
    pub m_explicit_data_layout: Option<i32>,
    pub m_scene_hashes: Option<Vec<(String, String)>>,
}

impl crate::Parse for AssetBundle {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let parent = NamedObject::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_preload_table = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_container = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = AssetInfo::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_main_asset = AssetInfo::parse(r, ver)?;
        let m_script_compatibility = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AssetBundleScriptInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_runtime_compatibility = if *ver >= crate::UnityVersion::new(4, 2, 0) {
            Some(r.read_u32()?)
        } else {
            None
        };
        let m_class_version_map = if *ver >= crate::UnityVersion::new(5, 4, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_i32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_class_compatibility = if *ver >= crate::UnityVersion::new(3, 5, 0) {
            Some(( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_i32()?; let b = r.read_u32()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };
        let m_asset_bundle_name = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_aligned_string()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_dependencies = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { r.read_aligned_string() }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_is_streamed_scene_asset_bundle = if *ver >= crate::UnityVersion::new(5, 0, 0) {
            let v = r.read_bool()?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };
        let m_path_flags = if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_explicit_data_layout = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            Some(r.read_i32()?)
        } else {
            None
        };
        let m_scene_hashes = if *ver >= crate::UnityVersion::new(2017, 3, 0) {
            let v = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = r.read_aligned_string()?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
            r.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(AssetBundle {
            parent,
            m_name,
            m_preload_table,
            m_container,
            m_main_asset,
            m_script_compatibility,
            m_runtime_compatibility,
            m_class_version_map,
            m_class_compatibility,
            m_asset_bundle_name,
            m_dependencies,
            m_is_streamed_scene_asset_bundle,
            m_path_flags,
            m_explicit_data_layout,
            m_scene_hashes,
        })
    }
}
