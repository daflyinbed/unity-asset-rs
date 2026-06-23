//! 自动生成。请勿手动修改。

use crate::subclasses::message::Message;

#[derive(Debug, Clone)]
pub struct ErrorLog {
    pub m_messages: Vec<Message>,
    pub m_has_errors: bool,
}

impl crate::Parse for ErrorLog {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_messages = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Message::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_has_errors = r.read_bool()?;

        Ok(ErrorLog {
            m_messages,
            m_has_errors,
        })
    }
}
