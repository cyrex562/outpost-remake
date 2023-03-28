//
// Created by cyrex562 on 2/13/23.
//

#ifndef OUTPOST_1_SRC_FILE_OPS_FILE_OPS_1_H_
#define OUTPOST_1_SRC_FILE_OPS_FILE_OPS_1_H_

// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "structs/structs_3xx/structs_30x.h"

void file_1038_774e(globals: &mut Globals,
                    param_1: *mut Struct307, param_2: u32, param_3: *mut u8, param_4: u16);

u16  pass1_1038_7b20(param_1: *mut u32, param_2: u32, param_3: u16);

u16 read_file_1038_7c02(globals: &mut Globals,
                       param_1: *mut u32,
                        param_2: u32,
                        param_3: u16,
                       param_4: u16);

void pass1_1038_5e16(globals: &mut Globals,
                     param_1: u32,
                     param_2: u32,
                     param_3: i16,
                     param_4: u16,
                    param_5: u16);

void file_1038_6118(globals: &mut Globals,
                    param_1: u32,
                    param_2: u32,
                    param_3: i16,
                    u8      *param_4,
                   param_5: u16);

void pass1_1030_de7c(globals: &mut Globals, param_1: u32, param_2: u32, param_3: u16);

void pass1_1030_dec4(param_1: u32,
                     param_2: u32,
                     param_3: i16,
                     u8      *param_4,
                     param_5: u16,
                     Globals *globals);

void pass1_1030_d61c(globals: &mut Globals, param_1: u32, param_2: u32, param_3: u16);

void  pass1_1030_d72e(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

void  pass1_1030_c230(param_1: u32, param_2: u32, param_3: u16);

void  pass1_1030_c29c(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

BOOL16  pass1_1030_c84e(param_1: u32, param_2: u32, param_3: u16);

BOOL16  pass1_1030_c894(param_1: u32, param_2: u32, BOOL16 param_3, param_4: *mut u8, param_5: u16);

void  pass1_1030_b768(param_1: u32, param_2: u32, param_3: u16);

void  file_1030_b836(param_1: *mut Struct401, param_2: u32, param_3: *mut u8, param_4: u16);

void  pass1_1030_7418(param_1: *mut Struct731, param_2: u32, param_3: i16, param_4: u16);

void  file_1030_778c(param_1: *mut Struct387, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

BOOL16  pass1_1030_5c1a(param_1: u32, param_2: u32, param_3: u16);

BOOL16  read_file_1030_5c52(param_1: u32, param_2: u32, param_3: u16, param_4: u16);

void  pass1_1030_5dbe(param_1: u32, param_2: u32, param_3: u16, param_4: u16)
;

void  file_1030_5e70(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

u16  read_file_1030_4e70(param_1: u32, param_2: *mut u32, u8 **param_3, long param_4, param_5: u16);

void  pass1_1030_5044(param_1: u32, param_2: u16, param_3: u16);

void  pass1_1030_56f6(param_1: u32, param_2: u32, param_3: u16, param_4: u16);

void  file_1030_581e(param_1: *mut Struct381, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

void  write_to_file_1030_32e4(param_1: u32, param_2: u32, param_3: u16);

void read_file_1030_33f0(globals: &mut Globals, param_1: *mut Struct430, param_2: u32);



#endif // OUTPOST_1_SRC_FILE_OPS_FILE_OPS_1_H_
