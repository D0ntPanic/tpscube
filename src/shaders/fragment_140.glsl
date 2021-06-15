#version 140
in vec3 v_world_pos;
in vec3 v_normal;
in vec3 v_color;
in float v_roughness;
out vec4 f_color;

uniform vec3 camera_pos;
uniform vec3 light_pos;
uniform vec3 light_color;

const float pi = 3.14159265358979;

float DistributionGGX(vec3 N, vec3 H, float roughness)
{
	float a = roughness * roughness;
	float aSquared = a * a;
	float NdotH = max(dot(N, H), 0.0);
	float denom = NdotH * NdotH * (aSquared - 1.0) + 1.0;
	return aSquared / (pi * denom * denom);
}

float GeometrySchlickGGX(float NdotV, float roughness)
{
	float r = roughness + 1.0;
	float k = (r * r) / 8.0;
	return NdotV / (NdotV * (1.0 - k) + k);
}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness)
{
	return GeometrySchlickGGX(max(dot(N, L), 0.0), roughness) *
		GeometrySchlickGGX(max(dot(N, V), 0.0), roughness);
}

vec3 FresnelSchlick(float cosTheta, vec3 F0)
{
	return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
}

void main()
{
	vec3 N = normalize(v_normal);
	vec3 V = normalize(camera_pos - v_world_pos);
	float ao = 1.0;

	// Standard PBR rendering foruma with a single point light
	vec3 Lo = vec3(0.0);
	vec3 L = normalize(light_pos - v_world_pos);
	vec3 H = normalize(V + L);
	float dist = length(light_pos - v_world_pos);
	vec3 radiance = light_color * (1.0 / (dist * dist));
	float NDF = DistributionGGX(N, H, v_roughness);
	float G = GeometrySmith(N, V, L, v_roughness);
	vec3 F = FresnelSchlick(max(dot(H, V), 0.0), vec3(0.04));
	vec3 kD = vec3(1.0) - F;
	vec3 spec = (NDF * G * F) / (4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.001);
	Lo += (kD * v_color / pi + spec) * radiance * max(dot(N, L), 0.0);

	vec3 ambient = vec3(0.05) * v_color * ao;
	vec3 linear_color = ambient + Lo;
	//vec3 color = pow(linear_color / (linear_color + vec3(1.0)), vec3(1.0 / 2.2));
	f_color = vec4(linear_color, 1.0);
}
