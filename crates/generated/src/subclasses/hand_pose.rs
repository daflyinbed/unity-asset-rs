//! 自动生成。请勿手动修改。

use crate::subclasses::xform::xform;

#[derive(Debug, Clone)]
pub struct HandPose {
    pub m_grab_x: xform,
    pub m_do_f_array: Vec<f32>,
    pub m_override: f32,
    pub m_close_open: f32,
    pub m_in_out: f32,
    pub m_grab: f32,
}

impl crate::Parse for HandPose {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_grab_x = xform::parse(rdr, ver)?;
        let m_do_f_array = ({
            let n = rdr.read_i32()? as usize;
            (0..n)
                .map(|_| rdr.read_f32())
                .collect::<Result<Vec<_>, _>>()
        })?;
        let m_override = rdr.read_f32()?;
        let m_close_open = rdr.read_f32()?;
        let m_in_out = rdr.read_f32()?;
        let m_grab = rdr.read_f32()?;

        Ok(HandPose {
            m_grab_x,
            m_do_f_array,
            m_override,
            m_close_open,
            m_in_out,
            m_grab,
        })
    }
}

impl crate::ToValue for HandPose {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GrabX".to_string(), self.m_grab_x.to_value())),
            Some(("m_DoFArray".to_string(), self.m_do_f_array.to_value())),
            Some(("m_Override".to_string(), self.m_override.to_value())),
            Some(("m_CloseOpen".to_string(), self.m_close_open.to_value())),
            Some(("m_InOut".to_string(), self.m_in_out.to_value())),
            Some(("m_Grab".to_string(), self.m_grab.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GrabX" => (self.m_grab_x).inspect_value(path, offset, limit),
                "m_DoFArray" => (self.m_do_f_array).inspect_value(path, offset, limit),
                "m_Override" => (self.m_override).inspect_value(path, offset, limit),
                "m_CloseOpen" => (self.m_close_open).inspect_value(path, offset, limit),
                "m_InOut" => (self.m_in_out).inspect_value(path, offset, limit),
                "m_Grab" => (self.m_grab).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_grab_x).inspect_value(path, offset, limit))
                .map(|v| ("m_GrabX".to_string(), v)),
            ((self.m_do_f_array).inspect_value(path, offset, limit))
                .map(|v| ("m_DoFArray".to_string(), v)),
            ((self.m_override).inspect_value(path, offset, limit))
                .map(|v| ("m_Override".to_string(), v)),
            ((self.m_close_open).inspect_value(path, offset, limit))
                .map(|v| ("m_CloseOpen".to_string(), v)),
            ((self.m_in_out).inspect_value(path, offset, limit))
                .map(|v| ("m_InOut".to_string(), v)),
            ((self.m_grab).inspect_value(path, offset, limit)).map(|v| ("m_Grab".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
