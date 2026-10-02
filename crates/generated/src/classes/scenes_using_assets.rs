//! 自动生成。请勿手动修改。

use crate::subclasses::build_report_scenes_using_asset::BuildReportScenesUsingAsset;

#[derive(Debug, Clone)]
pub struct ScenesUsingAssets {
    pub m_scenes_using_assets: Vec<BuildReportScenesUsingAsset>,
    pub m_list_of_scenes_using_each_asset: Vec<(String, Vec<String>)>,
}

impl crate::Parse for ScenesUsingAssets {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_scenes_using_assets = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| BuildReportScenesUsingAsset::parse(rdr, ver))
                .collect::<Result<Vec<_>, _>>()
        })?;
        rdr.align(4)?;
        let m_list_of_scenes_using_each_asset = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_aligned_string()?;
                        let b = ({
                            let n = rdr.read_i32()? as usize;
                            (0..n)
                                .map(|_| rdr.read_aligned_string())
                                .collect::<Result<Vec<_>, _>>()
                        })?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;

        Ok(ScenesUsingAssets {
            m_scenes_using_assets,
            m_list_of_scenes_using_each_asset,
        })
    }
}

impl crate::ToValue for ScenesUsingAssets {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some((
                "m_ScenesUsingAssets".to_string(),
                self.m_scenes_using_assets.to_value(),
            )),
            Some((
                "m_ListOfScenesUsingEachAsset".to_string(),
                self.m_list_of_scenes_using_each_asset.to_value(),
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
                "m_ScenesUsingAssets" => {
                    (self.m_scenes_using_assets).inspect_value(path, offset, limit)
                }
                "m_ListOfScenesUsingEachAsset" => {
                    (self.m_list_of_scenes_using_each_asset).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_scenes_using_assets).inspect_value(path, offset, limit))
                .map(|v| ("m_ScenesUsingAssets".to_string(), v)),
            ((self.m_list_of_scenes_using_each_asset).inspect_value(path, offset, limit))
                .map(|v| ("m_ListOfScenesUsingEachAsset".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
