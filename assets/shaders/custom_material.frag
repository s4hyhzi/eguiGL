#version 450
precision mediump float;

in vec4 color;
in vec2 texCoord;

out vec4 out_color;

uniform vec4 u_color;
uniform sampler2D u_texture;

void main() {
    out_color = texture2D(u_texture, texCoord) * u_color * color;
}