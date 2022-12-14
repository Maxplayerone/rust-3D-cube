#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 a_Color;

out vec2 f_Color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    f_Color = a_Color;
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}