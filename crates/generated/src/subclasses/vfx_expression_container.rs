//! 自动生成。请勿手动修改。

use crate::subclasses::expression::Expression;

#[derive(Debug, Clone)]
pub struct VfxExpressionContainer {
    pub m_expressions: Vec<Expression>,
    pub m_needs_local_to_world: bool,
    pub m_needs_world_to_local: bool,
}

impl crate::Parse for VfxExpressionContainer {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_expressions = ( { let n = r.read_i32()? as usize; (0..n).map(|_| { Expression::parse(r, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        r.align(4)?;
        let m_needs_local_to_world = r.read_bool()?;
        let m_needs_world_to_local = r.read_bool()?;
        r.align(4)?;

        Ok(VfxExpressionContainer {
            m_expressions,
            m_needs_local_to_world,
            m_needs_world_to_local,
        })
    }
}
