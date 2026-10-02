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

impl crate::ToValue for GraphicsStateInfoInvertProjectionMatrixVariant {
    fn to_value(&self) -> crate::Value {
        match self {
            Self::Default(v) => v.to_value(),
            Self::V6000_4(v) => v.to_value(),
        }
    }
    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        match self {
            Self::Default(v) => (v).inspect_value(path, offset, limit),
            Self::V6000_4(v) => (v).inspect_value(path, offset, limit),
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
        invert_projection_matrix = Some(GraphicsStateInfoInvertProjectionMatrixVariant::parse(
            rdr, ver,
        )?);
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

impl crate::ToValue for GraphicsStateInfo {
    fn to_value(&self) -> crate::Value {
        let fields: Vec<(String, crate::Value)> = [
            Some(("vertexLayout".to_string(), self.vertex_layout.to_value())),
            Some(("renderPass".to_string(), self.render_pass.to_value())),
            Some(("renderState".to_string(), self.render_state.to_value())),
            Some(("subPassIndex".to_string(), self.sub_pass_index.to_value())),
            Some(("topology".to_string(), self.topology.to_value())),
            Some(("forceCullMode".to_string(), self.force_cull_mode.to_value())),
            Some(("depthBias".to_string(), self.depth_bias.to_value())),
            crate::opt_field(
                "shadingRateCombinerPrimitive",
                &self.shading_rate_combiner_primitive,
            ),
            Some((
                "slopeDepthBias".to_string(),
                self.slope_depth_bias.to_value(),
            )),
            crate::opt_field(
                "shadingRateCombinerFragment",
                &self.shading_rate_combiner_fragment,
            ),
            Some(("userBackface".to_string(), self.user_backface.to_value())),
            crate::opt_field("baseShadingRate", &self.base_shading_rate),
            Some(("appBackface".to_string(), self.app_backface.to_value())),
            Some(("wireframe".to_string(), self.wireframe.to_value())),
            Some((
                "invertProjectionMatrix".to_string(),
                self.invert_projection_matrix.to_value(),
            )),
            crate::opt_field(
                "unfilterableTextureBindings",
                &self.unfilterable_texture_bindings,
            ),
            crate::opt_field(
                "nonfilteringSamplerBindings",
                &self.nonfiltering_sampler_bindings,
            ),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::Value::Struct(fields)
    }

    fn inspect_value(&self, path: &[String], offset: usize, limit: usize) -> Option<crate::Value> {
        if let Some((name, path)) = path.split_first() {
            return match name.as_str() {
                "vertexLayout" => (self.vertex_layout).inspect_value(path, offset, limit),
                "renderPass" => (self.render_pass).inspect_value(path, offset, limit),
                "renderState" => (self.render_state).inspect_value(path, offset, limit),
                "subPassIndex" => (self.sub_pass_index).inspect_value(path, offset, limit),
                "topology" => (self.topology).inspect_value(path, offset, limit),
                "forceCullMode" => (self.force_cull_mode).inspect_value(path, offset, limit),
                "depthBias" => (self.depth_bias).inspect_value(path, offset, limit),
                "shadingRateCombinerPrimitive" => {
                    (self.shading_rate_combiner_primitive).inspect_value(path, offset, limit)
                }
                "slopeDepthBias" => (self.slope_depth_bias).inspect_value(path, offset, limit),
                "shadingRateCombinerFragment" => {
                    (self.shading_rate_combiner_fragment).inspect_value(path, offset, limit)
                }
                "userBackface" => (self.user_backface).inspect_value(path, offset, limit),
                "baseShadingRate" => (self.base_shading_rate).inspect_value(path, offset, limit),
                "appBackface" => (self.app_backface).inspect_value(path, offset, limit),
                "wireframe" => (self.wireframe).inspect_value(path, offset, limit),
                "invertProjectionMatrix" => {
                    (self.invert_projection_matrix).inspect_value(path, offset, limit)
                }
                "unfilterableTextureBindings" => {
                    (self.unfilterable_texture_bindings).inspect_value(path, offset, limit)
                }
                "nonfilteringSamplerBindings" => {
                    (self.nonfiltering_sampler_bindings).inspect_value(path, offset, limit)
                }
                _ => None,
            };
        }
        let (path, offset, limit): (&[String], usize, usize) = (&[], 0, 0);
        let fields: Vec<(String, crate::Value)> = vec![
            ((self.vertex_layout).inspect_value(path, offset, limit))
                .map(|v| ("vertexLayout".to_string(), v)),
            ((self.render_pass).inspect_value(path, offset, limit))
                .map(|v| ("renderPass".to_string(), v)),
            ((self.render_state).inspect_value(path, offset, limit))
                .map(|v| ("renderState".to_string(), v)),
            ((self.sub_pass_index).inspect_value(path, offset, limit))
                .map(|v| ("subPassIndex".to_string(), v)),
            ((self.topology).inspect_value(path, offset, limit))
                .map(|v| ("topology".to_string(), v)),
            ((self.force_cull_mode).inspect_value(path, offset, limit))
                .map(|v| ("forceCullMode".to_string(), v)),
            ((self.depth_bias).inspect_value(path, offset, limit))
                .map(|v| ("depthBias".to_string(), v)),
            ((self.shading_rate_combiner_primitive).inspect_value(path, offset, limit))
                .map(|v| ("shadingRateCombinerPrimitive".to_string(), v)),
            ((self.slope_depth_bias).inspect_value(path, offset, limit))
                .map(|v| ("slopeDepthBias".to_string(), v)),
            ((self.shading_rate_combiner_fragment).inspect_value(path, offset, limit))
                .map(|v| ("shadingRateCombinerFragment".to_string(), v)),
            ((self.user_backface).inspect_value(path, offset, limit))
                .map(|v| ("userBackface".to_string(), v)),
            ((self.base_shading_rate).inspect_value(path, offset, limit))
                .map(|v| ("baseShadingRate".to_string(), v)),
            ((self.app_backface).inspect_value(path, offset, limit))
                .map(|v| ("appBackface".to_string(), v)),
            ((self.wireframe).inspect_value(path, offset, limit))
                .map(|v| ("wireframe".to_string(), v)),
            ((self.invert_projection_matrix).inspect_value(path, offset, limit))
                .map(|v| ("invertProjectionMatrix".to_string(), v)),
            ((self.unfilterable_texture_bindings).inspect_value(path, offset, limit))
                .map(|v| ("unfilterableTextureBindings".to_string(), v)),
            ((self.nonfiltering_sampler_bindings).inspect_value(path, offset, limit))
                .map(|v| ("nonfilteringSamplerBindings".to_string(), v)),
        ]
        .into_iter()
        .flatten()
        .collect();
        Some(crate::Value::Struct(fields))
    }
}
