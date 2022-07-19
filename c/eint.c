#include <stddef.h>
#include <stdint.h>

typedef __uint128_t uint128_t;

// HAC, 14.12 Algorithm Multiple-precision multiplication
// SEW-bit*SEW-bit multiply to generate a 2*SEW-bit product
void eint_widening_mul(uint64_t *w, uint64_t *x, uint64_t *y,
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
void eint_mul(uint64_t *w, uint64_t *x, uint64_t *y, size_t digits_count) {
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

// we should specify digits_count in C code to enable optimization from
// compiler. so don't export function eint_widening_mul to Rust code directly.
void eint_widening_mul_1024_batch(uint64_t *w, uint64_t *x, uint64_t *y,
                                  size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    eint_widening_mul(w, x, y, 16);
    w += 32;
    x += 16;
    y += 16;
  }
}

void eint_widening_mul_512_batch(uint64_t *w, uint64_t *x, uint64_t *y,
                                 size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    eint_widening_mul(w, x, y, 8);
    w += 16;
    x += 8;
    y += 8;
  }
}

void eint_widening_mul_256_batch(uint64_t *w, uint64_t *x, uint64_t *y,
                                 size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    eint_widening_mul(w, x, y, 4);
    w += 8;
    x += 4;
    y += 4;
  }
}

void eint_widening_mul_128_batch(uint64_t *w, uint64_t *x, uint64_t *y,
                                 size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    eint_widening_mul(w, x, y, 2);
    w += 4;
    x += 2;
    y += 2;
  }
}

void eint_mul_1024_batch(uint64_t *w, uint64_t *x, uint64_t *y, size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    eint_mul(w, x, y, 16);
    w += 16;
    x += 16;
    y += 16;
  }
}

void eint_mul_512_batch(uint64_t *w, uint64_t *x, uint64_t *y, size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    eint_mul(w, x, y, 8);
    w += 8;
    x += 8;
    y += 8;
  }
}

void eint_mul_256_batch(uint64_t *w, uint64_t *x, uint64_t *y, size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    eint_mul(w, x, y, 4);
    w += 4;
    x += 4;
    y += 4;
  }
}

void eint_mul_128_batch(uint64_t *w, uint64_t *x, uint64_t *y, size_t batch) {
  for (size_t i = 0; i < batch; i++) {
    eint_mul(w, x, y, 2);
    w += 2;
    x += 2;
    y += 2;
  }
}
