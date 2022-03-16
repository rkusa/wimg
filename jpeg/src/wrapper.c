
#include <stddef.h>

#ifndef WASM
#include <setjmp.h>
#endif

struct jpeg_decompress_struct {
  struct jpeg_error_mgr *err;
};
struct jpeg_compress_struct {
  struct jpeg_error_mgr *err;
};
struct jpeg_common_struct {
  struct jpeg_error_mgr *err;
};

typedef int boolean;
typedef unsigned char JSAMPLE;
typedef JSAMPLE *JSAMPROW;
typedef JSAMPROW *JSAMPARRAY;
typedef unsigned int JDIMENSION;

// error handling related
extern struct jpeg_error_mgr *jpeg_std_error(struct jpeg_error_mgr *err);
extern void jpeg_destroy(struct jpeg_common_struct *cinfo);

// decompress related
extern void jpeg_CreateDecompress(struct jpeg_decompress_struct *dinfo,
                                  int version, size_t structsize);
extern int jpeg_read_header(struct jpeg_decompress_struct *dinfo,
                            int require_image);
extern boolean jpeg_start_decompress(struct jpeg_decompress_struct *dinfo);
extern JDIMENSION jpeg_read_scanlines(struct jpeg_decompress_struct *dinfo,
                                      JSAMPARRAY scanlines,
                                      JDIMENSION max_lines);
extern boolean jpeg_finish_decompress(struct jpeg_decompress_struct *dinfo);
extern void jpeg_destroy_decompress(struct jpeg_decompress_struct *dinfo);

// compress related
extern void jpeg_CreateCompress(struct jpeg_compress_struct *cinfo, int version,
                                size_t structsize);
extern void jpeg_set_defaults(struct jpeg_compress_struct *cinfo);
extern void jpeg_set_quality(struct jpeg_compress_struct *cinfo, int quality,
                             boolean force_baseline);
extern void jpeg_start_compress(struct jpeg_compress_struct *cinfo,
                                boolean write_all_tables);
extern JDIMENSION jpeg_write_scanlines(struct jpeg_compress_struct *cinfo,
                                       JSAMPARRAY scanlines,
                                       JDIMENSION num_lines);
extern void jpeg_finish_compress(struct jpeg_compress_struct *cinfo);
extern void jpeg_destroy_compress(struct jpeg_compress_struct *cinfo);

// memory read and write
extern void jpeg_mem_dest(struct jpeg_compress_struct *cinfo,
                          unsigned char **outbuffer, unsigned long *outsize);
extern void jpeg_mem_src(struct jpeg_decompress_struct *cinfo,
                         const unsigned char *inbuffer, unsigned long insize);

struct jpeg_error_mgr {
  void (*error_exit)(struct jpeg_common_struct *cinfo);
  void (*emit_message)(struct jpeg_common_struct *cinfo, int msg_level);
  void (*output_message)(struct jpeg_common_struct *cinfo);
  void (*format_message)(struct jpeg_common_struct *cinfo, char *buffer);
#define JMSG_LENGTH_MAX 200 /* recommended size of format_message buffer */
  void (*reset_error_mgr)(struct jpeg_common_struct *cinfo);
  int msg_code;
#define JMSG_STR_PARM_MAX 80
  union {
    int i[8];
    char s[JMSG_STR_PARM_MAX];
  } msg_parm;
  int trace_level;
  long num_warnings;
  const char *const *jpeg_message_table;
  int last_jpeg_message;
  const char *const *addon_message_table;
  int first_addon_message;
  int last_addon_message;
};

struct wimg_error_mgr {
  struct jpeg_error_mgr pub;

#ifndef WASM
  jmp_buf setjmp_buffer;
#endif
};

struct result {
  int ok;
  char err[200];
};

struct result result_ok() {
  struct result res;
  res.ok = 1;
  return res;
}

struct result result_err(struct jpeg_common_struct *cinfo) {
  struct result res;
  res.ok = 0;
  (*cinfo->err->format_message)(cinfo, res.err);

  jpeg_destroy(cinfo);

  return res;
}

extern struct result
try_jpeg_CreateDecompress(struct jpeg_decompress_struct *dinfo, int version,
                          size_t structsize) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)dinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)dinfo);
  }
#endif

  jpeg_CreateDecompress(dinfo, version, structsize);

  return result_ok();
}

extern struct result try_jpeg_read_header(struct jpeg_decompress_struct *dinfo,
                                          int require_image) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)dinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)dinfo);
  }
#endif

  jpeg_read_header(dinfo, require_image);

  return result_ok();
}

extern struct result
try_jpeg_start_decompress(struct jpeg_decompress_struct *dinfo) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)dinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)dinfo);
  }
#endif

  jpeg_start_decompress(dinfo);

  return result_ok();
}

extern struct result
try_jpeg_read_scanlines(struct jpeg_decompress_struct *dinfo,
                        JSAMPARRAY scanlines, JDIMENSION max_lines) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)dinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)dinfo);
  }
#endif

  jpeg_read_scanlines(dinfo, scanlines, max_lines);

  return result_ok();
}

extern struct result
try_jpeg_finish_decompress(struct jpeg_decompress_struct *dinfo) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)dinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)dinfo);
  }
#endif

  jpeg_finish_decompress(dinfo);

  return result_ok();
}

extern struct result
try_jpeg_destroy_decompress(struct jpeg_decompress_struct *dinfo) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)dinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)dinfo);
  }
#endif

  jpeg_destroy_decompress(dinfo);

  return result_ok();
}

extern struct result try_jpeg_CreateCompress(struct jpeg_compress_struct *cinfo,
                                             int version, size_t structsize) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)cinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)cinfo);
  }
#endif

  jpeg_CreateCompress(cinfo, version, structsize);

  return result_ok();
}

extern struct result try_jpeg_set_defaults(struct jpeg_compress_struct *cinfo) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)cinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)cinfo);
  }
#endif

  jpeg_set_defaults(cinfo);

  return result_ok();
}

extern struct result try_jpeg_set_quality(struct jpeg_compress_struct *cinfo,
                                          int quality, boolean force_baseline) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)cinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)cinfo);
  }
#endif

  jpeg_set_quality(cinfo, quality, force_baseline);

  return result_ok();
}

extern struct result try_jpeg_start_compress(struct jpeg_compress_struct *cinfo,
                                             boolean write_all_tables) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)cinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)cinfo);
  }
#endif

  jpeg_start_compress(cinfo, write_all_tables);

  return result_ok();
}

extern struct result
try_jpeg_write_scanlines(struct jpeg_compress_struct *cinfo,
                         JSAMPARRAY scanlines, JDIMENSION num_lines) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)cinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)cinfo);
  }
#endif

  jpeg_write_scanlines(cinfo, scanlines, num_lines);

  return result_ok();
}

extern struct result
try_jpeg_finish_compress(struct jpeg_compress_struct *cinfo) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)cinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)cinfo);
  }
#endif

  jpeg_finish_compress(cinfo);

  return result_ok();
}

extern struct result
try_jpeg_destroy_compress(struct jpeg_compress_struct *cinfo) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)cinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)cinfo);
  }
#endif

  jpeg_destroy_compress(cinfo);

  return result_ok();
}

extern struct result try_jpeg_mem_dest(struct jpeg_compress_struct *cinfo,
                                       unsigned char **outbuffer,
                                       unsigned long *outsize) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)cinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)cinfo);
  }
#endif

  jpeg_mem_dest(cinfo, outbuffer, outsize);

  return result_ok();
}

extern struct result try_jpeg_mem_src(struct jpeg_decompress_struct *cinfo,
                                      const unsigned char *inbuffer,
                                      unsigned long insize) {
#ifndef WASM
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)cinfo->err;
  if (setjmp(err->setjmp_buffer)) {
    return result_err((struct jpeg_common_struct *)cinfo);
  }
#endif

  jpeg_mem_src(cinfo, inbuffer, insize);

  return result_ok();
}

#ifndef WASM
void error_exit(struct jpeg_common_struct *cinfo) {
  struct wimg_error_mgr *err = (struct wimg_error_mgr *)cinfo->err;
  longjmp(err->setjmp_buffer, 1);
}

extern struct jpeg_error_mgr *throwing_error_mgr(struct wimg_error_mgr *err) {
  jpeg_std_error(&err->pub);
  err->pub.error_exit = error_exit;

  return &err->pub;
}
#endif
