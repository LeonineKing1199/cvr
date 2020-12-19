extern crate cvr;

use cvr::rgb::iter::{LinearToSRGB, SRGBToLinear};

#[test]
fn linear_to_srgb() {
    let r = [1u8, 2, 3];
    let g = [4u8, 5, 6];
    let b = [7u8, 8, 9];

    let mut red_linear = [0f32; 3];
    let mut green_linear = [0f32; 3];
    let mut blue_linear = [0f32; 3];

    let mut red_srgb = [0u8; 3];
    let mut green_srgb = [0u8; 3];
    let mut blue_srgb = [0u8; 3];

    cvr::rgb::Iter::new(&r, &g, &b)
        .srgb_to_linear()
        .enumerate()
        .map(|(idx, [r, g, b])| {
            red_linear[idx] = r;
            green_linear[idx] = g;
            blue_linear[idx] = b;

            [r, g, b]
        })
        .linear_to_srgb()
        .enumerate()
        .for_each(|(idx, [r, g, b])| {
            red_srgb[idx] = r;
            green_srgb[idx] = g;
            blue_srgb[idx] = b;
        });

    assert_eq!(red_linear, [0.000303527, 0.000607054, 0.00091058103]);
    assert_eq!(green_linear, [0.001214108, 0.001517635, 0.0018211621]);
    assert_eq!(blue_linear, [0.002124689, 0.002428216, 0.002731743]);

    assert_eq!(red_srgb, r);
    assert_eq!(green_srgb, g);
    assert_eq!(blue_srgb, b);
}
