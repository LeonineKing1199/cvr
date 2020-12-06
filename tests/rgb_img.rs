extern crate cvr;

#[test]
fn rgb_img_packed() {
    let height = 3;
    let width = 3;
    let num_channels = 3;

    let total = height * width * num_channels;

    assert_eq!(total, 27);

    let buf: Vec<u8> = std::iter::successors(Some(1u8), |x| Some(x + 1))
        .take(total)
        .collect();

    let rgb_img = cvr::RgbImg::from_packed_buf(&buf, height, width);

    assert_eq!(rgb_img.r().len(), height * width);
    assert_eq!(rgb_img.g().len(), height * width);
    assert_eq!(rgb_img.b().len(), height * width);

    assert_eq!(rgb_img.r(), [1, 4, 7, 10, 13, 16, 19, 22, 25]);
    assert_eq!(rgb_img.g(), [2, 5, 8, 11, 14, 17, 20, 23, 26]);
    assert_eq!(rgb_img.b(), [3, 6, 9, 12, 15, 18, 21, 24, 27]);

    assert_eq!(rgb_img.height(), height);
    assert_eq!(rgb_img.width(), width);

    let packed = rgb_img.to_packed_buf();
    assert_eq!(packed, buf);
}
