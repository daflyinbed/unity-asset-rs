//! 文件名归一化。对齐 AssetRipper `SpecialFileNames.FixFileIdentifier`。
//!
//! collection（SerializedFile）、resource（`.resS`）、`FileIdentifier.path` 三方的
//! `name_fixed` 都用本函数，精确相等比较——不做候选名猜测。

/// 归一化文件标识：lowercase → strip `library/` / `resources/` → strip `archive:/` → basename。
///
/// 对齐 `SpecialFileNames.FixFileIdentifier`：
/// 1. `ToLowerInvariant`
/// 2. `FixDependencyName`：strip `library/` 或 `resources/` 前缀
/// 3. `FixResourcePath`：strip `archive:/` 前缀，取 basename
pub fn fix_file_identifier(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    let stripped = lower
        .strip_prefix("library/")
        .or_else(|| lower.strip_prefix("resources/"))
        .unwrap_or(&lower);
    if let Some(rest) = stripped.strip_prefix("archive:/") {
        basename(rest)
    } else {
        stripped.to_owned()
    }
}

/// 取路径最后一段分量（对齐 `System.IO.Path.GetFileName`，按 `/` 与 `\` 切分）。
fn basename(path: &str) -> String {
    let pos = path.rsplit(['/', '\\']).next();
    pos.map(str::to_owned).unwrap_or_else(|| path.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fix_basic() {
        assert_eq!(fix_file_identifier("CAB-1234.assets"), "cab-1234.assets");
        assert_eq!(
            fix_file_identifier("archive:/CAB-1234/CAB-1234.resS"),
            "cab-1234.ress"
        );
        assert_eq!(fix_file_identifier("library/12345"), "12345");
        assert_eq!(fix_file_identifier("resources/shared.tex"), "shared.tex");
        assert_eq!(fix_file_identifier("archive:/CAB-x/level0"), "level0");
    }

    #[test]
    fn fix_backslash() {
        assert_eq!(
            fix_file_identifier("archive:/CAB-x/sub\\file.resS"),
            "file.ress"
        );
    }
}
