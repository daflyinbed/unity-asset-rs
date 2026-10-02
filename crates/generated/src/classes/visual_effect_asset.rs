//! 自动生成。请勿手动修改。

use crate::subclasses::vfx_system_desc::VfxSystemDesc;
use crate::subclasses::visual_effect_info::VisualEffectInfo;

#[derive(Debug, Clone)]
pub struct VisualEffectAsset {
    pub m_name: String,
    pub m_infos: VisualEffectInfo,
    pub m_systems: Vec<VfxSystemDesc>,
}

impl crate::Parse for VisualEffectAsset {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_infos = VisualEffectInfo::parse(rdr, ver)?;
        let m_systems = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| VfxSystemDesc::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;

        Ok(VisualEffectAsset {
            m_name,
            m_infos,
            m_systems,
        })
    }
}

impl crate::ToValue for VisualEffectAsset {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_Infos".to_string(), self.m_infos.to_value())),
            Some(("m_Systems".to_string(), self.m_systems.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_Infos" => (self.m_infos).inspect_value(path, offset, limit),
                "m_Systems" => (self.m_systems).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_infos).inspect_value(path, offset, limit)).map(|v| ("m_Infos".to_string(), v)),
            ((self.m_systems).inspect_value(path, offset, limit))
                .map(|v| ("m_Systems".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
