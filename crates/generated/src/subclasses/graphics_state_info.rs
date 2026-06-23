//! 自动生成。请勿手动修改。

#[derive(Debug, Clone)]
pub struct GraphicsStateInfo {
    pub vertex_layout: u64,
    pub render_pass: u64,
    pub render_state: u64,
    pub sub_pass_index: i32,
    pub topology: i32,
    pub force_cull_mode: i32,
    pub depth_bias: f32,
    pub slope_depth_bias: f32,
    pub user_backface: bool,
    pub app_backface: bool,
    pub wireframe: bool,
    pub invert_projection_matrix: bool,
}

impl crate::Parse for GraphicsStateInfo {
    fn parse<R: crate::EndianReader>(
        r: &mut R,
        ver: &crate::UnityVersion,
    ) -> Result<Self, R::Error> {
        let vertex_layout = r.read_u64()?;
        let render_pass = r.read_u64()?;
        let render_state = r.read_u64()?;
        let sub_pass_index = r.read_i32()?;
        let topology = r.read_i32()?;
        let force_cull_mode = r.read_i32()?;
        let depth_bias = r.read_f32()?;
        let slope_depth_bias = r.read_f32()?;
        let user_backface = r.read_bool()?;
        let app_backface = r.read_bool()?;
        let wireframe = r.read_bool()?;
        let invert_projection_matrix = r.read_bool()?;
        r.align(4)?;

        Ok(GraphicsStateInfo {
            vertex_layout,
            render_pass,
            render_state,
            sub_pass_index,
            topology,
            force_cull_mode,
            depth_bias,
            slope_depth_bias,
            user_backface,
            app_backface,
            wireframe,
            invert_projection_matrix,
        })
    }
}
