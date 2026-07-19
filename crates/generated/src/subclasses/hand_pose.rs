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
