use crate::app_context::AppContext;
use crate::err_ops::{error_check_1000_0dc6, error_check_1000_17ce};
use crate::file_ops::read::{call_read_file_1020_a65e, read_file_1008_76e4, read_file_1028_d7ba, read_file_1028_def2, read_file_1030_5c52, read_from_file_1038_7c02};
use crate::file_ops::write::{call_write_to_file_1020_a644, write_to_file_1028_d7a0, write_to_file_1028_dce2, write_to_file_1030_5c1a, write_to_file_1038_7b20};
use crate::mem_funcs::Address;
use crate::mem_funcs::alloc_mem::{alloc_mem_1000_07fc, alloc_mem_1000_0a48};
use crate::other_funcs::zero_list_1008_3e38;
use crate::pass::pass15_funcs::pass1_1020_a43e;
use crate::pass::pass20_funcs::{pass1_1010_8ef2, pass1_1010_905e};
use crate::pass::pass7_funcs::pass1_1018_20ee;
use crate::pass::pass_funcs;
use crate::string_ops::misc::{copy_string_1000_3d3e, get_string_index_1000_3da4, string_fn_1000_3f9c};
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1008_41bc, process_struct_1008_4544, process_struct_1008_4772, process_struct_1008_47cc, process_struct_1008_4834, process_struct_1008_50c2, process_struct_1008_574a, process_struct_1008_8d8a, set_struct_1008_4016, set_struct_1008_56b4, set_struct_1008_9584};
use crate::structs::prog_structs_1::{Struct104, Struct197};
use crate::structs::prog_structs_12::Struct102;
use crate::structs::prog_structs_15::Struct194;
use crate::structs::prog_structs_18::Struct195;
use crate::structs::prog_structs_2::{Struct131, Struct199, Struct306, Struct7};
use crate::structs::prog_structs_23::{Struct184, Struct193};
use crate::structs::prog_structs_24::{Struct103, Struct182, Struct2111};
use crate::structs::prog_structs_25::Struct65;
use crate::structs::prog_structs_26::{Struct183, Struct196, Struct206};
use crate::structs::prog_structs_28::{FileObject, Struct207};
use crate::structs::prog_structs_29::Struct192;
use crate::structs::prog_structs_30::{Struct200, Struct201, Struct203, Struct417};
use crate::structs::prog_structs_5::Struct1;
use crate::structs::prog_structs_6::Struct675;
use crate::structs::prog_structs_7::{Struct189, Struct376, Struct44};
use crate::sys_ops::reg_class_1008_96d2;
use crate::typedefs::{HCURSOR16, HFILE16, HGDIOBJ16};
use crate::util::{CARRY1, CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, POPCOUNT, SBORROW2, ZEXT24};
use crate::winapi::{GetStockObject16, LoadCursor16};

pub unsafe fn pass1_1008_3d44(param_1: u16, param_2: u8) {
    let mut u_var1: u16;
    let local_res7: u8;
    let mut in_stack_00000008: u8;

    _param_1 = CONCAT13(local_res7, CONCAT12(param_2, param_1));
  // u_var1 = (_param_1  >> 0x10);
    _param_1.ptr_a_lo = 0x380a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    _param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((in_stack_00000008 & 1) != 0) {
        error_check_1000_17ce(_param_1);
    }
    return _param_1 & 0xffff0000 | CONCAT12(param_2, param_1) & 0xffff;
}

pub unsafe fn pass1_1008_3db2(
    param_1: u8,
    param_2: u16,
    param_3: u16,
    param_4: bool,
    param_5: u8,
    param_6: u8,
    param_7: u8,
    param_8: u8,
) {
    let pb_var1: Vec<u8>;
    let mut b_var2: bool;
    let mut b_var3: bool;
    let mut b_var4: bool;
    let mut b_var5: u8;
    let ppcVar6: fn();
    let mut cVar7: u8;
    let mut u_var8: u32;
    let mut b_var9: u8;
    let mut b_var10: u8;
    let mut i_var11: i32;
    let mut b_var12: u8;
    let mut b_var13: u8;
    let local_BX__1: &mut  u32;
    let pu_var14: &mut  u32;
    u32 * *local_SP;
    let unaff_bp: &mut  u32;
    let unaff_si: Vec<u8>;
    let pb_var15: Vec<u8>;
    let unaff_DI: Vec<u8>;
    let unaff_es: Vec<u8>;

    let puStack2: &mut  u32;
    let temp_179f29f37356: &mut  u32;
    let mut temp_5fdbfafefb: u32;

    loop {
        local_SP = &puStack2;
        local_SP = &puStack2;
        cVar7 = '\x0f';
        temp_179f29f37356 = unaff_bp;
        while {
            temp_179f29f37356 = (temp_179f29f37356 + -2);
            local_SP = local_SP + -1;
            unsafe { *local_SP = temp_179f29f37356 };
            cVar7 = cVar7 + -1;
            '\0' < cVar7
        } {}
        i_var11 = param_1;
        pb_var1 = (local_BX__1 + i_var11);
        b_var12 = param_3;
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        b_var2 = 9 < (unaff_si & 0xf);
        b_var5 = b_var2 | param_5;
        pb_var1 = (local_BX__1 + i_var11);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        b_var3 = 9 < (unaff_si + b_var5 * '\x06' & 0xf);
        pb_var1 = (local_BX__1 + i_var11);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        let pb_var1_val = unsafe { *pb_var1 };
        b_var9 = ((POPCOUNT(pb_var1_val) & 1) == 0) * 0x4;
        pb_var1 = (local_BX__1 + i_var11);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        b_var5 = 9 < b_var9 | b_var3 | b_var5;
        b_var10 = b_var9 + b_var5 * '\x06' & 0xf;
        pb_var1 = (local_BX__1 + i_var11);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        b_var5 = 9 < b_var10 | b_var5;
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        pb_var15 = unaff_DI + -1;
        pb_var1 = (local_BX__1 + pb_var15);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        pb_var1 = (local_BX__1 + pb_var15);
        unsafe { *pb_var1 = *pb_var1 | b_var12 };
        param_3 = param_3 - 1;
        pb_var1 = (local_BX__1 + pb_var15);
        b_var13 = param_3;
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pb_var1 = unaff_DI;
        unsafe { *pb_var1 = *pb_var1 & (local_BX__1 >> 8) };
        pb_var1 = (local_BX__1 + pb_var15);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        b_var12 = (b_var10 + b_var5 * '\x06' & 0xf) + 1;
        let pv_var15_val = unsafe { *pb_var15 };
        out(pv_var15_val, param_3);
        b_var4 = 9 < (b_var12 & 0xf);
        b_var5 = b_var4 | b_var5;
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pu_var14 = local_BX__1;

        if (pb_var1_val == 0) {}
        // goto code_r0x10083e29;
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        if ((POPCOUNT(pb_var1_val) & 1) == 0) {
            break;
        }
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pu_var14 = ((param_8 & 1) * 0x4000
            | (param_7 & 1) * 0x200
            | (param_6 & 1) * 0x100
            | (pb_var1_val < '\0') * 0x80
            | (pb_var1_val == 0) * 0x40
            | ((b_var4 | 9) < b_var10 | (9 < b_var9) | b_var3 | b_var2 | param_5 & 1) * 0x10
            | ((POPCOUNT(pb_var1_val) & 1) == 0) * 4);
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        pb_var1 = unaff_DI + 0x1008;
        unsafe { *pb_var1 = *pb_var1 ^ local_BX__1 };
        unsafe { b_var9 = *pb_var1 };
        unaff_DI[0x1008] = local_BX__1;
        temp_5fdbfafefb = (unaff_DI + 0x1008);
      // unaff_es = (temp_5fdbfafefb  >> 0x10);
        local_BX__1 = temp_5fdbfafefb;
        param_2 = param_2 - 1;
        if (param_2 == 0 || b_var9 == 0) {
            pb_var1 = (local_BX__1 + unaff_DI);
            unsafe { *pb_var1 = *pb_var1 | b_var13 };
            unaff_es = unaff_DI;
            if (pu_var14[1] != 0) {
                // code_r0x10083e29:
                u_var8 = pu_var14[1];
              // unaff_es = (u_var8  >> 0x10);
                local_BX__1 = u_var8;
                // code_r0x10083e2d:
                let local_bx_1_val = unsafe { *local_BX__1 };
                ppcVar6 = (local_bx_1_val + 4);
                puStack2 = unaff_bp;
                (**ppcVar6)();
            }
            return;
        }
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        b_var9 = (b_var12 + b_var5 * '\x06' & 0xf) - 1;
        b_var5 = 9 < (b_var9 & 0xf) | b_var5;
        b_var9 = b_var9 + b_var5 * '\x06' & 0xf;
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        param_5 = 9 < b_var9 | b_var5;
        param_1 = b_var9 + param_5 * '\x06' & 0xf;
        pb_var1 = (local_BX__1 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var13 };
        unaff_bp = local_SP;
        unaff_si = unaff_DI;
    }
    let pb_var1_val = unsafe { *pb_var1 };
    if (pb_var1_val == 0) {}
    // goto code_r0x10083e29;
    // goto code_r0x10083e2d;
}

pub unsafe fn pass1_1008_3e0e(param_1: &mut  Vec<u8>) {
    let mut local_es_3: u16;
    // fn_ptr_1: &mut  Vec<u8>;

  // local_es_3 = (param_1  >> 0x10);
    if ((param_1 + 4) != 0) {
        fn_ptr_1 = ((param_1 + 4) + 4);
        (**fn_ptr_1)();
    }
    return;
}

pub unsafe fn pass1_1008_3e54(param_1: &mut  u16, param_2: u16, param_3: u16, param_4: u16) {
    let mut local_es_6: u16;

  // local_es_6 = (param_1  >> 0x10);
    unsafe { *param_1 = param_4 };
    (param_1 + 2) = param_3;
    (param_1 + 4) = param_2;
    return param_1;
}

pub unsafe fn pass1_1008_3e76(param_1: &mut  u16, param_2: u16, param_3: u16, param_4: u16) {
    let mut local_es_6: u16;

  // local_es_6 = (param_1  >> 0x10);
    unsafe { *param_1 = param_4 };
    (param_1 + 2) = param_3;
    (param_1 + 4) = param_2;
    return;
}

pub unsafe fn pass1_1008_3e94(param_1: &mut  Struct199, param_2: Vec<u8>, param_3: Vec<u8>) {
    let mut in_stack_00000006: u16;

    unsafe { param_3 = *_param_1 };
    param_2 = param_1.field_0x2;
    return;
}

pub unsafe fn pass1_1008_3eb4(param_1: &mut  u16, param_2: u32, param_3: u32, param_4: u32) {
    let mut local_es_15: u16;

    unsafe { param_4 = *param_1 };
  // local_es_15 = (param_1  >> 0x10);
    param_3 = (param_1 + 2);
    param_2 = (param_1 + 4);
    return;
}

pub unsafe fn pass1_1008_3ee2(param_1: &mut  i32, param_2: &mut  i32) {
    let mut i_var1: i32;
    let mut i_var2: i32;
    i_var1 = *param_2 - *param_1;
    if (i_var1 < 0) {
        i_var1 = -i_var1;
    }
    *param_1 = i_var1 + 1;
    ctx.es_reg = (param_2 >> 0x10);
  // local_es_33 = (param_1  >> 0x10);
    i_var2 = param_1;
    i_var1 = (param_2 + 2) - (i_var2 + 2);
    if i_var1 < 0 {
        i_var1 = -i_var1;
    }
    (i_var2 + 2) = i_var1 + 1;
    i_var1 = (param_2 + 4) - (i_var2 + 4);
    if i_var1 < 0 {
        i_var1 = -i_var1;
    }
    (i_var2 + 4) = i_var1 + 1;
    return;
}

pub unsafe fn pass1_1008_3f32(param_1: &mut  i32, param_2: &mut  i32) {
    let pi_var1: &mut  i32;
    let mut local_es_15: u16;
    let mut local_es_22: u16;

    unsafe { *param_1 = *param_1 + *param_2 };
  // local_es_15 = (param_2  >> 0x10);
  // local_es_22 = (param_1  >> 0x10);
    pi_var1 = (param_1 + 2);
    unsafe { *pi_var1 = *pi_var1 + (param_2 + 2) };
    pi_var1 = (param_1 + 4);
    unsafe { *pi_var1 = *pi_var1 + (param_2 + 4) };
    return;
}

pub unsafe fn pass1_1008_3f92(in_struct_a: &mut Struct103, param_2: &mut Struct183) {
    let struct_a_hi: &mut  Struct103;
    let struct_a: &mut  Struct103;
    // fn_ptr_1: &mut  Vec<u8>;

    struct_a = in_struct_a;
    set_struct_1008_56b4(struct_a);
  // struct_a_hi = (in_struct_a  >> 0x10);
    struct_a.field_0x6 = 0;
    struct_a.field_0xa = 0;
    struct_a.field_0xe = 0;
    struct_a.field_0x10 = 0;
    struct_a.field_0x14 = 0;
    struct_a.field_0x18 = 0;
    struct_a.field_0x1c = 0;
    in_struct_a.ptr_1_lo = &PTR_LOOP_1050_48de;
    struct_a.ptr_1_hi = &ctx.PTR_LOOP_1050_1008;
    if (param_2 == 0x0) {
        return;
    }
    fn_ptr_1 = (param_2 + 8);
    (**fn_ptr_1)();
    pass1_1008_4214(struct_a, param_2);
    process_struct_1008_47cc(in_struct_a);
    process_struct_1008_4834(in_struct_a);
    return;
}

pub unsafe fn pass1_1008_405c(
    param_1: &mut  Struct103,
    param_2: &mut  u32,
    param_3: u16,
    param_5: i32,
    param_4: i32,
) -> i32 {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut local_AX_116: u16;
    let mut u_var4: u16;
    let mut u_var5: i32;
    let paVar6: &mut  Struct103;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut temp_5fb8438c55: u32;
    let lVar3: u32;

    paVar6 = param_1;
    set_struct_1008_56b4(paVar6);
  // u_var5 = (param_1  >> 0x10);
    paVar6.field_0x6 = 0;
    paVar6.field_0xa = 0;
    paVar6.field_0xe = 0;
    paVar6.field_0x10 = 0;
    paVar6.field_0x14 = 0;
    paVar6.field_0x18 = 0;
    paVar6.field_0x1c = 0;
    param_1.ptr_1_lo = &PTR_LOOP_1050_48de;
    paVar6.ptr_1_hi = &ctx.PTR_LOOP_1050_1008;
    i_var2 = param_4 * 8 + 0x1f;
    i_var2 = ((i_var2 + (i_var2 >> 0xf & 0x1f)) >> 5) << 2;
    _local_a = param_3;
    u_var4 = ((i_var2 * param_3) >> 0x20);
    lVar3 = (i_var2 * param_3) + 0x436;
    alloc_mem_1000_0a48(1, lVar3, ctx.g_struct_ptr_1);
    &paVar6.field_0x6 = lVar3;
    (&paVar6.field_0x6 + 2) = u_var4;
    process_struct_1008_47cc((param_1 & 0xffff | u_var5 << 0x10));
    &paVar6.field_0x18 = i_var2;
    (&paVar6.field_0x18 + 2) = i_var2 >> 0xf;
    paVar6.field_0x10 = 0x28;
    temp_5fb8438c55 = paVar6.field_0x10;
    (temp_5fb8438c55 + 4) = param_4;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 8) = _local_a;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 0xc) = 1;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 0xe) = 8;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 0x10) = 0;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 0x14) = paVar6.field_0x18 * _local_a;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 0x20) = 0x100;
    u_var1 = paVar6.field_0x10;
    (u_var1 + 0x24) = 0x100;
    process_struct_1008_4834(param_1);
    pass1_1008_4d84(paVar6.field_0xa, param_2);
    return;
}

pub unsafe fn pass1_1008_4214(param_1: &mut  Struct103, in_Struct183: &mut  Struct7) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let local_Struct183: &mut  Struct183;
    let mut local_es_5: u16;
    let mut in_stack_00000006: u16;
    let temp_862d998a63d: &mut  u32;
    // fn_ptr_1: &mut  Vec<u8>;

  // local_es_5 = (in_Struct183  >> 0x10);
    local_Struct183 = in_Struct183;
    param_1.field_0x6 = local_Struct183.field_0x1a;
    local_Struct183.field_0x1a = 0;
    pu_var1 = local_Struct183.field_0x4;
    u_var2 = &local_Struct183.field_0x6;
    if ((u_var2 | pu_var1) != 0) {
        unsafe { fn_ptr_1 = *pu_var1 };
        (**fn_ptr_1)();
    }
    &local_Struct183.field_0x4 = 0;
    local_Struct183.field_0xe = 0;
    local_Struct183.field_0x12 = 0;
    local_Struct183.field_0x16 = 0;
    local_Struct183.field_0x1e = 0;
    return '\0';
}

pub unsafe fn pass1_1008_431c(in_Struct184: &mut  Struct184, param_2: u8) {
    let pu_var1: &mut  u32;
    let mut u_var2: u32;
    let mut b_var3: bool;
    let mut local_AX_134: u16;
    let mut local_DX_134: u16;
    let local_Struct184: &mut  Struct184;
    let mut u_var5: i32;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut temp_5fa7e01195: u32;
    let mut u_var4: u32;

  // u_var5 = (in_Struct184  >> 0x10);
    local_Struct184 = in_Struct184;
    if (&local_Struct184.field_0x6 == 0) {
        process_struct_1008_47cc((in_Struct184 & 0xffff | u_var5 << 0x10));
    }
    if ((local_Struct184.field_0x8 | local_Struct184.field_0x6) == 0) {
        b_var3 = false;
    } else {
        if ((local_Struct184.field_0xc | local_Struct184.field_0xa) == 0) {
            process_struct_1008_4834((in_Struct184 & 0xffff | u_var5 << 0x10));
        }
        b_var3 = true;
    }
    if (b_var3) {
        if ((local_Struct184.field_0x16 | local_Struct184.field_0x14) == 0) {
            return;
        }
        local_6 = 0;
        while (true) {
            temp_5fa7e01195 = local_Struct184.field_0x10;
            pu_var1 = (temp_5fa7e01195 + 8);
            let pu_var1_val = unsafe { *pu_var1 };
            if (pu_var1_val == local_6 || pu_var1_val < local_6) {
                break;
            }
            u_var4 = local_6;
            process_struct_1008_4544(in_Struct184);
            u_var2 = local_Struct184.field_0x10;
            pass_funcs::pass1_1000_4906(
                (u_var4 & 0xffff | local_DX_134 << 0x10),
                param_2,
                (u_var2 + 4),
            );
            local_6 = local_6 + 1;
        }
    }
    return;
}

pub unsafe fn pass1_1008_43cc(param1: &mut  i32) {
    let mut b_var1: bool;
    let num_list: &mut  i32;
    let mut u_var2: i32;
    let mut local_4: u16;

  // u_var2 = (i32_list  >> 0x10);
    num_list = i32_list;
    if ((num_list + 3) == 0) {
        process_struct_1008_47cc((i32_list & 0xffff | u_var2 << 0x10));
    }
    if ((num_list + 3) == 0) {
        b_var1 = false;
    } else {
        if ((num_list + 5) == 0) {
            process_struct_1008_4834((i32_list & 0xffff | u_var2 << 0x10));
        }
        b_var1 = true;
    }
    if (!b_var1) {
        return 0;
    }
    return CONCAT22(num_list[0xb], num_list[10]);
}

pub unsafe fn pass1_1008_4426(param_1: &mut  Struct104) {
    let mut b_var1: bool;
    let local_bx_4: &mut  Struct104;
    let mut u_var2: i32;
    let mut local_4: u16;

  // u_var2 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    if (&local_bx_4.a == 0) {
        process_struct_1008_47cc((param_1 & 0xffff | u_var2 << 0x10));
    }
    if (&local_bx_4.a == 0) {
        b_var1 = false;
    } else {
        if (&local_bx_4.field_0xa == 0) {
            process_struct_1008_4834((param_1 & 0xffff | u_var2 << 0x10));
        }
        b_var1 = true;
    }
    if (!b_var1) {
        return 0;
    }
    return CONCAT22(&local_bx_4.field_0xc, &local_bx_4.field_0xa);
}

pub unsafe fn pass1_1008_4480(param_1: u32, in_struct_1: &mut  Struct417, param_3: u32) {
    let mut u_var1: u16;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut local_DX_29: u16;
    let mut local_DX_97: u16;
    let mut local_DX_122: u16;

    let mut u_var4: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3e94(
        in_struct_1,
        CONCAT22(ctx.stack_seg_reg, &local_6),
        CONCAT22(ctx.stack_seg_reg, &local_4),
    );
    u_var4 = process_struct_1008_4772(param_3);
  // local_DX_29 = (u_var4  >> 0x10);
    u_var1 = (u_var4 + 4);
    i_var2 = (u_var4 + 8);
    local_10 = 0;
    while (local_10 < i_var2) {
        u_var3 = local_6;
        local_6 = local_6 + 1;
        process_struct_1008_4544(param_1);
        _local_14 = CONCAT22(local_DX_97, u_var3);
        u_var4 = SEXT24(local_10);
        process_struct_1008_4544(param_3);
        _local_18 = (u_var4 & 0xffff | local_DX_122 << 0x10);
        local_1a = u_var1;
        while (local_1a != 0) {
            if (*_local_18 != -1) {
                unsafe { *_local_14 = *_local_18 };
            }
            _local_18 = CONCAT22(
                (_local_18 >> 0x10) + (-(0xfffe < local_18) & 0x6c),
                local_18 + 1,
            );
            _local_14 = CONCAT22(
                (_local_14 >> 0x10) + (-(0xfffe < local_14) & 0x6c),
                local_14 + 1,
            );
            local_1a = local_1a - 1;
        }
        local_10 = local_10 + 1;
    }
    return;
}

pub unsafe fn pass1_1008_4b5e(param_1: &mut  Struct189) {
    let mut i_var1: i32;
    let local_struct_1: &mut  Struct189;
    let mut local_es_3: u16;
    // fn_ptr: &mut  Vec<u8>;

  // local_es_3 = (param_1  >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.field_0x1e == 0) {
        fn_ptr = (param_1 + 8);
        i_var1 = (**fn_ptr)();
        if (i_var1 == 0) {
            return 0;
        }
    }
    return CONCAT22(local_struct_1.field_0x6, local_struct_1.field_0x4);
}

pub unsafe fn pass1_1008_4b8e(in_Struct7_1: &mut  Struct7, param_2: &mut  Struct131) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let mut i_var3: i32;
    let mut local_DX_11: u16;
    let ppVar4: &mut  Struct2111;
    let mut local_res6: u16;
    let mut in_stack_0000ffe4: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar4 = process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22((in_stack_0000ffe4 >> 0x10), 0x48),
    );
  // u_var2 = (ppVar4  >> 0x10);
    u_var1 = (ppVar4 + 0x18);
    i_var3 = (ppVar4 + 0x16) / 2;
    local_10 = 0;
    while (local_a._0_2_ = u_var1, local_10 < i_var3) {
        pass1_1008_4d26(
            &in_Struct7_1.func_ptr_2,
            u_var1 & 0xffff0000 | (local_10 * 4 + local_a),
            local_10,
        );
        local_10 = local_10 + 1;
    }
    local_12 = 0x100 - i_var3;
    while (local_12 < 0x100) {
        pass1_1008_4d26(
            &in_Struct7_1.func_ptr_2,
            u_var1 & 0xffff0000 | (local_10 * 4 + local_a),
            local_12,
        );
        local_12 = local_12 + 1;
        local_10 = local_10 + 1;
    }
    return;
}

pub unsafe fn pass1_1008_4cdc(param_1: &mut  Struct192) {
    let local_bx_4: &mut  Struct192;
    let mut local_es_4: u16;
    let mut temp_5f3eb4e5e7: u32;

  // local_es_4 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    param_1 = 0x4f1c;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    error_check_1000_17ce(local_bx_4.field_0xe);
    if (local_bx_4.field_0x12 != 0) {
        error_check_1000_17ce(local_bx_4.field_0x4);
    }
    param_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_4d26(param_1: &mut  Struct193, param_2: u32, param_3: u16) {
    let pu_var1: &mut  u16;
    let local_bx_4: &mut  Struct193;
    let mut i_var2: i32;
    let mut local_es_4: u16;
    let mut local_es_51: u16;
    let mut temp_5ffd109e1a: u16;
    let mut temp_5fce800a58: u32;

  // local_es_4 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    let pu_var1_val = unsafe { *pu_var1 };
    if (((local_bx_4.field_0x4 != 0) && (-1 < param_3))
        && (
            pu_var1 = &local_bx_4.field_0xc,
            pu_var1_val != param_3 && param_3 <= pu_var1_val,
        ))
    {
        temp_5ffd109e1a = (param_2 + 2);
        temp_5fce800a58 = local_bx_4.field_0x4;
      // local_es_51 = (temp_5fce800a58  >> 0x10);
        i_var2 = temp_5fce800a58;
        (i_var2 + param_3 * 4) = param_2;
        (i_var2 + param_3 * 4 + 2) = temp_5ffd109e1a;
        return 1;
    }
    return 0;
}

pub unsafe fn pass1_1008_4d72(param_1: u32) {
    let mut local_es_3: u16;

  // local_es_3 = (param_1  >> 0x10);
    return CONCAT22((param_1 + 6), (param_1 + 4));
}

pub unsafe fn pass1_1008_4d84(param_1: &mut  Struct194, param_2: &mut  u32) {
    let pu_var1: &mut  u16;
    let in_dx: &mut  Struct199;
    let local_bx_4: &mut  Struct194;
    let mut local_es_4: u16;
    let mut local_es_12: u16;
    let mut in_stack_0000000a: u16;
    let mut temp_5fed13c9c0: u32;

  // local_es_4 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x12 != 0) {
        local_bx_4.field_0xc = (param_2 + 3);
        error_check_1000_17ce(&local_bx_4.field_0x4);
        &local_bx_4.field_0x4 = 0;
        pu_var1 = (local_bx_4.field_0xc << 2);
        process_struct_1000_179c(pu_var1, in_dx);
        local_bx_4.field_0x4 = pu_var1;
        &local_bx_4.field_0x6 = in_dx;
    }
    if (local_bx_4.field_0xc != 0x100) {
        return;
    }
    pass_funcs::pass1_fn_1000_48a8(&local_bx_4.field_0x4, param_2[1], 0x400);
    return;
}

pub unsafe fn pass1_1008_507c(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    process_struct_1008_41bc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_5118(param_1: &mut  Struct376) {
    let mut local_es_3: u16;
    let mut temp_5f496a68b0: u32;

  // local_es_3 = (param_1  >> 0x10);
    if ((param_1 + 0x10) != 0) {
        temp_5f496a68b0 = (param_1 + 0x10);
        error_check_1000_0dc6(temp_5f496a68b0, (temp_5f496a68b0 >> 0x10));
    }
    return;
}

pub unsafe fn pass1_1008_5134(param_1: u32) {
    let pu_var1: &mut  u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut var4: u16;
    let mut local_AX_21: u16;
    let mut u_var5: i32;
    let lVar6: u32;
    let mut i_var7: i32;
    let local_bx_6: &mut  Struct196;
    let mut local_es_6: u16;
    let mut b_var8: bool;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

  // local_es_6 = (param_1  >> 0x10);
    local_bx_6 = param_1;
    sVar4 = &local_bx_6.field_0x4 * &local_bx_6.field_0x8;
    u_var5 = (sVar4 >> 0x20);
    local_AX_21 = sVar4;
    alloc_mem_1000_0a48(
        1,
        local_AX_21,
        (sVar4 >> 0x10),
        ctx.g_struct_ptr_1,
        (ctx.g_struct_ptr_1 >> 0x10),
    );
    local_bx_6.field_0x10 = local_AX_21;
    local_bx_6.field_0x12 = u_var5;
    if ((u_var5 | local_bx_6.field_0x10) == 0) {
        return;
    }
    i_var7 = local_bx_6.field_0x8;
    i_var2 = local_bx_6.field_0xa;
    lVar6 = CONCAT22(i_var2 - (i_var7 == 0), i_var7 + -1) * &local_bx_6.field_0x4;
    pu_var1 = &local_bx_6.field_0x10;
    u_var5 = lVar6;
    let pu_var1_val = unsafe { *pu_var1 };
    local_a = u_var5 + pu_var1_val;
  // local_8 = ((lVar6  >> 0x10) + CARRY2(u_var5, pu_var1_val)) * 0x100 + local_bx_6.field_0x12;
    _local_e = CONCAT22(i_var2, i_var7);
    local_10 = local_bx_6.field_0x2;
    while (_local_e != 0) {
        u_var3 = local_10 + 1;
        i_var7 = local_10 >> 0xf;
        process_struct_1008_4544(local_bx_6.field_0xc);
        pass_funcs::pass1_fn_1000_48a8(
            CONCAT22(local_8, local_a),
            CONCAT22(i_var7, local_10),
            local_bx_6.field_0x4,
        );
        i_var7 = local_bx_6.field_0x4;
        u_var5 = -i_var7;
        b_var8 = CARRY2(local_a, u_var5);
        local_a = local_a + u_var5;
        local_8 = local_8 + (b_var8 - (local_bx_6.field_0x6 + (i_var7 != 0))) * 0x100;
        local_10 = u_var3;
        _local_e = _local_e + -1;
    }
    return;
}

pub unsafe fn pass1_1008_5236(param_1: &mut  Struct197) {
    let pu_var1: &mut  u32;
    let mut i_var2: i32;
    let mut u_var3: u16;
    let mut u_var4: i32;
    let lVar5: u32;
    let mut i32_var6: i32;
    let local_bx_6: &mut  Struct197;
    let mut local_es_6: u16;
    let mut b_var7: bool;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // local_es_6 = (param_1  >> 0x10);
    local_bx_6 = param_1;
    i32_var6 = local_bx_6.field_0x8;
    i_var2 = local_bx_6.field_0xa;
    lVar5 = CONCAT22(i_var2 - (i32_var6 == 0), i32_var6 + -1) * &local_bx_6.field_0x4;
    pu_var1 = &local_bx_6.field_0x10;
    u_var4 = lVar5;
    let pu_var1_val = unsafe { *pu_var1 };
    local_6 = u_var4 + pu_var1_val;
  // local_4 = ((lVar5  >> 0x10) + CARRY2(u_var4, pu_var1_val)) * 0x100 + local_bx_6.field_0x12;
    _local_a = CONCAT22(i_var2, i32_var6);
    local_c = local_bx_6.field_0x2;
    while (_local_a != 0) {
        u_var3 = local_c + 1;
        i32_var6 = local_c >> 0xf;
        process_struct_1008_4544(local_bx_6.field_0xc);
        pass_funcs::pass1_fn_1000_48a8(
            CONCAT22(i32_var6, local_c),
            CONCAT22(local_4, local_6),
            local_bx_6.field_0x4,
        );
        i32_var6 = local_bx_6.field_0x4;
        u_var4 = -i32_var6;
        b_var7 = CARRY2(local_6, u_var4);
        local_6 = local_6 + u_var4;
        local_4 = local_4 + (b_var7 - (local_bx_6.field_0x6 + (i32_var6 != 0))) * 0x100;
        local_c = u_var3;
        _local_a = _local_a + -1;
    }
    return;
}

pub unsafe fn pass1_1008_52fc(param_1: &mut  Struct195) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let lVar4: u32;
    let mut local_DX_15: u16;
    let mut i_var5: i32;
    let mut i32_var6: i32;
    let local_bx_5: &mut  Struct195;
    let mut local_es_5: u16;
    let mut u_var7: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let temp_87f1a411929: &mut  u32;

  // local_es_5 = (param_1  >> 0x10);
    local_bx_5 = param_1;
    u_var7 = process_struct_1008_4772(local_bx_5.field_0xc);
  // local_DX_15 = (u_var7  >> 0x10);
    i32_var6 = u_var7;
    i_var5 = (i32_var6 + 4);
    u_var3 = i_var5 - 1;
    i_var5 = (i32_var6 + 6) - (i_var5 == 0);
    lVar4 = (i32_var6 + 8) + -1;
    u_var2 = param_1.field_0x0;
    temp_87f1a411929 = &local_bx_5.field_0x4;
    i32_var6 = (u_var2 >> 0xf) + (&local_bx_5.field_0x4 + 2) + CARRY2(u_var2, temp_87f1a411929);
    if ((i_var5 <= i32_var6) && (i_var5 < i32_var6 || (u_var3 < u_var2 + temp_87f1a411929))) {
        &local_bx_5.field_0x4 = u_var3 - u_var2;
        (&local_bx_5.field_0x4 + 2) = (i_var5 - (u_var2 >> 0xf)) - (u_var3 < u_var2);
    }
    u_var2 = local_bx_5.field_0x2;
    pu_var1 = &local_bx_5.field_0x8;
    i_var5 = (u_var2 >> 0xf) + (&local_bx_5.field_0x8 + 2) + CARRY2(u_var2, pu_var1);
    local_e._2_2_ = (lVar4 >> 0x10);
    if ((local_e._2_2_ <= i_var5)
        && ((
            local_e._0_2_ = lVar4,
            local_e._2_2_ < i_var5 || (local_e < u_var2 + pu_var1),
        )))
    {
        &local_bx_5.field_0x8 = local_e - u_var2;
        (&local_bx_5.field_0x8 + 2) = (local_e._2_2_ - (u_var2 >> 0xf)) - (local_e < u_var2);
    }
    return;
}

pub unsafe fn pass1_1008_570e(param_1: &mut  u16, param_2: u8) {
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_5784(param_1: &mut  Vec<u8>, param_2: u32) {
    param_1 = param_2;
    (param_1 + 4) = 0;
    return;
}

pub unsafe fn pass1_1008_57a4(param_1: u32, param_2: u32) {
    unsafe {
        unsafe { *param_1 = param_2 };
        (param_1 + 4) = 0;
    }
}

pub unsafe fn pass1_1008_57c4(param_1: &mut Struct44) {
    let u_var1: u8;
    let mut local_es_4: u16;

  // local_es_4 = (param_1  >> 0x10);
    param_1.ptr_a_lo = (s__s__s__1050_5bc0 + 4);
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    u_var1 = pass1_1008_5830((param_1 & 0xffff | local_es_4 << 0x10));
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return u_var1;
}

pub unsafe fn pass1_1008_57f0(param_1: u32, param_2: u16) -> libc::c_long {
    let mut b_var1: bool;
    let lVar2: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass1_1008_5784(CONCAT22(ctx.stack_seg_reg, local_a), param_1);
    local_c = 0;
    while {
        lVar2 = pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_a));
        if (lVar2 == 0) {
            return 0;
        }
        b_var1 = local_c != param_2;
        local_c = local_c + 1;
        b_var1
    } {}
    return lVar2;
}

pub unsafe fn pass1_1008_5830(param_1: &mut  Struct200) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let pu_var4: &mut  u32;
    let local_bx_5: &mut  Struct200;
    let mut i_var5: i32;
    let mut local_es_5: u16;
    let mut local_es_23: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7e01ca0f901: &mut  u32;
    let mut temp_5f0adb14fb: u32;
    // temp_87fa1582f55: &mut  Vec<u8>;
    let temp_8622404347c: &mut  u32;

    while (true) {
      // local_es_5 = (param_1  >> 0x10);
        local_bx_5 = param_1;
        if (local_bx_5.field_0x4 == 0) {
            break;
        }
        if (local_bx_5.field_0xa != 0) {
            temp_5f0adb14fb = local_bx_5.field_0x4;
          // local_es_23 = (temp_5f0adb14fb  >> 0x10);
            i_var5 = temp_5f0adb14fb;
            pu_var1 = (i_var5 + 8);
            u_var2 = (i_var5 + 10);
            if ((u_var2 | pu_var1) != 0) {
                unsafe { temp_87fa1582f55 = *pu_var1 };
                (**temp_87fa1582f55)();
            }
        }
        pu_var4 = local_bx_5.field_0x4;
        local_bx_5.field_0x4 = (pu_var4 + 4);
        if (pu_var4 != 0x0) {
            unsafe { ppc_var3 = *pu_var4 };
            (**ppc_var3)();
        }
    }
    local_bx_5.field_0x8 = 0;
    return;
}

pub unsafe fn pass1_fn_1008_58a6(param_1: u32, param_2: u32) {
    let pi_var1: &mut  i32;
    let local_AX_10: &mut  Struct201;
    let mut u_var2: i32;

    let mut i_var4: i32;
    let mut local_es_102: u16;
    let mut local_es_110: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let pu_var3: Vec<u8>;

    pu_var3 = _PTR_LOOP_1050_029c;
    alloc_mem_1000_07fc(_PTR_LOOP_1050_029c);
    u_var2 = pu_var3;
    local_6 = pu_var3 & 0xffff | ctx.dx_reg << 0x10;
    if ((ctx.dx_reg | u_var2) == 0) {
        local_6 = 0;
    } else {
        local_6 = ctx.s_1_1050_389a;
        (u_var2 + 2) = &ctx.PTR_LOOP_1050_1008;
        (u_var2 + 4) = 0;
        (u_var2 + 8) = 0;
        local_6 = s__s__s__1050_5bc0;
        (u_var2 + 2) = &ctx.PTR_LOOP_1050_1008;
    }
    if (local_6 == 0) {
        return;
    }
  // local_es_102 = (local_6  >> 0x10);
    (local_6 + 8) = param_2;
  // local_es_110 = (param_1  >> 0x10);
    i_var4 = param_1;
    (local_6 + 4) = (i_var4 + 4);
    (i_var4 + 4) = local_6;
    pi_var1 = (i_var4 + 8);
    unsafe { *pi_var1 = *pi_var1 + 1 };
    return;
}

pub unsafe fn pass1_1008_593c(param_1: &mut  u32, param_2: u32) {
    let pi_var1: &mut  i32;
    let ppc_var2: fn();
    let mut u_var3: i32;
    let pu_var4: Vec<u8>;

    let mut i_var5: i32;
    let mut local_es_4: u16;
    let mut local_es_150: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;
    let mut temp_5f74faf803: u32;

  // local_es_4 = (param_1  >> 0x10);
    i_var5 = param_1;
    if ((i_var5 + 8) == 0) {
        let param_1_val = unsafe { *param_1 };
        ppc_var2 = (param_1_val + 4);
        ppc_var2();
        return;
    }
    pu_var4 = _PTR_LOOP_1050_029c;
    alloc_mem_1000_07fc(_PTR_LOOP_1050_029c);
    u_var3 = pu_var4;
    local_6 = pu_var4 & 0xffff | ctx.dx_reg << 0x10;
    if ((ctx.dx_reg | u_var3) == 0) {
        local_6 = 0;
    } else {
        local_6 = ctx.s_1_1050_389a;
        (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
        (u_var3 + 4) = 0;
        (u_var3 + 8) = 0;
        local_6 = s__s__s__1050_5bc0;
        (u_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    }
    if (local_6 == 0) {
        return;
    }
    (local_6 + 8) = param_2;
    while {
        param_1 = (param_1 + 4);
      // local_es_150 = (param_1  >> 0x10);
        (param_1 + 4) != 0
    } {}
    todo!();
    //(param_1 + 4) = local_6;
    pi_var1 = (i_var5 + 8);
    unsafe { *pi_var1 = *pi_var1 + 1 };
    return;
}

pub unsafe fn pass1_1008_59f4(param_1: u32, param_2: u32) {
    let pi_var1: &mut  i32;
    let pu_var2: &mut  u32;
    let mut u_var3: i32;
    let mut local_AX_110: u16;
    let mut local_DX_110: u16;
    let mut i_var4: i32;
    let mut local_es_12: u16;
    let mut local_es_24: u16;
    let mut local_es_42: u16;
    let mut local_es_110: u16;
    let mut local_a: u32;
    let mut local_6: u32;
    let temp_8623d993e9d: &mut  u32;
    let temp_79fa913f4bf: &mut  u32;
    let mut temp_79f326fe867: u32;
    let fn_ptr_1: fn();

    local_6 = 0;
  // local_es_12 = (param_1  >> 0x10);
    temp_79f326fe867 = local_6;
    temp_79fa913f4bf = param_1;
    while {
        local_6 = temp_79f326fe867;
      // local_es_24 = (temp_79fa913f4bf  >> 0x10);
        i_var4 = temp_79fa913f4bf;
        temp_79fa913f4bf = (i_var4 + 4);
        local_a._0_2_ = temp_79fa913f4bf;
      // local_es_42 = (temp_79fa913f4bf  >> 0x10);
        if (((i_var4 + 6) | local_a) == 0) {
            break;
        }
        temp_79f326fe867 = temp_79fa913f4bf;
        (local_a + 8) != param_2
    } {}
    if (temp_79fa913f4bf != 0x0) {
        if (local_6 == 0) {
            local_AX_110 = (local_a + 4);
            local_DX_110 = (local_a + 6);
            local_6 = param_1;
        } else {
            local_AX_110 = (local_a + 4);
            local_DX_110 = (local_a + 6);
        }
      // local_es_110 = (local_6  >> 0x10);
        (local_6 + 4) = local_AX_110;
        (local_6 + 6) = local_DX_110;
        if ((param_1 + 10) != 0) {
            pu_var2 = (local_a + 8);
            u_var3 = (local_a + 10);
            if ((u_var3 | pu_var2) != 0) {
                unsafe { fn_ptr_1 = *pu_var2 };
                (**fn_ptr_1)();
            }
        }
        if (temp_79fa913f4bf != 0x0) {
            unsafe { fn_ptr_1 = *temp_79fa913f4bf };
            (**fn_ptr_1)();
        }
        pi_var1 = (param_1 + 8);
        unsafe { *pi_var1 = *pi_var1 + -1 };
        return;
    }
    return;
}

pub unsafe fn pass1_fn_1008_5ab8(param_1: u32) {
    let pi_var1: &mut  i32;
    let pu_var2: &mut  u32;
    let mut i_var3: i32;
    let mut local_es_4: u16;
    let mut u_var4: i32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    let temp_7e05aed193b: &mut  u32;
    let fn_ptr_1: fn();

  // local_es_4 = (param_1  >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 4) == 0) {
        return;
    }
    pu_var2 = (i_var3 + 4);
  // u_var4 = (pu_var2  >> 0x10);
    (i_var3 + 4) = (pu_var2 + 4);
    if ((u_var4 | pu_var2) != 0) {
        unsafe { fn_ptr_1 = *pu_var2 };
        (**fn_ptr_1)();
    }
    pi_var1 = (i_var3 + 8);
    unsafe { *pi_var1 = *pi_var1 + -1 };
    return;
}

pub unsafe fn pass1_1008_5b12(param_1: &mut Address<Struct306>) -> u32 {
    // let mut i_var1: u32;
    let mut i_var2: u32;
    // let mut local_es_23: u16;
    // let mut local_es_34: u16;
    let mut temp_5fb237ca1c: u32;

    if (param_1.field_0x0 != 0) && ((param_1.field_0x0 + 8) != 0) {
      // local_es_23 = (param_1  >> 0x10);
        // i_var1 = param_1;
        if (param_1._type.field_0x4) == 0 {
            ctx.es_reg = (param_1.field_0x0 >> 0x10);
            i_var2 = param_1.field_0x0;
        } else {
            temp_5fb237ca1c = param_1._type.field_0x4;
            //// ocal_es_34 = (temp_5fb237ca1c  >> 0x10);
            i_var2 = temp_5fb237ca1c;
        }
        param_1._type.field_0x4 = (i_var2 + 4);
        if param_1._type.field_0x4 != 0 {
            return 0;
        }
    }
    return 0;
}

pub unsafe fn pass1_fn_1008_5b9a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1008_57c4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_fn_1008_60e8(param_1: &String) -> FileObject {
    let mut u_var1: i32;
    // let in_dx: &mut  Struct199;
    let mut local_8: u16;
    let mut local_6: u16;

    if param_1 != 0x0 {
        u_var1 = get_string_index_1000_3da4(param_1);
        u_var1 = u_var1 + 1;
        process_struct_1000_179c(u_var1, ctx.dx_reg);
        if (in_dx | u_var1) != 0 {
            copy_string_1000_3d3e(CONCAT22(ctx.dx_reg, u_var1), param_1);
            return u_var1;
        }
    }
    return 0;
}

pub unsafe fn pass1_fn_1008_612e(param_1: u16, param_2: i32) {
    let mut in_ax: i32;
    let mut u_var1: i32;
    let var2: u32;
    let mut i_var3: i32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass_funcs::pass1_fn_1000_4d24();
    u_var1 = (param_2 - param_1) + 1;
    if ((u_var1 >> 0xf | u_var1) == 0) {
        return;
    }
    local_10 = 1;
    local_12 = param_1;
    loop {
        if (param_2 < local_12) {
            return;
        }
        var2 = local_10 * (0x7fff / u_var1);
      // i_var3 = (var2  >> 0x10);
        if (in_ax >> 0xf <= i_var3) {
            if (in_ax >> 0xf < i_var3) {
                return;
            }
            if (in_ax <= var2) {
                return;
            }
        }
        local_12 = local_12 + 1;
        local_10 = local_10 + 1;
    }
}

pub unsafe fn pass1_1008_6330(param_1: &mut  u16, param_2: u8) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
  // u_var2 = (param_1  >> 0x10);
    unsafe { *param_1 = 0x380a };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1008_6562(
    ctx: &mut AppContext,
    param_1: &mut Struct104,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u16,
) -> i32 {
    let mut i_var1: i32;
    let mut local_DX_57: u16;
    let mut u_var2: u16;
    let mut local_DX_103: u16;
    let mut local_DX_129: u16;
    let mut u_var3: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f1d1f0836: u32;

    let param_1_val = unsafe { *param_1 };
    if (param_1_val == 0x0) {
        return;
    }
    process_struct_1000_179c(0x1e, param_5);
    if ((param_5 | param_4) == 0) {
        param_4 = 0;
        u_var2 = 0;
    } else {
        temp_5f1d1f0836 = (param_1 + 4);
        pass1_1008_405c(
            param_4,
            param_5,
            temp_5f1d1f0836,
            (temp_5f1d1f0836 >> 0x10),
            param_2,
        );
        u_var2 = local_DX_57;
    }
    _local_6 = CONCAT22(u_var2, param_4);
    local_8 = 0;
    while (param_2 = param_2 & 0xffff0000 | (param_2 - 1), param_2 != 0) {
        i_var1 = param_3 + 1;
        process_struct_1008_4544(param_1_val);
        u_var2 = local_8 + 1;
        u_var3 = local_DX_103;
        process_struct_1008_4544(_local_6);
        pass_funcs::pass1_fn_1000_48a8(
            CONCAT22(local_DX_129, local_8),
            CONCAT22(u_var3, param_3),
            param_2._2_2_,
        );
        param_3 = i_var1;
        local_8 = u_var2;
    }
    return;
}

pub unsafe fn pass1_1008_6604(param_1: &mut  u16, param_2: u16, param_3: u16) {
    let mut u_var1: u32;

    let mut i_var2: i32;
    let mut u_var3: u32;
    let mut local_DX__1: u16;
    let mut local_DX_105: u16;
    let mut local_es_15: u16;
    let mut local_es_177: u16;
    let paVar4: &mut  Struct102;

    paVar4 = param_1;
    set_struct_1008_4016(paVar4);
  // local_es_15 = (param_1  >> 0x10);
    unsafe { *param_1 = 0x685a };
    paVar4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    alloc_mem_1000_0a48(1, 0x28, ctx.g_struct_ptr_1);
    &paVar4.field_0x10 = ctx.ax_reg;
    (&paVar4.field_0x10 + 2) = local_DX__1;
    i_var2 = param_3 * 8 + 0x1f;
    i_var2 = ((i_var2 + (i_var2 >> 0xf & 0x1f)) >> 5) << 2;
    &paVar4.field_0x18 = i_var2;
    (&paVar4.field_0x18 + 2) = i_var2 >> 0xf;
    local_DX_105 = ((i_var2 * param_2) >> 0x20);
    u_var3 = (i_var2 * param_2);
    alloc_mem_1000_0a48(1, u_var3, ctx.g_struct_ptr_1);
    &paVar4.field_0x6 = u_var3;
    (&paVar4.field_0x6 + 2) = local_DX_105;
    &paVar4.field_0x14 = &paVar4.field_0x6;
    (&paVar4.field_0x14 + 2) = local_DX_105;
    paVar4.field_0x10 = 0x28;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 4) = param_3;
    u_var1 = paVar4.field_0x10;
  // local_es_177 = (u_var1  >> 0x10);
    i_var2 = u_var1;
    (i_var2 + 8) = param_2;
    (i_var2 + 10) = param_2 >> 0xf;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 0xc) = 1;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 0xe) = 8;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 0x10) = 0;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 0x14) = paVar4.field_0x18 * param_2;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 0x20) = 0x100;
    u_var1 = paVar4.field_0x10;
    (u_var1 + 0x24) = 0x100;
    return;
}

pub unsafe fn pass1_1008_6732(param_1: &mut  Struct182) {
    let mut i_var1: i32;
    let mut local_es_3: u16;
    let mut temp_5f5866abc5: u32;

  // local_es_3 = (param_1  >> 0x10);
    i_var1 = param_1;
    param_1 = 0x685a;
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((i_var1 + 0x10) != 0) {
        temp_5f5866abc5 = (i_var1 + 0x10);
        error_check_1000_0dc6(temp_5f5866abc5, (temp_5f5866abc5 >> 0x10));
    }
    (i_var1 + 0x10) = 0;
    process_struct_1008_41bc(param_1);
    return;
}

pub unsafe fn pass1_1008_6834(param_1: u32, param_2: u8) {
    pass1_1008_6732(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_687a(param_1: &mut  Struct65, param_2: u32) {
    let mut i_var1: i32;
    let mut local_es_17: u16;

    set_struct_1008_9584(param_1, param_2);
  // local_es_17 = (param_1  >> 0x10);
    i_var1 = param_1;
    (i_var1 + 0xcc) = 0;
    (i_var1 + 0xce) = 0;
    process_struct_1008_574a((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
    param_1.ptr_a_lo = 0x6bfc;
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    (i_var1 + 0xdc) = 0;
    return;
}

pub unsafe fn pass1_1008_68ea(
    param_1: i32,
    param_2: u16,
    param_2_00: &mut  u32,
    param_4: u16,
    param_5: u16,
    param_6: i32,
) -> i32 {
    let pp_var1: fn();
    let mut local_6: u32;
    // temp_87f4899415d: &mut  Vec<u8>;

    if (param_6 == 0) {
        if ((param_1 + 0xce) != CONCAT22(param_4, param_2_00)) {
            if ((param_1 + 0xce) != 0) {
                temp_87f4899415d = ((param_1 + 0xce) + 0x10);
                (**temp_87f4899415d)();
            }
            (param_1 + 0xce) = CONCAT22(param_4, param_2_00);
            let param_2_val = unsafe { *param_2_00 };
            pp_var1 = (param_2_val + 0x10);
            (**pp_var1)();
            pp_var1 = ((param_1 + 0xce) + 0xc);
            (**pp_var1)();
            return;
        }
    } else {
        pass1_1008_3e0e(CONCAT13((param_2 >> 8), CONCAT12(param_2, param_1)));
    }
    return;
}

pub unsafe fn pass1_1008_6978(param_1: &mut  Struct675, param_2: u16, param_3: u32) {
    let mut in_ax: i32;
    let in_dx: &mut  Struct199;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    // func_ptr_1: &mut  Vec<u8>;

    process_struct_1000_179c(10, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) == 0) {
        local_6 = 0;
    } else {
        if (param_2 == 0) {
            param_2 = (param_1 + 0xcc);
        }
        unsafe { *_local_a = ctx.s_1_1050_389a };
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
        (in_ax + 4) = param_3;
        (in_ax + 8) = param_2;
        unsafe { *_local_a = 0x6c8c };
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
        local_6 = _local_a;
    }
    func_ptr_1 = ((param_1 + 0xd2) + 4);
    (**func_ptr_1)(0x1000, (param_1 + 0xd2), param_1._2_2_, local_6);
    return;
}

pub unsafe fn pass1_1008_6a04(param_1: u32) {
    let local_AX_30: Vec<u8>;


    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];
    // temp_87f48b94a86: &mut  Vec<u8>;

    pass1_1008_57a4(
        CONCAT22(ctx.stack_seg_reg, local_a),
        param_1 & 0xffff0000 | (param_1 + 0xd2),
    );
    while (true) {
        local_AX_30 = local_a;
        pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_AX_30));
        if ((ctx.dx_reg | local_AX_30) == 0) {
            break;
        }
        temp_87f48b94a86 = ((local_AX_30 + 4) + 0xc);
        (**temp_87f48b94a86)();
    }
    return;
}

pub unsafe fn pass1_1008_6a4a(param_1: u32, param_2: i32, param_3: u16, param_3_00: i32) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let local_AX_38: Vec<u8>;



    let mut local_e: [u8; 4];
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;
    // temp_87fb77c20c3: &mut  Vec<u8>;

    if (param_3_00 == 2) {
        i_var2 = param_1;
        pass1_1008_57a4(
            CONCAT22(ctx.stack_seg_reg, local_e),
            param_1 & 0xffff0000 | (i_var2 + 0xd2),
        );
        while {
            local_AX_38 = local_e;
            pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_AX_38));
            _local_6 = CONCAT22(ctx.dx_reg, local_AX_38);
            if ((ctx.dx_reg | local_AX_38) == 0) {
                break;
            }
            (local_AX_38 + 8) != param_2
        } {}
        if (_local_6 != 0) {
            temp_87fb77c20c3 = ((i_var2 + 0xd2) + 0xc);
            (**temp_87fb77c20c3)();
            local_a = 0;
            local_6 = local_e;
            pass1_1008_5b12(CONCAT22(ctx.stack_seg_reg, local_6));
            if ((ctx.dx_reg | local_6) != 0) {
                pp_var1 = ((local_6 + 4) + 0x10);
                local_4 = ctx.dx_reg;
                (**pp_var1)();
                (i_var2 + 0xce) = (local_6 + 4);
                return;
            }
            (i_var2 + 0xce) = 0;
        }
    }
    return;
}

pub unsafe fn pass1_1008_6b02(param_1: u32) {
    let mut i_var1: i32;
    let mut local_es_3: u16;
    // fn_ptr_1: &mut  Vec<u8>;

  // local_es_3 = (param_1  >> 0x10);
    i_var1 = param_1;
    if (((i_var1 + 0xd0) | (i_var1 + 0xce)) != 0) {
        fn_ptr_1 = ((i_var1 + 0xce) + 0x6c);
        (**fn_ptr_1)();
    }
    return;
}

pub unsafe fn pass1_1008_6b5a(param_1: &mut  u16, param_2: u8) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let temp_862bb3f10cd: &mut  u32;
    // fn_ptr_1: &mut  Vec<u8>;

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    unsafe { *param_1 = 0x6c8c };
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    pu_var1 = (i_var3 + 4);
    u_var2 = (i_var3 + 6);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { fn_ptr_1 = *pu_var1 };
        (**fn_ptr_1)();
    }
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (i_var3 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_6bb4(param_1: &mut  Struct203, param_2: u8) {
    let local_AX_8: &mut  Struct203;
    let mut u_var1: u16;

    local_AX_8 = param_1;
    local_AX_8 = local_AX_8 + 1;
    pass1_1008_57c4((param_1 & 0xffff0000 | ZEXT24(local_AX_8)));
  // u_var1 = (param_1  >> 0x10);
    param_1 = 0x380a;
    local_AX_8.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1 = ctx.s_1_1050_389a;
    local_AX_8.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1008_6c30(
    param_1: u8,
    param_2: i32,
    param_3: u8,
    param_4: i32,
    param_5: bool,
    param_6: bool,
    param_7: bool,
    param_8: bool,
    param_9: bool,
) -> i32 {
    let pc_var1: String;
    let pu8_var2: Vec<u8>;
    let mut u_var3: i32;
    let mut b_var4: bool;
    let mut b_var5: bool;
    let mut b_var6: u8;
    let mut b_var7: u8;
    let mut b_var8: u8;
    let mut b_var9: u8;
    let mut b_var10: u8;
    let mut i_var11: i32;
    let mut b_var12: u8;
    u16 * *local_SP;
    u16 * *unaff_bp;
    let mut unaff_si: i32;
    let mut unaff_DI: i32;

    let mut b_var13: bool;
    let in_stack_0000f71d: &mut  Struct1;
    let paStack2265: &mut  Struct1;
    let mut cStack3: u8;
    let puStack2: &mut  u16;
    let temp_179f295966e9: &mut  u16;
    let mut byte3: u8;

    loop {
        local_SP = &puStack2;
        local_SP = &puStack2;
        byte3 = '\x0f';
        temp_179f295966e9 = unaff_bp;
        while {
            temp_179f295966e9 = temp_179f295966e9 + -1;
            local_SP = local_SP + -1;
            unsafe { *local_SP = *temp_179f295966e9 };
            byte3 = byte3 + -1;
            '\0' < byte3
        } {}
        i_var11 = param_1;
        pu8_var2 = (param_4 + i_var11);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        b_var4 = 9 < (unaff_si & 0xf);
        b_var6 = b_var4 | param_7;
        pu8_var2 = (param_4 + i_var11);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        b_var5 = 9 < (unaff_si + b_var6 * '\x06' & 0xf);
        pu8_var2 = (param_4 + i_var11);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        let pu8_var2_val = unsafe { *pu8_var2 };
        b_var7 = ((POPCOUNT(pu8_var2_val) & 1) == 0) * 0x4;
        pu8_var2 = (param_4 + i_var11);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        b_var6 = 9 < b_var7 | b_var5 | b_var6;
        b_var8 = b_var7 + b_var6 * '\x06' & 0xf;
        pu8_var2 = (param_4 + i_var11);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        b_var6 = 9 < b_var8 | b_var6;
        b_var9 = b_var8 + b_var6 * '\x06' & 0xf;
        pu8_var2 = (param_4 + unaff_DI);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        unaff_si = unaff_DI + -1;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        b_var12 = param_3 - 1;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | b_var12 };
        b_var6 = 9 < b_var9 | b_var6;
        b_var10 = b_var9 + b_var6 * '\x06' & 0xf;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | b_var12 };
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | b_var12 };
        param_3 = param_3 - 2;
        b_var12 = (param_2 >> 8);
        b_var13 = CARRY1(u8_1050_086b, b_var12);
        u8_1050_086b = u8_1050_086b + b_var12;
        pc_var1 = &cStack3 + unaff_DI;
        unsafe { *pc_var1 = *pc_var1 + b_var10 + b_var13 };
        i_var11 = (param_4 + unaff_si);
        puStack2 = unaff_bp;
        let pc_var1_val = unsafe { *pc_var1 };
        if ((POPCOUNT(pc_var1_val) & 1) == 0) {}
        // goto code_r0x10086ca6;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        let pu8_var2_val = unsafe { *pu8_var2 };
        u_var3 = (param_9 & 1) * 0x4000
            | (param_8 & 1) * 0x200
            | (param_5 & 1) * 0x100
            | (pu8_var2_val < '\0') * 0x80
            | (pu8_var2_val == 0) * 0x40
            | ((9 < b_var9) | (9 < b_var8) | (9 < b_var7) | b_var5 | b_var4 | param_7 & 1) * 0x10
            | ((POPCOUNT(pu8_var2_val) & 1) == 0) * 4;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        paStack2265 = CONCAT22(u_var3, unaff_si);
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        pu8_var2 = (unaff_DI + 0x1007);
        unsafe { *pu8_var2 = *pu8_var2 ^ param_4 };
        unsafe { b_var7 = *pu8_var2 };
        *(unaff_DI + 0x1007) = param_4;
        param_4 = (unaff_DI + 0x1007);
        param_2 = i_var11 * 0x10 + -1;
        if (param_2 == 0 || b_var7 == 0) {
            break;
        }
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        b_var10 = b_var10 - 1;
        b_var6 = 9 < (b_var10 & 0xf) | b_var6;
        b_var7 = b_var10 + b_var6 * '\x06' & 0xf;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        param_7 = (9 < b_var7 | b_var6);
        param_1 = b_var7 + param_7 * '\x06' & 0xf;
        pu8_var2 = (param_4 + unaff_si);
        unsafe { *pu8_var2 = *pu8_var2 | param_3 };
        unaff_bp = local_SP;
    }
    pu8_var2 = (param_4 + unaff_si);
    unsafe { *pu8_var2 = *pu8_var2 | param_3 };
    zero_list_1008_3e38(paStack2265);
    in_stack_0000f71d = CONCAT22(u_var3, unaff_DI + 5);
    // code_r0x10086ca6:
    zero_list_1008_3e38(in_stack_0000f71d);
    return;
}

pub unsafe fn pass1_1008_6cec(
    param_1: u32,
    param_2: u16,
    param_3: u32,
    param_4: u16,
    param_5: u32,
) {
    pass1_1008_3e76(param_1, param_4, param_5, (param_5 >> 0x10));
    pass1_1008_3e76(
        (param_1 & 0xffff0000 | (param_1 + 6)),
        param_2,
        param_3,
        (param_3 >> 0x10),
    );
    return;
}

pub unsafe fn pass1_1008_6d8a(ctx: &mut AppContext, param_1: &mut FileObject, param_2: &String) {
    let mut local_AX_49: u16;
    let mut local_DX__1: u16;

    param_1.file = 0xffff;
    ctx.PTR_LOOP_1050_0312 = ctx.PTR_DAT_0005_0000_1050_0004.clone();
    string_fn_1000_3f9c(
        ctx,
        &ctx.s__1050_65a0,
        &ctx.g_alloc_addr_1050_1050,
        &ctx._PTR_s_SC_03d_1050_0314_1050_031c,
        &ctx.PTR_DAT_0005_0000_1050_0004,
    );
    local_AX_49 = pass1_fn_1008_60e8(param_2);
    param_1 = local_AX_49;
    (param_1 + 2) = local_DX__1;
    return param_1;
}

pub unsafe fn pass1_1008_6eee(param_1: &mut FileObject, param_1_00: u32) {
    let local_AX_45: Vec<u8>;
    let b_var1: bool;
    let pu_var2: Vec<u8>;

    let mut local_e: [u8; 4];
    let mut local_a: u32;
    let mut local_6: u32;

    unsafe { local_6 = *ctx._g_bool_1050_5748 };
    local_a = local_6;
    pass1_1020_a43e(CONCAT22(ctx.stack_seg_reg, local_e));
  // local_AX_45 = write_to_file_1028_d7a0(local_a, (local_a  >> 0x10), param_1_00);
    if (local_AX_45 != 0x0) {
        b_var1 = write_to_file_1030_5c1a(_PTR_LOOP_1050_5736, param_1_00);
        if (b_var1 != 0) {
            write_to_file_1028_dce2(ctx._PTR_LOOP_1050_65e2, param_1_00);
            if (b_var1 != 0) {
                write_to_file_1038_7b20(_PTR_LOOP_1050_5a64, param_1_00);
                if (b_var1 != 0) {
                    pu_var2 = local_e;
                    call_write_to_file_1020_a644(pu_var2, ctx.stack_seg_reg, param_1_00);
                    if (pu_var2 != 0x0) {
                        return;
                    }
                }
            }
        }
    }
    return;
}

pub unsafe fn pass1_1008_7006(param_1: &mut FileObject, param_1_00: u32) -> i32 {
    let pp_var1: fn();
    let mut i_var2: i32;
    let local_DXAX_37: &mut  u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    while (true) {
        if (PTR_LOOP_1050_0334 <= local_4) {
            return 1;
        }
        local_DXAX_37 = process_struct_1010_20ba(
            ctx.g_struct_var_1050_0ed0,
            CONCAT22(local_8, (local_4 * 2 + 800)),
        );
        unsafe { pp_var1 = (*local_DXAX_37 + 8) };
        i_var2 = (**pp_var1)(0x1010, local_DXAX_37, param_1_00);
        local_8 = local_DXAX_37;
        if (i_var2 == 0) {
            break;
        }
        local_4 = local_4 + 1;
    }
    return i_var2;
}

pub unsafe fn pass1_1008_7056(param_1: &mut FileObject, param_1_00: u32) -> i32 {
    let pp_var1: fn();
    let mut i_var2: i32;
    let local_DXAX_37: &mut  u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    while (true) {
        if (PTR_LOOP_1050_0334 <= local_4) {
            return 1;
        }
        local_DXAX_37 = process_struct_1010_20ba(
            ctx.g_struct_var_1050_0ed0,
            CONCAT22(local_8, (local_4 * 2 + 800)),
        );
        unsafe { pp_var1 = (*local_DXAX_37 + 0xc) };
        i_var2 = (**pp_var1)(0x1010, local_DXAX_37, param_1_00);
        local_8 = local_DXAX_37;
        if (i_var2 == 0) {
            break;
        }
        local_4 = local_4 + 1;
    }
    return i_var2;
}

pub unsafe fn pass1_1008_766e(param_1: u32, param_2: u32) {
    let pu_var1: &mut  u32;
    let struct_a: &mut  Struct199;
    let paVar2: &mut  Struct199;
    let mut local_DX_74: u16;
    let mut u_var3: u16;

    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    param_2 = 0;
    local_6 = 0;
    pu_var1 = &local_6;
    read_file_1008_76e4(param_1, pu_var1, ctx.stack_seg_reg);
    if (pu_var1 != 0x0) {
        if (local_6 != 0) {
            paVar2 = struct_a;
            process_struct_1000_179c(0xc, struct_a);
            if ((paVar2 | pu_var1) == 0) {
                pu_var1 = 0x0;
                u_var3 = 0;
            } else {
                pass1_1010_8ef2(CONCAT22(paVar2, pu_var1));
                u_var3 = local_DX_74;
            }
            param_2 = pu_var1;
            (param_2 + 2) = u_var3;
            pass1_1010_905e(param_2, local_6);
        }
        return;
    }
    return;
}

pub unsafe fn pass1_1008_7f06(
    param_1: u8,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: bool,
    param_6: u8,
    param_7: u8,
    param_8: u8,
    param_9: u8,
) {
    let pb_var1: Vec<u8>;
    let mut b_var2: bool;
    let mut b_var3: bool;
    let mut b_var4: bool;
    let mut b_var5: bool;
    let mut b_var6: u8;
    let mut b_var7: u8;
    let mut u_var8: u32;
    let mut struct_var9: &mut Struct65;
    let mut b_var10: u8;
    let mut ppu_var11: Vec<u16>;
    let mut pu_var12: Vec<u8>;
    let mut b_var13: u8;
    let mut b_var14: u8;
    let mut b_var15: u8;
    let mut hgdiobj_var16: HGDIOBJ16;
    let hcursor_var17: HCURSOR16;
    let mut b_var18: u8;
    let mut b_var19: u8;
    let mut u_var20: u16;
    let mut u_var21: u16;
    let mut i_var22: i32;
    u16 * *local_SP;
    u16 * *unaff_bp;
    let mut unaff_si: i32;
    let local_SI_28: Vec<u8>;
    let mut unaff_DI: i32;
    let mut local_es_112: u16;
    let mut local_es_174: u16;
    let local_es_214: Vec<u8>;

    let mut u_var23: i32;
    let mut iStack2251: i32;
    u16 * *ppuStack2249;
    // local_8c7: &mut  Vec<u8>;
    let mut uStack2245: u16;
    let mut uStack2243: u16;
    let mut uStack2241: u16;
    let mut uStack2239: i32;
    let puStack2237: Vec<u8>;
    let mut auStack2235: [u8; 2201];
    u16 * *ppuStack34;
    let puStack2: &mut  u16;
    let temp_179f862c9b4a: &mut  u16;
    let temp_5f36b4e76f: &mut  u16;
    let mut temp_5f3d48f035: u32;
    let mut temp_5fd4f68048: u32;
    // temp_32379f6118dbc5: &mut  Vec<u8>;
    let temp_36379f6118dbc5: Vec<u8>;
    let mut char_8: u8;

    while (true) {
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        local_SP = &puStack2;
        char_8 = '\x0f';
        temp_179f862c9b4a = unaff_bp;
        while {
            temp_179f862c9b4a = temp_179f862c9b4a + -1;
            local_SP = local_SP + -1;
            unsafe { *local_SP = *temp_179f862c9b4a };
            char_8 = char_8 + -1;
            '\0' < char_8
        } {}
        iStack2251 = param_1;
        pb_var1 = (param_4 + iStack2251);
        b_var18 = param_3;
        unsafe { *b_var1 = *pb_var1 | b_var18 };
        puStack2237 = auStack2235;
        b_var2 = 9 < (unaff_si & 0xf);
        b_var13 = b_var2 | param_6;
        b_var10 = unaff_si + b_var13 * '\x06' & 0xf;
        pb_var1 = (param_4 + iStack2251);
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        local_8c7 = &puStack2237;
        b_var3 = 9 < b_var10;
        b_var6 = b_var3 | b_var13;
        uStack2239 = CONCAT11(
            (unaff_si >> 8) + b_var13 + b_var6,
            b_var10 + b_var6 * '\x06',
        ) & 0xff0f;
        pb_var1 = (param_4 + iStack2251);
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        unsafe { b_var13 = *pb_var1 };
        b_var10 = ((POPCOUNT(b_var13) & 1) == 0) * 0x4;
        pb_var1 = (param_4 + iStack2251);
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        b_var4 = 9 < b_var10;
        b_var6 = b_var4 | b_var6;
        b_var10 = b_var10 + b_var6 * '\x06' & 0xf;
        pb_var1 = (param_4 + iStack2251);
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        b_var5 = 9 < b_var10;
        b_var7 = b_var5 | b_var6;
        b_var14 = b_var10 + b_var7 * '\x06' & 0xf;
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        local_SI_28 = (unaff_DI + -1);
        u_var23 = (param_9 & 1) * 0x4000
            | SBORROW2(unaff_DI, 1) * 0x800
            | (param_8 & 1) * 0x200
            | (param_7 & 1) * 0x100
            | (local_SI_28 < 0) * 0x80
            | (local_SI_28 == 0x0) * 0x40
            | (b_var5 | b_var4 | b_var3 | b_var2 | param_6 & 1) * 0x10
            | ((POPCOUNT(local_SI_28 & 0xff) & 1) == 0) * 4;
        pb_var1 = local_SI_28 + param_4;
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        pb_var1 = local_SI_28 + param_4;
        unsafe { *pb_var1 = *pb_var1 | b_var18 };
        u_var20 = param_3 - 1;
        pb_var1 = local_SI_28 + param_4;
        b_var19 = u_var20;
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        b_var18 = 9 < b_var14 | b_var7;
        b_var15 = b_var14 + b_var18 * '\x06' & 0xf;
        pb_var1 = local_SI_28 + param_4;
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        pb_var1 = local_SI_28 + param_4;
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        let local_si_28_val = unsafe { *local_SI_28 };
        out(local_si_28_val, u_var20);
        b_var10 = 9 < b_var15 | b_var18;
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        ppu_var11 = local_SP;
        temp_32379f6118dbc5 = &puStack2237;
        uStack2245 = param_4;
        uStack2243 = param_3;
        uStack2241 = param_2;
        pu_var12 = auStack2235;
        temp_36379f6118dbc5 = local_SP;
        puStack2 = unaff_bp;
        let pb_var1_val = unsafe { *pb_var1 };
        if (pb_var1_val == 0) {
            // goto LAB_1008_7f73;
        }
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        let pb_var1_val = unsafe { *pb_var1 };
        if ((POPCOUNT(pb_var1_val) & 1) == 0) {
            break;
        }
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        unsafe { b_var13 = *pb_var1 };
        unsafe { b_var6 = *pb_var1 };
        unsafe { b_var7 = *pb_var1 };
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        pb_var1 = (param_4 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        pb_var1 = (&ctx.PTR_LOOP_1050_1008 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 ^ param_4 };
        unsafe { b_var18 = *pb_var1 };
        *(&ctx.PTR_LOOP_1050_1008 + unaff_DI) = param_4;
        u_var21 = *(&ctx.PTR_LOOP_1050_1008 + unaff_DI);
        if (param_2 - 1 == 0 || b_var18 == 0) {
            pb_var1 = (u_var21 + unaff_DI);
            unsafe { *pb_var1 = *pb_var1 | b_var19 };
            pass1_1008_687a(
                CONCAT22(
                    unaff_DI,
                    (param_9 & 1) * 0x4000
                        | (param_8 & 1) * 0x200
                        | (param_7 & 1) * 0x100
                        | (b_var13 < '\0') * 0x80
                        | (b_var6 == 0) * 0x40
                        | ((9 < b_var15)
                            | (9 < b_var14)
                            | b_var5
                            | b_var4
                            | b_var3
                            | b_var2
                            | param_6 & 1)
                            * 0x10
                        | ((POPCOUNT(b_var7) & 1) == 0) * 4,
                ),
                CONCAT22(0x37, u_var20),
            );
            local_SP = &stack0xf721;
            ppu_var11 = local_SP;
            temp_32379f6118dbc5 = local_8c7;
            pu_var12 = puStack2237;
            temp_36379f6118dbc5 = local_SP;
            // LAB_1008_7f73:
            local_SP = temp_36379f6118dbc5;
            puStack2237 = pu_var12;
            local_8c7 = temp_32379f6118dbc5;
            local_SP = ppu_var11;
            temp_5f36b4e76f = (local_SP + 6);
          // local_es_112 = (temp_5f36b4e76f  >> 0x10);
            i_var22 = temp_5f36b4e76f;
            (i_var22 + 0xde) = (local_SP + 10);
            unsafe { *temp_5f36b4e76f = 0x8042 };
            (i_var22 + 2) = &ctx.PTR_LOOP_1050_1008;
            copy_string_1000_3d3e(
                (temp_5f36b4e76f & 0xffff0000 | (i_var22 + 0x5b)),
                s_SOLChildPar_1050_0358,
            );
            hgdiobj_var16 = GetStockObject16(5);
            temp_5f3d48f035 = (local_SP + 6);
            (temp_5f3d48f035 + 0xc6) = hgdiobj_var16;
            hcursor_var17 = LoadCursor16(0x7f00, 0);
            u_var8 = (local_SP + 6);
          // local_es_174 = (u_var8  >> 0x10);
            i_var22 = u_var8;
            (i_var22 + 0xc4) = hcursor_var17;
            (i_var22 + 200) = (s_572_bmp_1050_2007 + 1);
            (i_var22 + 0xac) = 0x44000000;
            u_var8 = (local_SP + 0xc);
            temp_5fd4f68048 = (local_SP + 6);
            (temp_5fd4f68048 + 0xbc) = (u_var8 + 8);
            struct_var9 = (local_SP + 6);
          // local_es_214 = (struct_var9  >> 0x10);
            (struct_var9 + 0xca) = (struct_var9 + 0xde);
            reg_class_1008_96d2(struct_var9, u_var23);
            return CONCAT22((local_SP + 8), (local_SP + 6));
        }
        pb_var1 = (u_var21 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        b_var13 = (b_var15 + b_var10 * '\x06' & 0xf) - 1;
        b_var10 = 9 < (b_var13 & 0xf) | b_var10;
        b_var13 = b_var13 + b_var10 * '\x06' & 0xf;
        pb_var1 = (u_var21 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        param_6 = 9 < b_var13 | b_var10;
        param_1 = b_var13 + param_6 * '\x06' & 0xf;
        pb_var1 = (u_var21 + unaff_DI);
        unsafe { *pb_var1 = *pb_var1 | b_var19 };
        param_2 = param_2 - 1;
        param_3 = u_var20;
        param_4 = u_var21;
        unaff_bp = local_SP;
        unaff_si = unaff_DI;
    }
    cRam105007c7 = cRam105007c7
        + ((param_9 & 1) * '@'
            | (param_8 & 1) * 0x2
            | param_7 & 1
            | ((b_var13 < '\0') * 0x80 >> 8))
        + b_var6
        + b_var7
        + b_var18
        + b_var10;
    bRam105047c7 = bRam105047c7 & 2;
    pb_var1 = (unaff_DI + param_4);
    unsafe { *pb_var1 = *pb_var1 | b_var19 + 1 };
    local_SP = local_SP;
    ppu_var11 = local_SP;
    temp_32379f6118dbc5 = &puStack2237;
    pu_var12 = auStack2235;
    temp_36379f6118dbc5 = local_SP;
    // goto LAB_1008_7f73;
}

pub unsafe fn pass1_1008_7ffa(param_1: &mut  u16, param_2: u8) {
    let mut i_var1: i32;
    let mut u_var2: u16;

    i_var1 = param_1;
    pass1_1008_57c4((param_1 & 0xffff0000 | (i_var1 + 0xd2)));
  // u_var2 = (param_1  >> 0x10);
    unsafe { *param_1 = 0x380a };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (i_var1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return;
}

pub unsafe fn pass1_1008_8aa2(param_1: &mut  u16) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let mut u_var3: i32;
    let ppcVar4: fn();
    let mut i_var5: i32;
    let mut local_es_4: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let temp_8628f43bde0: &mut  u32;
    let mut temp_5fc30efdd1: u32;

  // local_es_4 = (param_1  >> 0x10);
    i_var5 = param_1;
    unsafe { *param_1 = 0x8e9a };
    (i_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
    temp_5fc30efdd1 = (i_var5 + 4);
    if ((temp_5fc30efdd1 + 0x1c) != 0) {
        pu_var1 = (i_var5 + 4);
        u_var2 = (i_var5 + 6);
        if ((u_var2 | pu_var1) != 0) {
            unsafe { ppcVar4 = *pu_var1 };
            (**ppcVar4)();
        }
    }
    u_var2 = (i_var5 + 0x3a);
    u_var3 = (i_var5 + 0x3c);
    _local_10 = CONCAT22(u_var3, u_var2);
    if ((u_var3 | u_var2) != 0) {
        pass1_1008_5118(CONCAT22(u_var3, u_var2));
        error_check_1000_17ce(_local_10);
    }
    unsafe { *param_1 = ctx.s_1_1050_389a };
    (i_var5 + 2) = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_8b20(param_1: &mut  Struct206) {
    let pi_var1: &mut  i32;
    let mut i_var2: i32;
    let mut local_AX_71: u16;
    let mut local_DX_71: u16;
    let local_struct_1: &mut  Struct206;
    let mut local_es_6: u16;

    let mut local_1c: u32;
    let mut local_c: u16;
    let mut local_a: [u8; 2];
    let mut local_8: [u8; 2];
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f83cd8e8d: u32;
    let mut u_var3: u32;

  // local_es_6 = (param_1  >> 0x10);
    local_struct_1 = param_1;
    if (local_struct_1.field_0x8 != 0) {
        i_var2 = local_struct_1.field_0x40;
        pi_var1 = &local_struct_1.field_0x40;
        unsafe { *pi_var1 = *pi_var1 + 1 };
        if (i_var2 % local_struct_1.field_0x3e == 0) {
            local_struct_1.field_0x40 = 1;
            u_var3 = local_struct_1.field_0x8;
            pass1_1018_20ee(_PTR_LOOP_1050_0382, u_var3);
            u_var3 = u_var3 & 0xffff | local_DX_71 << 0x10;
            pass1_1008_3e94(
                &local_struct_1.field_0x28,
                CONCAT22(ctx.stack_seg_reg, local_a),
                CONCAT22(ctx.stack_seg_reg, local_8),
            );
            process_struct_1008_8d8a(
                (param_1 & 0xffff | local_es_6 << 0x10),
                u_var3,
                local_struct_1.field_0x4,
            );
            pass1_1008_4480(
                local_struct_1.field_0x4,
                (param_1 & 0xffff0000 | ZEXT24(&local_struct_1.field_0x28)),
                u_var3,
            );
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1008_8bc6(param_1: u32) {
    let mut local_AX_42: u16;
    let mut local_DX_42: u16;
    let mut i_var2: i32;
    let mut local_es_6: u16;

    let mut local_1a: u32;
    let mut local_a: [u8; 2];
    let mut local_8: [u8; 2];
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5f9f6c97b2: u32;
    let mut u_var1: u32;

  // local_es_6 = (param_1  >> 0x10);
    i_var2 = param_1;
    if ((i_var2 + 8) == 0) {
        return;
    }
    u_var1 = (i_var2 + 8);
    pass1_1018_20ee(_PTR_LOOP_1050_0382, u_var1);
    u_var1 = u_var1 & 0xffff | local_DX_42 << 0x10;
    pass1_1008_3e94(
        (i_var2 + 0x28),
        CONCAT22(ctx.stack_seg_reg, local_a),
        CONCAT22(ctx.stack_seg_reg, local_8),
    );
    process_struct_1008_8d8a(
        (param_1 & 0xffff | local_es_6 << 0x10),
        u_var1,
        (i_var2 + 4),
    );
    pass1_1008_4480(
        (i_var2 + 4),
        (param_1 & 0xffff0000 | ZEXT24((i_var2 + 0x28))),
        u_var1,
    );
    return;
}

pub unsafe fn pass1_1008_8c4e(in_struct: &mut  Struct207, param_2: u32) {
    let mut u_var1: i32;
    let mut u_var2: u16;
    let mut u_var3: u32;
    let struct_a: &mut  Struct199;
    let paVar4: &mut  Struct199;
    let mut local_DX_97: u16;
    let mut u_var5: u16;
    let struct_1: &mut  Struct207;
    let mut local_es_4: u16;
    let mut u_var6: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

  // local_es_4 = (in_struct  >> 0x10);
    struct_1 = in_struct;
    u_var6 = process_struct_1008_4772(struct_1.Struct104_field_4);
  // struct_a = (u_var6  >> 0x10);
    u_var1 = 0;
    if ((struct_1.field_0xc == 0) || (struct_1.field_0xe == 0)) {
        paVar4 = struct_a;
        process_struct_1000_179c(0x14, struct_a);
        _local_e = CONCAT22(paVar4, u_var1);
        if ((paVar4 | u_var1) == 0) {
            u_var2 = 0;
            u_var5 = 0;
        } else {
            u_var3 = in_struct & 0xffff0000 | ZEXT24(struct_1 + 1);
            process_struct_1008_50c2(_local_e, (u_var6 + 8), (u_var6 + 4), u_var3, param_2);
            u_var2 = u_var3;
            u_var5 = local_DX_97;
        }
        pass1_1008_5134(CONCAT22(u_var5, u_var2));
    }
    pass1_1008_4480(
        param_2,
        (in_struct & 0xffff0000 | ZEXT24(struct_1 + 1)),
        struct_1.Struct104_field_4,
    );
    return;
}
