#version 450 core 
in vec2 fScreenCoordN;
out vec4 oColor;
uniform sampler2D screenTex;

void main() {
    // vec3 black = vec3(0x0A / 255.0, 0x4D / 255.0, 0x68 / 255.0);
    // vec3 white = vec3(0x00 / 255.0, 0xFF / 255.0, 0xCA / 255.0);
    vec3 black = vec3(0x39 / 255.0, 0x36 / 255.0, 0x46 / 255.0);
    vec3 white = vec3(0xF4 / 255.0, 0xEE / 255.0, 0xE0 / 255.0);
    vec2 invertedCoords = vec2(fScreenCoordN.x, 1 - fScreenCoordN.y);
    float bufferColor = texture(screenTex, invertedCoords).r;
    
    vec3 finalColor = (1 - bufferColor) * black + bufferColor * white;
    oColor = vec4(vec3(finalColor), 1.0);
}