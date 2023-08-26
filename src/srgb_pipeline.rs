use notan::prelude::*;

//language=glsl
const IMAGE_VERTEX: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 a_pos;
    layout(location = 1) in vec2 a_uvs;
    layout(location = 2) in vec4 a_color;

    layout(location = 0) out vec4 v_color;
    layout(location = 1) out vec2 v_uvs;
    layout(set = 0, binding = 0) uniform Locals {
        mat4 u_projection;
    };

    void main() {
        v_color = a_color;
        v_uvs = a_uvs;
        gl_Position = u_projection * vec4(a_pos, 0.0, 1.0);
    }
    "#
};

//language=glsl
const IMAGE_FRAGMENT: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) in vec2 v_uvs;
    layout(location = 1) in vec4 v_color;

    layout(binding = 0) uniform sampler2D u_texture;

    layout(location = 0) out vec4 color;

    void main() {
        color = texture(u_texture, v_uvs) * v_color;
    }
    "#
};

pub(crate) fn create_srgb_pipeline(gfx: &mut Graphics) -> Result<Pipeline, String> {
    gfx.create_pipeline()
        .from(&IMAGE_VERTEX, &IMAGE_FRAGMENT)
        .with_srgb_space(true)
        .with_vertex_info(
            &VertexInfo::new()
                .attr(0, VertexFormat::Float32x2)
                .attr(1, VertexFormat::Float32x2)
                .attr(2, VertexFormat::Float32x4),
        )
        .with_texture_location(0, "u_texture")
        .build()
}
