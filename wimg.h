#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Image {
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
  uint32_t width;
  uint32_t height;
} Image;

void image_destroy(struct Image *img);

struct Image *resize(struct Image *img, uint32_t new_width, uint32_t new_height);

struct Image *jpeg_decode(uint8_t *ptr, uintptr_t size);

struct Image *jpeg_encode(struct Image *img);
