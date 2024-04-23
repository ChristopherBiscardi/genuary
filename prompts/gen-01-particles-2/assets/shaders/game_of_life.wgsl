@group(0) @binding(0) var texture: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(1) var texture_vector: texture_storage_2d<rgba8unorm, read>;
// @group(0) @binding(2) var sampler_vector: sampler;

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
    // let alive = randomNumber > 0.9;
    // let color = vec4<f32>(f32(randomNumber), f32(randomNumber), 0.0, 1.0);
    let color = textureLoad(texture_vector, location);
    // let color = textureLoad(texture_vector, vec2<i32>(i32(num_workgroups.x + invocation_id.x), i32(invocation_id.y)));
    textureStore(texture, location, color);
}

fn is_alive(location: vec2<i32>, offset_x: i32, offset_y: i32) -> i32 {
    let value: vec4<f32> = textureLoad(texture, location + vec2<i32>(offset_x, offset_y));
    return i32(value.w);
}

fn count_alive(location: vec2<i32>) -> i32 {
    return is_alive(location, -1, -1) +
           is_alive(location, -1,  0) +
           is_alive(location, -1,  1) +
           is_alive(location,  0, -1) +
           is_alive(location,  0,  1) +
           is_alive(location,  1, -1) +
           is_alive(location,  1,  0) +
           is_alive(location,  1,  1);
}

fn get_color(location: vec2<i32>, offset_x: i32, offset_y: i32) -> vec4<f32> {
    let value: vec4<f32> = textureLoad(texture, location + vec2<i32>(offset_x, offset_y));
    return vec4(value.xyz, 1.0);
}

fn get_new_color(location: vec2<i32>) -> vec4<f32> {
    let a = get_color(location, -1, -1);
    let b = get_color(location, -1,  0);
    let c = get_color(location, -1,  1);
    let d = get_color(location,  0, -1);
    let e = get_color(location,  0,  1);
    let f = get_color(location,  1, -1);
    let g = get_color(location,  1,  0);
    let h = get_color(location,  1,  1);

    var color = (a.xyz * a.a) +
    (b.xyz * b.a) +
    (c.xyz * c.a) +
    (d.xyz * d.a) +
    (e.xyz * e.a) +
    (f.xyz * f.a) +
    (g.xyz * g.a) +
    (h.xyz * h.a);
    color = color / 3.;
    //  (
    //     a.a +
    //     b.a +
    //     c.a +
    //     d.a +
    //     e.a +
    //     f.a +
    //     g.a +
    //     h.a
    // );
    return vec4<f32>(color.xyz, 1.0);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let direction_color = textureLoad(texture_vector, location);
    // let direction = direction_color / 256. * 3;

    let n_alive = count_alive(location);

    let was_alive = is_alive(location, 0, 0);

    var alive: bool;
    if (n_alive == 3) {
        alive = true;
    } else if (n_alive == 2) {
        let currently_alive = is_alive(location, 0, 0);
        alive = bool(currently_alive);
    } else {
        alive = false;
    }
    
    var color = vec4<f32>(f32(alive));
    if (alive == true) {
        // color = textureLoad(texture_vector, location);
        let new_color = get_new_color(location);
        color = mix(new_color, textureLoad(texture_vector, location), new_color.b);
        color.a = 1.0;
    }

    if (bool(was_alive) != alive && alive == true) {
        color = vec4<f32>(1.0);
    }

    storageBarrier();

    textureStore(texture, location + vec2<i32>(-1, 0), color);
}