
struct CameraUniform {
    projection_view: mat4x4<f32>;
    position: vec4<f32>;
};

[[group(0), binding(0)]]
var<uniform> u_camera: CameraUniform;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] diffuse_color: vec3<f32>;
};

struct InstanceInput {
    [[location(3)]] position: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] builtin_position: vec4<f32>;
    [[location(1)]] diffuse_color: vec3<f32>;
    [[location(2)]] normal: vec3<f32>;
    [[location(3)]] position: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.normal = model.normal;
    let final_pos = model.position.xyz + instance.position.xyz;
    let model_space = vec4<f32>(final_pos, 1.0);

    out.position = model_space.xyz;
    out.diffuse_color = model.diffuse_color;

    out.builtin_position = u_camera.projection_view * model_space;
    return out;
}

[[stage(fragment), early_depth_test]]
fn fs_main(
    in: VertexOutput,
) -> [[location(0)]] vec4<f32> {
    let normal = normalize(in.normal);

    var light_pos = u_camera.position.xyz;
    light_pos = vec3<f32>(light_pos.x, 100.0, light_pos.z);

    let view_dir = normalize(light_pos.xyz - in.position);
    let half_dir = normalize(view_dir);

    let specular_strength = pow(max(dot(normal, half_dir), 0.0), 1.0);
    let specular_color = specular_strength;

    let surface_color = vec4<f32>(in.diffuse_color, 1.0);

    let result = (specular_color) * surface_color.xyz;

    return vec4<f32>(result, surface_color.a);
}