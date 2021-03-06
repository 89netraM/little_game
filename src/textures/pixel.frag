#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
	precision highp float;
#else
	precision mediump float;
#endif

varying vec2 tex_coord_v;
varying vec3 normalInterp;
varying vec3 vertPos;

uniform vec3 color;
uniform sampler2D tex;

void main() {
	vec3 normal = normalize(normalInterp);
	vec3 lightDir = normalize(-vertPos);

	float lambertian = max(dot(lightDir, normal), 0.0);

	vec4 tex_color = texture2D(tex, tex_coord_v);
	gl_FragColor = tex_color * vec4(color + lambertian - length(vertPos) / 10.0 - lambertian, 1.0);
}
