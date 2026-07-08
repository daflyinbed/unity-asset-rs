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
        let mut r = None;
        let mut rgba = None;
        let mut g = None;
        let mut b = None;
        let mut a = None;
        if *ver >= crate::UnityVersion::new(6000, 3, 0) {
            r = Some(rdr.read_f32()?);
            g = Some(rdr.read_f32()?);
            b = Some(rdr.read_f32()?);
            a = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 2, 15) {
            rgba = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 1, 1) {
            r = Some(rdr.read_f32()?);
            g = Some(rdr.read_f32()?);
            b = Some(rdr.read_f32()?);
            a = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 64) {
            rgba = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 15) {
            r = Some(rdr.read_f32()?);
            g = Some(rdr.read_f32()?);
            b = Some(rdr.read_f32()?);
            a = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(6000, 0, 7) {
            rgba = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 27) {
            r = Some(rdr.read_f32()?);
            g = Some(rdr.read_f32()?);
            b = Some(rdr.read_f32()?);
            a = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 24) {
            rgba = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 3, 3) {
            r = Some(rdr.read_f32()?);
            g = Some(rdr.read_f32()?);
            b = Some(rdr.read_f32()?);
            a = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 2, 20) {
            rgba = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 15) {
            r = Some(rdr.read_f32()?);
            g = Some(rdr.read_f32()?);
            b = Some(rdr.read_f32()?);
            a = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2022, 1, 13) {
            rgba = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 12) {
            r = Some(rdr.read_f32()?);
            g = Some(rdr.read_f32()?);
            b = Some(rdr.read_f32()?);
            a = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2021, 3, 9) {
            rgba = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 4, 15) {
            r = Some(rdr.read_f32()?);
            g = Some(rdr.read_f32()?);
            b = Some(rdr.read_f32()?);
            a = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 4, 14) {
            rgba = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 4, 13) {
            r = Some(rdr.read_f32()?);
            g = Some(rdr.read_f32()?);
            b = Some(rdr.read_f32()?);
            a = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(2018, 4, 11) {
            rgba = Some(rdr.read_u32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 2) {
            r = Some(rdr.read_f32()?);
            g = Some(rdr.read_f32()?);
            b = Some(rdr.read_f32()?);
            a = Some(rdr.read_f32()?);
        } else if *ver >= crate::UnityVersion::new(5, 0, 1) {
            rgba = Some(rdr.read_u32()?);
        } else {
            r = Some(rdr.read_f32()?);
            g = Some(rdr.read_f32()?);
            b = Some(rdr.read_f32()?);
            a = Some(rdr.read_f32()?);
        }

        Ok(ColorRgba {
            r,
            rgba,
            g,
            b,
            a,
        })
    }
}
