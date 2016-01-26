#version 140
uniform mat4 matrix;
uniform sampler2D image;
in vec2 position;
in ivec2 texture_position;
out vec2 uv;

void main() {
    ivec2 size = textureSize(image, 0);
    gl_Position = matrix * vec4(position, 0.0, 1.0);
    float x = float(texture_position.x) / float(size.x);
    float y = 1 - (float(texture_position.y)/float(size.y));
    uv = vec2(x, y);
}


