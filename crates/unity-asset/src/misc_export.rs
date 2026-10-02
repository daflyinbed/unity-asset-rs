//! 杂项资源的原始数据导出（受 `export` feature 门控）。
//!
//! 对齐 AssetRipper 的四个 BinaryAssetExporter：
//! - **TextAsset** → 按内容猜扩展名（json / txt / bytes，`TextAssetExportCollection`）。
//!   `m_script` 可为任意二进制（typed parse 失败回退 TypeTree，原始字节经
//!   `Value::Bytes` 保留，见 `typetree/interpret.rs`）
//! - **Font** → ttf / otf（按字体数据 magic `OTTO`，`FontExtensions.GetFontExtension`）
//! - **VideoClip** → 取 `m_original_path` 扩展名（`VideoClipExtensions.GetExtensionFromPath`），
//!   数据来自 `m_external_resources` 流文件
//! - **MovieTexture** → ogv（内联 `m_movie_data`，AssetRipper 固定用 Theora 扩展名）

use std::error::Error;
use std::fmt;

use generated::{ClassIDType, UnityObject};

use crate::typetree::Value;
use crate::{ClassId, Environment, EnvironmentObject, ParsedObject};

/// 一个可直接写盘的原始导出文件。
#[derive(Debug, Clone)]
pub struct ExportedRaw {
    /// 不带点的文件扩展名。
    pub extension: String,
    pub data: Vec<u8>,
}

/// 原始数据导出失败。
#[derive(Debug)]
pub enum MiscExportError {
    /// 对象不是这四类资源之一。
    UnsupportedClass { class_id: crate::ClassId },
    /// 序列化数据为空或流文件缺失。
    MissingData,
}

impl fmt::Display for MiscExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedClass { class_id } => {
                write!(f, "unsupported class for raw export: {class_id}")
            }
            Self::MissingData => f.write_str("导出数据缺失（内联为空或流文件不可用）"),
        }
    }
}

impl Error for MiscExportError {}

/// 导出 TextAsset / Font / VideoClip / MovieTexture 的原始数据。
pub fn export_raw(
    env: &Environment,
    obj: &EnvironmentObject,
) -> Result<ExportedRaw, MiscExportError> {
    // TextAsset 的 m_script 可能是任意二进制（protobuf / 加密 blob 等）：
    // - 合法 UTF-8 → typed parse 成功，直接取 `m_script` 字节；
    // - 非法 UTF-8 → typed parse 失败回退 TypeTree，interpret 保留原始字节
    //   （`Value::Bytes`），此处同样按原始字节导出。
    // 对齐 AssetRipper `TextAssetExporter.Export`：写盘 `Script_C49.Data`。
    if obj.class_id == ClassId::from(ClassIDType::TextAsset)
        && let Some(script) = text_asset_script(&obj.object)
    {
        let extension = if is_json(&script) {
            "json"
        } else if is_plain_text(&script) {
            "txt"
        } else {
            "bytes"
        };
        return Ok(ExportedRaw {
            extension: extension.to_string(),
            data: script,
        });
    }
    match &obj.object {
        ParsedObject::Typed(UnityObject::Font(f)) if !f.m_font_data.is_empty() => {
            // AssetRipper：首 4 字节为 "OTTO" → otf，否则 ttf（<4 字节兜底 ttf）。
            let extension = if f.m_font_data.starts_with(b"OTTO") {
                "otf"
            } else {
                "ttf"
            };
            Ok(ExportedRaw {
                extension: extension.to_string(),
                data: f.m_font_data.clone(),
            })
        }
        ParsedObject::Typed(UnityObject::VideoClip(v)) => {
            let data = env
                .streamed_resource_data(&v.m_external_resources)
                .ok_or(MiscExportError::MissingData)?;
            if data.is_empty() {
                return Err(MiscExportError::MissingData);
            }
            // AssetRipper GetExtensionFromPath：取 m_original_path 的扩展名，无则 bytes。
            let extension = std::path::Path::new(&v.m_original_path)
                .extension()
                .and_then(|e| e.to_str())
                .map(sanitize_extension)
                .unwrap_or_else(|| "bytes".to_string());
            Ok(ExportedRaw {
                extension,
                data: data.to_vec(),
            })
        }
        ParsedObject::Typed(UnityObject::MovieTexture(m)) => {
            let data = m
                .m_movie_data
                .as_ref()
                .ok_or(MiscExportError::MissingData)?;
            if data.is_empty() {
                return Err(MiscExportError::MissingData);
            }
            Ok(ExportedRaw {
                extension: "ogv".to_string(),
                data: data.clone(),
            })
        }
        ParsedObject::Typed(UnityObject::Font(_)) => Err(MiscExportError::MissingData),
        _ => Err(MiscExportError::UnsupportedClass {
            class_id: obj.class_id,
        }),
    }
}

/// 取 TextAsset 的 `m_Script` 原始字节。
///
/// - `Typed`：m_script 为 String（typed parse 保证合法 UTF-8），取其字节；
/// - `Dynamic`：typed parse 因非法 UTF-8 失败回退 TypeTree 的产物。`m_Script`
///   为 `Value::Bytes`（原始字节）或 `Value::String`（本就是文本）。
///
/// 其他形态（非 struct / 缺字段 / 字段非 bytes-string）返回 `None`，
/// 由 [`export_raw`] 落到 `UnsupportedClass`。
fn text_asset_script(object: &ParsedObject) -> Option<Vec<u8>> {
    match object {
        ParsedObject::Typed(UnityObject::TextAsset(t)) => Some(t.m_script.as_bytes().to_vec()),
        ParsedObject::Dynamic(value) => {
            let Value::Struct(fields) = value else {
                return None;
            };
            match fields.iter().find(|(k, _)| k == "m_Script") {
                Some((_, Value::Bytes(b))) => Some(b.clone()),
                Some((_, Value::String(s))) => Some(s.as_bytes().to_vec()),
                _ => None,
            }
        }
        _ => None,
    }
}

fn sanitize_extension(ext: &str) -> String {
    ext.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .take(16)
        .collect()
}

/// 对齐 AssetRipper `IsPlainText`：全部字符为非控制字符或空白。
fn is_plain_text(bytes: &[u8]) -> bool {
    // TextAsset 语义上按原始字节存储；无效 UTF-8 视为非纯文本。
    let Ok(text) = std::str::from_utf8(bytes) else {
        return false;
    };
    text.chars().all(|c| !c.is_control() || c.is_whitespace())
}

/// 对齐 AssetRipper `IsValidJson`（JsonDocument.Parse）。
fn is_json(bytes: &[u8]) -> bool {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return false;
    };
    let trimmed = text.trim();
    if !(trimmed.starts_with('{') && trimmed.ends_with('}')
        || trimmed.starts_with('[') && trimmed.ends_with(']'))
    {
        return false;
    }
    let mut parser = JsonParser {
        bytes: trimmed.as_bytes(),
        pos: 0,
        depth: 0,
    };
    parser.parse_value() && parser.pos == parser.bytes.len()
}

/// 最小 JSON 语法校验器（无第三方依赖，覆盖 AssetRipper JsonDocument.Parse 的拒绝面）。
struct JsonParser<'a> {
    bytes: &'a [u8],
    pos: usize,
    depth: usize,
}

impl JsonParser<'_> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.pos += 1;
        }
    }

    fn expect(&mut self, b: u8) -> bool {
        self.skip_ws();
        if self.peek() == Some(b) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn parse_value(&mut self) -> bool {
        if self.depth > 128 {
            return false;
        }
        self.skip_ws();
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => self.parse_string(),
            Some(b't') => self.parse_literal(b"true"),
            Some(b'f') => self.parse_literal(b"false"),
            Some(b'n') => self.parse_literal(b"null"),
            Some(c) if c == b'-' || c.is_ascii_digit() => self.parse_number(),
            _ => false,
        }
    }

    fn parse_object(&mut self) -> bool {
        self.depth += 1;
        let ok = (|| {
            if !self.expect(b'{') {
                return false;
            }
            self.skip_ws();
            if self.peek() == Some(b'}') {
                self.pos += 1;
                return true;
            }
            loop {
                if !self.parse_string() || !self.expect(b':') || !self.parse_value() {
                    return false;
                }
                self.skip_ws();
                match self.peek() {
                    Some(b',') => self.pos += 1,
                    Some(b'}') => {
                        self.pos += 1;
                        return true;
                    }
                    _ => return false,
                }
            }
        })();
        self.depth -= 1;
        ok
    }

    fn parse_array(&mut self) -> bool {
        self.depth += 1;
        let ok = (|| {
            if !self.expect(b'[') {
                return false;
            }
            self.skip_ws();
            if self.peek() == Some(b']') {
                self.pos += 1;
                return true;
            }
            loop {
                if !self.parse_value() {
                    return false;
                }
                self.skip_ws();
                match self.peek() {
                    Some(b',') => self.pos += 1,
                    Some(b']') => {
                        self.pos += 1;
                        return true;
                    }
                    _ => return false,
                }
            }
        })();
        self.depth -= 1;
        ok
    }

    fn parse_string(&mut self) -> bool {
        if !self.expect(b'"') {
            return false;
        }
        while let Some(c) = self.peek() {
            match c {
                b'"' => {
                    self.pos += 1;
                    return true;
                }
                b'\\' => {
                    self.pos += 1;
                    if !matches!(
                        self.peek(),
                        Some(b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' | b'u')
                    ) {
                        return false;
                    }
                    if self.peek() == Some(b'u') {
                        // \uXXXX
                        if self.bytes.len() < self.pos + 5 {
                            return false;
                        }
                        let hex = &self.bytes[self.pos + 1..self.pos + 5];
                        if !hex.iter().all(|b| b.is_ascii_hexdigit()) {
                            return false;
                        }
                        self.pos += 4;
                    }
                    self.pos += 1;
                }
                0x00..=0x1F => return false, // 控制字符必须转义
                _ => self.pos += 1,
            }
        }
        false
    }

    fn parse_number(&mut self) -> bool {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        let digits_start = self.pos;
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.pos += 1;
        }
        if self.pos == digits_start {
            return false;
        }
        if self.peek() == Some(b'.') {
            self.pos += 1;
            let frac_start = self.pos;
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.pos += 1;
            }
            if self.pos == frac_start {
                return false;
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            let exp_start = self.pos;
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.pos += 1;
            }
            if self.pos == exp_start {
                return false;
            }
        }
        self.pos > start
    }

    fn parse_literal(&mut self, lit: &[u8]) -> bool {
        if self.bytes.len() >= self.pos + lit.len()
            && &self.bytes[self.pos..self.pos + lit.len()] == lit
        {
            self.pos += lit.len();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_detection_matches_expected_shapes() {
        assert!(is_json(br#" {"a": [1, 2.5e3], "b": "\u00e9"} "#));
        assert!(is_json(b"[null, true, false]"));
        assert!(!is_json(br#"{"a": }"#)); // 缺值
        assert!(!is_json(br#"{"a" "b"}"#)); // 缺冒号
        assert!(!is_json(b"[1, 2")); // 未闭合
        assert!(!is_json(b"truncated")); // 非对象/数组开头
        assert!(!is_json(b"\xff\xfe")); // 非 UTF-8
    }

    #[test]
    fn plain_text_detection() {
        assert!(is_plain_text(b"hello world"));
        assert!(is_plain_text(b"line\nwith\ttabs"));
        assert!(!is_plain_text(b"bin\x00\x01data"));
        assert!(!is_plain_text(b"\xff\xfe"));
    }

    #[test]
    fn video_extension_sanitized() {
        assert_eq!(sanitize_extension("mp4"), "mp4");
        assert_eq!(sanitize_extension(""), "");
    }

    /// Dynamic TextAsset（二进制 m_script 回退 TypeTree 的产物）→ 原始字节 + bytes 扩展名。
    #[test]
    fn dynamic_text_asset_exports_raw_bytes() {
        use generated::classes::text_asset::TextAsset;

        // Dynamic：m_Name 为 String，m_Script 为 Bytes（非法 UTF-8）
        let dynamic = ParsedObject::Dynamic(Value::Struct(vec![
            ("m_Name".to_string(), Value::String("bin".to_string())),
            ("m_Script".to_string(), Value::Bytes(vec![0xff, 0x00, 0xfe])),
        ]));
        assert_eq!(
            text_asset_script(&dynamic).as_deref(),
            Some(&[0xff, 0x00, 0xfe][..])
        );

        // Dynamic 的 m_Script 也可能本来就是文本（其他原因回退）
        let dynamic_text = ParsedObject::Dynamic(Value::Struct(vec![
            ("m_Name".to_string(), Value::String("t".to_string())),
            ("m_Script".to_string(), Value::String("{}".to_string())),
        ]));
        assert_eq!(
            text_asset_script(&dynamic_text).as_deref(),
            Some(&b"{}"[..])
        );

        // Typed：m_script 字节直取
        let typed = ParsedObject::Typed(UnityObject::TextAsset(TextAsset {
            m_name: "t".to_string(),
            m_script: "hello".to_string(),
            m_path_name: None,
        }));
        assert_eq!(text_asset_script(&typed).as_deref(), Some(&b"hello"[..]));

        // 非 TextAsset 结构 / 缺 m_Script → None
        let other = ParsedObject::Dynamic(Value::Struct(vec![(
            "m_Name".to_string(),
            Value::String("x".to_string()),
        )]));
        assert_eq!(text_asset_script(&other), None);
    }
}
