#pragma clang diagnostic push
#pragma ide diagnostic ignored "OCInconsistentNamingInspection"
#pragma ide diagnostic   ignored "readability-non-const-parameter"

#include "fn_ptr_defs.h"
#include "fn_ptr_ops/fn_ptr_ops_6.h"
#include "globals.h"
#include "op_dos_interrupts.h"
#include "op_int.h"
#include "op_win_def.h"
#include "op_winapi.h"
#include "string_consts.h"
#include "string_ops.h"
#include "struct_ops/struct_ops_1.h"
#include "sys_ops/sys_ops_11.h"
#include "sys_ops/sys_ops_12.h"
#include "unk/unk_14.h"
#include "unk/unk_15.h"
#include "unk/unk_16.h"
#include "utils.h"
#include "win_ops/win_ops_3.h"

#include <stdarg.h>
#include <stdbool.h>

i16 *entry(Globals *globals,
           u16 param_1,
           u16 param_2,
           u16 param_3,
           u16 param_4,
           u16 param_5,
           CONTEXT *in_task_context,
           u16 param_7,
           i32 param_8);

void init_1000_23be(struct Globals *globals, u16 param_1, u16 param_2, u16 param_3);

int main(int argc, char **argv)
{
    Globals globals = {};
    CONTEXT task_context = {};
    i16 *result = entry(&globals, 0, 0, 0, 0, 0, &task_context, 0, 0);
    return 0;
}

i16 *entry(Globals *globals,
           u16 param_1,
           u16 param_2,
           u16 param_3,
           u16 param_4,
           u16 param_5,
           CONTEXT *in_task_context,
           u16 param_7,
           i32 param_8)

{
    u8     puVar1;
    u16      u_var2;
    cstring  pcVar3;
    void*fn_ptr_1;
    u16      uVar5 = 0;
    cstring  string_1;
    u16     *pu_var_6;
    cstring  puVar7 = NULL;
    cstring  pcVar8;
    cstring  unaff_SS = NULL;
    bool     bVar9;
    u32      DVar10;
    u32      result;
    u32      uVar12;
    i16      iVar13;
    u16      iVar14;
    u8      *pu8_var15;
    u32      dos_version_ptr;

    result = CONCAT22(param_7, globals->PTR_LOOP_1050_5f84);
    do
    {
        dos_version_ptr = 0;
        InitTask16(in_task_context);
        globals->PTR_LOOP_1050_5f84 = result;
        if((param_8 != 0) && (bVar9 = param_1 < 0xff00, param_1 = param_1 + 0x100, globals->PTR_LOOP_1050_5f7e = param_5, bVar9))
        {
            globals->PTR_LOOP_1050_5f48 = param_1;
            globals->PTR_LOOP_1050_5f4a = param_3;
            globals->PTR_LOOP_1050_5f4c = param_4;
            globals->PTR_LOOP_1050_5f4e = param_2;
            globals->PTR_LOOP_1050_5f50 = param_5;
            LockSegment16((HGLOBAL16)0x1538);
            globals->PTR_LOOP_1050_5f52 = (result >> 0x10);
            globals->PTR_LOOP_1050_5f84 = result;
            DVar10                      = GetVersion16();
            globals->PTR_LOOP_1050_5f52 = (result >> 0x10);
            globals->PTR_LOOP_1050_5f84 = result;
            globals->PTR_LOOP_1050_5f80 = CONCAT11(DVar10, (DVar10 >> 8));
            // get dos version info
            fn_ptr_1                    = swi(DOS_INT_21);
            uVar12                      = result;
            result                      = ((Int21GetDosVersInfo)fn_ptr_1)((void*)dos_version_ptr);
            globals->PTR_LOOP_1050_5f52 = (uVar12 >> 0x10);
            globals->PTR_LOOP_1050_5f84 = uVar12;
            globals->DAT_1050_5f82      = CONCAT11(result, (result >> 8));
            globals->DAT_1050_5f87      = 0;
            // wait for something
            WaitEvent16(0x1000);
            globals->PTR_LOOP_1050_5f84 = result;
            pu8_var15                   = globals->PTR_LOOP_1050_5f4c;
            // initialize the app
            param_8                     = InitApp16((HINSTANCE16)0x1538);
            globals->PTR_LOOP_1050_5f84 = result;
            if(param_8 != 0)
                break;
        }
        in_task_context = (struct CONTEXT *)0x1538;
        param_8         = CONCAT11((param_8 >> 8), 0xff);
        pass1_1000_24db(globals, param_8, 0);
        globals->PTR_LOOP_1050_5f84 = result;
    } while(true);
    // app init at this point?
    interrupt_vector_op_1000_23ea(NULL, param_2, param_5, 0, unaff_SS);
    globals->PTR_LOOP_1050_5f84 = result;
    pass1_1000_262c(NULL, 0x238d, 0x1538, unaff_SS, 0x1538);
    globals->PTR_LOOP_1050_5f84 = result;
    pass1_1000_27d6(NULL, (int)(result >> 0x10));
    result = ret_op_1000_55ac(uVar5);
    uVar5  = result;
    init_1000_23be(globals, param_1, (result >> 0x10), unaff_SS);
    fn_ptr_op_1000_24cd(NULL, uVar5, 0);
    iVar14 = 0x15;
    iVar13 = 0x15;
    pass1_1000_25a8(NULL, param_5, 0x1538);
    pass1_1000_2913(NULL, iVar13, param_5, 0x1538);
    string_1 = poss_str_op_1000_28dc(NULL, iVar14);
    if(string_1 != 0x0)
    {
        iVar13 = 9;
        if(*string_1 == 'M')
        {
            iVar13 = 0xf;
        }
        string_1 = string_1 + iVar13;
        iVar13   = 0x22;
        pcVar8   = string_1;
        do
        {
            if(iVar13 == 0)
                break;
            iVar13 = iVar13 + -1;
            pcVar3 = pcVar8;
            pcVar8 = pcVar8 + 1;
        } while(*pcVar3 != '\r');
        pcVar8[-1] = '\0';
    }
    // END OF APP EXECUTION
    FatalAppExit16(0x1538, string_1);
    FatalExit();
    puVar7 = &globals->PTR_LOOP_1050_63fe;
    do
    {
        puVar1   = puVar7;
        puVar7   = puVar7 + 1;
        u_var2    = puVar1;
        pu_var_6 = puVar7;
        if((u_var2 == uVar5) || (pu_var_6 = (u_var2 + 1), pu_var_6 == 0x0))
        {
            return pu_var_6;
        }
        iVar13 = -1;
        do
        {
            if(iVar13 == 0)
                break;
            iVar13 -= 1;
            puVar1 = *puVar7;
            puVar7 += 1;
        } while(puVar1 != '\0');
    } while(true);
}


void init_op_1008_54aa(struct Globals *globals,
                       u8             *param_1,
                       char           *param_2,
                       u8             *param_3,
                       u8             *param_4,
                       u16             param_5,
                       u16             param_6,
                       u16             param_7,
                       u16             param_8)

{
    FnPtr3   *fn_ptr_a;
    u16       var_3 = 0;
    u16       in_cx_reg;
    u16       in_dx_reg;
    u32       var_4;
    u16       var_7 = 0;
    u16       var_5;
    u16       var_8 = 0;
    u16       var_9;
    u16       var_10 = 0;
    u32       var_11;
    u32      *var_12;
    u32       var_13;
    u16       var_14;

    if(param_3 != 0x0)
    {
        return;
    }
    get_date_time_op_1000_435c(NULL, 0x0, in_cx_reg, in_dx_reg, &var_14, param_8);
    set_globals_1000_4d0c(NULL, param_5);
    pass1_1000_1fea(NULL);
    globals->PTR_LOOP_1050_03a0 = mem_op_1000_1902(NULL, 0x0, 0x32, 0x0, 0x12, 0x1000, in_dx_reg);
    globals->PTR_LOOP_1050_029c = mem_op_1000_1902(NULL, 0x0, 0x64, 0x0, 0xc, 0x1000, (globals->PTR_LOOP_1050_03a0 >> 0x10));
    globals->PTR_LOOP_1050_4fb8 = mem_op_1000_1902(NULL, 0x0, 0x64, 0x0, 0x10, 0x1000, (globals->PTR_LOOP_1050_029c >> 0x10));
    globals->PTR_LOOP_1050_68a2 = mem_op_1000_1902(NULL, 0x0, 0x64, 0x0, 0xe, 0x1000, (globals->PTR_LOOP_1050_4fb8 >> 0x10));
    globals->PTR_LOOP_1050_5744 = mem_op_1000_1902(NULL, 0x0, 0x1f4, 0x0, 0x42, 0x1000, (globals->PTR_LOOP_1050_68a2 >> 0x10));
    var_11                      = mem_op_1000_1902(NULL, 0x0, 0x32, 0x0, 0x6, 0x1000, (globals->PTR_LOOP_1050_5744 >> 0x10));
    var_4                       = (var_11 >> 0x10);
    globals->PTR_LOOP_1050_5768 = var_11;
    globals->PTR_LOOP_1050_038c = param_4;
    globals->PTR_LOOP_1050_038e = param_3;
    globals->PTR_LOOP_1050_0390 = param_1;
    globals->PTR_LOOP_1050_576a = var_4;
    var_3                       = str_op_1008_60e8(param_2, var_4);
    globals->PTR_LOOP_1050_0392 = CONCAT22(var_4, var_3);
    mem_op_1000_179c(NULL, 0xc, var_4, 0x1000);
    if((var_4 | var_3) == 0x0)
    {
        var_3 = 0x0;
        var_5 = 0x0;
    }
    else
    {
        struct_op_1008_0000(CONCAT13((var_4 >> 0x8), CONCAT12(var_4, var_3)));
        var_5 = var_7;
    }
    var_12 = CONCAT22(var_5, var_3);
    if(globals->PTR_LOOP_1050_0392 != 0x0)
    {
        fn_ptr_a = (*var_12 + 0x4);
        (*fn_ptr_a)(0x1000, var_3, var_5, globals->PTR_LOOP_1050_0392, (globals->PTR_LOOP_1050_0392 >> 0x10));
    }
    var_13   = *var_12;
    fn_ptr_a = var_13 + 0x4;
    (**fn_ptr_a)(0x1000, var_3, var_5, 0, 0);
    var_9 = var_8;
    win_msg_op_1008_9498(&globals->PTR_LOOP_1050_1000, param_8);
    if(var_12 != 0x0)
    {
        fn_ptr_a = var_13;
        (*fn_ptr_a)(0x1000, var_3, var_5, 0x1, 0);
        var_9 = var_10;
    }
    var_11 = mem_op_1000_1b68(var_9, 0x1000, globals->PTR_LOOP_1050_03a0, (globals->PTR_LOOP_1050_03a0 >> 0x10));
    var_11 = mem_op_1000_1b68((var_11 >> 0x10), 0x1000, globals->PTR_LOOP_1050_029c, (globals->PTR_LOOP_1050_029c >> 0x10));
    var_11 = mem_op_1000_1b68((var_11 >> 0x10), 0x1000, globals->PTR_LOOP_1050_4fb8, (globals->PTR_LOOP_1050_4fb8 >> 0x10));
    var_11 = mem_op_1000_1b68((var_11 >> 0x10), 0x1000, globals->PTR_LOOP_1050_68a2, (globals->PTR_LOOP_1050_68a2 >> 0x10));
    mem_op_1000_1b68((var_11 >> 0x10), 0x1000, globals->PTR_LOOP_1050_5744, (globals->PTR_LOOP_1050_5744 >> 0x10));
}


void init_1000_23be(struct Globals *globals, u16 param_1, u16 param_2, u16 param_3)

{
    init_op_1008_54aa(globals,
                      globals->PTR_LOOP_1050_5f52,
                      CONCAT22(globals->PTR_LOOP_1050_5f50, globals->PTR_LOOP_1050_5f4e),
                      globals->PTR_LOOP_1050_5f4a,
                      globals->PTR_LOOP_1050_5f4c,
                      &globals->PTR_LOOP_1050_1050,
                      param_1,
                      param_2,
                      param_3);
}

#pragma clang diagnostic pop
