//! 自动生成。请勿手动修改。

use crate::PPtr;
use crate::subclasses::color_rgba32::ColorRgba32;
use crate::subclasses::vector3f::Vector3f;

#[derive(Debug, Clone)]
pub struct ParticleAnimator {
    pub m_game_object: PPtr,
    pub does_animate_color: bool,
    pub color_animation_0: ColorRgba32,
    pub color_animation_1: ColorRgba32,
    pub color_animation_2: ColorRgba32,
    pub color_animation_3: ColorRgba32,
    pub color_animation_4: ColorRgba32,
    pub world_rotation_axis: Vector3f,
    pub local_rotation_axis: Vector3f,
    pub size_grow: f32,
    pub rnd_force: Vector3f,
    pub force: Vector3f,
    pub damping: f32,
    pub stop_simulation: bool,
    pub autodestruct: bool,
}

impl crate::Parse for ParticleAnimator {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_game_object = crate::PPtr::parse(rdr, ver)?;
        let does_animate_color = rdr.read_bool()?;
        rdr.align(4)?;
        let color_animation_0 = ColorRgba32::parse(rdr, ver)?;
        let color_animation_1 = ColorRgba32::parse(rdr, ver)?;
        let color_animation_2 = ColorRgba32::parse(rdr, ver)?;
        let color_animation_3 = ColorRgba32::parse(rdr, ver)?;
        let color_animation_4 = ColorRgba32::parse(rdr, ver)?;
        let world_rotation_axis = Vector3f::parse(rdr, ver)?;
        let local_rotation_axis = Vector3f::parse(rdr, ver)?;
        let size_grow = rdr.read_f32()?;
        let rnd_force = Vector3f::parse(rdr, ver)?;
        let force = Vector3f::parse(rdr, ver)?;
        let damping = rdr.read_f32()?;
        let stop_simulation = rdr.read_bool()?;
        let autodestruct = rdr.read_bool()?;

        Ok(ParticleAnimator {
            m_game_object,
            does_animate_color,
            color_animation_0,
            color_animation_1,
            color_animation_2,
            color_animation_3,
            color_animation_4,
            world_rotation_axis,
            local_rotation_axis,
            size_grow,
            rnd_force,
            force,
            damping,
            stop_simulation,
            autodestruct,
        })
    }
}

impl crate::ToValue for ParticleAnimator {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("m_GameObject".to_string(), self.m_game_object.to_value())),
            Some((
                "Does Animate Color?".to_string(),
                self.does_animate_color.to_value(),
            )),
            Some((
                "colorAnimation[0]".to_string(),
                self.color_animation_0.to_value(),
            )),
            Some((
                "colorAnimation[1]".to_string(),
                self.color_animation_1.to_value(),
            )),
            Some((
                "colorAnimation[2]".to_string(),
                self.color_animation_2.to_value(),
            )),
            Some((
                "colorAnimation[3]".to_string(),
                self.color_animation_3.to_value(),
            )),
            Some((
                "colorAnimation[4]".to_string(),
                self.color_animation_4.to_value(),
            )),
            Some((
                "worldRotationAxis".to_string(),
                self.world_rotation_axis.to_value(),
            )),
            Some((
                "localRotationAxis".to_string(),
                self.local_rotation_axis.to_value(),
            )),
            Some(("sizeGrow".to_string(), self.size_grow.to_value())),
            Some(("rndForce".to_string(), self.rnd_force.to_value())),
            Some(("force".to_string(), self.force.to_value())),
            Some(("damping".to_string(), self.damping.to_value())),
            Some((
                "stopSimulation".to_string(),
                self.stop_simulation.to_value(),
            )),
            Some(("autodestruct".to_string(), self.autodestruct.to_value())),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "m_GameObject" => (self.m_game_object).inspect_value(path, offset, limit),
                "Does Animate Color?" => {
                    (self.does_animate_color).inspect_value(path, offset, limit)
                }
                "colorAnimation[0]" => (self.color_animation_0).inspect_value(path, offset, limit),
                "colorAnimation[1]" => (self.color_animation_1).inspect_value(path, offset, limit),
                "colorAnimation[2]" => (self.color_animation_2).inspect_value(path, offset, limit),
                "colorAnimation[3]" => (self.color_animation_3).inspect_value(path, offset, limit),
                "colorAnimation[4]" => (self.color_animation_4).inspect_value(path, offset, limit),
                "worldRotationAxis" => {
                    (self.world_rotation_axis).inspect_value(path, offset, limit)
                }
                "localRotationAxis" => {
                    (self.local_rotation_axis).inspect_value(path, offset, limit)
                }
                "sizeGrow" => (self.size_grow).inspect_value(path, offset, limit),
                "rndForce" => (self.rnd_force).inspect_value(path, offset, limit),
                "force" => (self.force).inspect_value(path, offset, limit),
                "damping" => (self.damping).inspect_value(path, offset, limit),
                "stopSimulation" => (self.stop_simulation).inspect_value(path, offset, limit),
                "autodestruct" => (self.autodestruct).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.m_game_object).inspect_value(path, offset, limit))
                .map(|v| ("m_GameObject".to_string(), v)),
            ((self.does_animate_color).inspect_value(path, offset, limit))
                .map(|v| ("Does Animate Color?".to_string(), v)),
            ((self.color_animation_0).inspect_value(path, offset, limit))
                .map(|v| ("colorAnimation[0]".to_string(), v)),
            ((self.color_animation_1).inspect_value(path, offset, limit))
                .map(|v| ("colorAnimation[1]".to_string(), v)),
            ((self.color_animation_2).inspect_value(path, offset, limit))
                .map(|v| ("colorAnimation[2]".to_string(), v)),
            ((self.color_animation_3).inspect_value(path, offset, limit))
                .map(|v| ("colorAnimation[3]".to_string(), v)),
            ((self.color_animation_4).inspect_value(path, offset, limit))
                .map(|v| ("colorAnimation[4]".to_string(), v)),
            ((self.world_rotation_axis).inspect_value(path, offset, limit))
                .map(|v| ("worldRotationAxis".to_string(), v)),
            ((self.local_rotation_axis).inspect_value(path, offset, limit))
                .map(|v| ("localRotationAxis".to_string(), v)),
            ((self.size_grow).inspect_value(path, offset, limit))
                .map(|v| ("sizeGrow".to_string(), v)),
            ((self.rnd_force).inspect_value(path, offset, limit))
                .map(|v| ("rndForce".to_string(), v)),
            ((self.force).inspect_value(path, offset, limit)).map(|v| ("force".to_string(), v)),
            ((self.damping).inspect_value(path, offset, limit)).map(|v| ("damping".to_string(), v)),
            ((self.stop_simulation).inspect_value(path, offset, limit))
                .map(|v| ("stopSimulation".to_string(), v)),
            ((self.autodestruct).inspect_value(path, offset, limit))
                .map(|v| ("autodestruct".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
