#version 140
uniform mat4 matrix;
uniform sampler2D image;
in vec2 position;
in ivec2 texture_position;
out vec2 uv;

void main()
{
    vec2 size = vec2(textureSize(image, 0));
    uv = vec2(texture_position) / size;
    uv.y = 1.0 - uv.y;
    gl_Position = matrix * vec4(position, 0.0, 1.0);
}


