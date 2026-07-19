//! SerializedFile 解析：从 `BundleFile.bytes` 读出 header / types（含内嵌 typetree）/
//! objects，并按内嵌 typetree 把对象解码成 `Value`。
//!
//! 二进制布局见 `docs/object-layer.md` §3。

use bytes::Bytes;

use crate::UnityVersion;
use crate::cursor::{Cursor, Endian};
use crate::error::{Result, SerializedFileSnafu, TruncatedSnafu};
use crate::typetree::{self, RefTypeTable, TypeNode};

/// 解析后的 SerializedFile。
pub struct SerializedFile {
    /// format version（header.version，现代 17-22）。
    pub version: u32,
    pub unity_version: String,
    pub unity_version_parsed: UnityVersion,
    pub endian: Endian,
    pub types: Vec<SerializedType>,
    /// `[SerializeReference]` 引用类型（version >= 20）。
    pub ref_types: Vec<SerializedType>,
    /// 外部文件引用（PPtr `file_id > 0` 索引本列表）。
    pub externals: Vec<FileIdentifier>,
    pub objects: Vec<ObjectInfo>,
    data: Bytes,
}

/// 一个 SerializedType，可能带内嵌 typetree 根节点。
pub struct SerializedType {
    pub class_id: i32,
    pub script_type_index: Option<i16>,
    pub node: Option<TypeNode>,
    pub ref_class_name: Option<String>,
    pub ref_namespace: Option<String>,
    pub ref_assembly: Option<String>,
}

/// 对象定位信息。
pub struct ObjectInfo {
    pub path_id: i64,
    /// 已含 data_offset。
    pub byte_start: u64,
    pub byte_size: u32,
    pub type_id: i32,
    /// 指向 `SerializedFile.types` 的索引。
    pub type_idx: usize,
}

/// 外部文件引用。对齐 AssetRipper `FileIdentifier`。
///
/// 一个 SerializedFile 可通过 externals 链接到其它 SerializedFile，形成共享依赖。
/// PPtr 的 `file_id > 0` 索引到本列表（第 `i` 项对应 `file_id == i + 1`）。
#[derive(Debug, Clone)]
pub struct FileIdentifier {
    /// 原始路径（磁盘相对路径，如 `archive:/CAB-xxxx/CAB-xxxx.assets`）。
    pub path_name_origin: String,
    /// `fix_file_identifier(path_name_origin)`——与 collection / resource 的
    /// `name_fixed` 精确匹配。
    pub path_name: String,
    /// 16 字节 GUID。
    pub guid: [u8; 16],
    /// AssetType：`0` = Normal（按 path_name 匹配），`1` = Meta（按 Guid 匹配）。
    pub type_: i32,
}

impl ObjectInfo {
    pub fn class_id(&self, sf: &SerializedFile) -> i32 {
        sf.types.get(self.type_idx).map(|t| t.class_id).unwrap_or(0)
    }

    /// 对象的原始字节切片。`byte_start + byte_size` 越界视为截断错误而非 panic。
    pub fn data<'a>(&self, sf: &'a SerializedFile) -> Result<&'a [u8]> {
        let data = sf.data.as_ref();
        let s = self.byte_start as usize;
        let e = self.byte_size as usize;
        let end = s
            .checked_add(e)
            .filter(|&end| end <= data.len() && s <= end)
            .ok_or_else(|| {
                TruncatedSnafu {
                    reason: format!(
                        "object {} bytes [{s}..{}] 越界（buf len {}）",
                        self.path_id,
                        s.saturating_add(e),
                        data.len()
                    ),
                }
                .build()
            })?;
        Ok(&data[s..end])
    }

    /// 解析该对象为 [`ParsedObject`]。自动从 `sf` 取版本/endian/RefTypeTable,
    /// 内部 dispatch(typed parse → TypeTree fallback / MonoBehaviour 分流)。
    ///
    /// 这是常规用户的推荐入口——不需要手动构造 `RefTypeTable`。需要复用 refs
    /// 的高级场景仍可直接调自由函数 [`crate::object_parser::parse_object`]。
    pub fn parse(&self, sf: &SerializedFile) -> Result<crate::object_parser::ParsedObject> {
        let refs = sf.ref_type_table();
        crate::object_parser::parse_object(sf, self, &refs)
    }
}

impl SerializedFile {
    /// 由 `ref_types` 构建引用类型查表。
    pub fn ref_type_table(&self) -> RefTypeTable {
        let mut t = RefTypeTable::empty();
        for rt in &self.ref_types {
            if let (Some(c), Some(n), Some(a), Some(node)) = (
                &rt.ref_class_name,
                &rt.ref_namespace,
                &rt.ref_assembly,
                &rt.node,
            ) {
                t.push(c.clone(), n.clone(), a.clone(), node.clone());
            }
        }
        t
    }
}

impl SerializedFile {
    /// 解析一个 SerializedFile。
    pub fn parse(bytes: Bytes) -> Result<SerializedFile> {
        let buf: &[u8] = bytes.as_ref();
        // prologue 是大端。
        let mut r = Cursor::new(buf, Endian::Big);

        let metadata_size = r.read_u32("metadata size")? as u64;
        let file_size = r.read_u32("file size")? as u64;
        let version = r.read_u32("format version")?;
        let mut data_offset = r.read_u32("data offset")? as u64;

        let endian = if version >= 9 {
            let e = r.read_u8()?;
            r.read_bytes(3, "reserved")?;
            if version >= 22 {
                // 大文件格式：metadata_size / file_size 此处不使用，消费即可
                let _ = r.read_u32("metadata size (large)")?;
                let _ = r.read_i64("file size (large)")?;
                data_offset = r.read_i64("data offset (large)")? as u64;
                let _unknown = r.read_i64("unknown")?;
            }
            if e == 0 { Endian::Little } else { Endian::Big }
        } else {
            // version < 9：endian 字节在文件末尾（file_size - metadata_size 处）
            let pos = file_size.saturating_sub(metadata_size) as usize;
            r.seek(pos)?;
            let e = r.read_u8()?;
            if e == 0 { Endian::Little } else { Endian::Big }
        };
        r.set_endian(endian);

        let unity_version = if version >= 7 {
            r.read_cstr("unity version")?
        } else {
            String::new()
        };
        if version >= 8 {
            let _target_platform = r.read_i32("target platform")?;
        }
        let enable_type_tree = if version >= 13 { r.read_bool()? } else { true };

        // ReadTypes
        let type_count = r.read_i32("type count")? as usize;
        let mut types = Vec::with_capacity(type_count);
        for _ in 0..type_count {
            types.push(read_serialized_type(
                &mut r,
                version,
                enable_type_tree,
                false,
            )?);
        }

        // big_id_enabled（7 <= version < 14）
        let big_id_enabled = if (7..14).contains(&version) {
            r.read_i32("big id enabled")? != 0
        } else {
            false
        };

        // ReadObjects
        let object_count = r.read_i32("object count")? as usize;
        let mut objects = Vec::with_capacity(object_count);
        for _ in 0..object_count {
            objects.push(read_object_info(
                &mut r,
                version,
                big_id_enabled,
                data_offset,
                &types,
            )?);
        }

        // ReadScripts（version >= 11）
        if version >= 11 {
            let script_count = r.read_i32("script count")? as usize;
            for _ in 0..script_count {
                let _file_idx = r.read_i32("script file idx")?;
                if version < 14 {
                    let _ = r.read_i32("script local id")?;
                } else {
                    r.align(4)?;
                    let _ = r.read_i64("script local id")?;
                }
            }
        }

        // ReadExternals
        let external_count = r.read_i32("external count")? as usize;
        let mut externals = Vec::with_capacity(external_count);
        for _ in 0..external_count {
            if version >= 6 {
                let _ = r.read_cstr("external temp")?;
            }
            let mut guid = [0u8; 16];
            let mut type_ = 0i32;
            if version >= 5 {
                let guid_bytes = r.read_bytes(16, "external guid")?;
                guid.copy_from_slice(guid_bytes);
                type_ = r.read_i32("external type")?;
            }
            let path_name_origin = r.read_cstr("external path")?;
            let path_name = crate::names::fix_file_identifier(&path_name_origin);
            externals.push(FileIdentifier {
                path_name_origin,
                path_name,
                guid,
                type_,
            });
        }

        // ReadRefTypes（version >= 20）
        let mut ref_types = Vec::new();
        if version >= 20 {
            let ref_type_count = r.read_i32("ref type count")? as usize;
            for _ in 0..ref_type_count {
                ref_types.push(read_serialized_type(
                    &mut r,
                    version,
                    enable_type_tree,
                    true,
                )?);
            }
        }

        let unity_version_parsed = UnityVersion::parse(&unity_version);

        Ok(SerializedFile {
            version,
            unity_version,
            unity_version_parsed,
            endian,
            types,
            ref_types,
            externals,
            objects,
            data: bytes,
        })
    }
}

fn read_serialized_type(
    r: &mut Cursor,
    version: u32,
    enable_type_tree: bool,
    is_ref_type: bool,
) -> Result<SerializedType> {
    let class_id = r.read_i32("class id")?;

    if version >= 16 {
        let _is_stripped = r.read_bool()?;
    }
    let script_type_index = if version >= 17 {
        Some(r.read_i16("script type index")?)
    } else {
        None
    };

    if version >= 13 {
        let need_script_id = (is_ref_type && script_type_index.unwrap_or(0) >= 0)
            || (version < 16 && class_id < 0)
            || (version >= 16 && class_id == 114);
        if need_script_id {
            let _ = r.read_bytes(16, "script id")?;
        }
        let _ = r.read_bytes(16, "old type hash")?;
    }

    let mut node = None;
    if enable_type_tree {
        if version >= 12 || version == 10 {
            let (flat, str_buf) = typetree::parse_blob(r, version)?;
            node = Some(typetree::build_tree(&flat, &str_buf)?);
        } else {
            return Err(SerializedFileSnafu {
                reason: "legacy typetree 格式（version < 10）未实现".to_string(),
            }
            .build());
        }
    }

    let mut ref_class_name = None;
    let mut ref_namespace = None;
    let mut ref_assembly = None;
    if version >= 21 {
        if is_ref_type {
            ref_class_name = Some(r.read_cstr("ref class name")?);
            ref_namespace = Some(r.read_cstr("ref namespace")?);
            ref_assembly = Some(r.read_cstr("ref assembly")?);
        } else {
            let dep_count = r.read_i32("type dep count")? as usize;
            for _ in 0..dep_count {
                let _ = r.read_i32("type dep")?;
            }
        }
    }

    Ok(SerializedType {
        class_id,
        script_type_index,
        node,
        ref_class_name,
        ref_namespace,
        ref_assembly,
    })
}

fn read_object_info(
    r: &mut Cursor,
    version: u32,
    big_id: bool,
    data_offset: u64,
    types: &[SerializedType],
) -> Result<ObjectInfo> {
    // path_id
    let path_id = if big_id {
        r.read_i64("path id")?
    } else if version < 14 {
        r.read_i32("path id")? as i64
    } else {
        r.align(4)?;
        r.read_i64("path id")?
    };

    // byte_start（未含 data_offset）
    let byte_start_raw = if version >= 22 {
        r.read_i64("byte start")? as u64
    } else {
        r.read_u32("byte start")? as u64
    };
    let byte_size = r.read_u32("byte size")?;
    let type_id = r.read_i32("type id")?;

    let type_idx = if version < 16 {
        let _class_id = r.read_u16("class id")? as i32;
        types
            .iter()
            .position(|t| t.class_id == type_id)
            .unwrap_or(0)
    } else {
        type_id as usize
    };

    if version < 11 {
        let _ = r.read_u16("is destroyed")?;
    }
    if (11..17).contains(&version) {
        let _ = r.read_i16("script type index")?;
    }
    if version == 15 || version == 16 {
        let _ = r.read_u8()?;
    }

    Ok(ObjectInfo {
        path_id,
        byte_start: byte_start_raw + data_offset,
        byte_size,
        type_id,
        type_idx,
    })
}
