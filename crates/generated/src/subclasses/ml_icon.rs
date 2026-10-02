//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct MlIcon {
    pub m_name: String,
    pub m_model_folder_path: String,
    pub m_portal_folder_path: String,
}

impl crate::Parse for MlIcon {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_model_folder_path = rdr.read_aligned_string()?;
        let m_portal_folder_path = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(MlIcon {
            m_name,
            m_model_folder_path,
            m_portal_folder_path,
        })
    }
}

impl crate::ToValue for MlIcon {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some((
                "m_ModelFolderPath".to_string(),
                self.m_model_folder_path.to_value(),
            )),
            Some((
                "m_PortalFolderPath".to_string(),
                self.m_portal_folder_path.to_value(),
            )),
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
                "m_ModelFolderPath" => {
                    (self.m_model_folder_path).inspect_value(path, offset, limit)
                }
                "m_PortalFolderPath" => {
                    (self.m_portal_folder_path).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_model_folder_path).inspect_value(path, offset, limit))
                .map(|v| ("m_ModelFolderPath".to_string(), v)),
            ((self.m_portal_folder_path).inspect_value(path, offset, limit))
                .map(|v| ("m_PortalFolderPath".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
