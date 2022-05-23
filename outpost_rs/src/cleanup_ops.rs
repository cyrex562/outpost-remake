

#include "structs/structs_11.h"
#include "types.h"
#include "globals.h"
#include "fn_ptr_ops/fn_ptr_ops_7.h"
#include "win_ops/win_ops_1.h"

void pass1_1040_869a(globals: &mut Globals, Struct18* param_1)

{
    param_1->field_0x0 = 0x8ddc;
    (param_1->field_0x2)      = &globals.PTR_LOOP_1050_1040;
    fn_ptr_1000_17ce((param_1->field_0x90), 0x1000);
    fn_ptr_1000_17ce((param_1->field_0x94), 0x1000);
    ui_cleanup_op_1040_782c(param_1, 0x1000);
}
