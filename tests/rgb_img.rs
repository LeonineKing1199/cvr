extern crate cvr;

#[test]
fn rgb_img_from_packed_buf() {
    let buf = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let height = 1;
    let width = 3;

    let rgb_img = cvr::RgbImg::from_packed_buf(&buf, height, width);

    assert_eq!(rgb_img.r(), [1, 4, 7]);
    assert_eq!(rgb_img.g(), [2, 5, 8]);
    assert_eq!(rgb_img.b(), [3, 6, 9]);

    assert_eq!(rgb_img.height(), height);
    assert_eq!(rgb_img.width(), width);
}

#[test]
fn rgb_img_to_packed_buf() {
    let buf = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let height = 1;
    let width = 3;

    let rgb_img = cvr::RgbImg::from_packed_buf(&buf, height, width);

    let packed = rgb_img.to_packed_buf();

    assert_eq!(packed, [1u8, 2, 3, 4, 5, 6, 7, 8, 9]);
}
