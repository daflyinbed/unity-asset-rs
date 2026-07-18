//! 读 AB 内嵌 TypeTree 的 blob 格式（format version >= 12 或 == 10），
//! 并把扁平节点列表按 `level` 还原成嵌套 `TypeNode` 树。

use crate::cursor::Cursor;
use crate::error::{EmptyTypeTreeSnafu, Result, TypeTreeSnafu};
use crate::typetree::common_string;
use crate::typetree::{TypeKind, TypeNode};

/// AB TypeTree 的原始扁平节点（按磁盘定长记录读出）。
#[derive(Debug, Clone)]
pub struct BlobNode {
    pub version: i16,
    pub level: u8,
    pub type_flags: u8,
    pub type_str_offset: u32,
    pub name_str_offset: u32,
    pub byte_size: i32,
    pub index: i32,
    pub meta_flag: u32,
    pub ref_type_hash: Option<u64>,
}

/// 读 blob 格式 TypeTree。返回 (扁平节点列表, 字符串缓冲)。
///
/// 单节点磁盘布局 `{endian}hBBIIiii[Q]`（version>=19 加 u64 ref_type_hash），
/// 定长 24（或 32）字节。对应 UnityPy `_get_blob_node_struct`。
pub fn parse_blob(r: &mut Cursor, file_ver: u32) -> Result<(Vec<BlobNode>, Vec<u8>)> {
    let node_count = r.read_i32("typetree node count")? as usize;
    let str_buf_size = r.read_i32("typetree string buffer size")? as usize;
    let has_ref_hash = file_ver >= 19;
    let node_struct_size = if has_ref_hash { 32 } else { 24 };

    let nodes_bytes = r.read_bytes(node_count * node_struct_size, "typetree nodes")?;
    let str_buf = r.read_bytes(str_buf_size, "typetree string buffer")?.to_vec();

    let mut nr = Cursor::new(nodes_bytes, r.endian());
    let mut nodes = Vec::with_capacity(node_count);
    for _ in 0..node_count {
        let version = nr.read_i16("node version")?;
        let level = nr.read_u8()?;
        let type_flags = nr.read_u8()?;
        let type_str_offset = nr.read_u32("type str offset")?;
        let name_str_offset = nr.read_u32("name str offset")?;
        let byte_size = nr.read_i32("byte size")?;
        let index = nr.read_i32("index")?;
        let meta_flag = nr.read_u32("meta flag")?;
        let ref_type_hash = if has_ref_hash {
            Some(nr.read_u64("ref type hash")?)
        } else {
            None
        };
        nodes.push(BlobNode {
            version,
            level,
            type_flags,
            type_str_offset,
            name_str_offset,
            byte_size,
            index,
            meta_flag,
            ref_type_hash,
        });
    }
    Ok((nodes, str_buf))
}

/// 把扁平节点列表按 `level` 还原成 `TypeNode` 嵌套树（递归，深度 = typetree 深度）。
pub fn build_tree(flat: &[BlobNode], str_buf: &[u8]) -> Result<TypeNode> {
    if flat.is_empty() {
        return Err(EmptyTypeTreeSnafu.build());
    }
    let mut idx = 0usize;
    build_subtree(flat, &mut idx, 0, str_buf)
}

fn build_subtree(
    flat: &[BlobNode],
    idx: &mut usize,
    expected_level: u8,
    str_buf: &[u8],
) -> Result<TypeNode> {
    if *idx >= flat.len() {
        return Err(TypeTreeSnafu {
            reason: "build_tree: 节点耗尽".to_string(),
        }
        .build());
    }
    let raw = &flat[*idx];
    if raw.level != expected_level {
        return Err(TypeTreeSnafu {
            reason: format!(
                "build_tree: level 不匹配（期望 {expected_level}，实际 {}）",
                raw.level
            ),
        }
        .build());
    }
    *idx += 1;

    let mut node = TypeNode {
        kind: TypeKind::from_unity(&resolve_str(raw.type_str_offset, str_buf)),
        name: resolve_str(raw.name_str_offset, str_buf),
        byte_size: raw.byte_size,
        version: raw.version,
        meta_flag: raw.meta_flag,
        children: Vec::new(),
    };

    while *idx < flat.len() && flat[*idx].level > expected_level {
        node.children
            .push(build_subtree(flat, idx, expected_level + 1, str_buf)?);
    }
    Ok(node)
}

/// 解析 AB 字符串偏移：bit31=0 本地 buffer null-terminated；bit31=1 CommonString 索引。
fn resolve_str(offset: u32, str_buf: &[u8]) -> String {
    if offset & 0x8000_0000 == 0 {
        let start = offset as usize;
        if start >= str_buf.len() {
            return String::new();
        }
        let end = str_buf[start..]
            .iter()
            .position(|&b| b == 0)
            .map(|p| start + p)
            .unwrap_or(str_buf.len());
        String::from_utf8_lossy(&str_buf[start..end]).into_owned()
    } else {
        common_string::get(offset & 0x7FFF_FFFF).to_owned()
    }
}
