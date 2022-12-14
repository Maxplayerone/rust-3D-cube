#version 330 core
out vec4 FragColor;

in vec2 f_Color;

void main()
{
    FragColor = vec4(f_Color, 0.0, 1.0);
}