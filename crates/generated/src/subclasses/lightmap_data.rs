//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct LightmapData {
    pub m_lightmap: Option<PPtr>,
    pub sh_0: Option<f32>,
    pub m_indirect_lightmap: Option<PPtr>,
    pub sh_1: Option<f32>,
    pub m_dir_lightmap: Option<PPtr>,
    pub sh_2: Option<f32>,
    pub m_shadow_mask: Option<PPtr>,
    pub sh_3: Option<f32>,
    pub sh_4: Option<f32>,
    pub sh_5: Option<f32>,
    pub sh_6: Option<f32>,
    pub sh_7: Option<f32>,
    pub sh_8: Option<f32>,
    pub sh_9: Option<f32>,
    pub sh_10: Option<f32>,
    pub sh_11: Option<f32>,
    pub sh_12: Option<f32>,
    pub sh_13: Option<f32>,
    pub sh_14: Option<f32>,
    pub sh_15: Option<f32>,
    pub sh_16: Option<f32>,
    pub sh_17: Option<f32>,
    pub sh_18: Option<f32>,
    pub sh_19: Option<f32>,
    pub sh_20: Option<f32>,
    pub sh_21: Option<f32>,
    pub sh_22: Option<f32>,
    pub sh_23: Option<f32>,
    pub sh_24: Option<f32>,
    pub sh_25: Option<f32>,
    pub sh_26: Option<f32>,
}

impl crate::Parse for LightmapData {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut m_lightmap = None;
        let mut sh_0 = None;
        let mut m_indirect_lightmap = None;
        let mut sh_1 = None;
        let mut m_dir_lightmap = None;
        let mut sh_2 = None;
        let mut m_shadow_mask = None;
        let mut sh_3 = None;
        let mut sh_4 = None;
        let mut sh_5 = None;
        let mut sh_6 = None;
        let mut sh_7 = None;
        let mut sh_8 = None;
        let mut sh_9 = None;
        let mut sh_10 = None;
        let mut sh_11 = None;
        let mut sh_12 = None;
        let mut sh_13 = None;
        let mut sh_14 = None;
        let mut sh_15 = None;
        let mut sh_16 = None;
        let mut sh_17 = None;
        let mut sh_18 = None;
        let mut sh_19 = None;
        let mut sh_20 = None;
        let mut sh_21 = None;
        let mut sh_22 = None;
        let mut sh_23 = None;
        let mut sh_24 = None;
        let mut sh_25 = None;
        let mut sh_26 = None;
        if *ver >= crate::UnityVersion::new(5, 6, 0) {
            m_lightmap = Some(crate::PPtr::parse(rdr, ver)?);
            m_dir_lightmap = Some(crate::PPtr::parse(rdr, ver)?);
            m_shadow_mask = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 0) {
            m_lightmap = Some(crate::PPtr::parse(rdr, ver)?);
            m_indirect_lightmap = Some(crate::PPtr::parse(rdr, ver)?);
        } else if *ver >= crate::UnityVersion::new(3, 5, 0) {
            sh_0 = Some(rdr.read_f32()?);
            sh_1 = Some(rdr.read_f32()?);
            sh_2 = Some(rdr.read_f32()?);
            sh_3 = Some(rdr.read_f32()?);
            sh_4 = Some(rdr.read_f32()?);
            sh_5 = Some(rdr.read_f32()?);
            sh_6 = Some(rdr.read_f32()?);
            sh_7 = Some(rdr.read_f32()?);
            sh_8 = Some(rdr.read_f32()?);
            sh_9 = Some(rdr.read_f32()?);
            sh_10 = Some(rdr.read_f32()?);
            sh_11 = Some(rdr.read_f32()?);
            sh_12 = Some(rdr.read_f32()?);
            sh_13 = Some(rdr.read_f32()?);
            sh_14 = Some(rdr.read_f32()?);
            sh_15 = Some(rdr.read_f32()?);
            sh_16 = Some(rdr.read_f32()?);
            sh_17 = Some(rdr.read_f32()?);
            sh_18 = Some(rdr.read_f32()?);
            sh_19 = Some(rdr.read_f32()?);
            sh_20 = Some(rdr.read_f32()?);
            sh_21 = Some(rdr.read_f32()?);
            sh_22 = Some(rdr.read_f32()?);
            sh_23 = Some(rdr.read_f32()?);
            sh_24 = Some(rdr.read_f32()?);
            sh_25 = Some(rdr.read_f32()?);
            sh_26 = Some(rdr.read_f32()?);
        } else {
            m_lightmap = Some(crate::PPtr::parse(rdr, ver)?);
            m_indirect_lightmap = Some(crate::PPtr::parse(rdr, ver)?);
        }

        Ok(LightmapData {
            m_lightmap,
            sh_0,
            m_indirect_lightmap,
            sh_1,
            m_dir_lightmap,
            sh_2,
            m_shadow_mask,
            sh_3,
            sh_4,
            sh_5,
            sh_6,
            sh_7,
            sh_8,
            sh_9,
            sh_10,
            sh_11,
            sh_12,
            sh_13,
            sh_14,
            sh_15,
            sh_16,
            sh_17,
            sh_18,
            sh_19,
            sh_20,
            sh_21,
            sh_22,
            sh_23,
            sh_24,
            sh_25,
            sh_26,
        })
    }
}

impl crate::ToValue for LightmapData {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            crate::opt_field("m_Lightmap", &self.m_lightmap),
            crate::opt_field("sh[0]", &self.sh_0),
            crate::opt_field("m_IndirectLightmap", &self.m_indirect_lightmap),
            crate::opt_field("sh[1]", &self.sh_1),
            crate::opt_field("m_DirLightmap", &self.m_dir_lightmap),
            crate::opt_field("sh[2]", &self.sh_2),
            crate::opt_field("m_ShadowMask", &self.m_shadow_mask),
            crate::opt_field("sh[3]", &self.sh_3),
            crate::opt_field("sh[4]", &self.sh_4),
            crate::opt_field("sh[5]", &self.sh_5),
            crate::opt_field("sh[6]", &self.sh_6),
            crate::opt_field("sh[7]", &self.sh_7),
            crate::opt_field("sh[8]", &self.sh_8),
            crate::opt_field("sh[9]", &self.sh_9),
            crate::opt_field("sh[10]", &self.sh_10),
            crate::opt_field("sh[11]", &self.sh_11),
            crate::opt_field("sh[12]", &self.sh_12),
            crate::opt_field("sh[13]", &self.sh_13),
            crate::opt_field("sh[14]", &self.sh_14),
            crate::opt_field("sh[15]", &self.sh_15),
            crate::opt_field("sh[16]", &self.sh_16),
            crate::opt_field("sh[17]", &self.sh_17),
            crate::opt_field("sh[18]", &self.sh_18),
            crate::opt_field("sh[19]", &self.sh_19),
            crate::opt_field("sh[20]", &self.sh_20),
            crate::opt_field("sh[21]", &self.sh_21),
            crate::opt_field("sh[22]", &self.sh_22),
            crate::opt_field("sh[23]", &self.sh_23),
            crate::opt_field("sh[24]", &self.sh_24),
            crate::opt_field("sh[25]", &self.sh_25),
            crate::opt_field("sh[26]", &self.sh_26),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_Lightmap" => (self.m_lightmap).inspect_value(path, offset, limit),
                "sh[0]" => (self.sh_0).inspect_value(path, offset, limit),
                "m_IndirectLightmap" => {
                    (self.m_indirect_lightmap).inspect_value(path, offset, limit)
                }
                "sh[1]" => (self.sh_1).inspect_value(path, offset, limit),
                "m_DirLightmap" => (self.m_dir_lightmap).inspect_value(path, offset, limit),
                "sh[2]" => (self.sh_2).inspect_value(path, offset, limit),
                "m_ShadowMask" => (self.m_shadow_mask).inspect_value(path, offset, limit),
                "sh[3]" => (self.sh_3).inspect_value(path, offset, limit),
                "sh[4]" => (self.sh_4).inspect_value(path, offset, limit),
                "sh[5]" => (self.sh_5).inspect_value(path, offset, limit),
                "sh[6]" => (self.sh_6).inspect_value(path, offset, limit),
                "sh[7]" => (self.sh_7).inspect_value(path, offset, limit),
                "sh[8]" => (self.sh_8).inspect_value(path, offset, limit),
                "sh[9]" => (self.sh_9).inspect_value(path, offset, limit),
                "sh[10]" => (self.sh_10).inspect_value(path, offset, limit),
                "sh[11]" => (self.sh_11).inspect_value(path, offset, limit),
                "sh[12]" => (self.sh_12).inspect_value(path, offset, limit),
                "sh[13]" => (self.sh_13).inspect_value(path, offset, limit),
                "sh[14]" => (self.sh_14).inspect_value(path, offset, limit),
                "sh[15]" => (self.sh_15).inspect_value(path, offset, limit),
                "sh[16]" => (self.sh_16).inspect_value(path, offset, limit),
                "sh[17]" => (self.sh_17).inspect_value(path, offset, limit),
                "sh[18]" => (self.sh_18).inspect_value(path, offset, limit),
                "sh[19]" => (self.sh_19).inspect_value(path, offset, limit),
                "sh[20]" => (self.sh_20).inspect_value(path, offset, limit),
                "sh[21]" => (self.sh_21).inspect_value(path, offset, limit),
                "sh[22]" => (self.sh_22).inspect_value(path, offset, limit),
                "sh[23]" => (self.sh_23).inspect_value(path, offset, limit),
                "sh[24]" => (self.sh_24).inspect_value(path, offset, limit),
                "sh[25]" => (self.sh_25).inspect_value(path, offset, limit),
                "sh[26]" => (self.sh_26).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_lightmap).inspect_value(path, offset, limit))
                .map(|v| ("m_Lightmap".to_string(), v)),
            ((self.sh_0).inspect_value(path, offset, limit)).map(|v| ("sh[0]".to_string(), v)),
            ((self.m_indirect_lightmap).inspect_value(path, offset, limit))
                .map(|v| ("m_IndirectLightmap".to_string(), v)),
            ((self.sh_1).inspect_value(path, offset, limit)).map(|v| ("sh[1]".to_string(), v)),
            ((self.m_dir_lightmap).inspect_value(path, offset, limit))
                .map(|v| ("m_DirLightmap".to_string(), v)),
            ((self.sh_2).inspect_value(path, offset, limit)).map(|v| ("sh[2]".to_string(), v)),
            ((self.m_shadow_mask).inspect_value(path, offset, limit))
                .map(|v| ("m_ShadowMask".to_string(), v)),
            ((self.sh_3).inspect_value(path, offset, limit)).map(|v| ("sh[3]".to_string(), v)),
            ((self.sh_4).inspect_value(path, offset, limit)).map(|v| ("sh[4]".to_string(), v)),
            ((self.sh_5).inspect_value(path, offset, limit)).map(|v| ("sh[5]".to_string(), v)),
            ((self.sh_6).inspect_value(path, offset, limit)).map(|v| ("sh[6]".to_string(), v)),
            ((self.sh_7).inspect_value(path, offset, limit)).map(|v| ("sh[7]".to_string(), v)),
            ((self.sh_8).inspect_value(path, offset, limit)).map(|v| ("sh[8]".to_string(), v)),
            ((self.sh_9).inspect_value(path, offset, limit)).map(|v| ("sh[9]".to_string(), v)),
            ((self.sh_10).inspect_value(path, offset, limit)).map(|v| ("sh[10]".to_string(), v)),
            ((self.sh_11).inspect_value(path, offset, limit)).map(|v| ("sh[11]".to_string(), v)),
            ((self.sh_12).inspect_value(path, offset, limit)).map(|v| ("sh[12]".to_string(), v)),
            ((self.sh_13).inspect_value(path, offset, limit)).map(|v| ("sh[13]".to_string(), v)),
            ((self.sh_14).inspect_value(path, offset, limit)).map(|v| ("sh[14]".to_string(), v)),
            ((self.sh_15).inspect_value(path, offset, limit)).map(|v| ("sh[15]".to_string(), v)),
            ((self.sh_16).inspect_value(path, offset, limit)).map(|v| ("sh[16]".to_string(), v)),
            ((self.sh_17).inspect_value(path, offset, limit)).map(|v| ("sh[17]".to_string(), v)),
            ((self.sh_18).inspect_value(path, offset, limit)).map(|v| ("sh[18]".to_string(), v)),
            ((self.sh_19).inspect_value(path, offset, limit)).map(|v| ("sh[19]".to_string(), v)),
            ((self.sh_20).inspect_value(path, offset, limit)).map(|v| ("sh[20]".to_string(), v)),
            ((self.sh_21).inspect_value(path, offset, limit)).map(|v| ("sh[21]".to_string(), v)),
            ((self.sh_22).inspect_value(path, offset, limit)).map(|v| ("sh[22]".to_string(), v)),
            ((self.sh_23).inspect_value(path, offset, limit)).map(|v| ("sh[23]".to_string(), v)),
            ((self.sh_24).inspect_value(path, offset, limit)).map(|v| ("sh[24]".to_string(), v)),
            ((self.sh_25).inspect_value(path, offset, limit)).map(|v| ("sh[25]".to_string(), v)),
            ((self.sh_26).inspect_value(path, offset, limit)).map(|v| ("sh[26]".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
