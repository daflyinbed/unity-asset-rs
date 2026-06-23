//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::asset_bundle_script_info::AssetBundleScriptInfo;
use crate::subclasses::asset_info::AssetInfo;

#[derive(Debug, Clone)]
pub struct AssetBundle {
    pub m_object_hide_flags: u32,
    pub m_extension_ptr: PPtr,
    pub m_name: String,
    pub m_preload_table: Vec<PPtr>,
    pub m_container: Vec<(String, AssetInfo)>,
    pub m_main_asset: AssetInfo,
    pub m_script_compatibility: Vec<AssetBundleScriptInfo>,
}

impl crate::Parse for AssetBundle {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_object_hide_flags = r.read_u32()?;
        let m_extension_ptr = crate::PPtr::parse(r, ver)?;
        let m_name = r.read_aligned_string()?;
        r.align(4)?;
        let m_preload_table = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { crate::PPtr::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let m_container = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { ( { let a = r.read_aligned_string()?; let b = AssetInfo::parse(r, ver)?; Ok((a, b)) } ) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_main_asset = AssetInfo::parse(r, ver)?;
        let m_script_compatibility = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { AssetBundleScriptInfo::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;

        Ok(AssetBundle {
            m_object_hide_flags,
            m_extension_ptr,
            m_name,
            m_preload_table,
            m_container,
            m_main_asset,
            m_script_compatibility,
        })
    }
}
