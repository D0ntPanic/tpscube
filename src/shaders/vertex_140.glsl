#version 140
uniform mat4 view_proj_matrix;
uniform mat4 model_matrix;
uniform mat3 normal_matrix;
in vec3 pos;
in vec3 normal;
in vec3 color;
in float roughness;
out vec3 v_world_pos;
out vec3 v_color;
out vec3 v_normal;
out float v_roughness;

void main() {
    v_world_pos = vec3(model_matrix * vec4(pos, 1.0));
    v_normal = normal_matrix * normal;
    gl_Position = view_proj_matrix * vec4(v_world_pos, 1.0);
    v_color = color;
    v_roughness = roughness;
}
