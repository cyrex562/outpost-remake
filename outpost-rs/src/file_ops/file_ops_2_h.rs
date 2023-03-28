//
// Created by cyrex562 on 2/13/23.
//

#ifndef OUTPOST_1_SRC_FILE_OPS_FILE_OPS_2_H_
#define OUTPOST_1_SRC_FILE_OPS_FILE_OPS_2_H_

// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "structs/structs_3xx/structs_38x.h"

void  pass1_1030_227a(param_1: u32, param_2: u32, param_3: u16, param_4: u16);

void  pass1_1030_232e(param_1: u32, param_2: u32, param_3: i16, param_4: u16, param_5: u16);

void  pass1_1030_2aca(Struct730 *param_1, param_2: u32, param_3: u16, param_4: u16);

void pass1_1030_2c8a(Globals   *globals,
                     Struct373 *param_1,
                     u32        param_2,
                     i16        param_3,
                     u8        *param_4,
                     u16        param_5);

void  pass1_1030_16d6(param_1: u32, param_2: u32, param_3: u16);

void  file_1030_1730(param_1: u32, param_2: u32);

u16  pass1_1030_1978(Struct730 *param_1, param_2: u32, param_3: u16, param_4: u16);

void  file_1030_19b4(Struct370 *param_1, param_2: u32, param_3: i16, param_4: u16, param_5: u16);

u16  pass1_1030_1a9c(param_1: u32, param_2: u32, param_3: u16);

u16 file_1030_1b18(Globals   *globals,
                   Struct370 *param_1,
                   u32        param_2,
                   i16        param_3,
                   u8        *param_4,
                   u16        param_5);

u16  write_file_fn_1028_e56c(param_1: u16, param_2: u16, param_3: u32, param_4: u16);

BOOL16  pass1_1028_d7a0(param_1: u16, param_2: u16, param_3: u32, param_4: u16);

i16  read_file_1028_d7ba(param_1: u16, param_2: u16, param_3: u32, param_4: u16, param_5: u16);

u32  write_to_file_1028_dce2(param_1: *mut u32, param_2: u32, param_3: u16);

void  read_file_1028_def2(param_1: u32, param_2: u32, param_3: u16, param_4: u16);

BOOL16  write_to_file_1028_b5ec(param_1: u32, param_2: u32, param_3: u16)
;

void  file_1028_b81a(param_1: u32, param_2: u32, param_3: i16, param_4: u16, u8 *param_5);

BOOL16  pass1_1028_b2c8(param_1: u32, param_2: u32, BOOL16 param_3, param_4: u16);

u16  pass1_1028_64d6(param_1: u32, param_2: u32, param_3: u16);

void  pass1_1028_65e2(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

BOOL16  write_to_file_1028_5f82(param_1: u32, param_2: u32, param_3: u16)
;

void  pass1_1028_5fcc(param_1: i16, param_2: *mut u8, param_3: i16, param_4: u16);

void  pass1_1028_4a1a(param_1: u32, param_2: u32, param_3: u16);

void  pass1_1028_4a5a(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

void  write_to_file_1028_3d0e(param_1: u32, param_2: u32, param_3: u16, param_4: u16);

void  pass1_1028_3d92(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16, param_6: u16);

BOOL16  pass1_1028_2418(param_1: u32, param_2: u32, param_3: u16);

BOOL16  file_1028_24a2(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

u16  write_to_file_1028_1452(param_1: u32, param_2: u32, param_3: u16);

void  pass1_1028_14d8(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

BOOL16  pass1_1020_e94e(param_1: u32, param_2: u32, param_3: u16);

void  pass1_1020_e994(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

u16  write_to_file_1028_0234(param_1: u32, param_2: u32, param_3: u16);

void  pass1_1028_0374(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

BOOL16  write_to_file_1020_e6a4(param_1: u32, param_2: u32, param_3: u16)
;

void  pass1_1020_e70e(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

BOOL16  write_to_file_1020_d3d4(param_1: u32, param_2: u32, param_3: u16)
;

BOOL16  pass1_1020_d41a(param_1: u32, param_2: u32, BOOL16 param_3, param_4: *mut u8, param_5: u16);

BOOL16  pass1_1020_a644(param_1: u16, param_2: u16, param_3: u32, param_4: u16);

BOOL16  read_file_1020_a65e(param_1: u32, param_2: u32, param_3: u16, param_4: u16);

void  pass1_1020_2488(param_1: u32, param_2: u16, param_3: u16)
;

void  pass1_1018_6630(param_1: u32, param_2: u16, param_3: u16);

void  write_to_file_1010_ed58(param_1: u32, param_2: u32, param_3: u16)
;



#endif // OUTPOST_1_SRC_FILE_OPS_FILE_OPS_2_H_
