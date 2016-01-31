#version 140
uniform sampler2D image;
uniform float opacity;
in vec2 uv;
out vec4 color;


void main()
{
    color = texture(image, uv);
    color.a = color.a * opacity;
}

