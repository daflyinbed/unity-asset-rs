//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct ColorRgba {
    pub r: Option<f32>,
    pub rgba: Option<u32>,
    pub g: Option<f32>,
    pub b: Option<f32>,
    pub a: Option<f32>,
}

impl crate::Parse for ColorRgba {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let r = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let rgba = if *ver >= crate::UnityVersion::new(5, 0, 1) {
            Some(rdr.read_u32()?)
        } else {
            None
        };
        let g = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let b = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };
        let a = if *ver >= crate::UnityVersion::new(3, 4, 0) {
            Some(rdr.read_f32()?)
        } else {
            None
        };

        Ok(ColorRgba {
            r,
            rgba,
            g,
            b,
            a,
        })
    }
}
