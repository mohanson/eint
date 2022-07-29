#include <stddef.h>
#include <stdint.h>

typedef __uint128_t uint128_t;
typedef __int128_t int128_t;

// HAC, 14.12 Algorithm Multiple-precision multiplication
// SEW-bit*SEW-bit multiply to generate a 2*SEW-bit product
void c_widening_mul_u(uint64_t *w, uint64_t *x, uint64_t *y,
                       size_t digits_count) {
  for (size_t i = 0; i < 2 * digits_count; i++) {
    w[i] = 0;
  }
  for (size_t i = 0; i < digits_count; i++) {
    uint64_t c = 0;
    for (size_t j = 0; j < digits_count; j++) {
      uint128_t uv = ((uint128_t)x[j]) * y[i] + w[i + j] + c;
      w[i + j] = (uint64_t)uv;
      c = *((uint64_t *)&uv + 1);
    }
    w[i + digits_count] = c;
  }
}

// SEW-bit*SEW-bit multiply to generate a SEW-bit product
void c_wrapping_mul(uint64_t *w, uint64_t *x, uint64_t *y, size_t digits_count) {
  for (size_t i = 0; i < digits_count; i++) {
    w[i] = 0;
  }
  for (size_t i = 0; i < digits_count; i++) {
    uint64_t c = 0;
    size_t inner_count = digits_count - i;
    for (size_t j = 0; j < inner_count; j++) {
      uint128_t uv = ((uint128_t)x[j]) * y[i] + w[i + j] + c;
      w[i + j] = (uint64_t)uv;
      c = *((uint64_t *)&uv + 1);
    }
    if ((i + inner_count) < digits_count) {
      w[i + inner_count] = c;
    }
  }
}

// SEW-bit*SEW-bit addition to generate a SEW-bit result with overflowing flag
uint64_t c_overflowing_add(uint64_t *w, uint64_t *x, uint64_t *y, size_t digits_count) {
  uint64_t c = 0;
  for (size_t i = 0; i < digits_count; i++) {
    uint128_t temp = (uint128_t)x[i] + (uint128_t)y[i] + c;
    w[i] = (uint64_t)temp;
    c = *((uint64_t *)&temp + 1);
  }
  return c;
}

// SEW-bit*SEW-bit subtraction to generate a SEW-bit result with borrowing flag
uint64_t c_overflowing_sub(uint64_t *w, uint64_t *x, uint64_t *y, size_t digits_count) {
  int64_t c = 0;
  for (size_t i = 0; i < digits_count; i++) {
    int128_t temp = (int128_t)x[i] - (int128_t)y[i] + c;
    w[i] = (uint64_t)temp;
    c = *((int64_t *)&temp + 1);
  }
  return (uint64_t)c;
}

// we should specify digits_count in C code to enable optimization from
// compiler. so don't export function eint_widening_mul to Rust code directly.
void c_widening_mul_u_1024_batch(uint64_t *w, uint64_t *x, uint64_t *y,
                                  size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    c_widening_mul_u(w, x, y, 16);
    w += 32;
    x += 16;
    y += 16;
  }
}

void c_widening_mul_u_512_batch(uint64_t *w, uint64_t *x, uint64_t *y,
                                 size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    c_widening_mul_u(w, x, y, 8);
    w += 16;
    x += 8;
    y += 8;
  }
}

void c_widening_mul_u_256_batch(uint64_t *w, uint64_t *x, uint64_t *y,
                                 size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    c_widening_mul_u(w, x, y, 4);
    w += 8;
    x += 4;
    y += 4;
  }
}

void c_widening_mul_u_128_batch(uint64_t *w, uint64_t *x, uint64_t *y,
                                 size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    c_widening_mul_u(w, x, y, 2);
    w += 4;
    x += 2;
    y += 2;
  }
}

void c_wrapping_mul_1024_batch(uint64_t *w, uint64_t *x, uint64_t *y, size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    c_wrapping_mul(w, x, y, 16);
    w += 16;
    x += 16;
    y += 16;
  }
}

void c_wrapping_mul_512_batch(uint64_t *w, uint64_t *x, uint64_t *y, size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    c_wrapping_mul(w, x, y, 8);
    w += 8;
    x += 8;
    y += 8;
  }
}

void c_wrapping_mul_256_batch(uint64_t *w, uint64_t *x, uint64_t *y, size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    c_wrapping_mul(w, x, y, 4);
    w += 4;
    x += 4;
    y += 4;
  }
}

void c_wrapping_mul_128_batch(uint64_t *w, uint64_t *x, uint64_t *y, size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    c_wrapping_mul(w, x, y, 2);
    w += 2;
    x += 2;
    y += 2;
  }
}
