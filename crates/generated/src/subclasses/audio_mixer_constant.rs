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
            groups = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { GroupConstant::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            group_gui_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            effects = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { EffectConstant::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            effect_gui_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            num_side_chain_buffers = Some(rdr.read_u32()?);
            snapshots = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SnapshotConstant::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            snapshot_gui_ds = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
            group_name_buffer = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            snapshot_name_buffer = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            plugin_effect_name_buffer = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?);
            rdr.align(4)?;
            exposed_parameter_names = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
            exposed_parameter_indices = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?);
        if *ver >= crate::UnityVersion::new(2022, 1, 0) {
        } else if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            group_connections = Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { GroupConnection::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?);
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
