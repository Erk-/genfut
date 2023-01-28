#include <stdio.h>

#include "matmul.h"

int main() {
  int a[] = { 1, 2, 3, 4 };
  int b[] = { 2, 3, 4, 1 };

  struct futhark_context_config *cfg = futhark_context_config_new();
  struct futhark_context *ctx = futhark_context_new(cfg);

  struct futhark_i32_2d *a_arr = futhark_new_i32_2d(ctx, a, 2, 2);
  struct futhark_i32_2d *b_arr = futhark_new_i32_2d(ctx, b, 2, 2);

  struct futhark_i32_2d *res_arr = NULL;

  futhark_entry_matmul(ctx, &res_arr, a_arr, b_arr);
  futhark_context_sync(ctx);

  int res[4];
  futhark_values_i32_2d(ctx, res_arr, res);

  for(int i = 0; i < 4; i++) {
      printf("%d ", res[i]);
      if (i == 1) {
        printf("\n");
      }
  }
  printf("\n");

  futhark_free_i32_2d(ctx, a_arr);
  futhark_free_i32_2d(ctx, b_arr);
  futhark_free_i32_2d(ctx, res_arr);
  
  futhark_context_free(ctx);
  futhark_context_config_free(cfg);
}
