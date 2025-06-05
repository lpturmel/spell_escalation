#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var texture: texture_2d<f32>;
@group(2) @binding(1) var samp: sampler;
@group(2) @binding(2) var<uniform> color_a: vec4<f32>;
@group(2) @binding(3) var<uniform> color_b: vec4<f32>;

fn luminance(rgb: vec3<f32>) -> f32 {
    return dot(rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let mask      = textureSample(texture, samp, in.uv);
    let t         = saturate(luminance(mask.rgb));
    let tint      = mix(color_a, color_b, t);

    return vec4<f32>(tint.rgb, mask.a);
}
