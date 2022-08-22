use crate::render::ogl::ShaderType;

pub const SIMPLE_VERTEX: ShaderType = ShaderType::Vertex(BASIC_VERTEX);
pub const SIMPLE_FRAGMENT: ShaderType = ShaderType::Fragment(BASIC_FRAG);

const BASIC_VERTEX: &str = r"
#version 330 core

layout (location = 0) in vec3 aPos;

uniform uint color;

out uint colorPos;

void main()
{
    colorPos = color;
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

const BASIC_FRAG: &str = r"
#version 330 core

in uint colorPos;
out vec4 FragColor;

void main()
{
    float r = (float)(colorPos >> 24);
    float g = (float)(colorPos >> 16);
    float b = (float)(colorPos >> 8);
    float a = (float)colorPos;
    FragColor = vec4(r, g, b, a);
}";
