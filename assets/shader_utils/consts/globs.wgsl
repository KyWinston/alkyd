#define_import_path global_values


// The circle family
const PI:f32  =         3.14159265359;
const HALF_PI =         1.57079632679;
const NEG_HALF_PI =    -1.57079632679;
const NEG_QUARTER_PI = -0.78539816339;
const QUARTER_PI =     -0.78539816339;
const TAU:f32 =         6.28318530718;

// Euler's number / Napier's constant
const E: f32 =          2.71828182845;

// Pythagoras' constants
const SQRT_OF_2:f32 =   1.41421356237;
const SQRT_OF_3:f32 =   1.73205080756;

// The golden ratio
const PHI:f32 =         1.61803398874;

struct NoiseProperties {
    octaves: i32,
    lacunarity: f32,
    gain: f32,
    amplitude: f32,
    frequency: f32
}

const identity_matrix: mat4x4<f32> = mat4x4<f32>(
    vec4<f32>(1.0, 0.0, 0.0, 0.0),
    vec4<f32>(0.0, 1.0, 0.0, 0.0),
    vec4<f32>(0.0, 0.0, 1.0, 0.0),
    vec4<f32>(0.0, 0.0, 0.0, 1.0)
);