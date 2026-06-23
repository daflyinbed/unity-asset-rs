//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct AnimationEvent {
    pub time: f32,
    pub function_name: String,
    pub data: String,
    pub object_reference_parameter: PPtr,
    pub float_parameter: f32,
    pub int_parameter: i32,
    pub message_options: i32,
}

impl crate::Parse for AnimationEvent {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let time = r.read_f32()?;
        let function_name = r.read_aligned_string()?;
        r.align(4)?;
        let data = r.read_aligned_string()?;
        r.align(4)?;
        let object_reference_parameter = crate::PPtr::parse(r, ver)?;
        let float_parameter = r.read_f32()?;
        let int_parameter = r.read_i32()?;
        let message_options = r.read_i32()?;

        Ok(AnimationEvent {
            time,
            function_name,
            data,
            object_reference_parameter,
            float_parameter,
            int_parameter,
            message_options,
        })
    }
}
