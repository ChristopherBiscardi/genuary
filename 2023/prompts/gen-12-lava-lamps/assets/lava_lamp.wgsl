@group(0) @binding(0) var texture: texture_storage_2d<rgba8unorm, read_write>;

struct Time {
    time_since_startup: f32,
};
@group(0) @binding(1)
var<uniform> time: Time;

fn hash(value: u32) -> u32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return state;
}

fn randomFloat(value: u32) -> f32 {
    return f32(hash(value)) / 4294967295.0;
}

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let randomNumber = randomFloat(invocation_id.y * num_workgroups.x + invocation_id.x);
    let alive = randomNumber > 0.9;
    let color = vec4<f32>(f32(alive));

    textureStore(texture, location, color);
}

// fn is_alive(location: vec2<i32>, offset_x: i32, offset_y: i32) -> i32 {
//     let value: vec4<f32> = textureLoad(texture, location + vec2<i32>(offset_x, offset_y));
//     return i32(value.x);
// }

// fn count_alive(location: vec2<i32>) -> i32 {
//     return is_alive(location, -1, -1) +
//            is_alive(location, -1,  0) +
//            is_alive(location, -1,  1) +
//            is_alive(location,  0, -1) +
//            is_alive(location,  0,  1) +
//            is_alive(location,  1, -1) +
//            is_alive(location,  1,  0) +
//            is_alive(location,  1,  1);
// }

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let scene_size = vec3<f32>(1280., 720., 0.);
    // - scene_size centers the scene
    var current_point = vec3<f32>(f32(invocation_id.x), f32(invocation_id.y), -5.) - (scene_size / 2.0);

    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    var color = vec4<f32>(0.,0.,0.,1.);
    for (var i = 0; i < 10; i++) {
        let distance_to_scene = scene(current_point);
        if distance_to_scene < 0.1 {
            color = vec4<f32>(1.);
            // color = vec4f(normalize(vec3f(distance_to_scene)), 1.0);
            // color = vec4<f32>(calc_normal(current_point).xyz, 1.0);
            break;
        } else {
            current_point.z += distance_to_scene;
        }
    }

    // let n_alive = count_alive(location);

    // var alive: bool;
    // if (n_alive == 3) {
    //     alive = true;
    // } else if (n_alive == 2) {
    //     let currently_alive = is_alive(location, 0, 0);
    //     alive = bool(currently_alive);
    // } else {
    //     alive = false;
    // }
    // let color = vec4<f32>(f32(alive));

    storageBarrier();

    textureStore(texture, location, color);
}

// polynomial smooth min
fn smin( a: f32, b: f32, k: f32 ) -> f32
{
    let h: f32 = max( k-abs(a-b), 0.0 )/k;
    return min( a, b ) - h*h*k*(1.0/4.0);
}

fn scene(current_point: vec3f) -> f32 {
    var m = 1000.;
    for (var i = 0; i < 10; i++) {
        m = smin(
            sd_sphere(current_point, vec3<f32>(
                sin(time.time_since_startup * f32(i)) * 150.,
                cos(time.time_since_startup * f32(i)) * 150.,
                100. * f32(i)), 100.
            ),
            m,
            0.5
        );
    }
    return m;
    // return sd_sphere(current_point, vec3<f32>(
    //     sin(time.time_since_startup) * 100.,
    //     cos(time.time_since_startup) * 100.,
    //     0.
    // ), 100.);
    // return domain_expansion(current_point, vec3f(400.,400.,400.));
}

// a sphere in the center of the world
fn sd_sphere( current_point: vec3f, center_point: vec3f, radius: f32 ) -> f32 {
  return length(current_point - center_point) - radius;
}

// I've never seen a full episode of jjk in my life
fn domain_expansion( current_position: vec3f, spacing: vec3f ) -> f32 {
    let q: vec3f = current_position - spacing * round(current_position / spacing);
    return sd_sphere(q, vec3<f32>(
        sin(time.time_since_startup) * 100.,
        cos(time.time_since_startup) * 100.,
        0.
    ), 100.);
}

fn calc_normal( p: vec3f ) -> vec3f // for function f(p)
{
    let h: f32 = 0.0001; // replace by an appropriate value
    let k: vec2f = vec2(1.,-1.);
    return normalize( k.xyy * scene( p + k.xyy*h ) + 
                      k.yyx * scene( p + k.yyx*h ) + 
                      k.yxy * scene( p + k.yxy*h ) + 
                      k.xxx * scene( p + k.xxx*h ) );
}