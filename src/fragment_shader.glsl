#version 140

in vec2 v_texture_position;
out vec4 color;

uniform sampler2D tex;

void main() {
  color = texture(tex, v_texture_position);
}
