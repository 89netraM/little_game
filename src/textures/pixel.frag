#version 440
#ifdef GL_FRAGMENT_PRECISION_HIGH
	precision highp float;
#else
	precision mediump float;
#endif

varying vec2 tex_coord_v;
varying vec3 normalInterp;
varying vec3 vertPos;

const vec3 color = vec3(1.0, 1.0, 1.0);
uniform sampler2D tex;

void main() {
	vec3 normal = normalize(normalInterp);
	vec3 lightDir = normalize(-vertPos);

	float lambertian = max(dot(lightDir, normal), 0.0);

	vec4 tex_color = texture2D(tex, tex_coord_v);
	gl_FragColor = tex_color * vec4(color + lambertian * 0.1, 1.0);
}
