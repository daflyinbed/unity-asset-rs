//! 自动生成。请勿手动修改。

use crate::subclasses::guid::Guid;

#[derive(Debug, Clone)]
pub struct AudioMixerGroupView {
    pub guids: Vec<Guid>,
    pub name: String,
}

impl crate::Parse for AudioMixerGroupView {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let guids = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| Guid::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        let name = rdr.read_aligned_string()?;

        Ok(AudioMixerGroupView { guids, name })
    }
}

impl crate::ToValue for AudioMixerGroupView {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("guids".to_string(), self.guids.to_value())),
            Some(("name".to_string(), self.name.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "guids" => (self.guids).inspect_value(path, offset, limit),
                "name" => (self.name).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.guids).inspect_value(path, offset, limit)).map(|v| ("guids".to_string(), v)),
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
