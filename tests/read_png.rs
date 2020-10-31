extern crate cvr;

#[test]
fn cvr_png_swap_channels() {
    let img = cvr::png::read_rgb8(std::fs::File::open("tests/images/parrot.png").unwrap()).unwrap();

    cvr::png::write_rgb8(
        &img,
        std::fs::File::create("tests/images/parrot-copy.png").unwrap(),
    )
    .unwrap();
}
