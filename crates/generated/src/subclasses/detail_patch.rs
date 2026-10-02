//! 自动生成。请勿手动修改。

use crate::subclasses::aabb::Aabb;

#[derive(Debug, Clone)]
pub struct DetailPatch {
    pub bounds: Option<Aabb>,
    pub layer_indices: Vec<u8>,
    pub number_of_objects: Option<Vec<u8>>,
    pub coverage: Option<Vec<u8>>,
}

impl crate::Parse for DetailPatch {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut bounds = None;
        let mut layer_indices = None;
        let mut number_of_objects = None;
        let mut coverage = None;
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
        } else {
            bounds = Some(Aabb::parse(rdr, ver)?);
        }
        layer_indices = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2022, 2, 0) {
            coverage = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
            number_of_objects = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        }

        Ok(DetailPatch {
            bounds,
            layer_indices: layer_indices.unwrap(),
            number_of_objects,
            coverage,
        })
    }
}

impl crate::ToValue for DetailPatch {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("bounds", &self.bounds),
            Some(("layerIndices".to_string(), self.layer_indices.to_value())),
            crate::opt_field("numberOfObjects", &self.number_of_objects),
            crate::opt_field("coverage", &self.coverage),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "bounds" => (self.bounds).inspect_value(path, offset, limit),
                "layerIndices" => (self.layer_indices).inspect_value(path, offset, limit),
                "numberOfObjects" => (self.number_of_objects).inspect_value(path, offset, limit),
                "coverage" => (self.coverage).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.bounds).inspect_value(path, offset, limit)).map(|v| ("bounds".to_string(), v)),
            ((self.layer_indices).inspect_value(path, offset, limit))
                .map(|v| ("layerIndices".to_string(), v)),
            ((self.number_of_objects).inspect_value(path, offset, limit))
                .map(|v| ("numberOfObjects".to_string(), v)),
            ((self.coverage).inspect_value(path, offset, limit))
                .map(|v| ("coverage".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
