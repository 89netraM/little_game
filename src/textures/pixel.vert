#version 440

attribute vec3 position;
attribute vec2 tex_coord;
attribute vec3 normal;

uniform mat3 n_transform;
uniform mat3 scale;
uniform mat4 proj;
uniform mat4 view;
uniform mat4 transform;
uniform vec3 light_position;

varying vec2 tex_coord_v;
varying vec3 normalInterp;
varying vec3 vertPos;

void main(){
	gl_Position = proj * view * transform * vec4(scale * position, 1.0);
	vec4 vertPos4 = view * transform * vec4(scale * position, 1.0);
	vertPos = vec3(vertPos4) / vertPos4.w;
	normalInterp = mat3(view) * n_transform * normal;
	tex_coord_v = tex_coord;
}
