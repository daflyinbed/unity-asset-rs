//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct NavMeshSettings {
    pub m_nav_mesh: Option<PPtr>,
    pub m_nav_mesh_data: Option<PPtr>,
}

impl crate::Parse for NavMeshSettings {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_nav_mesh = None;
        let mut m_nav_mesh_data = None;
        if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_nav_mesh_data = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
            m_nav_mesh = Some(crate::PPtr::parse(rdr, ver)?);
        }

        Ok(NavMeshSettings {
            m_nav_mesh,
            m_nav_mesh_data,
        })
    }
}

impl crate::ToValue for NavMeshSettings {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_NavMesh", &self.m_nav_mesh),
            crate::opt_field("m_NavMeshData", &self.m_nav_mesh_data),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_NavMesh" => (self.m_nav_mesh).inspect_value(path, offset, limit),
                "m_NavMeshData" => (self.m_nav_mesh_data).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_nav_mesh).inspect_value(path, offset, limit))
                .map(|v| ("m_NavMesh".to_string(), v)),
            ((self.m_nav_mesh_data).inspect_value(path, offset, limit))
                .map(|v| ("m_NavMeshData".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
