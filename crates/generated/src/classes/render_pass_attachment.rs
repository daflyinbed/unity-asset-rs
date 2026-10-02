//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct RenderPassAttachment {}

impl crate::Parse for RenderPassAttachment {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        Ok(RenderPassAttachment {})
    }
}

impl crate::ToValue for RenderPassAttachment {
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
