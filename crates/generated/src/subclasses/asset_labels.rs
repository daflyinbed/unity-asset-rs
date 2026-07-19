//! 自动生成。请勿手动修改。

/// 版本枚举：AssetLabelsMLabelsVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum AssetLabelsMLabelsVariant {
    Default(Vec<String>),
    V2017_1(Vec<String>),
}

impl crate::Parse for AssetLabelsMLabelsVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(2017, 1, 0) {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            rdr.align(4)?;
            Ok(Self::V2017_1(v))
        } else {
            let v = ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_aligned_string())
                    .collect::<Result<Vec<_>, _>>()
            })?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct AssetLabels {
    pub m_labels: AssetLabelsMLabelsVariant,
}

impl crate::Parse for AssetLabels {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_labels = AssetLabelsMLabelsVariant::parse(rdr, ver)?;

        Ok(AssetLabels { m_labels })
    }
}
