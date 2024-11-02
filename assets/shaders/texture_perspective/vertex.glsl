#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 textCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform float texWidthScale;

//uniform vec4 diffuse;

//out vec4 Color;
out vec4 Color;
out vec2 TCoord;

void main()
{
    gl_Position = projection * view * model * vec4(position, 1.0);

    if (texWidthScale > 0.0){
        TCoord = vec2(textCoord.x * texWidthScale, textCoord.y);
    } else {
        TCoord = textCoord;
    }

//    Color = diffuse;
//    TextCoord = textCoord;
}