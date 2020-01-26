#include <stdio.h>
#include <unistd.h>

#include "cst_wave.h"
#include "flite.h"

cst_voice *register_cmu_us_kal(const char *voxdir);

const int IOVEC_LEN = 1000;
static char IOVEC[IOVEC_LEN] = {0};

void compute(__wasi_fd_t in, __wasi_fd_t out) {
  struct __wasi_iovec_t iovec_st = {&IOVEC, IOVEC_LEN};
  const struct __wasi_iovec_t *iovecs = {&iovec_st};
  __wasi_size_t nread;
  __wasi_errno_t res;
  res = __wasi_fd_read(in, iovecs, 1, &nread);

  if (res != 0) {
    return;
  }

  cst_voice *voice = NULL;
  cst_wave *output = NULL;

  flite_init();
  voice = register_cmu_us_kal(NULL);
  output = flite_text_to_wave(&IOVEC, voice);
  cst_wave_save_riff_wasi_fd(output, out);
}

__wasi_size_t cst_wasi_fwrite(__wasi_fd_t fd, const void *buf, long size,
                              long count) {
  struct __wasi_ciovec_t ciovec_st = {.buf = buf, .buf_len = size * count};
  const struct __wasi_ciovec_t *ciovecs = {&ciovec_st};
  __wasi_size_t nwritten;
  __wasi_errno_t res;
  res = __wasi_fd_write(fd, ciovecs, 1, &nwritten);
  if (res != 0)
    return 0;
  return nwritten;
}

int cst_wave_save_riff_wasi_fd(cst_wave *w, __wasi_fd_t fd) {
  const char *info;
  short d_short;
  int d_int, n;
  int num_bytes;

  info = "RIFF";
  cst_wasi_fwrite(fd, info, 4, 1);

  num_bytes =
      (cst_wave_num_samples(w) * cst_wave_num_channels(w) * sizeof(short)) + 8 +
      16 + 12;

  cst_wasi_fwrite(fd, &num_bytes, 4, 1); /* num bytes in whole file */

  info = "WAVE";
  cst_wasi_fwrite(fd, info, 1, 4);

  info = "fmt ";
  cst_wasi_fwrite(fd, info, 1, 4);

  num_bytes = 16; /* size of header */
  cst_wasi_fwrite(fd, &num_bytes, 4, 1);

  d_short = RIFF_FORMAT_PCM; /* sample type */
  cst_wasi_fwrite(fd, &d_short, 2, 1);

  d_short = cst_wave_num_channels(w); /* number of channels */
  cst_wasi_fwrite(fd, &d_short, 2, 1);

  d_int = cst_wave_sample_rate(w); /* sample rate */
  cst_wasi_fwrite(fd, &d_int, 4, 1);

  d_int = (cst_wave_sample_rate(w) * cst_wave_num_channels(w) *
           sizeof(short)); /* average bytes per second */
  cst_wasi_fwrite(fd, &d_int, 4, 1);

  d_short = (cst_wave_num_channels(w) * sizeof(short)); /* block align */
  cst_wasi_fwrite(fd, &d_short, 2, 1);

  d_short = 2 * 8; /* bits per sample */
  cst_wasi_fwrite(fd, &d_short, 2, 1);

  info = "data";
  cst_wasi_fwrite(fd, info, 1, 4);

  d_int = (cst_wave_num_channels(w) * cst_wave_num_samples(w) *
           sizeof(short)); /* bytes in data */
  cst_wasi_fwrite(fd, &d_int, 4, 1);

  n = cst_wasi_fwrite(fd, cst_wave_samples(w), sizeof(short),
                      cst_wave_num_channels(w) * cst_wave_num_samples(w));

  if (n != cst_wave_num_channels(w) * cst_wave_num_samples(w))
    return -1;
  else
    return 0;
}
