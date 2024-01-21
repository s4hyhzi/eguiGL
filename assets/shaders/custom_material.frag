#version 450
precision mediump float;

in vec4 color;

out vec4 out_color;

uniform vec4 u_color;

void main() {
    out_color = color * u_color;
}