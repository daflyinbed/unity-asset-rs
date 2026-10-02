//! 自动生成。请勿手动修改。

use crate::subclasses::avatar_constant::AvatarConstant;
use crate::subclasses::human_description::HumanDescription;

#[derive(Debug, Clone)]
pub struct Avatar {
    pub m_name: String,
    pub m_avatar_size: u32,
    pub m_avatar: AvatarConstant,
    pub m_tos: Vec<(u32, String)>,
    pub m_human_description: Option<HumanDescription>,
}

impl crate::Parse for Avatar {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let m_avatar_size = rdr.read_u32()?;
        let m_avatar = AvatarConstant::parse(rdr, ver)?;
        let m_tos = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| {
                    ({
                        let a = rdr.read_u32()?;
                        let b = rdr.read_aligned_string()?;
                        Ok((a, b))
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_human_description = if *ver >= crate::UnityVersion::new(2019, 1, 0) {
            Some(HumanDescription::parse(rdr, ver)?)
        } else {
            None
        };

        Ok(Avatar {
            m_name,
            m_avatar_size,
            m_avatar,
            m_tos,
            m_human_description,
        })
    }
}

impl crate::ToValue for Avatar {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_Name".to_string(), self.m_name.to_value())),
            Some(("m_AvatarSize".to_string(), self.m_avatar_size.to_value())),
            Some(("m_Avatar".to_string(), self.m_avatar.to_value())),
            Some(("m_TOS".to_string(), self.m_tos.to_value())),
            crate::opt_field("m_HumanDescription", &self.m_human_description),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Name" => (self.m_name).inspect_value(path, offset, limit),
                "m_AvatarSize" => (self.m_avatar_size).inspect_value(path, offset, limit),
                "m_Avatar" => (self.m_avatar).inspect_value(path, offset, limit),
                "m_TOS" => (self.m_tos).inspect_value(path, offset, limit),
                "m_HumanDescription" => {
                    (self.m_human_description).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_name).inspect_value(path, offset, limit)).map(|v| ("m_Name".to_string(), v)),
            ((self.m_avatar_size).inspect_value(path, offset, limit))
                .map(|v| ("m_AvatarSize".to_string(), v)),
            ((self.m_avatar).inspect_value(path, offset, limit))
                .map(|v| ("m_Avatar".to_string(), v)),
            ((self.m_tos).inspect_value(path, offset, limit)).map(|v| ("m_TOS".to_string(), v)),
            ((self.m_human_description).inspect_value(path, offset, limit))
                .map(|v| ("m_HumanDescription".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
