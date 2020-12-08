extern crate cvr;

use cvr::Greyscale;

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

    let img_iter = rgb_img.iter();
    let pixels = <Vec<_> as std::iter::FromIterator<(u8, u8, u8)>>::from_iter(img_iter);

    assert_eq!(pixels.len(), height * width);

    for (idx, (r, g, b)) in std::iter::successors(Some((1u8, 2u8, 3u8)), |(x, y, z)| {
        Some((*x + 3, *y + 3, *z + 3))
    })
    .take(height * width)
    .enumerate()
    {
        assert_eq!(pixels[idx], (r, g, b));
    }

    let grey: Vec<_> = rgb_img.iter().greyscale().collect();
    assert_eq!(grey, [1, 4, 7, 10, 13, 16, 19, 22, 25]);
}
