varying vec3 v_worldPosition;
varying vec3 v_normal;
varying vec3 v_color;
varying float v_roughness;

uniform vec3 u_cameraPosition;
uniform vec3 u_lightPosition;
uniform vec3 u_lightColor;

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
	vec3 V = normalize(u_cameraPosition - v_worldPosition);
	float ao = 1.0;

	// Standard PBR rendering foruma with a single point light
	vec3 Lo = vec3(0.0);
	vec3 L = normalize(u_lightPosition - v_worldPosition);
	vec3 H = normalize(V + L);
	float dist = length(u_lightPosition - v_worldPosition);
	vec3 radiance = u_lightColor * (1.0 / (dist * dist));
	float NDF = DistributionGGX(N, H, v_roughness);
	float G = GeometrySmith(N, V, L, v_roughness);
	vec3 F = FresnelSchlick(max(dot(H, V), 0.0), vec3(0.04));
	vec3 kD = vec3(1.0) - F;
	vec3 spec = (NDF * G * F) / (4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.001);
	Lo += (kD * v_color / pi + spec) * radiance * max(dot(N, L), 0.0);

	vec3 ambient = vec3(0.05) * v_color * ao;
	vec3 linearColor = ambient + Lo;
	vec3 color = pow(linearColor / (linearColor + vec3(1.0)), vec3(1.0 / 2.2));
	gl_FragColor = vec4(color, 1.0);
}
