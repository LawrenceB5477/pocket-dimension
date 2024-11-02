#version 330 core

layout (location = 0) in vec3 position;
//layout (location = 1) in vec2 textCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec4 diffuse;

//out vec4 Color;
out vec4 Color;

void main()
{
    gl_Position = projection * view * model * vec4(position, 1.0);
    Color = diffuse;
//    TextCoord = textCoord;
}