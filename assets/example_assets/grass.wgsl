#import bevy_pbr::pbr_types::{PbrInput, pbr_input_new};
#import bevy_pbr::pbr_functions as fns;


@group(2) @binding(100)
var displacement:texture_2d<f32>;

@group(2) @binding(101)
var displacement_s:sampler;

struct FragmentInput {
    @location(0) index: u32,
    @location(1) position:vec4<f32>,
    @location(2) normal:vec3<f32>,
    @location(3) world_position:vec3<f32>,
    @location(4) uv:vec2<f32>, 
    @location(5) color:vec4<f32>
};

@fragment
fn fragment(input: FragmentInput,
    @builtin(front_facing) is_front:bool) -> @location(0) vec4<f32> {
    var pbr_input:PbrInput = pbr_input_new();
    pbr_input.world_normal = fns::prepare_world_normal(
        input.normal,
        true,
        is_front
    );
    pbr_input.material.perceptual_roughness = 1.0;
    pbr_input.frag_coord = input.position;
    pbr_input.world_position = vec4(input.world_position,1.0);
    pbr_input.material.base_color = input.color;
    pbr_input.N = normalize(pbr_input.world_normal);
    pbr_input.V = fns::calculate_view(pbr_input.world_position, false);

    return fns::apply_pbr_lighting(pbr_input);
}

