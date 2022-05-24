
struct CameraUniform {
    projection_view: mat4x4<f32>;
    position: vec4<f32>;
};

[[group(0), binding(0)]]
var<uniform> u_camera: CameraUniform;

struct VertexOutput {
    [[builtin(position)]] builtin_position: vec4<f32>;
    [[location(1)]] diffuse_color: vec3<f32>;
    [[location(2)]] normal: vec3<f32>;
    [[location(3)]] position: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] position: vec3<f32>,
    [[location(1)]] normal: vec3<f32>,
    [[location(2)]] diffuse_color: vec3<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.normal = normal;
    let model_space = vec4<f32>(position, 1.0);
    out.position = model_space.xyz;
    out.diffuse_color = diffuse_color;

    out.builtin_position = u_camera.projection_view * model_space;
    return out;
}

[[stage(fragment), early_depth_test]]
fn fs_main(
    in: VertexOutput,
) -> [[location(0)]] vec4<f32> {
    let normal = normalize(in.normal);

    let view_dir = normalize(u_camera.position.xyz - in.position);
    let half_dir = normalize(view_dir);

    let specular_strength = pow(max(dot(normal, half_dir), 0.0), 1.0);
    let specular_color = specular_strength;

    let surface_color = vec4<f32>(in.diffuse_color, 1.0);

    let result = (specular_color) * surface_color.xyz;

    return vec4<f32>(result, surface_color.a);
}