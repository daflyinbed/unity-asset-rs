//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BuildTargetGraphicsApIs {
    pub m_build_target: String,
    pub m_ap_is: Vec<i32>,
    pub m_automatic: bool,
}

impl crate::Parse for BuildTargetGraphicsApIs {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_build_target = rdr.read_aligned_string()?;
        let m_ap_is = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_i32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_automatic = rdr.read_bool()?;
        rdr.align(4)?;

        Ok(BuildTargetGraphicsApIs {
            m_build_target,
            m_ap_is,
            m_automatic,
        })
    }
}

impl crate::ToValue for BuildTargetGraphicsApIs {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_BuildTarget".to_string(), self.m_build_target.to_value())),
            Some(("m_APIs".to_string(), self.m_ap_is.to_value())),
            Some(("m_Automatic".to_string(), self.m_automatic.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_BuildTarget" => (self.m_build_target).inspect_value(path, offset, limit),
                "m_APIs" => (self.m_ap_is).inspect_value(path, offset, limit),
                "m_Automatic" => (self.m_automatic).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_build_target).inspect_value(path, offset, limit))
                .map(|v| ("m_BuildTarget".to_string(), v)),
            ((self.m_ap_is).inspect_value(path, offset, limit)).map(|v| ("m_APIs".to_string(), v)),
            ((self.m_automatic).inspect_value(path, offset, limit))
                .map(|v| ("m_Automatic".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
