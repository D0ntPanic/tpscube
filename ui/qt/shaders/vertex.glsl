uniform mat4 u_modelViewProjection;
attribute vec4 a_position;
attribute vec3 a_color;

varying vec3 v_color;

void main()
{
	gl_Position = u_modelViewProjection * a_position;
	v_color = a_color;
}
