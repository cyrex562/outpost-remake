//
// Created by cyrex on 2/22/2022.
//

#ifndef OUTPOST_1_SRC_SYS_OPS_SYS_OPS_12_H_
#define OUTPOST_1_SRC_SYS_OPS_SYS_OPS_12_H_

// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "stdbool.h"
// #include "structs/structs_0xx/structs_9x.h"

void pass1_1000_27d6(globals: &mut Globals, u16 param_1);


u16 *pass1_1000_2950(globals: &mut Globals, i16 param_1, param_2: u16, param_3: u16, u16 param_4);




u16 pass1_1000_2a00(u16 *param_1, i16 param_2, param_3: u16, u16 param_4, u16 param_5, u8 param_6);




u16 pass1_1000_2b5c(param_1: u16, param_2: u16, param_3: u16, u16 param_4, u16 param_5, i16 param_6, u16 param_7, u16 param_8);




u16 mem_1000_2bb6(param_1: u16, i16 *param_2, i16 param_3, u16 param_4, u16 param_5, u16 param_6, u8 param_7, u16 param_8);




void mem_1000_2ce8(i16 *param_1, param_2: u16, u16 param_3);




u16 *pass1_1000_2d34(param_1: u16, param_2: u16, u8 *param_3, u8 param_4, u16 *param_5, i16 param_6);


u32 mem_op_1000_1b68(param_1: u16, param_2: u16, u32 u32_ptr);




u32  mem_op_1000_1b9a(param_1: u16, u32 param_2, param_3: u16, u16 param_4);




BOOL16 mem_op_1000_1dfa(i16 param_1, u8 param_2, param_3: u16, u16 param_4);




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 pass1_1000_1e61(globals: &mut Globals, param_1: u16, param_2: u16, param_3: u16, u16 param_4);




u16  _SHI_INVOKEERRORHANDLER1(void);




void pass1_1000_201c(i16 param_1, i16 param_2, u16 param_3);




u16  pass1_1000_21d2(u8 param_1, long param_2, param_3: u16, u16 param_4, u8 param_5);




//i16 *entry(param_1: u16, param_2: u16, param_3: u16, u16 param_4, u16 param_5, CONTEXT *in_task_context, u16 param_7, i16 param_8);


i16 *interrupt_vector_op_1000_23ea(globals: &mut Globals,
                           param_1: u16,
                           param_2: u16,
                           i16 param_3,
                           u16 param_4);


void set_interrupt_vector_1000_256b(Globals *globals);




i16 *exit_1000_25cc(i16 param_1, param_2: u16, u16 param_3);


u16 pass1_1000_0e08(i16 param_1, u16 param_2);



long  pass1_1000_0ed4(param_1: u16, param_2: u16, param_3: u16, u16 param_4, u16 param_5,
                     u16 param_6, u16 param_7);



u16 pass1_1000_0fb8(param_1: u16, param_2: u16, i16 param_3, u16 param_4, u16 param_5, u16 *param_6, u16 param_7);




u32  pass1_1000_1284(u32 param_1, u16 param_2);




void  mem_op_1000_131c(param_1: u16, param_2: u16, i16 param_3, u16 param_4);




long  mem_op_1000_13ce(WORD param_1);




void  mem_op_1000_1408(param_1: u16, param_2: u16, param_3: u16, WORD param_4);




BOOL16  mem_op_1000_14f2(param_1: u16, param_2: u16, i16 param_3, u16 param_4, u16 param_5);




DWORD  mem_op_1000_1532(WORD param_1);


long mem_op_1000_1558(globals: &mut Globals, param_1: u16, param_2: u16, u16 param_3);




void pass1_1000_15ce(u16 *param_1, param_2: u16, WORD param_3);




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u16 mem_op_1000_160a(globals: &mut Globals, param_2: u16, u16 param_1);


u16 mem_1000_167a(globals: &mut Globals, param_1: u16, param_2: u16, u16 param_3);


u16 pass1_1000_16aa(globals: &mut Globals,
                    u16      param_1,
                    u16      param_2,
                    u16      param_3,
                    u16      param_4,
                    u16      param_5,
                    u16      param_6);


u16 pass1_1000_180c(globals: &mut Globals, param_1: u16, param_2: u16, u16 param_3);


long pass1_1000_183c(globals: &mut Globals, param_1: u16, param_2: u16, u16 param_3);




u32  mem_op_1000_18ec(param_1: u16, param_2: u16, u16 param_3);




// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 mem_op_1000_1902(globals: &mut Globals, param_1: u16, param_2: u16, param_3: u16, u16 param_4, u16 param_5, u16 param_6);




u32  mem_1000_0016(u32 param_1, u16 param_2);




u32 mem_op_1000_0052(param_1: u16, u16 param_2);


u32 pass1_1000_010c(globals: &mut Globals, i16 param_1, param_2: u16, param_3: u16, param_4: u32, u16 param_5, u16 param_6);


bool mem_op_1000_01b0(globals: &mut Globals, param_1: u16, u16 param_2);




i16 mem_op_1000_0308(i16 param_1, i16 param_2);


u32 mem_op_1000_03c6(
  globals: &mut Globals, param_1: u16, i16 param_2, param_3: u16, param_4: u32, u16 param_5, u8 param_6, u16 param_7);




u32 mem_op_1000_0510(param_1: u16, param_2: u16, u16 param_3);




u32 mem_op_1000_05e2(param_1: u16, i16 param_2, param_3: u16, u16 param_4, u16 param_5);




u32 mem_1000_0668(u16 param_1);




u16 mem_1000_0670(param_1: u16, i16 *param_2, param_3: u16, u32 *param_4, i16 param_5, WORD param_6);




Struct99 * pass1_1000_07fc(param_1: u16, u32 param_2);




u32 mem_op_1000_0838(param_1: u16, u16 param_2);




u16  pass1_1000_093a(i16 *param_1, param_2: u16, u16 param_3);




u8 *pass1_1000_09a0(u16 *param_1, u16 param_2);




long  mem_op_1000_0a48(u8 param_1, param_2: u16, i16 param_3, param_4: u32, u16 param_5);




u32 mem_op_1000_0b20(param_1: u16, param_2: u16, param_3: u16, u16 param_4);




#endif // OUTPOST_1_SRC_SYS_OPS_SYS_OPS_12_H_
