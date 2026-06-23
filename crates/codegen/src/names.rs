//! 命名规范（详见 `docs/codegen-design.md` §2）。
//!
//! - 字段名：保留 `m_`/`k_` 前缀，snake_case 化；关键字冲突用 `r#` 转义。
//! - 类型名：PascalCase + 缩写规则化（`AABB → Aabb`、`ColorRGBA → ColorRgba`），`PPtr<X>` 例外保留。

/// Rust 保留字 + 弱保留字（不可作为裸标识符使用）。
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn",
    // 弱保留（rustc 警告）
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", "try",
];

/// Unity 字段名 → Rust 字段名。
///
/// 保留 `m_`/`k_` 前缀，snake_case 化，关键字用 `r#` 转义。
/// 首字符是数字时加 `f_` 前缀（Rust 标识符不能以数字开头）。
pub fn to_rust_field_name(unity_name: &str) -> String {
    let mut s = to_snake_case(unity_name);
    // 首字符是数字 → 加前缀（Rust 标识符不能以数字开头）
    if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        s = format!("f_{s}");
    }
    if RUST_KEYWORDS.contains(&s.as_str()) {
        format!("r#{s}")
    } else {
        s
    }
}

/// Unity 类型名 → Rust 类型名。
///
/// PascalCase + 缩写规则化（连续 ≥2 个大写字母 → 首大写其余小写）。
/// `PPtr<X>` 原样保留（含尖括号）。
/// Rust 原始类型/关键字作 class 名时加 `_` 后缀（如 `float → float_`、`void → void_`），
/// 因为 struct 名不支持 `r#` 转义。
pub fn to_rust_type_name(unity_name: &str) -> String {
    // PPtr<X> 原样保留（generate 时 PPtr 例外）
    if unity_name.starts_with("PPtr<") {
        return unity_name.to_string();
    }
    let collapsed = collapse_acronyms(unity_name);
    // Rust 原始类型 / 关键字：struct 名不能用 r#，加 _ 后缀
    if RUST_PRIMITIVE_TYPES_OR_KEYWORDS.contains(&collapsed.as_str()) {
        format!("{collapsed}_")
    } else {
        collapsed
    }
}

/// Rust 原始类型 + 关键字（不能作为 type 名）。
/// 注意：`Self`、`macro` 等也包含。
const RUST_PRIMITIVE_TYPES_OR_KEYWORDS: &[&str] = &[
    "bool", "char", "i8", "i16", "i32", "i64", "i128", "isize",
    "u8", "u16", "u32", "u64", "u128", "usize",
    "f32", "f64",
    "str", "String", "Vec", "Option", "Result",
    "float", "double", "int", "void",  // Unity 用了这些
    "Self", "self", "crate", "super", "mod",
    "type", "fn", "struct", "enum", "trait", "impl",
    "match", "if", "else", "for", "while", "loop",
    "return", "break", "continue",
    "let", "const", "static", "mut", "ref",
    "move", "as", "in", "use", "pub", "where",
    "async", "await", "dyn", "extern",
    "true", "false",
];

/// snake_case 化。保留前缀（`m_Foo` → `m_foo`，不是 `foo`）。
///
/// 借助 `heck::ToSnakeCase` 处理边界 case（数字、连续大写、非标识符字符等）。
fn to_snake_case(s: &str) -> String {
    use heck::ToSnakeCase;
    s.to_snake_case()
}

/// PascalCase 缩写规则化：连续 ≥2 个大写字母 → 首大写其余小写。
///
/// 例：`AABB → Aabb`、`ColorRGBA → ColorRgba`、`GUID → Guid`、`Texture2D → Texture2D`（数字保留）。
fn collapse_acronyms(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_uppercase() {
            // 找连续大写段 [i..j)
            let mut j = i;
            while j < chars.len() && chars[j].is_uppercase() {
                j += 1;
            }
            if j - i == 1 {
                // 单个大写：原样
                out.push(chars[i]);
                i = j;
            } else {
                // 多个大写：检查末尾是否"贴着小写"（最后一个大写属于下一个 word）
                let next_is_lower = j < chars.len() && chars[j].is_lowercase();
                let acronym_end = if next_is_lower { j - 1 } else { j };
                // 首字母大写，其余小写
                out.push(chars[i]);
                for &c in &chars[i + 1..acronym_end] {
                    out.push(c.to_ascii_lowercase());
                }
                i = if next_is_lower { j - 1 } else { j };
            }
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_names_basic() {
        assert_eq!(to_rust_field_name("m_Width"), "m_width");
        assert_eq!(to_rust_field_name("m_TextureFormat"), "m_texture_format");
        assert_eq!(to_rust_field_name("m_GameObject"), "m_game_object");
        assert_eq!(to_rust_field_name("image_data"), "image_data");
    }

    #[test]
    fn field_names_non_identifier_chars() {
        // Unity TypeTree 里偶尔出现带空格/点的字段名
        assert_eq!(to_rust_field_name("image data"), "image_data");
        assert_eq!(to_rust_field_name("Type vs."), "type_vs");
    }

    #[test]
    fn field_names_acronyms() {
        // heck 处理连续大写
        assert_eq!(to_rust_field_name("m_RGBColor"), "m_rgb_color");
        assert_eq!(to_rust_field_name("RGB"), "rgb");
    }

    #[test]
    fn field_name_keywords() {
        assert_eq!(to_rust_field_name("type"), "r#type");
        assert_eq!(to_rust_field_name("match"), "r#match");
        assert_eq!(to_rust_field_name("self"), "r#self");
        assert_eq!(to_rust_field_name("m_Type"), "m_type"); // m_ 前缀后不冲突
    }

    #[test]
    fn type_names_basic() {
        assert_eq!(to_rust_type_name("Texture2D"), "Texture2D");
        assert_eq!(to_rust_type_name("Vector3f"), "Vector3f");
        assert_eq!(to_rust_type_name("StreamingInfo"), "StreamingInfo");
    }

    #[test]
    fn type_names_acronyms() {
        assert_eq!(to_rust_type_name("ColorRGBA"), "ColorRgba");
        assert_eq!(to_rust_type_name("AABB"), "Aabb");
        assert_eq!(to_rust_type_name("GUID"), "Guid");
        assert_eq!(to_rust_type_name("JSONParser"), "JsonParser");
        assert_eq!(to_rust_type_name("RGBColor"), "RgbColor");
    }

    #[test]
    fn type_names_pptr_preserved() {
        assert_eq!(to_rust_type_name("PPtr<Component>"), "PPtr<Component>");
        assert_eq!(to_rust_type_name("PPtr<$GameObject>"), "PPtr<$GameObject>");
    }
}
