#version 140
uniform sampler2D tex;
in vec2 f_tex_coords;
out vec4 f_color;


void main() {
    f_color = texture(tex, f_tex_coords);
}
