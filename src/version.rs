/// Unity 引擎版本（从 SerializedFile 的 unity_version 字符串解析）。
///
/// 仅用于版本门控判断（如 PPtr 的 path_id 在 ver>=5 时是 i64）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnityVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl UnityVersion {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// 解析 "2017.4.39f1" 之类；无法解析的部分归 0。
    pub fn parse(s: &str) -> Self {
        let mut parts = s.split('.');
        let major = parts.next().and_then(parse_digits).unwrap_or(0);
        let minor = parts.next().and_then(parse_digits).unwrap_or(0);
        let patch = parts.next().and_then(parse_digits).unwrap_or(0);
        Self {
            major,
            minor,
            patch,
        }
    }
}

fn parse_digits(s: &str) -> Option<u16> {
    let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

impl PartialOrd for UnityVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UnityVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version() {
        let v = UnityVersion::parse("2017.4.39f1");
        assert_eq!(v, UnityVersion::new(2017, 4, 39));
        assert!(v >= UnityVersion::new(5, 0, 0));
        assert!(v < UnityVersion::new(2018, 0, 0));
        assert_eq!(UnityVersion::parse("0.0.0"), UnityVersion::new(0, 0, 0));
    }
}
