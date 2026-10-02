//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct ComponentPair {
    pub first: Option<i32>,
    pub component: Option<PPtr>,
    pub second: Option<PPtr>,
}

impl crate::Parse for ComponentPair {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut first = None;
        let mut component = None;
        let mut second = None;
        if *ver >= crate::UnityVersion::new(5, 5, 0) {
            component = Some(crate::PPtr::parse(rdr, ver)?);
        } else {
            first = Some(rdr.read_i32()?);
            second = Some(crate::PPtr::parse(rdr, ver)?);
        }

        Ok(ComponentPair {
            first,
            component,
            second,
        })
    }
}

impl crate::ToValue for ComponentPair {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("first", &self.first),
            crate::opt_field("component", &self.component),
            crate::opt_field("second", &self.second),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "first" => (self.first).inspect_value(path, offset, limit),
                "component" => (self.component).inspect_value(path, offset, limit),
                "second" => (self.second).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.first).inspect_value(path, offset, limit)).map(|v| ("first".to_string(), v)),
            ((self.component).inspect_value(path, offset, limit))
                .map(|v| ("component".to_string(), v)),
            ((self.second).inspect_value(path, offset, limit)).map(|v| ("second".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
