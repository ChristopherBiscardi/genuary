#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals;

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {


    let a = vec4(
               perlinNoise2(mesh.uv.yy * 10) % 2 / perlinNoise2(mesh.uv.yy * 110),

0.7,
        perlinNoise2(mesh.uv.xx * 20) % 2 / perlinNoise2(mesh.uv.xx * 10),

        1);

    let b = vec4(
        perlinNoise2(mesh.uv.xx * 20) % 2 / perlinNoise2(mesh.uv.xx * 10),
0.7,
       perlinNoise2(mesh.uv.yy * 10) % 2 / perlinNoise2(mesh.uv.yy * 110),
        1);

    let c = vec4(
        perlinNoise2(mesh.uv.xx * 10) % 2 / perlinNoise2(mesh.uv.xx * 100),
0.7,
       perlinNoise2(mesh.uv.yy * 10) % 2 / perlinNoise2(mesh.uv.yy * 10),
        1);
    // let shape = length(mesh.uv - 0.5) - 0.1;
    // let shape = step(
    //     (sin(globals.time) + 1. / 2.),
    //     sdBox((mesh.uv - 0.5) * 2, vec2(0.5))
    // );
    let shape = it(mesh.uv, 0.) * it(mesh.uv + 0.1, 20.);
    let t2 = it(mesh.uv - 0.2, 40.);
    let t = vec4(shape,shape,shape,1);
    return mix(
        mix(a, b, t),
        c,
        t2);
    // return vec4(shape, shape, shape, 1.0);
    // return material_color * textureSample(base_color_texture, base_color_sampler, mesh.uv) * COLOR_MULTIPLIER;
}

fn it(xy: vec2f, offset: f32) -> f32 {
    return step(
        (sin(globals.time + offset) + 1. / 2.),
        sdBox((xy - 0.5) * 2, vec2(0.5))
    );
}

fn s(xy: vec2f) -> f32 {
return    length(xy -0.5) - 0.1;
}

fn sdBox( p: vec2f, b: vec2f ) -> f32
{
    let d: vec2f = abs(p)-b;
    return length(max(d,vec2(0.0))) + min(max(d.x,d.y),0.0);
}

fn permute4(x: vec4f) -> vec4f { return ((x * 34. + 1.) * x) % vec4f(289.); }
fn fade2(t: vec2f) -> vec2f { return t * t * t * (t * (t * 6. - 15.) + 10.); }

fn perlinNoise2(P: vec2f) -> f32 {
    var Pi: vec4f = floor(P.xyxy) + vec4f(0., 0., 1., 1.);
    let Pf = fract(P.xyxy) - vec4f(0., 0., 1., 1.);
    Pi = Pi % vec4f(289.); // To avoid truncation effects in permutation
    let ix = Pi.xzxz;
    let iy = Pi.yyww;
    let fx = Pf.xzxz;
    let fy = Pf.yyww;
    let i = permute4(permute4(ix) + iy);
    var gx: vec4f = 2. * fract(i * 0.0243902439) - 1.; // 1/41 = 0.024...
    let gy = abs(gx) - 0.5;
    let tx = floor(gx + 0.5);
    gx = gx - tx;
    var g00: vec2f = vec2f(gx.x, gy.x);
    var g10: vec2f = vec2f(gx.y, gy.y);
    var g01: vec2f = vec2f(gx.z, gy.z);
    var g11: vec2f = vec2f(gx.w, gy.w);
    let norm = 1.79284291400159 - 0.85373472095314 *
        vec4f(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
    g00 = g00 * norm.x;
    g01 = g01 * norm.y;
    g10 = g10 * norm.z;
    g11 = g11 * norm.w;
    let n00 = dot(g00, vec2f(fx.x, fy.x));
    let n10 = dot(g10, vec2f(fx.y, fy.y));
    let n01 = dot(g01, vec2f(fx.z, fy.z));
    let n11 = dot(g11, vec2f(fx.w, fy.w));
    let fade_xy = fade2(Pf.xy);
    let n_x = mix(vec2f(n00, n01), vec2f(n10, n11), vec2f(fade_xy.x));
    let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy;
}