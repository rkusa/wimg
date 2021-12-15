#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum ImageFormat {
  RGB8 = 1,
  RGBA8 = 2,
  JPEG = 3,
  PNG = 4,
};
typedef uint8_t ImageFormat;

typedef struct Image {
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
  ImageFormat format;
  uint32_t width;
  uint32_t height;
} Image;

char *last_error_message(void);

void error_message_destroy(char *s);

void image_destroy(struct Image *img);

struct Image *resize(struct Image *img, uint32_t new_width, uint32_t new_height);

uint64_t hash(struct Image *img);

void hash(struct Image *img, uint8_t *out);

struct Image *jpeg_decode(uint8_t *ptr, uintptr_t size);

struct Image *jpeg_encode(struct Image *img);

struct Image *png_decode(uint8_t *ptr, uintptr_t size);

struct Image *png_encode(struct Image *img);
