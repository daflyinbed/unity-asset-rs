//! 自动生成。请勿手动修改。

use crate::subclasses::render_state_block::RenderStateBlock;

#[derive(Debug, Clone)]
pub struct RenderStateInfo {
    pub render_state: RenderStateBlock,
}

impl crate::Parse for RenderStateInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let render_state = RenderStateBlock::parse(rdr, ver)?;

        Ok(RenderStateInfo { render_state })
    }
}

impl crate::ToValue for RenderStateInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [Some((
            "renderState".to_string(),
            self.render_state.to_value(),
        ))]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "renderState" => (self.render_state).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.render_state).inspect_value(path, offset, limit))
                .map(|v| ("renderState".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
