//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct LuminSigningCert {
    pub m_cert_path: String,
    pub m_private_key_path: String,
}

impl crate::Parse for LuminSigningCert {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_cert_path = rdr.read_aligned_string()?;
        let m_private_key_path = rdr.read_aligned_string()?;
        rdr.align(4)?;

        Ok(LuminSigningCert {
            m_cert_path,
            m_private_key_path,
        })
    }
}

impl crate::ToValue for LuminSigningCert {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_CertPath".to_string(), self.m_cert_path.to_value())),
            Some((
                "m_PrivateKeyPath".to_string(),
                self.m_private_key_path.to_value(),
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
                "m_CertPath" => (self.m_cert_path).inspect_value(path, offset, limit),
                "m_PrivateKeyPath" => (self.m_private_key_path).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_cert_path).inspect_value(path, offset, limit))
                .map(|v| ("m_CertPath".to_string(), v)),
            ((self.m_private_key_path).inspect_value(path, offset, limit))
                .map(|v| ("m_PrivateKeyPath".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
