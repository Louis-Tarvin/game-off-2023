#import bevy_core_pipeline::fullscreen_vertex_shader FullscreenVertexOutput

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;
struct TransitionSettings {
    progress: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2)
var<uniform> settings: TransitionSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(screen_texture, texture_sampler, in.uv);
    // Rotate the coordinates by 45 degrees
    let angle: f32 = 45.0 * 3.14159 / 180.0; // Convert degrees to radians
    let rotated_uv: vec2<f32> = vec2<f32>(
        (in.uv.x - 0.5) * cos(angle) - in.uv.y * sin(angle) + 0.5,
        in.uv.x * sin(angle) + in.uv.y * cos(angle)
    );
    let rotated_uv2: vec2<f32> = vec2<f32>(
        (in.uv.x - 0.5) * cos(-angle) - in.uv.y * sin(-angle) + 0.5,
        in.uv.x * sin(-angle) + in.uv.y * cos(-angle)
    );
    var val: f32;
    if in.uv.x <= 0.5 {
        val = fract(rotated_uv2.x * 8.0);
    } else {
        val = 1.0 - fract(rotated_uv.x * 8.0);
    }
    //val *= (1.0 - in.uv.y);

    var alpha: f32;
    if val < settings.progress {
        color = vec4<f32>(0.0);
    }
    return vec4<f32>(color);
}
