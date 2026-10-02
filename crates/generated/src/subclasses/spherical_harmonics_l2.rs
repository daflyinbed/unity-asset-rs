//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct SphericalHarmonicsL2 {
    pub sh_0: f32,
    pub sh_1: f32,
    pub sh_2: f32,
    pub sh_3: f32,
    pub sh_4: f32,
    pub sh_5: f32,
    pub sh_6: f32,
    pub sh_7: f32,
    pub sh_8: f32,
    pub sh_9: f32,
    pub sh_10: f32,
    pub sh_11: f32,
    pub sh_12: f32,
    pub sh_13: f32,
    pub sh_14: f32,
    pub sh_15: f32,
    pub sh_16: f32,
    pub sh_17: f32,
    pub sh_18: f32,
    pub sh_19: f32,
    pub sh_20: f32,
    pub sh_21: f32,
    pub sh_22: f32,
    pub sh_23: f32,
    pub sh_24: f32,
    pub sh_25: f32,
    pub sh_26: f32,
}

impl crate::Parse for SphericalHarmonicsL2 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let sh_0 = rdr.read_f32()?;
        let sh_1 = rdr.read_f32()?;
        let sh_2 = rdr.read_f32()?;
        let sh_3 = rdr.read_f32()?;
        let sh_4 = rdr.read_f32()?;
        let sh_5 = rdr.read_f32()?;
        let sh_6 = rdr.read_f32()?;
        let sh_7 = rdr.read_f32()?;
        let sh_8 = rdr.read_f32()?;
        let sh_9 = rdr.read_f32()?;
        let sh_10 = rdr.read_f32()?;
        let sh_11 = rdr.read_f32()?;
        let sh_12 = rdr.read_f32()?;
        let sh_13 = rdr.read_f32()?;
        let sh_14 = rdr.read_f32()?;
        let sh_15 = rdr.read_f32()?;
        let sh_16 = rdr.read_f32()?;
        let sh_17 = rdr.read_f32()?;
        let sh_18 = rdr.read_f32()?;
        let sh_19 = rdr.read_f32()?;
        let sh_20 = rdr.read_f32()?;
        let sh_21 = rdr.read_f32()?;
        let sh_22 = rdr.read_f32()?;
        let sh_23 = rdr.read_f32()?;
        let sh_24 = rdr.read_f32()?;
        let sh_25 = rdr.read_f32()?;
        let sh_26 = rdr.read_f32()?;

        Ok(SphericalHarmonicsL2 {
            sh_0,
            sh_1,
            sh_2,
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

impl crate::ToValue for SphericalHarmonicsL2 {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("sh[ 0]".to_string(), self.sh_0.to_value())),
            Some(("sh[ 1]".to_string(), self.sh_1.to_value())),
            Some(("sh[ 2]".to_string(), self.sh_2.to_value())),
            Some(("sh[ 3]".to_string(), self.sh_3.to_value())),
            Some(("sh[ 4]".to_string(), self.sh_4.to_value())),
            Some(("sh[ 5]".to_string(), self.sh_5.to_value())),
            Some(("sh[ 6]".to_string(), self.sh_6.to_value())),
            Some(("sh[ 7]".to_string(), self.sh_7.to_value())),
            Some(("sh[ 8]".to_string(), self.sh_8.to_value())),
            Some(("sh[ 9]".to_string(), self.sh_9.to_value())),
            Some(("sh[10]".to_string(), self.sh_10.to_value())),
            Some(("sh[11]".to_string(), self.sh_11.to_value())),
            Some(("sh[12]".to_string(), self.sh_12.to_value())),
            Some(("sh[13]".to_string(), self.sh_13.to_value())),
            Some(("sh[14]".to_string(), self.sh_14.to_value())),
            Some(("sh[15]".to_string(), self.sh_15.to_value())),
            Some(("sh[16]".to_string(), self.sh_16.to_value())),
            Some(("sh[17]".to_string(), self.sh_17.to_value())),
            Some(("sh[18]".to_string(), self.sh_18.to_value())),
            Some(("sh[19]".to_string(), self.sh_19.to_value())),
            Some(("sh[20]".to_string(), self.sh_20.to_value())),
            Some(("sh[21]".to_string(), self.sh_21.to_value())),
            Some(("sh[22]".to_string(), self.sh_22.to_value())),
            Some(("sh[23]".to_string(), self.sh_23.to_value())),
            Some(("sh[24]".to_string(), self.sh_24.to_value())),
            Some(("sh[25]".to_string(), self.sh_25.to_value())),
            Some(("sh[26]".to_string(), self.sh_26.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "sh[ 0]" => (self.sh_0).inspect_value(path, offset, limit),
                "sh[ 1]" => (self.sh_1).inspect_value(path, offset, limit),
                "sh[ 2]" => (self.sh_2).inspect_value(path, offset, limit),
                "sh[ 3]" => (self.sh_3).inspect_value(path, offset, limit),
                "sh[ 4]" => (self.sh_4).inspect_value(path, offset, limit),
                "sh[ 5]" => (self.sh_5).inspect_value(path, offset, limit),
                "sh[ 6]" => (self.sh_6).inspect_value(path, offset, limit),
                "sh[ 7]" => (self.sh_7).inspect_value(path, offset, limit),
                "sh[ 8]" => (self.sh_8).inspect_value(path, offset, limit),
                "sh[ 9]" => (self.sh_9).inspect_value(path, offset, limit),
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
            ((self.sh_0).inspect_value(path, offset, limit)).map(|v| ("sh[ 0]".to_string(), v)),
            ((self.sh_1).inspect_value(path, offset, limit)).map(|v| ("sh[ 1]".to_string(), v)),
            ((self.sh_2).inspect_value(path, offset, limit)).map(|v| ("sh[ 2]".to_string(), v)),
            ((self.sh_3).inspect_value(path, offset, limit)).map(|v| ("sh[ 3]".to_string(), v)),
            ((self.sh_4).inspect_value(path, offset, limit)).map(|v| ("sh[ 4]".to_string(), v)),
            ((self.sh_5).inspect_value(path, offset, limit)).map(|v| ("sh[ 5]".to_string(), v)),
            ((self.sh_6).inspect_value(path, offset, limit)).map(|v| ("sh[ 6]".to_string(), v)),
            ((self.sh_7).inspect_value(path, offset, limit)).map(|v| ("sh[ 7]".to_string(), v)),
            ((self.sh_8).inspect_value(path, offset, limit)).map(|v| ("sh[ 8]".to_string(), v)),
            ((self.sh_9).inspect_value(path, offset, limit)).map(|v| ("sh[ 9]".to_string(), v)),
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
