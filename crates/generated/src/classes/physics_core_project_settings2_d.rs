//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct PhysicsCoreProjectSettings2D {
    pub m_physics_core_settings: PPtr,
}

impl crate::Parse for PhysicsCoreProjectSettings2D {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_physics_core_settings = crate::PPtr::parse(rdr, ver)?;

        Ok(PhysicsCoreProjectSettings2D {
            m_physics_core_settings,
        })
    }
}

impl crate::ToValue for PhysicsCoreProjectSettings2D {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "m_PhysicsCoreSettings".to_string(),
            self.m_physics_core_settings.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_PhysicsCoreSettings" => {
                    (self.m_physics_core_settings).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_physics_core_settings).inspect_value(path, offset, limit))
                .map(|v| ("m_PhysicsCoreSettings".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
