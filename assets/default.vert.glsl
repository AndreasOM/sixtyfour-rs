#version 410

// from assets

layout(location=0)in vec2 v;
layout(location=0)out vec2 p;
void main() {
    gl_Position = vec4( v, 0.0, 1.0);
    p = v;
}
