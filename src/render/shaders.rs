use crate::render::ogl::ShaderType;

pub const BASIC_VERTEX: ShaderType = ShaderType::Vertex(VERTEX_SOURCE);
pub const BASIC_FRAGMENT: ShaderType = ShaderType::Fragment(FRAGMENT_SOURCE);

const VERTEX_SOURCE: &str = r"
#version 330 core

layout (location = 0) in vec3 aPos;

// uniform vec3 color;

out vec3 out_color;

void main()
{
    out_color = vec3(1, 0, 0);
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

const FRAGMENT_SOURCE: &str = r"
#version 330 core

in vec3 out_color;
out vec4 FragColor;

void main()
{
    // float r = (float)(out_color >> 24);
    // float g = (float)(out_color >> 16);
    // float b = (float)(out_color >> 8);
    // float a = (float)out_color;
    FragColor = vec4(1, 0, 0, a);
}";
