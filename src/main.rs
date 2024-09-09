pub mod mandel;

fn main() {
    const HEIGHT: i32 = 250 * 8;
    const WIDTH: i32 = 312 * 8;

    const X1: f64 = -2.0;
    const Y1: f64 = 2.0;
    const X2: f64 = 2.0;
    const Y2: f64 = -2.0;

    mandel::generate_image(
        X1,
        Y1,
        X2,
        Y2,
        WIDTH,
        HEIGHT,
    );
}
