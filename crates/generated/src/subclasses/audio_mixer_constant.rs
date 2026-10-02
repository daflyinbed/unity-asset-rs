//! 自动生成。请勿手动修改。

use crate::subclasses::effect_constant::EffectConstant;
use crate::subclasses::group_connection::GroupConnection;
use crate::subclasses::group_constant::GroupConstant;
use crate::subclasses::guid::Guid;
use crate::subclasses::snapshot_constant::SnapshotConstant;

#[derive(Debug, Clone)]
pub struct AudioMixerConstant {
    pub groups: Vec<GroupConstant>,
    pub group_gui_ds: Vec<Guid>,
    pub effects: Vec<EffectConstant>,
    pub effect_gui_ds: Vec<Guid>,
    pub num_side_chain_buffers: u32,
    pub snapshots: Vec<SnapshotConstant>,
    pub snapshot_gui_ds: Vec<Guid>,
    pub group_name_buffer: Vec<u8>,
    pub snapshot_name_buffer: Vec<u8>,
    pub plugin_effect_name_buffer: Vec<u8>,
    pub exposed_parameter_names: Vec<u32>,
    pub exposed_parameter_indices: Vec<u32>,
    pub group_connections: Option<Vec<GroupConnection>>,
}

impl crate::Parse for AudioMixerConstant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut groups = None;
        let mut group_gui_ds = None;
        let mut effects = None;
        let mut effect_gui_ds = None;
        let mut num_side_chain_buffers = None;
        let mut snapshots = None;
        let mut snapshot_gui_ds = None;
        let mut group_name_buffer = None;
        let mut snapshot_name_buffer = None;
        let mut plugin_effect_name_buffer = None;
        let mut exposed_parameter_names = None;
        let mut exposed_parameter_indices = None;
        let mut group_connections = None;
        groups = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| GroupConstant::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        group_gui_ds = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Guid::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        effects = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| EffectConstant::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        effect_gui_ds = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Guid::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        num_side_chain_buffers = Some(rdr.read_u32()?);
        snapshots = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| SnapshotConstant::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        snapshot_gui_ds = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| Guid::parse(rdr, ver))
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        group_name_buffer = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        snapshot_name_buffer = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        plugin_effect_name_buffer = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n).map(|_| rdr.read_u8()).collect::<Result<Vec<_>, _>>()
            })?,
        );
        rdr.align(4)?;
        exposed_parameter_names = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        exposed_parameter_indices = Some(
            ({
                let n = rdr.read_i32()? as usize;
                (0..n)
                    .map(|_| rdr.read_u32())
                    .collect::<Result<Vec<_>, _>>()
            })?,
        );
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            group_connections = Some(
                ({
                    let n = rdr.read_i32()? as usize;
                    (0..n)
                        .map(|_| GroupConnection::parse(rdr, ver))
                        .collect::<Result<Vec<_>, _>>()
                })?,
            );
        } else {
        }

        Ok(AudioMixerConstant {
            groups: groups.unwrap(),
            group_gui_ds: group_gui_ds.unwrap(),
            effects: effects.unwrap(),
            effect_gui_ds: effect_gui_ds.unwrap(),
            num_side_chain_buffers: num_side_chain_buffers.unwrap(),
            snapshots: snapshots.unwrap(),
            snapshot_gui_ds: snapshot_gui_ds.unwrap(),
            group_name_buffer: group_name_buffer.unwrap(),
            snapshot_name_buffer: snapshot_name_buffer.unwrap(),
            plugin_effect_name_buffer: plugin_effect_name_buffer.unwrap(),
            exposed_parameter_names: exposed_parameter_names.unwrap(),
            exposed_parameter_indices: exposed_parameter_indices.unwrap(),
            group_connections,
        })
    }
}

impl crate::ToValue for AudioMixerConstant {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("groups".to_string(), self.groups.to_value())),
            Some(("groupGUIDs".to_string(), self.group_gui_ds.to_value())),
            Some(("effects".to_string(), self.effects.to_value())),
            Some(("effectGUIDs".to_string(), self.effect_gui_ds.to_value())),
            Some((
                "numSideChainBuffers".to_string(),
                self.num_side_chain_buffers.to_value(),
            )),
            Some(("snapshots".to_string(), self.snapshots.to_value())),
            Some(("snapshotGUIDs".to_string(), self.snapshot_gui_ds.to_value())),
            Some((
                "groupNameBuffer".to_string(),
                self.group_name_buffer.to_value(),
            )),
            Some((
                "snapshotNameBuffer".to_string(),
                self.snapshot_name_buffer.to_value(),
            )),
            Some((
                "pluginEffectNameBuffer".to_string(),
                self.plugin_effect_name_buffer.to_value(),
            )),
            Some((
                "exposedParameterNames".to_string(),
                self.exposed_parameter_names.to_value(),
            )),
            Some((
                "exposedParameterIndices".to_string(),
                self.exposed_parameter_indices.to_value(),
            )),
            crate::opt_field("groupConnections", &self.group_connections),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "groups" => (self.groups).inspect_value(path, offset, limit),
                "groupGUIDs" => (self.group_gui_ds).inspect_value(path, offset, limit),
                "effects" => (self.effects).inspect_value(path, offset, limit),
                "effectGUIDs" => (self.effect_gui_ds).inspect_value(path, offset, limit),
                "numSideChainBuffers" => {
                    (self.num_side_chain_buffers).inspect_value(path, offset, limit)
                }
                "snapshots" => (self.snapshots).inspect_value(path, offset, limit),
                "snapshotGUIDs" => (self.snapshot_gui_ds).inspect_value(path, offset, limit),
                "groupNameBuffer" => (self.group_name_buffer).inspect_value(path, offset, limit),
                "snapshotNameBuffer" => {
                    (self.snapshot_name_buffer).inspect_value(path, offset, limit)
                }
                "pluginEffectNameBuffer" => {
                    (self.plugin_effect_name_buffer).inspect_value(path, offset, limit)
                }
                "exposedParameterNames" => {
                    (self.exposed_parameter_names).inspect_value(path, offset, limit)
                }
                "exposedParameterIndices" => {
                    (self.exposed_parameter_indices).inspect_value(path, offset, limit)
                }
                "groupConnections" => (self.group_connections).inspect_value(path, offset, limit),
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.groups).inspect_value(path, offset, limit)).map(|v| ("groups".to_string(), v)),
            ((self.group_gui_ds).inspect_value(path, offset, limit))
                .map(|v| ("groupGUIDs".to_string(), v)),
            ((self.effects).inspect_value(path, offset, limit)).map(|v| ("effects".to_string(), v)),
            ((self.effect_gui_ds).inspect_value(path, offset, limit))
                .map(|v| ("effectGUIDs".to_string(), v)),
            ((self.num_side_chain_buffers).inspect_value(path, offset, limit))
                .map(|v| ("numSideChainBuffers".to_string(), v)),
            ((self.snapshots).inspect_value(path, offset, limit))
                .map(|v| ("snapshots".to_string(), v)),
            ((self.snapshot_gui_ds).inspect_value(path, offset, limit))
                .map(|v| ("snapshotGUIDs".to_string(), v)),
            ((self.group_name_buffer).inspect_value(path, offset, limit))
                .map(|v| ("groupNameBuffer".to_string(), v)),
            ((self.snapshot_name_buffer).inspect_value(path, offset, limit))
                .map(|v| ("snapshotNameBuffer".to_string(), v)),
            ((self.plugin_effect_name_buffer).inspect_value(path, offset, limit))
                .map(|v| ("pluginEffectNameBuffer".to_string(), v)),
            ((self.exposed_parameter_names).inspect_value(path, offset, limit))
                .map(|v| ("exposedParameterNames".to_string(), v)),
            ((self.exposed_parameter_indices).inspect_value(path, offset, limit))
                .map(|v| ("exposedParameterIndices".to_string(), v)),
            ((self.group_connections).inspect_value(path, offset, limit))
                .map(|v| ("groupConnections".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
