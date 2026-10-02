//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct HierarchyState {
    pub expanded: Vec<PPtr>,
    pub selection: Vec<PPtr>,
    pub scrollposition_x: f32,
    pub scrollposition_y: f32,
}

impl crate::Parse for HierarchyState {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let expanded = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let selection = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| crate::PPtr::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let scrollposition_x = rdr.read_f32()?;
        let scrollposition_y = rdr.read_f32()?;

        Ok(HierarchyState {
            expanded,
            selection,
            scrollposition_x,
            scrollposition_y,
        })
    }
}

impl crate::ToValue for HierarchyState {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("expanded".to_string(), self.expanded.to_value())),
            Some(("selection".to_string(), self.selection.to_value())),
            Some((
                "scrollposition_x".to_string(),
                self.scrollposition_x.to_value(),
            )),
            Some((
                "scrollposition_y".to_string(),
                self.scrollposition_y.to_value(),
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
                "expanded" => (self.expanded).inspect_value(path, offset, limit),
                "selection" => (self.selection).inspect_value(path, offset, limit),
                "scrollposition_x" => (self.scrollposition_x).inspect_value(path, offset, limit),
                "scrollposition_y" => (self.scrollposition_y).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.expanded).inspect_value(path, offset, limit))
                .map(|v| ("expanded".to_string(), v)),
            ((self.selection).inspect_value(path, offset, limit))
                .map(|v| ("selection".to_string(), v)),
            ((self.scrollposition_x).inspect_value(path, offset, limit))
                .map(|v| ("scrollposition_x".to_string(), v)),
            ((self.scrollposition_y).inspect_value(path, offset, limit))
                .map(|v| ("scrollposition_y".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
