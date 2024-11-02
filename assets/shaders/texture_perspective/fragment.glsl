#version 330 core
out vec4 OutColor;

in vec2 TCoord;

uniform sampler2D texture1;

void main()
{
    OutColor = texture(texture1, TCoord);
}