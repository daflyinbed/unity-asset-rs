//! 自动生成的 Unity 类型与 parser。请勿手动修改。
//!
//! 由 `crates/codegen` 生成（详见 `docs/codegen-design.md`）。

pub mod classes;
pub mod subclasses;
mod unity_object;
mod class_id;

pub use class_id::ClassIDType;
pub use unity_object::UnityObject;

/// 解析 trait。每个生成的 struct 实现它。
pub trait Parse: Sized {
    fn parse<R: EndianReader>(r: &mut R, ver: &UnityVersion) -> Result<Self, R::Error>;
}

/// 字节序 reader 抽象。
/// 实际实现由 `crates/unity-asset` 提供（为现有 Cursor impl）。
pub trait EndianReader {
    type Error;
    fn read_bool(&mut self) -> Result<bool, Self::Error>;
    fn read_i8(&mut self) -> Result<i8, Self::Error>;
    fn read_u8(&mut self) -> Result<u8, Self::Error>;
    fn read_i16(&mut self) -> Result<i16, Self::Error>;
    fn read_u16(&mut self) -> Result<u16, Self::Error>;
    fn read_i32(&mut self) -> Result<i32, Self::Error>;
    fn read_u32(&mut self) -> Result<u32, Self::Error>;
    fn read_i64(&mut self) -> Result<i64, Self::Error>;
    fn read_u64(&mut self) -> Result<u64, Self::Error>;
    fn read_f32(&mut self) -> Result<f32, Self::Error>;
    fn read_f64(&mut self) -> Result<f64, Self::Error>;
    fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>, Self::Error>;
    fn read_aligned_string(&mut self) -> Result<String, Self::Error>;
    fn align(&mut self, n: usize) -> Result<(), Self::Error>;
}

/// Unity 版本号（生成代码里用到）。
/// 实际定义在 `crates/unity-asset-primitives`，但为保持 generated crate 自包含，
/// 这里复制定义。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnityVersion {
    pub major: u16,
    pub minor: u16,
    pub build: u16,
}

impl UnityVersion {
    pub const fn new(major: u16, minor: u16, build: u16) -> Self {
        Self {
            major,
            minor,
            build,
        }
    }
}

/// PPtr<X> —— 跨引用指针。所有 PPtr 在磁盘上都是 m_FileID + m_PathID 两整数。
///
/// MVP 不带 generic：避免依赖 target class 存在（如 PrefabInstance 在某些 TPK 版本被改名）。
/// 用户拿不到类型区分（PPtr<GameObject> vs PPtr<Texture2D> 都变成 PPtr），但能编译。
/// 未来要做类型安全可改为 generic + PhantomData<fn() -> T>。
#[derive(Debug, Clone)]
pub struct PPtr {
    pub m_file_id: i32,
    pub m_path_id: i64,
}

impl PPtr {
    pub fn parse<R: EndianReader>(
        r: &mut R,
        _ver: &UnityVersion,
    ) -> Result<Self, R::Error> {
        let m_file_id = r.read_i32()?;
        // 简化：m_PathID 永远按 i64 读。
        // 老版本（ver < 5）实际是 i32，但生成器暂不做版本分支（D5 后改进）。
        let m_path_id = r.read_i64()?;
        Ok(PPtr {
            m_file_id,
            m_path_id,
        })
    }
}
