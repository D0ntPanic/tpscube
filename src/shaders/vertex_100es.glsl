#version 100
precision highp float;
uniform mat4 view_proj_matrix;
uniform mat4 model_matrix;
uniform mat3 normal_matrix;
attribute vec3 pos;
attribute vec3 normal;
attribute vec3 color;
attribute float roughness;
varying vec3 v_world_pos;
varying vec3 v_color;
varying vec3 v_normal;
varying float v_roughness;

void main() {
    v_world_pos = vec3(model_matrix * vec4(pos, 1.0));
    v_normal = normal_matrix * normal;
    gl_Position = view_proj_matrix * vec4(v_world_pos, 1.0);
    v_color = color;
    v_roughness = roughness;
}
