//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct InputAxis {
    pub m_name: String,
    pub descriptive_name: String,
    pub descriptive_negative_name: String,
    pub negative_button: String,
    pub positive_button: String,
    pub alt_negative_button: String,
    pub alt_positive_button: String,
    pub gravity: f32,
    pub dead: f32,
    pub sensitivity: f32,
    pub snap: bool,
    pub invert: bool,
    pub r#type: i32,
    pub axis: i32,
    pub joy_num: i32,
}

impl crate::Parse for InputAxis {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_name = rdr.read_aligned_string()?;
        let descriptive_name = rdr.read_aligned_string()?;
        let descriptive_negative_name = rdr.read_aligned_string()?;
        let negative_button = rdr.read_aligned_string()?;
        let positive_button = rdr.read_aligned_string()?;
        let alt_negative_button = rdr.read_aligned_string()?;
        let alt_positive_button = rdr.read_aligned_string()?;
        let gravity = rdr.read_f32()?;
        let dead = rdr.read_f32()?;
        let sensitivity = rdr.read_f32()?;
        let snap = rdr.read_bool()?;
        let invert = rdr.read_bool()?;
        rdr.align(4)?;
        let r#type = rdr.read_i32()?;
        let axis = rdr.read_i32()?;
        let joy_num = rdr.read_i32()?;

        Ok(InputAxis {
            m_name,
            descriptive_name,
            descriptive_negative_name,
            negative_button,
            positive_button,
            alt_negative_button,
            alt_positive_button,
            gravity,
            dead,
            sensitivity,
            snap,
            invert,
            r#type,
            axis,
            joy_num,
        })
    }
}
