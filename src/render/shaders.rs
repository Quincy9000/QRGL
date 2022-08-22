use crate::render::ogl::ShaderType;

pub const SIMPLE_VERTEX: ShaderType = ShaderType::Vertex(BASIC_VERTEX);
pub const SIMPLE_FRAMENT: ShaderType = ShaderType::Fragment(BASIC_FRAG);

const BASIC_VERTEX: &str = r"
#version 330 core

layout (location = 0) in vec3 aPos;

uniform vec3 color;

out vec3 colorPos;

void main()
{
    colorPos = color;
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

const BASIC_FRAG: &str = r"
#version 330 core

in vec3 colorPos;
out vec4 FragColor;

void main()
{
    FragColor = vec4(colorPos, 1.0f);
}";
