#import shells::{shell_props,ShellProperties}

// @group(2) @binding(0)
// var<uniform> shell_props:ShellProperties;


struct FragmentInput {
    @location(0) index: u32,
    @location(1) position:vec4<f32>,
    @location(2) normal:vec3<f32>,
    @location(3) world_position:vec3<f32>,
    @location(4) uv:vec2<f32>, 
    @location(5) density:f32,
    @location(6) count:u32,
    @location(7) variance:vec2<f32>,
    @location(8) color:vec4<f32>
};

struct FragmentOutput{
    @location(0) color:vec4<f32>
};

@fragment
fn fragment(input: FragmentInput) -> FragmentOutput{
    var out:FragmentOutput;
    var new_uv = input.uv * input.density;
    var local_uv = fract(new_uv) * 2 - 1;
    let local_dist = length(local_uv);
    let seed = u32(new_uv.x) + 100 * u32(new_uv.y) + 1000;

    let shell_index = f32(input.index); 
    let shell_count = f32(input.count); 

    let rand = mix(input.variance.x,input.variance.y, hash(seed));
    let h = shell_index / shell_count;
    if ((local_dist) > (1.0 * (rand - h)) && shell_index > 0) {
        discard;
    }
    var ndotl = clamp(dot(input.normal, vec3(0.0,10.0,5.0)) * 0.5 + 0.5,0.0,1.0);
    ndotl = ndotl * ndotl;
    let AO = clamp(pow(h, 2.0),0.0,1.0) + 0.2;
    out.color = vec4<f32>(input.color.rgb * ndotl * AO, 1.0);

    return out;
}


fn hash(i:u32) ->f32{
				// integer hash copied from Hugo Elias
				var n = (i << 13) ^ i;
				n = n * (n * n * 15731 + 7901729) + 835873109;
				return f32(n & u32(2147483647)) / f32(2147483647);
			}