//! 自动生成。请勿手动修改。


/// 版本枚举：GraphicsStateInfoInvertProjectionMatrixVariant 的类型跨版本变化。
#[derive(Debug, Clone)]
pub enum GraphicsStateInfoInvertProjectionMatrixVariant {
    Default(bool),
    V6000_4(bool),
}

impl crate::Parse for GraphicsStateInfoInvertProjectionMatrixVariant {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            let v = rdr.read_bool()?;
            Ok(Self::V6000_4(v))
        } else {
            let v = rdr.read_bool()?;
            rdr.align(4)?;
            Ok(Self::Default(v))
        }
    }
}
#[derive(Debug, Clone)]
pub struct GraphicsStateInfo {
    pub vertex_layout: u64,
    pub render_pass: u64,
    pub render_state: u64,
    pub sub_pass_index: i32,
    pub topology: i32,
    pub force_cull_mode: i32,
    pub depth_bias: f32,
    pub shading_rate_combiner_primitive: Option<i32>,
    pub slope_depth_bias: f32,
    pub shading_rate_combiner_fragment: Option<i32>,
    pub user_backface: bool,
    pub base_shading_rate: Option<i32>,
    pub app_backface: bool,
    pub wireframe: bool,
    pub invert_projection_matrix: GraphicsStateInfoInvertProjectionMatrixVariant,
    pub unfilterable_texture_bindings: Option<u64>,
    pub nonfiltering_sampler_bindings: Option<u64>,
}

impl crate::Parse for GraphicsStateInfo {
    fn parse<R: crate::EndianReader>(
        rdr: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let mut vertex_layout = None;
        let mut render_pass = None;
        let mut render_state = None;
        let mut sub_pass_index = None;
        let mut topology = None;
        let mut force_cull_mode = None;
        let mut depth_bias = None;
        let mut shading_rate_combiner_primitive = None;
        let mut slope_depth_bias = None;
        let mut shading_rate_combiner_fragment = None;
        let mut user_backface = None;
        let mut base_shading_rate = None;
        let mut app_backface = None;
        let mut wireframe = None;
        let mut invert_projection_matrix = None;
        let mut unfilterable_texture_bindings = None;
        let mut nonfiltering_sampler_bindings = None;
            vertex_layout = Some(rdr.read_u64()?);
            render_pass = Some(rdr.read_u64()?);
            render_state = Some(rdr.read_u64()?);
            sub_pass_index = Some(rdr.read_i32()?);
            topology = Some(rdr.read_i32()?);
            force_cull_mode = Some(rdr.read_i32()?);
        if *ver >= crate::UnityVersion::new(6000, 1, 0) {
            shading_rate_combiner_primitive = Some(rdr.read_i32()?);
            shading_rate_combiner_fragment = Some(rdr.read_i32()?);
            base_shading_rate = Some(rdr.read_i32()?);
        } else {
        }
            depth_bias = Some(rdr.read_f32()?);
            slope_depth_bias = Some(rdr.read_f32()?);
            user_backface = Some(rdr.read_bool()?);
            app_backface = Some(rdr.read_bool()?);
            wireframe = Some(rdr.read_bool()?);
            invert_projection_matrix = Some(GraphicsStateInfoInvertProjectionMatrixVariant::parse(rdr, ver)?);
        if *ver >= crate::UnityVersion::new(6000, 4, 0) {
            unfilterable_texture_bindings = Some(rdr.read_u64()?);
            nonfiltering_sampler_bindings = Some(rdr.read_u64()?);
            rdr.align(4)?;
        } else {
        }

        Ok(GraphicsStateInfo {
            vertex_layout: vertex_layout.unwrap(),
            render_pass: render_pass.unwrap(),
            render_state: render_state.unwrap(),
            sub_pass_index: sub_pass_index.unwrap(),
            topology: topology.unwrap(),
            force_cull_mode: force_cull_mode.unwrap(),
            depth_bias: depth_bias.unwrap(),
            shading_rate_combiner_primitive,
            slope_depth_bias: slope_depth_bias.unwrap(),
            shading_rate_combiner_fragment,
            user_backface: user_backface.unwrap(),
            base_shading_rate,
            app_backface: app_backface.unwrap(),
            wireframe: wireframe.unwrap(),
            invert_projection_matrix: invert_projection_matrix.unwrap(),
            unfilterable_texture_bindings,
            nonfiltering_sampler_bindings,
        })
    }
}
