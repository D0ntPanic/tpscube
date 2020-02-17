uniform mat4 u_viewProjectionMatrix;
uniform mat4 u_modelMatrix;
uniform mat3 u_normalMatrix;
attribute vec3 a_position;
attribute vec3 a_normal;
attribute vec3 a_color;
attribute float a_roughness;

varying vec3 v_worldPosition;
varying vec3 v_normal;
varying vec3 v_color;
varying float v_roughness;

void main()
{
	v_worldPosition = vec3(u_modelMatrix * vec4(a_position, 1));
	v_normal = u_normalMatrix * a_normal;
	gl_Position = u_viewProjectionMatrix * vec4(v_worldPosition, 1);
	v_color = a_color;
	v_roughness = a_roughness;
}
