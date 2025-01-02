use noise::{utils::*, Checkerboard, OpenSimplex, Worley};
pub fn write_example_to_file(
    map: &NoiseMap,
    filename: &str,
) {
    map.write_to_file(&filename)
}

const SIZE: (u32, u32) = (3840, 2160);

// fn main() {
//     let checker = Checkerboard::new(0);

//     write_example_to_file(
//         &PlaneMapBuilder::<Checkerboard, 5>::new(checker)
//             .set_size(SIZE.0 as usize, SIZE.1 as usize)
//             .set_x_bounds(-20.0, 20.0)
//             .set_y_bounds(-20.0, 20.0)
//             // .set_x_bounds(
//             //     -(SIZE.0 as f64) / 2.,
//             //     SIZE.0 as f64,
//             // )
//             // .set_y_bounds(
//             //     -(SIZE.1 as f64) / 2.,
//             //     SIZE.1 as f64,
//             // )
//             .build(),
//         "checkerboard.png",
//     );
// }

// fn main() {
//     let noise_gen = OpenSimplex::new(0);
//     write_example_to_file(
//         &PlaneMapBuilder::<OpenSimplex, 3>::new(noise_gen)
//             .set_size(SIZE.0 as usize, SIZE.1 as usize)
//             .set_x_bounds(-20.0, 20.0)
//             .set_y_bounds(-20.0, 20.0)
//             .build(),
//         "open_simplex.png",
//     );
// }

fn main() {
    let noise_gen = Worley::new(0);
    write_example_to_file(
        &PlaneMapBuilder::<Worley, 3>::new(Worley::new(0))
            .set_size(SIZE.0 as usize, SIZE.1 as usize)
            .set_x_bounds(-5.0, 5.0)
            // .set_y_bounds(-20.0, 20.0)
            .build(),
        "open_simplex.png",
    );
}
