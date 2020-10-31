extern crate png;

#[derive(Debug)]
pub enum Error {
    Decoding(::png::DecodingError),
    InvalidColorType(::png::ColorType),
    InvalidBitDepth(::png::BitDepth),
    Encoding(::png::EncodingError),
}

impl std::convert::From<::png::DecodingError> for Error {
    fn from(err: ::png::DecodingError) -> Error {
        Error::Decoding(err)
    }
}

impl std::convert::From<::png::EncodingError> for Error {
    fn from(err: ::png::EncodingError) -> Error {
        Error::Encoding(err)
    }
}

pub fn read_rgb8<Reader: std::io::Read>(reader: Reader) -> Result<crate::RgbImg, Error> {
    let mut decoder = ::png::Decoder::new(reader);

    let mut transformations = ::png::Transformations::IDENTITY;
    transformations.set(::png::Transformations::STRIP_ALPHA, true);

    decoder.set_transformations(transformations);

    let (info, mut png_reader) = decoder.read_info()?;

    let ::png::OutputInfo { color_type, .. } = info;
    if color_type != ::png::ColorType::RGB && color_type != ::png::ColorType::RGBA {
        return Err(Error::InvalidColorType(color_type));
    }

    if info.bit_depth != ::png::BitDepth::Eight {
        return Err(Error::InvalidBitDepth(info.bit_depth));
    }

    let height = info.height as usize;
    let width = info.width as usize;

    let (mut r, mut g, mut b) = (
        minivec::MiniVec::<u8>::with_capacity(3 * height * width),
        minivec::MiniVec::<u8>::with_capacity(3 * height * width),
        minivec::MiniVec::<u8>::with_capacity(3 * height * width),
    );

    let r_ptr = r.as_mut_ptr();
    let g_ptr = g.as_mut_ptr();
    let b_ptr = b.as_mut_ptr();

    let num_channels = if color_type == ::png::ColorType::RGBA {
        4
    } else {
        3
    };

    let num_rows = height;
    let num_cols = width;

    for row_idx in 0..num_rows {
        let row = png_reader.next_row()?.unwrap();

        for col_idx in 0..num_cols {
            let write_offset = row_idx * num_cols + col_idx;
            let read_offset = num_channels * col_idx;

            unsafe {
                r_ptr.add(write_offset).write(row[read_offset + 0]);
                g_ptr.add(write_offset).write(row[read_offset + 1]);
                b_ptr.add(write_offset).write(row[read_offset + 2]);
            };
        }
    }

    let total = height * width;
    unsafe {
        r.set_len(total);
        g.set_len(total);
        b.set_len(total);
    };

    let rgb_img = crate::RgbImg {
        r,
        g,
        b,
        height: height as u32,
        width: width as u32,
    };

    Ok(rgb_img)
}

pub fn write_rgb8<Writer: std::io::Write>(
    img: &crate::RgbImg,
    writer: Writer,
) -> Result<(), Error> {
    let height = img.height();
    let width = img.width();
    let mut png_encoder = png::Encoder::new(writer, width, height);
    png_encoder.set_color(::png::ColorType::RGB);
    png_encoder.set_depth(::png::BitDepth::Eight);

    let mut png_writer = png_encoder.write_header()?;
    png_writer.write_image_data(&img.to_packed_buf())?;

    Ok(())
}
