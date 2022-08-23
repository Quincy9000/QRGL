use crate::render::ogl::ShaderType;

pub const BASIC_VERTEX_2D: ShaderType = ShaderType::Vertex(VERTEX_SOURCE);
pub const BASIC_FRAGMENT_2D: ShaderType = ShaderType::Fragment(FRAGMENT_SOURCE);

const VERTEX_SOURCE: &str = r"
#version 330 core

layout (location = 0) in vec2 aPos;

uniform vec3 color;

out vec3 out_color;

void main()
{
    out_color = vec3(color.r, color.g, color.b);
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
}";

const FRAGMENT_SOURCE: &str = r"
#version 330 core

in vec3 out_color;
out vec4 FragColor;

void main()
{
    FragColor = vec4(out_color.r, out_color.g, out_color.b, 1);
}";
