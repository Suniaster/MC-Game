// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>;
    view_pos: vec4<f32>;
};
[[group(0), binding(0)]] // 2.
var<uniform> camera: CameraUniform;

struct InstanceInput {
    [[location(2)]] color: vec3<f32>;
    [[location(3)]] model_matrix_0: vec4<f32>;
    [[location(4)]] model_matrix_1: vec4<f32>;
    [[location(5)]] model_matrix_2: vec4<f32>;
    [[location(6)]] model_matrix_3: vec4<f32>;

    [[location(7)]] normal_matrix_0: vec3<f32>;
    [[location(8)]] normal_matrix_1: vec3<f32>;
    [[location(9)]] normal_matrix_2: vec3<f32>;
};
 

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] world_position: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    let normal_matrix = mat3x3<f32>(
        instance.normal_matrix_0,
        instance.normal_matrix_1,
        instance.normal_matrix_2,
    );


    var out: VertexOutput;
    out.color = instance.color;
    out.world_normal = normal_matrix * model.normal;

    var world_position: vec4<f32> = model_matrix * vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = camera.view_proj * world_position;
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let object_color: vec4<f32> =  vec4<f32>(in.color, 1.0);
    let ambient_strength = 0.1;

    let view_dir = normalize(camera.view_pos.xyz - in.world_position);
    let half_dir = normalize(view_dir);
    
    let specular_strength = pow(max(dot(in.world_normal, half_dir), 0.0), 32.0);
    let specular_color = specular_strength;

    let result = (specular_color) * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}
 