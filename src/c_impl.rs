#[link(name = "eint-c-impl", kind = "static")]
extern "C" {
    pub fn eint_widening_mul_128_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
    pub fn eint_widening_mul_256_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
    pub fn eint_widening_mul_512_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
    pub fn eint_widening_mul_1024_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);

    pub fn eint_mul_128_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
    pub fn eint_mul_256_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
    pub fn eint_mul_512_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
    pub fn eint_mul_1024_batch(w: *mut u64, x: *const u64, y: *const u64, batch: usize);
}

pub fn widening_mul_128(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
    unsafe {
        eint_widening_mul_128_batch(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            batch,
        );
    }
}

pub fn widening_mul_256(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
    unsafe {
        eint_widening_mul_256_batch(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            batch,
        );
    }
}

pub fn widening_mul_512(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
    unsafe {
        eint_widening_mul_512_batch(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            batch,
        );
    }
}

pub fn widening_mul_1024(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
    unsafe {
        eint_widening_mul_1024_batch(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            batch,
        );
    }
}

pub fn mul_128(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
    unsafe {
        eint_mul_128_batch(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            batch,
        );
    }
}

pub fn mul_256(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
    unsafe {
        eint_mul_256_batch(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            batch,
        );
    }
}

pub fn mul_512(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
    unsafe {
        eint_mul_512_batch(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            batch,
        );
    }
}

pub fn mul_1024(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
    unsafe {
        eint_mul_1024_batch(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            batch,
        );
    }
}
