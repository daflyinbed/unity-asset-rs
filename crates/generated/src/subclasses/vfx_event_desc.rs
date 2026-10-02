//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct VfxEventDesc {
    pub name: String,
    pub play_systems: Vec<u32>,
    pub stop_systems: Vec<u32>,
    pub init_systems: Option<Vec<u32>>,
}

impl crate::Parse for VfxEventDesc {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let name = rdr.read_aligned_string()?;
        let play_systems = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let stop_systems = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_u32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let init_systems = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Some(v)
        } else {
            None
        };

        Ok(VfxEventDesc {
            name,
            play_systems,
            stop_systems,
            init_systems,
        })
    }
}

impl crate::ToValue for VfxEventDesc {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("name".to_string(), self.name.to_value())),
            Some(("playSystems".to_string(), self.play_systems.to_value())),
            Some(("stopSystems".to_string(), self.stop_systems.to_value())),
            crate::opt_field("initSystems", &self.init_systems),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "name" => (self.name).inspect_value(path, offset, limit),
                "playSystems" => (self.play_systems).inspect_value(path, offset, limit),
                "stopSystems" => (self.stop_systems).inspect_value(path, offset, limit),
                "initSystems" => (self.init_systems).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.name).inspect_value(path, offset, limit)).map(|v| ("name".to_string(), v)),
            ((self.play_systems).inspect_value(path, offset, limit))
                .map(|v| ("playSystems".to_string(), v)),
            ((self.stop_systems).inspect_value(path, offset, limit))
                .map(|v| ("stopSystems".to_string(), v)),
            ((self.init_systems).inspect_value(path, offset, limit))
                .map(|v| ("initSystems".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
