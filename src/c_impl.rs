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

    pub fn eint_add(w: *mut u64, x: *const u64, y: *const u64, digits_count: usize) -> u64;
    pub fn eint_sub(w: *mut u64, x: *const u64, y: *const u64, digits_count: usize) -> u64;
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

pub fn add_128(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
    unsafe {
        eint_add(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            2,
        )
    }
}

pub fn add_256(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
    unsafe {
        eint_add(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            4,
        )
    }
}

pub fn add_512(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
    unsafe {
        eint_add(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            8,
        )
    }
}

pub fn add_1024(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
    unsafe {
        eint_add(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            16,
        )
    }
}

pub fn sub_128(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
    unsafe {
        eint_sub(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            2,
        )
    }
}

pub fn sub_256(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
    unsafe {
        eint_sub(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            4,
        )
    }
}

pub fn sub_512(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
    unsafe {
        eint_sub(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            8,
        )
    }
}

pub fn sub_1024(w: &mut [u8], x: &[u8], y: &[u8]) -> u64 {
    unsafe {
        eint_sub(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            16,
        )
    }
}
