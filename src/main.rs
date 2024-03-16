use std::ops::{Add, Sub, Mul, Div};
use log::info;

fn map_range<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T
    where T: Add<T, Output=T> +
    Sub<T, Output=T> +
    Mul<T, Output=T> +
    Div<T, Output=T>
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn main() {
    env_logger::init();

    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        info!("remaining: {} ", image_height - j);
        for i in 0..image_width {
            let r = map_range((0, image_width - 1), (0, 255), i);
            let g = map_range((0, image_height - 1), (0, 255), j);
            let b = 0;

            println!("{r} {g} {b}");
        }
    }
    info!("Done.")
}
