
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


[[group(1), binding(0)]]
var depth_texture: texture_depth_2d;
[[group(1), binding(1)]]
var depth_sampler: sampler_comparison;

[[stage(fragment), early_depth_test]]
fn fs_main(
    in: VertexOutput,
) -> [[location(0)]] vec4<f32> {
    let normal = normalize(in.normal);
    
    let ambient_strength = 0.1;
    let light_color = vec3<f32>(1.0, 1.0, 1.0);
    let light_pos = vec3<f32>(0.0, 100.0, 0.0);
    let ambient_color = light_color * ambient_strength;

    let light_dir = normalize(light_pos - in.position);

    let diffuse_strength = max(dot(in.normal, light_dir), 0.0);
    let diffuse_color = light_color * diffuse_strength;

    let result = (ambient_color + diffuse_color) * in.diffuse_color;

    return vec4<f32>(result, 1.0);
}