#include <stddef.h>

struct jpeg_decompress_struct;
struct jpeg_compress_struct;
struct jpeg_common_struct {
  struct jpeg_error_mgr *err;
};

typedef int boolean;
typedef unsigned char JSAMPLE;
typedef JSAMPLE *JSAMPROW;
typedef JSAMPROW *JSAMPARRAY;
typedef unsigned int JDIMENSION;

// error handling related
extern "C" struct jpeg_error_mgr *jpeg_std_error(struct jpeg_error_mgr *err);
extern "C" void jpeg_destroy(jpeg_common_struct *cinfo);

// decompress related
extern "C" void jpeg_CreateDecompress(jpeg_decompress_struct *dinfo,
                                      int version, size_t structsize);
extern "C" int jpeg_read_header(jpeg_decompress_struct *dinfo,
                                int require_image);
extern "C" boolean jpeg_start_decompress(jpeg_decompress_struct *dinfo);
extern "C" JDIMENSION jpeg_read_scanlines(jpeg_decompress_struct *dinfo,
                                          JSAMPARRAY scanlines,
                                          JDIMENSION max_lines);
extern "C" boolean jpeg_finish_decompress(jpeg_decompress_struct *dinfo);
extern "C" void jpeg_destroy_decompress(jpeg_decompress_struct *dinfo);

// compress related
extern "C" void jpeg_CreateCompress(jpeg_compress_struct *cinfo, int version,
                                    size_t structsize);
extern "C" void jpeg_set_defaults(jpeg_compress_struct *cinfo);
extern "C" void jpeg_set_quality(jpeg_compress_struct *cinfo, int quality,
                                 boolean force_baseline);
extern "C" void jpeg_start_compress(jpeg_compress_struct *cinfo,
                                    boolean write_all_tables);
extern "C" JDIMENSION jpeg_write_scanlines(jpeg_compress_struct *cinfo,
                                           JSAMPARRAY scanlines,
                                           JDIMENSION num_lines);
extern "C" void jpeg_finish_compress(jpeg_compress_struct *cinfo);
extern "C" void jpeg_destroy_compress(jpeg_compress_struct *cinfo);

// memory read and write
extern "C" void jpeg_mem_dest(jpeg_compress_struct *cinfo,
                              unsigned char **outbuffer,
                              unsigned long *outsize);
extern "C" void jpeg_mem_src(jpeg_decompress_struct *cinfo,
                             const unsigned char *inbuffer,
                             unsigned long insize);

struct jpeg_error_mgr {
  void (*error_exit)(jpeg_common_struct *cinfo);
  void (*emit_message)(jpeg_common_struct *cinfo, int msg_level);
  void (*output_message)(jpeg_common_struct *cinfo);
  void (*format_message)(jpeg_common_struct *cinfo, char *buffer);
};

struct result {
  bool ok;
  char err[200];
};

result ok() {
  result res;
  res.ok = true;
  return res;
}

extern "C" result try_jpeg_CreateDecompress(jpeg_decompress_struct *dinfo,
                                            int version, size_t structsize) {
  try {
    jpeg_CreateDecompress(dinfo, version, structsize);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_read_header(jpeg_decompress_struct *dinfo,
                                       int require_image) {
  try {
    jpeg_read_header(dinfo, require_image);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_start_decompress(jpeg_decompress_struct *dinfo) {
  try {
    jpeg_start_decompress(dinfo);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_read_scanlines(jpeg_decompress_struct *dinfo,
                                          JSAMPARRAY scanlines,
                                          JDIMENSION max_lines) {
  try {
    jpeg_read_scanlines(dinfo, scanlines, max_lines);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_finish_decompress(jpeg_decompress_struct *dinfo) {
  try {
    jpeg_finish_decompress(dinfo);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_destroy_decompress(jpeg_decompress_struct *dinfo) {
  try {
    jpeg_destroy_decompress(dinfo);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_CreateCompress(jpeg_compress_struct *cinfo,
                                          int version, size_t structsize) {
  try {
    jpeg_CreateCompress(cinfo, version, structsize);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_set_defaults(jpeg_compress_struct *cinfo) {
  try {
    jpeg_set_defaults(cinfo);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_set_quality(jpeg_compress_struct *cinfo, int quality,
                                       boolean force_baseline) {
  try {
    jpeg_set_quality(cinfo, quality, force_baseline);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_start_compress(jpeg_compress_struct *cinfo,
                                          boolean write_all_tables) {
  try {
    jpeg_start_compress(cinfo, write_all_tables);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_write_scanlines(jpeg_compress_struct *cinfo,
                                           JSAMPARRAY scanlines,
                                           JDIMENSION num_lines) {
  try {
    jpeg_write_scanlines(cinfo, scanlines, num_lines);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_finish_compress(jpeg_compress_struct *cinfo) {
  try {
    jpeg_finish_compress(cinfo);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_destroy_compress(jpeg_compress_struct *cinfo) {
  try {
    jpeg_destroy_compress(cinfo);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_mem_dest(jpeg_compress_struct *cinfo,
                                    unsigned char **outbuffer,
                                    unsigned long *outsize) {
  try {
    jpeg_mem_dest(cinfo, outbuffer, outsize);
    return ok();
  } catch (result &res) {
    return res;
  }
}

extern "C" result try_jpeg_mem_src(jpeg_decompress_struct *cinfo,
                                   const unsigned char *inbuffer,
                                   unsigned long insize) {
  try {
    jpeg_mem_src(cinfo, inbuffer, insize);
    return ok();
  } catch (result &res) {
    return res;
  }
}

void error_exit(jpeg_common_struct *cinfo) {
  result res;
  res.ok = false;
  (*cinfo->err->format_message)(cinfo, res.err);

  jpeg_destroy(cinfo);
  throw res;
}

extern "C" struct jpeg_error_mgr *
throwing_error_mgr(struct jpeg_error_mgr *err) {
  jpeg_std_error(err);
  err->error_exit = error_exit;
  return err;
}
