#version 450 core
layout(location = 0) in vec2 iCoord;
out vec2 fScreenCoordN;

void main() {
    gl_Position = vec4(iCoord, 0, 1.0);
    fScreenCoordN = iCoord * 0.5 + 0.5;
}