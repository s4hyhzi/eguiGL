#version 450
layout (location = 0) in vec4 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec2 texCoord;
out vec4 color;
void main() {
    gl_Position = aPos;
    color = aColor;
    texCoord = aTexCoord;
}