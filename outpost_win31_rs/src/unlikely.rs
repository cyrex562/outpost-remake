// #include "types.h"
// #include "structs.h"
// #include "globals.h"
// #include "sys_api.h"
// #include "block_1000.h"
// #include "utils.h"
// #include "structs_2.h"
// #include "func_ptrs.h"
// #include "entry.h"
// #include <stdbool.h>
// #include <stdio.h>
pub unsafe fn pass1_1000_0000(mut param_1: *mut u16, mut param_2: *mut u16, mut param_3: u16) {
    let mut pu_var1: *mut u16;
    let mut pu_var2: *mut u16;
    let mut u_var3: u16;

    // for (u_var3 = param_3 >> 0x1; u_var3 != 0; u_var3 -= 1) {
    u_var3 = param_3 >> 0x1;
    while u_var3 != 0 {
        pu_var2 = param_2;
        param_2 = param_2 + 1;
        pu_var1 = param_1;
        param_1 = param_1 + 1;
        *pu_var2 = *pu_var1;
        u_var3 -= 1;
    }
}
