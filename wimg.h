#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum ImageFormat {
  RGB8 = 1,
  RGBA8,
  JPEG,
  PNG,
  AVIF,
  WEBP,
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

uint64_t hash(uint8_t *ptr, uintptr_t size, uint32_t seed);

void hash(uint8_t *ptr, uintptr_t size, uint32_t seed, uint8_t *out);

uint32_t jpeg_seed(void);

struct Image *jpeg_decode(uint8_t *ptr, uintptr_t size);

struct Image *jpeg_encode(struct Image *img);

uint32_t png_seed(void);

struct Image *png_decode(uint8_t *ptr, uintptr_t size);

struct Image *png_encode(struct Image *img);

uint32_t avif_seed(void);

struct Image *avif_encode(struct Image *img);

uint32_t webp_seed(void);

struct Image *webp_encode(struct Image *img);
