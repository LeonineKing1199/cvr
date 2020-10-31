#![warn(clippy::pedantic)]

extern crate minivec;

pub mod png;

pub struct RgbImg {
    r: minivec::MiniVec<u8>,
    g: minivec::MiniVec<u8>,
    b: minivec::MiniVec<u8>,
    height: u32,
    width: u32,
}

impl RgbImg {
    pub fn new() -> RgbImg {
        RgbImg {
            r: minivec::MiniVec::new(),
            g: minivec::MiniVec::new(),
            b: minivec::MiniVec::new(),
            height: 0,
            width: 0,
        }
    }

    pub fn from_packed_buf(buf: &[u8], height: u32, width: u32) -> RgbImg {
        let total = height as usize * width as usize;

        let mut r = minivec::MiniVec::<u8>::with_capacity(total);
        let mut g = minivec::MiniVec::<u8>::with_capacity(total);
        let mut b = minivec::MiniVec::<u8>::with_capacity(total);

        let r_ptr = r.as_mut_ptr();
        let g_ptr = g.as_mut_ptr();
        let b_ptr = b.as_mut_ptr();

        buf.chunks_exact(3)
            .enumerate()
            .for_each(|(idx, pixel)| -> () {
                unsafe {
                    r_ptr.add(idx).write(pixel[0]);
                    g_ptr.add(idx).write(pixel[1]);
                    b_ptr.add(idx).write(pixel[2]);
                }
            });

        unsafe {
            r.set_len(total);
            g.set_len(total);
            b.set_len(total);
        }

        Self {
            r,
            g,
            b,
            height,
            width,
        }
    }

    pub fn to_packed_buf(&self) -> Vec<u8> {
        let r = self.r();
        let g = self.g();
        let b = self.b();

        let mut vec = vec![std::mem::MaybeUninit::<u8>::uninit(); self.total() as usize * 3];

        let ptr = vec.as_mut_ptr();

        unsafe {
            for idx in 0..self.total() {
                ptr.add(idx as usize * 3 as usize + 0)
                    .write(std::mem::MaybeUninit::<u8>::new(r[idx as usize]));

                ptr.add(idx as usize * 3 as usize + 1)
                    .write(std::mem::MaybeUninit::<u8>::new(g[idx as usize]));

                ptr.add(idx as usize * 3 as usize + 2)
                    .write(std::mem::MaybeUninit::<u8>::new(b[idx as usize]));
            }
        }

        unsafe { std::mem::transmute::<_, Vec<u8>>(vec) }
    }

    pub fn r(&self) -> &[u8] {
        &self.r
    }

    pub fn g(&self) -> &[u8] {
        &self.g
    }

    pub fn b(&self) -> &[u8] {
        &self.b
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn total(&self) -> u32 {
        self.height() * self.width()
    }
}
