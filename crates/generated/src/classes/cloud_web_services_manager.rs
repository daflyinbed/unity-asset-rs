//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct CloudWebServicesManager {}

impl crate::Parse for CloudWebServicesManager {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        Ok(CloudWebServicesManager {})
    }
}

impl crate::ToValue for CloudWebServicesManager {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = Vec::new();
        crate::Value::Struct(fields)
    }
    fn inspect_value(
        &self,
        path: &[String],
        _offset: usize,
        _limit: usize,
    ) -> Option<crate::Value> {
        path.is_empty().then(|| crate::Value::Struct(Vec::new()))
    }
}
