//
// Created by cyrex562 on 2/13/23.
//

#ifndef OUTPOST_1_SRC_FILE_OPS_FILE_OPS_3_H_
#define OUTPOST_1_SRC_FILE_OPS_FILE_OPS_3_H_

// #include "globals.h"
// #include "op_int.h"
// #include "op_windef.h"
// #include "structs/structs_2xx/structs_21x.h"
// #include "structs/structs_7xx/structs_72x.h"

void pass1_1018_0000(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16);

void pass1_1010_89f0(param_1: u16, param_2: u16, param_3: u16, param_4: u32, HINSTANCE16 param_5, param_6: u16);

void write_to_file_1010_6372(param_1: *mut Struct729, param_2: u32, param_3: u16);

void pass1_1010_648a(param_1: u32, param_2: u32, param_3: i16, param_4: u16);

void write_to_file_1010_6846(param_1: u32, param_2: u32, param_3: u16)
;

void pass1_1010_68c6(param_1: *mut Struct248, param_2: u32, param_3: u16, param_4: *mut u8, param_5: u16);

u16 pass1_1010_5dc6(param_1: u32, param_2: u32, param_3: u16);

void pass1_1010_5e56(param_1: u32, param_2: u32, param_3: i16, param_4: u16, param_5: u16);

void find_n_load_rsrc_1010_4e9e(param_1: u32, HGLOBAL16 param_2);

void pass1_1010_404a(param_1: u32, param_2: u32, param_3: i16, param_4: u16)
;

void pass1_1010_0ad2(param_1: u32, param_2: u32, param_3: u16);

void file_1010_0c7c(Globals          *globals,
                    param_1: *mut Struct228,
                    param_2: u32,
                    param_3: i16,
                    u8               *param_4,
                   param_5: u16);

void pass1_1008_e5da(param_1: u32, param_2: u32, HFILE16 param_3, param_4: u16);

void file_1008_e70e(param_1: u32, param_2: u32, param_3: i16, param_4: *mut u8, param_5: u16, param_6: u16);

void pass1_1008_c98e(param_1: u32, param_2: u32, HFILE16 param_3, param_4: u16);

void pass1_1008_c9d4(param_1: u32, param_2: u32, param_3: i16, param_4: u16, u64 param_5);

void pass1_1008_ba38(param_1: u32, param_2: u32, HFILE16 param_3, param_4: u16);

void file_1008_bb5e(Globals   *globals,
                    param_1: *mut Struct199,
                    param_2: u32,
                    param_3: i16,
                    u8        *param_4,
                    param_5: u16,
                   param_6: u16);

void file_1008_7548(param_1: u32, long *param_2, HFILE16 param_3, param_4: u16);

void file_1008_76e4(param_1: u32, long *param_2, param_3: u16, param_4: u16, param_5: u16);

u16 file_1008_77cc(param_1: u32, long *param_2, param_3: *mut u8, HFILE16 param_4, param_5: u16)
;

void  pass1_1008_7898(param_1: u32, param_2: *mut u32, param_3: u16, param_4: u16, HFILE16 param_5, param_6: u16);

u16  write_to_file_1008_7954(param_1: u32, param_2: *mut u32, param_3: u16, HFILE16 param_4, param_5: u16);

void pass1_1008_79f0(param_1: u32, long param_2, HFILE16 param_3, param_4: u16);

void write_to_file_1008_7a22(param_1: u32, long param_2, HFILE16 param_3, param_4: u16);

u16 pass1_1008_7ad4(param_1: u32, long *param_2, param_3: u16, HFILE16 param_4, param_5: u16)
;

u16 write_to_file_1008_7b4c(param_1: u32,param_2: *mut u16, HFILE16 param_3, param_4: u16);

BOOL16 read_file_1008_7bc8(param_1: u32,param_2: *mut u16, HFILE16 param_3, param_4: u16);

void read_file_1008_7c6e(param_1: u16, param_2: u16, char *param_3, HFILE16 param_4);

BOOL16 write_to_file_1008_7cac(param_1: u32, param_2: u16);

void  read_file_1008_7cfe(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16);

BOOL16  read_file_1008_7dee(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16, SEGPTR param_6, HFILE16 param_7);

BOOL16  write_to_file_1008_7e1c(param_1: u16, param_2: u16, param_3: u16, param_4: u16, char *buf_to_write, HFILE16 file_handle);

void close_file_1008_6dd0(param_1: *mut u32, HFILE16 param_2);

BOOL16 file_fn_1008_6e02(param_1: *mut u32, LPCSTR in_string, param_3: u16);

BOOL16 read_file_1008_6e78(param_1: u32, param_2: u16, char *in_string, param_4: u16);

void file_fn_1008_6eee(param_1: u16, param_2: u16, param_3: u32);

void read_file_1008_6f7a(param_1: u16, param_2: u16, param_3: u32, param_4: u16);

u16 write_to_file_1008_70a6(param_1: *mut u32, LPCSTR param_2);

BOOL16 read_file_1008_7146(param_1: i16, param_2: u16, LPCSTR param_3, param_4: u16)
;

u16 read_file_1008_71a0(param_1: u32, param_2: u16);

BOOL16 file_fn_1008_726c(param_1: u32, param_2: u16, HFILE16 file_handle)
;



#endif // OUTPOST_1_SRC_FILE_OPS_FILE_OPS_3_H_
