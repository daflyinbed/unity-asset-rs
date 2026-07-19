//! 自动生成。请勿手动修改。

use crate::PPtr;

#[derive(Debug, Clone)]
pub struct TriggerModule {
    pub enabled: bool,
    pub collision_shape0: Option<PPtr>,
    pub inside: i32,
    pub collision_shape1: Option<PPtr>,
    pub outside: i32,
    pub collision_shape2: Option<PPtr>,
    pub enter: i32,
    pub collision_shape3: Option<PPtr>,
    pub exit: i32,
    pub collision_shape4: Option<PPtr>,
    pub collider_query_mode: Option<i32>,
    pub collision_shape5: Option<PPtr>,
    pub radius_scale: f32,
    pub primitives: Option<Vec<PPtr>>,
}

impl crate::Parse for TriggerModule {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut enabled = None;
        let mut collision_shape0 = None;
        let mut inside = None;
        let mut collision_shape1 = None;
        let mut outside = None;
        let mut collision_shape2 = None;
        let mut enter = None;
        let mut collision_shape3 = None;
        let mut exit = None;
        let mut collision_shape4 = None;
        let mut collider_query_mode = None;
        let mut collision_shape5 = None;
        let mut radius_scale = None;
        let mut primitives = None;
        enabled = Some(rdr.read_bool()?);
        rdr.align(4)?;
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
        } else {
            collision_shape0 = Some(crate::PPtr::parse(rdr, ver)?);
            collision_shape1 = Some(crate::PPtr::parse(rdr, ver)?);
            collision_shape2 = Some(crate::PPtr::parse(rdr, ver)?);
            collision_shape3 = Some(crate::PPtr::parse(rdr, ver)?);
            collision_shape4 = Some(crate::PPtr::parse(rdr, ver)?);
            collision_shape5 = Some(crate::PPtr::parse(rdr, ver)?);
        }
        inside = Some(rdr.read_i32()?);
        outside = Some(rdr.read_i32()?);
        enter = Some(rdr.read_i32()?);
        exit = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            collider_query_mode = Some(rdr.read_i32()?);
        } else {
        }
        radius_scale = Some(rdr.read_f32()?);
        if *ver >= crate::UnityVersion::new(2020, 2, 0) {
            primitives = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| crate::PPtr::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
            rdr.align(4)?;
        } else {
        }

        Ok(TriggerModule {
            enabled: enabled.unwrap(),
            collision_shape0,
            inside: inside.unwrap(),
            collision_shape1,
            outside: outside.unwrap(),
            collision_shape2,
            enter: enter.unwrap(),
            collision_shape3,
            exit: exit.unwrap(),
            collision_shape4,
            collider_query_mode,
            collision_shape5,
            radius_scale: radius_scale.unwrap(),
            primitives,
        })
    }
}
