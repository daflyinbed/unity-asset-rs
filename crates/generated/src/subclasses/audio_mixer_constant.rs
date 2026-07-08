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
        let groups = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { GroupConstant::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let group_gui_ds = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let effects = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { EffectConstant::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let effect_gui_ds = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let num_side_chain_buffers = rdr.read_u32()?;
        let snapshots = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { SnapshotConstant::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let snapshot_gui_ds = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { Guid::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?;
        let group_name_buffer = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let snapshot_name_buffer = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let plugin_effect_name_buffer = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u8() }).collect::<Result<Vec<_>, _>>() } )?;
        rdr.align(4)?;
        let exposed_parameter_names = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let exposed_parameter_indices = ( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { rdr.read_u32() }).collect::<Result<Vec<_>, _>>() } )?;
        let group_connections = if *ver >= crate::UnityVersion::new(2021, 2, 0) {
            Some(( { let n = rdr.read_i32()? as usize; (0..n).map(|_| { GroupConnection::parse(rdr, ver) }).collect::<Result<Vec<_>, _>>() } )?)
        } else {
            None
        };

        Ok(AudioMixerConstant {
            groups,
            group_gui_ds,
            effects,
            effect_gui_ds,
            num_side_chain_buffers,
            snapshots,
            snapshot_gui_ds,
            group_name_buffer,
            snapshot_name_buffer,
            plugin_effect_name_buffer,
            exposed_parameter_names,
            exposed_parameter_indices,
            group_connections,
        })
    }
}
