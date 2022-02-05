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

typedef struct Context Context;

typedef struct Image {
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
  ImageFormat format;
  uint32_t width;
  uint32_t height;
} Image;

struct Context *context_new(void);

void context_drop(struct Context *img);

char *last_error_message(struct Context *ctx);

void error_message_drop(char *s);

struct Image *image_new(void);

void image_drop(struct Image *img);

int32_t resize(struct Context *ctx,
               struct Image *img,
               uint32_t new_width,
               uint32_t new_height,
               bool maintain_aspect,
               struct Image *out);

uint64_t hash(uint8_t *ptr, uintptr_t size, uint32_t seed);

uint32_t jpeg_seed(void);

int32_t jpeg_decode(struct Context *ctx, uint8_t *ptr, uintptr_t size, struct Image *out);

int32_t jpeg_encode(struct Context *ctx, struct Image *img, struct Image *out);

void jpeg_set_encode_quality(struct Context *ctx, uint16_t quality);

uint32_t png_seed(void);

int32_t png_decode(struct Context *ctx, uint8_t *ptr, uintptr_t size, struct Image *out);

int32_t png_encode(struct Context *ctx, struct Image *img, struct Image *out);

uint32_t avif_seed(void);

int32_t avif_encode(struct Context *ctx, struct Image *img, struct Image *out);

void avif_set_encode_quality(struct Context *ctx, uint16_t quality);

void avif_set_encode_speed(struct Context *ctx, uint8_t speed);

uint32_t webp_seed(void);

int32_t webp_encode(struct Context *ctx, struct Image *img, struct Image *out);

void webp_set_encode_quality(struct Context *ctx, uint16_t quality);
