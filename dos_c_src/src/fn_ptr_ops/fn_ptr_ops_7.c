//
// Created by cyrex on 2/23/2022.
//

#include "structs_1.h"
#include "fn_ptr_ops_1.h"
#include <winapi.h>
#include "unk/unk_11.h"
#include "globals.h"
#include "utils.h"
#include "types.h"
#include "fn_ptr_ops_6.h"
#include "fn_ptr_ops_7.h"
void fn_ptr_1000_17ce(astruct_18 *param_1, u16 param_2)

{
    if(param_1 != (astruct_18 *)0x0)
    {
        call_fn_ptr_1000_0dc6(param_1, param_1._2_2_, param_2);
    }
    return;
}
void fn_ptr_1000_17ce(astruct_18 *param_1,u16 param_2)

{
  if (param_1 != 0x0) {
    call_fn_ptr_1000_0dc6(param_1,param_1,param_2);
  }
}
