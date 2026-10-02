//! 自动生成。请勿手动修改。

use crate::subclasses::layout_data_three::LayoutDataThree;
use crate::subclasses::layout_data_two::LayoutDataTwo;

#[derive(Debug, Clone)]
pub struct TestObjectWithSpecialLayoutTwo {
    pub different_layout: LayoutDataTwo,
    pub same_layout: LayoutDataThree,
}

impl crate::Parse for TestObjectWithSpecialLayoutTwo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let different_layout = LayoutDataTwo::parse(rdr, ver)?;
        let same_layout = LayoutDataThree::parse(rdr, ver)?;

        Ok(TestObjectWithSpecialLayoutTwo {
            different_layout,
            same_layout,
        })
    }
}

impl crate::ToValue for TestObjectWithSpecialLayoutTwo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "differentLayout".to_string(),
                self.different_layout.to_value(),
            )),
            Some(("sameLayout".to_string(), self.same_layout.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "differentLayout" => (self.different_layout).inspect_value(path, offset, limit),
                "sameLayout" => (self.same_layout).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.different_layout).inspect_value(path, offset, limit))
                .map(|v| ("differentLayout".to_string(), v)),
            ((self.same_layout).inspect_value(path, offset, limit))
                .map(|v| ("sameLayout".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
