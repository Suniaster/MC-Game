
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

    var light_pos = u_camera.position.xyz;
    light_pos = vec3<f32>(light_pos.x, 100.0, light_pos.z);

    let view_dir = normalize(light_pos.xyz - in.position);
    let half_dir = normalize(view_dir);

    let specular_strength = pow(max(dot(normal, half_dir), 0.0), 1.0);
    let specular_color = specular_strength;

    let surface_color = vec4<f32>(in.diffuse_color, 1.0);

    let result = (specular_color) * surface_color.xyz;
    let color_result = result * in.builtin_position.xyz;

    return vec4<f32>(color_result, 1.0);
    // let x = in.builtin_position.x / 1024.0;
    // let y = in.builtin_position.y / 768.0;
    // return vec4<f32>(x, y, in.builtin_position.z, surface_color.a);

    // let texture_pos = vec2<f32>(x, y);
    // let depth = textureSampleCompare(
    //     depth_texture, 
    //     depth_sampler, 
    //     texture_pos, 
    //     in.builtin_position.w
    // );
    // return vec4<f32>(vec3<f32>(depth), 1.0);

    // return textureSample(depth_texture, depth_sampler, texture_pos);
}