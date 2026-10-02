//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct BoneWeights4 {
    pub weight_0: f32,
    pub weight_1: f32,
    pub weight_2: f32,
    pub weight_3: f32,
    pub bone_index_0: i32,
    pub bone_index_1: i32,
    pub bone_index_2: i32,
    pub bone_index_3: i32,
}

impl crate::Parse for BoneWeights4 {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let weight_0 = rdr.read_f32()?;
        let weight_1 = rdr.read_f32()?;
        let weight_2 = rdr.read_f32()?;
        let weight_3 = rdr.read_f32()?;
        let bone_index_0 = rdr.read_i32()?;
        let bone_index_1 = rdr.read_i32()?;
        let bone_index_2 = rdr.read_i32()?;
        let bone_index_3 = rdr.read_i32()?;

        Ok(BoneWeights4 {
            weight_0,
            weight_1,
            weight_2,
            weight_3,
            bone_index_0,
            bone_index_1,
            bone_index_2,
            bone_index_3,
        })
    }
}

impl crate::ToValue for BoneWeights4 {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("weight[0]".to_string(), self.weight_0.to_value())),
            Some(("weight[1]".to_string(), self.weight_1.to_value())),
            Some(("weight[2]".to_string(), self.weight_2.to_value())),
            Some(("weight[3]".to_string(), self.weight_3.to_value())),
            Some(("boneIndex[0]".to_string(), self.bone_index_0.to_value())),
            Some(("boneIndex[1]".to_string(), self.bone_index_1.to_value())),
            Some(("boneIndex[2]".to_string(), self.bone_index_2.to_value())),
            Some(("boneIndex[3]".to_string(), self.bone_index_3.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "weight[0]" => (self.weight_0).inspect_value(path, offset, limit),
                "weight[1]" => (self.weight_1).inspect_value(path, offset, limit),
                "weight[2]" => (self.weight_2).inspect_value(path, offset, limit),
                "weight[3]" => (self.weight_3).inspect_value(path, offset, limit),
                "boneIndex[0]" => (self.bone_index_0).inspect_value(path, offset, limit),
                "boneIndex[1]" => (self.bone_index_1).inspect_value(path, offset, limit),
                "boneIndex[2]" => (self.bone_index_2).inspect_value(path, offset, limit),
                "boneIndex[3]" => (self.bone_index_3).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.weight_0).inspect_value(path, offset, limit))
                .map(|v| ("weight[0]".to_string(), v)),
            ((self.weight_1).inspect_value(path, offset, limit))
                .map(|v| ("weight[1]".to_string(), v)),
            ((self.weight_2).inspect_value(path, offset, limit))
                .map(|v| ("weight[2]".to_string(), v)),
            ((self.weight_3).inspect_value(path, offset, limit))
                .map(|v| ("weight[3]".to_string(), v)),
            ((self.bone_index_0).inspect_value(path, offset, limit))
                .map(|v| ("boneIndex[0]".to_string(), v)),
            ((self.bone_index_1).inspect_value(path, offset, limit))
                .map(|v| ("boneIndex[1]".to_string(), v)),
            ((self.bone_index_2).inspect_value(path, offset, limit))
                .map(|v| ("boneIndex[2]".to_string(), v)),
            ((self.bone_index_3).inspect_value(path, offset, limit))
                .map(|v| ("boneIndex[3]".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
