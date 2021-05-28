use crate::pass::pass16_funcs;
use crate::structs::prog_structs_16::Struct493;
use crate::app_context::AppContext;
use crate::sys_ops::pass1_1028_a188;
use crate::pass::pass14_funcs::{pass1_fn_1008_612e, pass1_1008_5b12, pass1_1008_5784, pass1_1008_3e76, pass1_1008_6cec, pass1_1008_3eb4, pass1_1008_3e94, pass1_1008_57c4};
use crate::util::{CONCAT12, CONCAT13, CONCAT22, CARRY2, SUB42, CONCAT11, CONCAT31, ZEXT24};
use crate::pass::pass17_funcs::{pass1_1030_25b2, pass1_1030_387c, pass1_1030_3548, pass1_1030_835a, pass1_1030_38b8, pass1_1030_375a, pass1_1030_730a, pass1_1030_72d0, pass1_1030_7296, pass1_1030_6c4c, pass1_1030_6d80, pass1_1030_7d7c, pass1_1030_6522, pass1_1030_64ce, pass1_1030_73a8, pass1_1030_627e, pass1_1030_1d58, pass1_1030_5b5c, pass1_1030_73ee, pass1_1030_4bbe, pass1_1030_177a, pass1_1030_61b0, pass1_1030_615a, pass1_1030_5b1c, pass1_1030_4c06, pass1_1030_4782, pass1_1028_ebee, pass1_1030_5c8a, pass1_1030_2112, pass1_1030_7e5a, pass1_1030_61fe, pass1_1030_684c, pass1_1030_6222, pass1_1030_5a80, pass1_1030_565a};
use crate::pass::pass6_funcs::{pass1_1038_3fb0, pass1_1038_4d6e, pass1_1038_52b8, pass1_1038_540a, pass1_1038_518c, pass1_1038_56d6, pass1_1038_5464, pass1_1038_3fca, pass1_1038_387e, pass1_1038_565e, pass1_1038_5770, pass1_1038_3222};
use crate::pass::pass12_funcs::{pass1_1008_c6fa, pass1_1008_c6ae, pass1_1008_c626, pass1_1008_bde0};
use crate::string_ops::misc::{pass1_1028_ae66, string_fn_1000_3f9c, pass1_1028_87f0};
use crate::pass::pass8_funcs::{pass1_1010_043a, pass1_1010_65d0};
use crate::structs::prog_structs_27::{Struct817, Struct819, Struct826, Struct825, Struct840, Struct839, Struct836, Struct835, Struct834, Struct830, Struct842, Struct848, Struct847, Struct846, Struct843};
use crate::err_ops::{error_check_1000_17ce, pass1_1030_4538};
use crate::structs::prog_structs_7::{Struct44, Struct215};
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1008_574a, process_struct_1008_8e9e, pass1_1030_44be, struct_fn_1000_160a, process_struct_1010_20ba};
use crate::structs::prog_structs_2::{Struct199, Struct7};
use crate::structs::prog_structs_21::{Struct1120, Struct845};
use crate::structs::prog_structs_26::{Struct820, Struct821, Struct822, Struct829, Struct1121, Struct374};
use crate::structs::prog_structs_5::Struct1;
use crate::structs::prog_structs_24::{Struct828, Struct841};
use crate::structs::prog_structs_12::{Struct827, Struct94};
use crate::structs::prog_structs_19::Struct500;
use crate::structs::prog_structs_28::{Struct1095, Struct781};
use crate::structs::prog_structs_29::{Struct764, Struct763};
use crate::pass::pass15_funcs::{pass1_1020_c872, pass1_1020_c860, pass1_1020_d99e, pass1_1020_e91e, pass1_1020_d888, pass1_1020_d5c8, pass1_1020_ca0c, pass1_1020_ce08, pass1_1020_d3a4, pass1_1020_e81c, pass1_1020_cd06, pass1_1020_d08e, pass1_1020_d954, pass1_1020_e8f6, pass1_1020_d866, pass1_1020_d5a6, pass1_1020_c9ea, pass1_1020_cde6, pass1_1020_d37c, pass1_1020_e7fa, pass1_1020_cce4, pass1_1020_d06c};
use crate::pass::pass13_funcs::pass1_1008_8faa;
use crate::pass::pass19_funcs::{pass1_1018_dd1e, pass1_1018_dcf6};
use crate::structs::prog_structs_15::Struct833;
use crate::list_funcs::zero_list_1008_6c90;
use crate::structs::prog_structs_30::Struct939;
use crate::pass::pass5_funcs::{pass1_1030_bcde, pass1_1030_bcae, pass1_1030_c09c, pass1_1030_c71e, pass1_1030_be56, pass1_1030_c06e, pass1_1030_c6f6, pass1_1030_be34, pass1_1030_bc24, pass1_1030_b936};
use crate::structs::prog_structs_17::Struct1115;
use crate::structs::prog_structs_20::{Struct832, Struct514};
use crate::pass::pass_funcs::pass1_1000_4906;
use crate::structs::prog_structs_9::Struct844;
use crate::pass::pass3_funcs::{pass1_1028_2812, pass1_1028_2b1c, pass1_1028_17ae, pass1_1028_5652, pass1_1028_57c8, pass1_1028_530a, pass1_1028_5a6a, pass1_1028_5e18, pass1_1028_5546, pass1_1028_58a6, pass1_1028_53e8, pass1_1028_5f00, pass1_1028_5988, pass1_1028_5c76, pass1_1028_25fc, pass1_1028_1fc8, pass1_1028_44fe, pass1_1028_3816, pass1_1028_49de, pass1_1028_26d6, pass1_1028_3ec8, pass1_1028_00cc, pass1_1028_1be8, pass1_1028_4376, pass1_1028_0b64, pass1_1028_0c50, pass1_1028_408e, pass1_1028_34a6, pass1_1028_3580, pass1_1028_3692, pass1_1028_2bfe, pass1_1028_48c0, pass1_1028_27f0, pass1_1028_2afa, pass1_1028_178c, pass1_1028_5630, pass1_1028_57a6, pass1_1028_52e8, pass1_1028_5a48, pass1_1028_5df6, pass1_1028_5524, pass1_1028_5884, pass1_1028_53c6, pass1_1028_5ed8, pass1_1028_5966, pass1_1028_5c54, pass1_1028_25da, pass1_1028_1f56, pass1_1028_44d2, pass1_1028_37a6, pass1_1028_49aa, pass1_1028_26b4, pass1_1028_3e94, pass1_1028_0068, pass1_1028_1bbc, pass1_1028_4354, pass1_1028_0b42, pass1_1028_0c24, pass1_1028_406c, pass1_1028_3484, pass1_1028_355e, pass1_1028_3670, pass1_1028_2bdc, pass1_1028_489e};
use crate::pass::pass18_funcs::{pass1_1030_d942, pass1_1030_c9e4, pass1_1030_d8f6, pass1_1030_c9a8};

pub fn pass1_1028_ed2c(param_1: u32, param_2: u16, param_3: u16, param_4: u16, param_5: u32) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    
    let mut u_var3: u16;
    let in_dx: &mut  Struct199;
    
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x1e, in_dx);
    if ((in_dx | in_ax) == 0) {
        in_ax = 0;
        u_var4 = 0;
    } else {
        pass1_1030_565a(CONCAT22(in_dx, in_ax), param_5);
        u_var4 = ctx.dx_reg;
    }
  // u_var6 = (param_1  >> 0x10);
    u_var1 = (param_1 + 0x52);
    u_var5 = u_var4;
    u_var3 = in_ax;
    pass1_1030_4782(u_var1, (u_var1 >> 0x10), 1, 1, param_2_00);
    pass1_1030_5a80(CONCAT22(u_var4, in_ax), CONCAT22(u_var5, u_var3));
    u_var2 = (in_ax + 4);
    pass1_1030_6222(ctx._PTR_LOOP_1050_5740, 1, CONCAT22(u_var5, u_var3), u_var2);
    pass1_1030_1358((param_1 + 0x16), in_ax, u_var4, u_var2 & 0xffffff);
    return;
}

pub fn pass1_1028_edc4(param_1: u32, param_2: u16, param_2_00: u32, param_4: u32) {
    let mut u_var1: u16;
    let mut u_var2: u32;
    let struct_a: &mut  Struct199;
    let paVar3: &mut  Struct199;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: [u8; 4];
    let mut local_16: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = param_2_00;
    pass1_1030_64ce(ctx._PTR_LOOP_1050_5740, param_2_00, param_4, local_1a, unaff_ss);
    u_var2 = param_2_00;
    paVar3 = struct_a;
    local_e = u_var2;
    local_a = u_var2;
    process_struct_1000_179c(0x21e, struct_a);
    u_var1 = u_var2;
    u_var4 = paVar3 | u_var1;
    if (u_var4 == 0) {
        u_var1 = 0;
        u_var4 = 0;
    } else {
        pass1_1038_3222((u_var2 & 0xffff | ZEXT24(paVar3) << 0x10), local_e, param_4);
    }
    _local_12 = CONCAT22(u_var4, u_var1);
    local_16 = (u_var1 + 4);
    pass1_1030_1358(
        (param_1 + 0x1a),
        u_var1,
        u_var4,
        local_16 & 0xffff | ((u_var1 + 6) & 0xff) << 0x10,
    );
    return;
}

pub fn pass1_1028_ee54(param_1: u32, param_2: u32, param_3: u32) {
    let pu_var1: Vec<u8>;
    
    
    let mut unaff_ss: u16;
    let mut local_26: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: [u8; 4];
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    local_6 = param_2;
    pass1_1030_64ce(ctx._PTR_LOOP_1050_5740, param_2, param_3, local_16, unaff_ss);
    local_a = param_2;
    pu_var1 = _PTR_LOOP_1050_5744;
    alloc_mem_1000_07fc(ctx._PTR_LOOP_1050_5744);
    local_e = pu_var1;
    local_c = ctx.dx_reg | local_e;
    if (local_c == 0) {
        local_e = 0;
        local_c = 0;
    } else {
        pass1_1030_684c(
            (pu_var1 & 0xffff | ctx.dx_reg << 0x10),
            local_6,
            (local_6 >> 0x10),
            local_a,
            (local_a >> 0x10),
            param_3,
        );
    }
    local_12 = (local_e + 4);
    pass1_1030_61fe(ctx._PTR_LOOP_1050_5740, local_12, local_6, param_3);
    pass1_1030_1358(
        (param_1 + 0x1e),
        local_e,
        local_c,
        local_12 & 0xffff | (local_12 & 0xff) << 0x10,
    );
    return local_12;
}

pub fn pass1_1028_ef00(param_1: u32, param_2: u16, uparam_2_00: i32, param_4: u16, param_5: u32) {
    let mut u_var1: i32;
    let in_dx: &mut  Struct199;
    
    let mut u_var2: u16;
    let pu_var3: &mut  u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u32;

    if (param_2_00 == 4) {
        process_struct_1000_179c(0x16, in_dx);
        if ((in_dx | param_2_00) != 0) {
            pass1_1030_b936(param_2_00, CONCAT22(4, in_dx), param_5);
            u_var2 = ctx.dx_reg;
            // goto LAB_1028_ef8b;
        }
    } else {
        if (param_2_00 == 0xc) {
            process_struct_1000_179c(0xe, in_dx);
            if ((in_dx | param_2_00) != 0) {
                pu_var3 = pass1_1030_bc24(param_2_00, in_dx, 0xc, param_5);
              // u_var2 = (pu_var3  >> 0x10);
                param_2_00 = pu_var3;
                // goto LAB_1028_ef8b;
            }
        } else {
            u_var1 = param_2_00;
            process_struct_1000_179c(0xe, in_dx);
            if ((in_dx | u_var1) != 0) {
                pu_var3 = pass1_1028_b22c(CONCAT22(in_dx, u_var1), param_2_00, param_5);
              // u_var2 = (pu_var3  >> 0x10);
                param_2_00 = pu_var3;
                // goto LAB_1028_ef8b;
            }
        }
    }
    param_2_00 = 0;
    u_var2 = 0;
    // LAB_1028_ef8b:
    pass1_1030_1358(
        (param_1 + 0x22),
        param_2_00,
        u_var2,
        (param_2_00 + 4) & 0xffff | ((param_2_00 + 6) & 0xff) << 0x10,
    );
    return;
}

pub fn pass1_1030_0000(param_1: u16, param_2: u16, param_3: u16) -> &mut  Struct763 {
    let in_dx: &mut  Struct199;
    let mut u_var1: i32;
    let mut in_bx: i32;
    let paVar2: &mut  Struct763;
    let pu_var3: &mut  u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    // Segment:    7
    // Offset:     000516c0
    // Length:     ef76
    // Min Alloc:  ef76
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    pu_var3 = CONCAT22(in_dx, in_bx);
    match (param_1_00 - 1) {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_489e(in_bx, in_dx);
                return paVar2;
            }
        }
        9 => {
            process_struct_1000_179c(0x22, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_2bdc(in_bx, in_dx);
                return paVar2;
            }
        }
        10 => {
            process_struct_1000_179c(0x26, in_dx);
            u_var1 = in_dx | in_bx;
            // goto joined_r0x103002a1;
        }
        0xb => {
            process_struct_1000_179c(0x2c, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_3670(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0xc => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_355e(in_bx, in_dx);
                return paVar2;
            }
        }
        0xd => {
            process_struct_1000_179c(0x26, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_3484(in_bx, in_dx);
                return paVar2;
            }
        }
        0xe => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_406c(in_bx, in_dx);
                return paVar2;
            }
        }
        0xf | 0x32 | 0x33 | 0x5f | 0x60 => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_0c24(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x10 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_0b42(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x11 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_4354(in_bx, in_dx);
                return paVar2;
            }
        }
        0x12 | 0x13 | 0x14 | 0x61 | 0x62 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass16_funcs::pass1_1028_4b84(in_bx, in_dx);
                return paVar2;
            }
        }
        0x15 | 0x16 | 0x17 => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_1bbc(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        _ => {
            // default:
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_b354(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x1a | 0x1b | 0x1c => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1030_be34(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x1d | 0x1e | 0x1f => {
            process_struct_1000_179c(0x26, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_0068(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x20 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass16_funcs::pass1_1028_50d8(in_bx, in_dx);
                return paVar2;
            }
        }
        0x21 | 0x22 | 0x23 => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_3e94(in_bx, in_dx);
                return paVar2;
            }
        }
        0x24 | 0x25 | 0x26 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1020_d06c(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x27 | 0x28 | 0x5c | 0x5d | 0x5e => {
            process_struct_1000_179c(0x22, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1030_c6f6(in_bx, in_dx);
                return paVar2;
            }
        }
        0x29 | 0x2a => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1020_cce4(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x2b => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_26b4(in_bx, in_dx);
                return paVar2;
            }
        }
        0x2c | 0x2d => {
            process_struct_1000_179c(0x2a, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_49aa(in_bx, in_dx);
                return paVar2;
            }
        }
        0x2e | 0x2f => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1020_e7fa(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x30 | 0x31 | 0x6b | 0x6c => {
            process_struct_1000_179c(0x22, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1020_d37c(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x34 | 0x35 => {
            process_struct_1000_179c(0x2c, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_37a6(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x36 => {
            process_struct_1000_179c(0x26, in_dx);
            u_var1 = in_dx | in_bx;
            // joined_r0x103002a1:
            if (u_var1 != 0) {
                pass1_1030_c06e(CONCAT22(in_dx, in_bx));
                return CONCAT22(u_var1, in_bx);
            }
        }
        0x37 | 0x38 => {
            process_struct_1000_179c(0x9a, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1030_c9a8(in_bx, in_dx);
                return paVar2;
            }
        }
        0x39 | 0x3a => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass16_funcs::pass1_1028_60bc(in_bx, in_dx);
                return paVar2;
            }
        }
        0x3b | 0x3c => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_44d2(in_bx, in_dx);
                return paVar2;
            }
        }
        0x3d => {
            process_struct_1000_179c(0x22, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1020_cde6(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x3e => {
            process_struct_1000_179c(0x26, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_1f56(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x3f => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_25da(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x40 => {
            process_struct_1000_179c(0x22, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1020_c9ea(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x46 | 0x69 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1020_d5a6(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x47 | 0x48 | 0x49 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1020_d866(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x4b | 0x4c | 0x4d => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1030_d8f6(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x4e | 0x4f | 0x50 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_5c54(in_bx, in_dx);
                return paVar2;
            }
        }
        0x51 | 0x52 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_5966(in_bx, in_dx);
                return paVar2;
            }
        }
        0x53 | 0x54 | 0x55 => {
            process_struct_1000_179c(0x22, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_5ed8(in_bx, in_dx);
                return paVar2;
            }
        }
        0x56 | 0x57 | 0x58 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_53c6(in_bx, in_dx);
                return paVar2;
            }
        }
        0x59 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_5884(in_bx, in_dx);
                return paVar2;
            }
        }
        0x5a | 0x5b => {
            process_struct_1000_179c(0x26, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_5524(in_bx, in_dx);
                return paVar2;
            }
        }
        99 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_5df6(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        100 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_5a48(in_bx, in_dx);
                return paVar2;
            }
        }
        0x65 | 0x66 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_52e8(in_bx, in_dx);
                return paVar2;
            }
        }
        0x67 | 0x68 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_57a6(in_bx, in_dx);
                return paVar2;
            }
        }
        0x6d => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_5630(in_bx, in_dx);
                return paVar2;
            }
        }
        0x6f | 0x70 | 0x71 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) == 0) {
                pu_var3 = 0x0;
            } else {
                pu_var3 = pass1_1020_d866(CONCAT22(in_dx, in_bx));
            }
        }
        0x72 | 0x76 => {
            process_struct_1000_179c(0x26, (pu_var3 >> 0x10));
            if (pu_var3 != 0x0) {
                paVar2 = pass1_1020_e8f6(pu_var3);
                return paVar2;
            }
        }
        0x73 | 0x77 | 0x78 => {
            process_struct_1000_179c(0x2c, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1020_d954(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x74 => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_178c(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x75 => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_2afa(CONCAT22(in_dx, in_bx));
                return paVar2;
            }
        }
        0x79 | 0x7a | 0x7b | 0x7c | 0x7d | 0x7e => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                paVar2 = pass1_1028_27f0(in_bx, in_dx);
                return paVar2;
            }
        }
    }
    return 0x0;
}

pub fn pass1_1030_07ac(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: Vec<u8>,
) -> i32 {
    let in_dx: &mut  Struct764;
    let mut u_var1: i32;
    let in_bx: &mut  Struct764;
    let pu_var2: &mut  u16;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut in_stack_0000fff4: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = param_6;
  // u_var4 = (param_6  >> 0x10);
    match param_2_00 - 1 {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_48c0(in_bx, in_dx, param_2_00, param_6);
            }
        }
        9 => {
            process_struct_1000_179c(0x22, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_2bfe(in_bx, in_dx, param_2_00, param_6);
            }
        }
        10 => {
            process_struct_1000_179c(0x26, in_dx);
            u_var1 = in_dx | in_bx;
            // goto joined_r0x10300adb; },
        }
        0xb => {
            process_struct_1000_179c(0x2c, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_3692(
                    CONCAT22(in_dx, in_bx),
                    param_2_00,
                    u_var3,
                    CONCAT22(in_stack_0000fff4, u_var4),
                );
            }
        }
        0xc => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_3580(
                    CONCAT22(in_dx, in_bx),
                    param_2_00,
                    u_var3,
                    CONCAT22(in_stack_0000fff4, u_var4),
                );
            }
        }
        0xd => {
            process_struct_1000_179c(0x26, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_34a6(
                    CONCAT22(in_dx, in_bx),
                    param_2_00,
                    u_var3,
                    CONCAT22(in_stack_0000fff4, u_var4),
                );
            }
        }
        0xe => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_408e(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0xf | 0x32 | 0x33 | 0x5f | 0x60 => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_0c50(in_bx, CONCAT22(param_2_00, in_dx), param_6);
            }
        }
        0x10 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_0b64(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x11 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_4376(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x12 | 0x13 | 0x14 | 0x61 | 0x62 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass16_funcs::pass1_1028_4ba6(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x15 | 0x16 | 0x17 => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_1be8(in_bx, CONCAT22(param_2_00, in_dx), param_6);
            }
        }
        _ => {
            // default:
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_b39e(CONCAT22(in_dx, in_bx), param_2_00, param_6);
            }
        }
        0x1a | 0x1b | 0x1c => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1030_be56(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x1d | 0x1e | 0x1f => {
            process_struct_1000_179c(0x26, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_00cc(in_bx, CONCAT22(param_2_00, in_dx), param_6);
            }
        }
        0x20 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass16_funcs::pass1_1028_50fa(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x21 | 0x22 | 0x23 => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_3ec8(
                    CONCAT22(in_dx, in_bx),
                    param_2_00,
                    u_var3,
                    CONCAT22(in_stack_0000fff4, u_var4),
                );
            }
        }
        0x24 | 0x25 | 0x26 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1020_d08e(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x27 | 0x28 | 0x5c | 0x5d | 0x5e => {
            process_struct_1000_179c(0x22, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1030_c71e(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x29 | 0x2a => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1020_cd06(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x2b => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_26d6(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x2c | 0x2d => {
            process_struct_1000_179c(0x2a, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_49de(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x2e | 0x2f => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1020_e81c(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x30 | 0x31 | 0x6b | 0x6c => {
            process_struct_1000_179c(0x22, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1020_d3a4(CONCAT22(in_dx, in_bx), param_1_00, param_2_00, param_6);
            }
        }
        0x34 | 0x35 => {
            process_struct_1000_179c(0x2c, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_3816(in_bx, CONCAT22(param_2_00, in_dx), param_6);
            }
        }
        0x36 => {
            process_struct_1000_179c(0x26, in_dx);
            u_var1 = in_dx | in_bx;
            // joined_r0x10300adb:
            if (u_var1 != 0) {
                pass1_1030_c09c(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x37 | 0x38 => {
            process_struct_1000_179c(0x9a, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1030_c9e4(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x39 | 0x3a => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass16_funcs::pass1_1028_611e(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x3b | 0x3c => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_44fe(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x3d => {
            process_struct_1000_179c(0x22, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1020_ce08(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x3e => {
            process_struct_1000_179c(0x26, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_1fc8(in_bx, CONCAT22(param_2_00, in_dx), param_6);
            }
        }
        0x3f => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_25fc(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x40 => {
            process_struct_1000_179c(0x22, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1020_ca0c(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x46 | 0x69 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1020_d5c8(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x47 | 0x48 | 0x49 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1020_d888(in_bx, in_dx, param_2_00, u_var3);
            }
        }
        0x4b | 0x4c | 0x4d => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1030_d942(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x4e | 0x4f | 0x50 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_5c76(in_bx, CONCAT22(param_2_00, in_dx), param_6);
            }
        }
        0x51 | 0x52 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_5988(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x53 | 0x54 | 0x55 => {
            process_struct_1000_179c(0x22, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_5f00(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x56 | 0x57 | 0x58 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_53e8(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x59 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_58a6(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x5a | 0x5b => {
            process_struct_1000_179c(0x26, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_5546(in_bx, in_dx, param_2_00, param_6);
            }
        }
        99 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_5e18(in_bx, in_dx, param_2_00, param_6);
            }
        }
        100 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_5a6a(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x65 | 0x66 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_530a(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x67 | 0x68 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_57c8(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x6d => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_5652(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x6f | 0x70 | 0x71 => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) == 0) {
                in_bx = 0x0;
                in_dx = 0x0;
            } else {
                pu_var2 = pass1_1020_d888(in_bx, in_dx, param_2_00, u_var3);
              // in_dx = (pu_var2  >> 0x10);
                in_bx = pu_var2;
            }
        }
        0x72 | 0x76 => {
            process_struct_1000_179c(0x26, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1020_e91e(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x73 | 0x77 | 0x78 => {
            process_struct_1000_179c(0x2c, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1020_d99e(CONCAT22(in_dx, in_bx), param_1_00, param_2_00, param_6);
            }
        }
        0x74 => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_17ae(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x75 => {
            process_struct_1000_179c(0x24, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_2b1c(in_bx, in_dx, param_2_00, param_6);
            }
        }
        0x79 | 0x7a | 0x7b | 0x7c | 0x7d | 0x7e => {
            process_struct_1000_179c(0x20, in_dx);
            if ((in_dx | in_bx) != 0) {
                pass1_1028_2812(in_bx, in_dx, param_2_00, param_6);
            }
        }
    }
    return 0;
}

pub fn pass1_1030_10b0(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_3_00: u32,
    param_5: Vec<u8>,
) {
    let mut u_var1: u32;
    
    let paVar2: &mut  Struct493;
    let mut in_dx: u16;
    let mut u_var3: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_07ac(
        param_1,
        param_2,
        param_3,
        param_3_00,
        (param_3_00 >> 0x10),
        param_5,
    );
    u_var1 = (in_ax + 4);
    u_var3 = in_dx;
  // paVar2 = pass1_1028_e1ec(CONCAT22(param_2, param_1), param_5, (param_5  >> 0x10));
    if ((u_var3 | paVar2) != 0) {
        pass1_1030_7e5a(CONCAT22(u_var3, paVar2), u_var1);
    }
    local_a = (u_var1 >> 0x10);
    pass1_1030_1358(
        (param_1 + 0x26),
        in_ax,
        in_dx,
        u_var1 & 0xffff | (local_a & 0xff) << 0x10,
    );
    return;
}

pub fn pass1_1030_1120(param_1: u32) {
    
    let in_dx: &mut  Struct199;
    
    let mut u_var1: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x3b2, in_dx);
    if ((in_dx | in_ax) == 0) {
        in_ax = 0;
        u_var1 = 0;
    } else {
        pass1_1030_2112(in_ax, in_dx, 0);
        u_var1 = ctx.dx_reg;
    }
    pass1_1030_1358(
        (param_1 + 0x2a),
        in_ax,
        u_var1,
        (in_ax + 4) & 0xffff | ((in_ax + 6) & 0xff) << 0x10,
    );
    return;
}

pub fn pass1_1030_117a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_11aa(param_1: &mut  Struct846, param_2: u32, param_3: u32) {
    let local_bx_4: &mut  Struct843;
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    param_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = 0;
    local_bx_4.field_0x6 = 0;
    local_bx_4.field_0xa = 0;
    local_bx_4.field_0xe = param_3;
    local_bx_4.field_0x12 = 0;
    local_bx_4.field_0x16 = param_2;
    local_bx_4.field_0x1a = 1;
    param_1 = (s_462_bmp_1050_1620 + 4);
    local_bx_4.field_0x2 = 0x1030;
    if (local_bx_4.field_0xe == 0) {
        local_bx_4.field_0xe = 5;
    }
    if (local_bx_4.field_0x16 == 0) {
        local_bx_4.field_0x16 = 5;
    }
    pass1_1030_1550(param_1);
    local_bx_4.field_0x6 = 0;
    return;
}

pub fn pass1_1030_1244(param_1: &mut  u16) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut u_var3: i32;
    let ppcVar4: fn();
    let mut u_var5: u32;
    let mut i32_var6: i32;
    let mut i_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut local_6: u32;

  // u_var9 = (param_1  >> 0x10);
    i32_var6 = param_1;
    unsafe { *param_1 = (s_462_bmp_1050_1620 + 4) };
    (i32_var6 + 2) = 0x1030;
    if ((i32_var6 + 0x1a) != 0) {
        local_6 = 1;
        while (true) {
            pu_var1 = (i32_var6 + 10);
            let pu_var1_val = unsafe { *pu_var1 };
            if (pu_var1_val < local_6 || pu_var1_val == local_6) {
                break;
            }
            i_var8 = local_6 * 4;
            u_var5 = (i32_var6 + 6);
          // u_var10 = (u_var5  >> 0x10);
            i_var7 = u_var5;
            pu_var2 = (i_var7 + i_var8);
            u_var3 = (i_var7 + i_var8 + 2);
            if ((u_var3 | pu_var2) != 0) {
                unsafe {
                    ppcVar4 = *pu_var2;
                    (**ppcVar4)();
                }
            }
            local_6 = local_6 + 1;
        }
    }
    error_check_1000_17ce((i32_var6 + 6));
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
        (i32_var6 + 2) = &ctx.PTR_LOOP_1050_1008;
    }
    return;
}

pub fn pass1_1030_12ca(param_1: u32) {
    let pu_var1: &mut  u32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut local_6: u32;

    local_6 = 1;
    while (true) {
      // u_var4 = (param_1  >> 0x10);
        i_var3 = param_1;
        pu_var1 = (i_var3 + 10);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val < local_6 || pu_var1_val == local_6) {
            (i_var3 + 4) = 0;
            return;
        }
        u_var2 = (i_var3 + 6);
        if ((u_var2 + local_6 * 4) == 0) {
            break;
        }
        local_6 = local_6 + 1;
    }
    return;
}

pub fn pass1_1030_1312(a: u16, b: u16, c: u16) {
    let mut local_6: u32;

    return;
}

pub fn pass1_1030_1358(param_1: u32, param_2: u16, param_3: u16, param_4: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let mut u_var6: u16;
    let mut bVar7: bool;

    if (param_4 == 0) {
        return;
    }
  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 10);
    unsafe {
        if ((*pu_var1 < param_4 || *pu_var1 == param_4) || ((i_var4 + 6) == 0)) {
            pu_var2 = (i_var4 + 0x14);
            bVar7 = *pu_var2 < param_4;
            if ((bVar7 || *pu_var2 == param_4)
                && (bVar7
                    || (
                        pu_var2 = (i_var4 + 0x12),
                        *pu_var2 < param_4 || *pu_var2 == param_4,
                    )))
            {
                pass1_1030_1550((param_1 & 0xffff | u_var5 << 0x10));
            }
            pu_var1 = (i_var4 + 0x12);
            if (*pu_var1 < param_4 || *pu_var1 == param_4) {
                return;
            }
            if ((i_var4 + 6) == 0) {
                return;
            }
            pu_var2 = (i_var4 + 0xc);
            bVar7 = *pu_var2 < param_4;
            if ((bVar7 || *pu_var2 == param_4)
                && (bVar7
                    || (
                        pu_var2 = (i_var4 + 10),
                        *pu_var2 < param_4 || *pu_var2 == param_4,
                    )))
            {
                (i_var4 + 10) = (param_4 + 1);
                (i_var4 + 0xc) = (param_4 + 1 >> 0x10);
            }
        }
    }
    u_var3 = (i_var4 + 6);
  // u_var6 = (u_var3  >> 0x10);
    i_var4 = u_var3;
    (i_var4 + param_4 * 4) = param_2;
    (i_var4 + param_4 * 4 + 2) = param_3;
    return;
}

pub fn pass1_1030_13f6(param_1: u32, param_2: u32) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let pu_var3: &mut  u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_4 = 0;
    pu_var3 = pass1_1030_1312(param_1, param_2);
    if (pu_var3 != 0x0) {
        local_4 = 1;
      // u_var2 = (param_1  >> 0x10);
        if (((param_1 + 0x1a) != 0) && (pu_var3 != 0x0)) {
            unsafe {
                pp_var1 = *pu_var3;
                (**pp_var1)();
            }
        }
        pass1_1030_1358(param_1, 0, 0, param_2);
        (param_1 + 4) = 1;
    }
    return local_4;
}

pub fn pass1_1030_145a(param_1: &mut  Struct844, param_2: libc::c_long) {
    let mut u_var1: u32;
    let mut u_var2: u16;
    let local_bx_4: &mut  Struct844;
    let mut u_var3: u16;

  // u_var3 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    error_check_1000_17ce(local_bx_4.field_0x6);
    local_bx_4.field_0x6 = 0;
    local_bx_4.field_0xa = 0;
    u_var1 = local_bx_4.field_0x16 + param_2;
  // u_var2 = (u_var1  >> 0x10);
    if (u_var1 < &local_bx_4.field_0xe) {
        u_var1 = local_bx_4.field_0xe;
        u_var2 = local_bx_4.field_0x10;
    }
    local_bx_4.field_0xe = u_var1;
    local_bx_4.field_0x10 = u_var2;
    local_bx_4.field_0x12 = 0;
    return;
}

pub fn pass1_1030_14b4(param_1: u32, param_2: u16, param_3: u16, param_4: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut u_var3: u32;
    let local_bx_11: &mut  Struct845;
    let mut i_var4: i32;
    let mut u_var5: i32;
    let mut u_var6: u16;
    let mut bVar7: bool;

  // u_var5 = (param_1  >> 0x10);
    local_bx_11 = param_1;
    pu_var1 = &local_bx_11.field_0xa;
    unsafe {
        if ((*pu_var1 < param_4 || *pu_var1 == param_4) || (local_bx_11.field_0x6 == 0)) {
            pu_var2 = &local_bx_11.field_0x14;
            bVar7 = *pu_var2 < param_4;
            if ((bVar7 || *pu_var2 == param_4)
                && (bVar7
                    || (
                        pu_var2 = &local_bx_11.field_0x12,
                        *pu_var2 < param_4 || *pu_var2 == param_4,
                    )))
            {
                pass1_1030_1550((param_1 & 0xffff | u_var5 << 0x10));
            }
            pu_var1 = &local_bx_11.field_0x12;
            if ((*pu_var1 < param_4 || *pu_var1 == param_4) || (local_bx_11.field_0x6 == 0)) {
                return;
            }
            pu_var2 = &local_bx_11.field_0xc;
            bVar7 = *pu_var2 < param_4;
            if ((bVar7 || *pu_var2 == param_4)
                && (bVar7
                    || (
                        pu_var2 = &local_bx_11.field_0xa,
                        *pu_var2 < param_4 || *pu_var2 == param_4,
                    )))
            {
                local_bx_11.field_0xa = (param_4 + 1);
                local_bx_11.field_0xc = (param_4 + 1 >> 0x10);
            }
        }
    }
    u_var3 = local_bx_11.field_0x6;
  // u_var6 = (u_var3  >> 0x10);
    i_var4 = u_var3;
    (i_var4 + param_4 * 4) = param_2;
    (i_var4 + param_4 * 4 + 2) = param_3;
    return;
}

pub fn ret_1030_154c() {
    return;
}

pub fn pass1_1030_1550(param_1: &mut  Struct846) {
    let pu_var1: &mut  u32;
    let paVar2: &mut  Struct94;
    let mut u_var3: i32;
    // let ctx.dx_reg: &mut  u16;
    let local_bx_4: &mut  Struct846;
    let mut u_var4: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var4 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    if (&local_bx_4.field_0x12 == 0) {
        paVar2 = local_bx_4.field_0xe;
        ctx.g_u16_ptr_1050_5f2e = local_bx_4.field_0x10;
    } else {
        u_var3 = local_bx_4.field_0x12;
        pu_var1 = &local_bx_4.field_0x16;
        unsafe {
            paVar2 = (u_var3 + *pu_var1);
            ctx.g_u16_ptr_1050_5f2e =
                (local_bx_4.field_0x14 + local_bx_4.field_0x18 + CARRY2(u_var3, *pu_var1));
        }
    }
    _local_6 = CONCAT22(ctx.g_u16_ptr_1050_5f2e, paVar2);
    if (local_bx_4.field_0x6 == 0) {
        if (ctx.g_struct_ptr_1 == 0) {
            g_struct_ptr_1 = paVar2;
            struct_fn_1000_160a();
        } else {
        }
        u_var3 = paVar2 << 2;
        alloc_mem_1000_1708(u_var3, 0, 1, g_struct_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    } else {
        u_var3 = paVar2 * 4;
        alloc_mem_1000_0ed4(
            1,
            u_var3,
            (ctx.g_u16_ptr_1050_5f2e * 2 + CARRY2(paVar2, paVar2)) * 2
                + CARRY2(paVar2 * 2, paVar2 * 2),
            local_bx_4.field_0x6,
        );
        ctx.g_u16_ptr_1050_5f2e = ctx.dx_reg;
    }
    local_a = CONCAT22(ctx.g_u16_ptr_1050_5f2e, u_var3);
    if ((ctx.g_u16_ptr_1050_5f2e | u_var3) != 0) {
        &local_bx_4.field_0x12 = _local_6;
        local_bx_4.field_0x6 = local_a;
    }
    return;
}

pub fn pass1_1030_15fe(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1030_1244(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1030_1628(param_1: &mut  Struct1) {
    let local_bx_4: &mut  Struct847;
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    param_1.field_0x0 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = 0;
    local_bx_4.field_0x8 = 0;
    param_1.field_0x0 = 0x17ba;
    local_bx_4.field_0x2 = 0x1030;
    return param_1;
}

pub fn pass1_1030_165e(param_1: &mut  Struct848, param_2: u32, param_3: u32) {
    
    let local_bx_4: &mut  Struct848;
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    param_1.field_0x0 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    &local_bx_4.field_0x4 = 0;
    local_bx_4.field_0x8 = param_3;
    param_1.field_0x0 = 0x17ba;
    local_bx_4.field_0x2 = 0x1030;
    pass1_1030_5c8a(ctx._PTR_LOOP_1050_5736, param_2);
    local_bx_4.field_0x4 = param_3;
    local_bx_4.field_0x6 = ctx.dx_reg;
    return;
}

pub fn pass1_1030_16b2(param_1: &mut  u16) {
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    unsafe {
        *param_1 = 0x17ba;
        (param_1 + 2) = 0x1030;
        *param_1 = ctx.s_1_1050_389a;
        (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    }
    return;
}

pub fn pass1_1028_e2ac(param_1: u32) {
    let mut in_stack_00000011: u8;
    let mut local_6: u32;

    local_6 = (param_1 + in_stack_00000011 * 4 + 0x2e);
    (*local_6)();
    return;
}

pub fn pass1_1028_e2e0(param_1: u32) {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let mut in_stack_00000011: u8;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_4: u16;

    local_4 = in_stack_00000011;
    if (local_4 == 0xff) {
        pass1_1028_ebee(param_1);
        return;
    }
  // u_var2 = (param_1  >> 0x10);
    iVar1 = param_1 + 0x2e;
    local_a = (iVar1 + local_4 * 4 + 2);
    (**(iVar1 + local_4 * 4))();
    return;
}

pub fn pass1_1028_e332(param_1: u32, param_2: u16, uparam_3: i32) {
    let mut local_8: u32;

    if ((param_3 != 0) && (param_3 < 10)) {
        pass1_1030_13f6(
            (param_1 + 10 + param_3 * 4),
            CONCAT22(param_3, param_2) & 0xffffff,
        );
    }
    return;
}

pub fn pass1_1028_e372(param_1: u32, param_2: u16, param_3: u16) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let ppcVar4: fn();
    let pa_var5: &mut  Struct493;
    let mut u_var6: u32;
    let mut u_var7: i32;
    
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u32;

    if (param_3 >> 8 != 0xff) {
        u_var1 = (param_1 + 10 + (param_3 >> 8) * 4);
        u_var2 = (u_var1 + 10);
        u_var7 = param_3 & 0xff;
        _local_10 = CONCAT22(param_3, param_2) & 0xffffff;
        pa_var5 = pass1_1028_e1ec(param_1, param_2, param_3);
        u_var3 = &pa_var5.field_0x8;
      // pa_var5 = pass1_1028_e1ec(param_1, u_var3, (u_var3  >> 0x10));
        _local_1c = CONCAT22(u_var7, pa_var5);
        local_20 = 1;
        while (local_20 < u_var2) {
            if (local_20 != _local_10) {
                u_var6 = _local_10;
                pass1_1030_1312(u_var1, local_20);
                if ((ctx.dx_reg | u_var6) != 0) {
                    u_var3 = (u_var6 + 4);
                    pass1_1030_13f6(u_var1, local_20);
                    ppcVar4 = (*_local_1c + 0x18);
                    (**ppcVar4)(0x1030, pa_var5, u_var7, u_var3);
                }
            }
            local_20 = local_20 + 1;
        }
    }
    return;
}

pub fn pass1_1028_e44a(param_1: u32, param_2: libc::c_long) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let mut u_var3: u32;
    let mut u_var4: i32;
    let mut u_var5: u32;
    
    let mut u_var6: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_12: u32;
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    pass1_1028_e372(param_1, param_2, (param_2 >> 0x10));
  // u_var6 = (param_1  >> 0x10);
    u_var1 = (param_1 + 0x26);
    u_var2 = (param_1 + 0x1e);
    u_var3 = (u_var2 + 10);
    local_12 = 1;
    while (local_12 < u_var3) {
        u_var5 = u_var3;
        pass1_1030_1312(u_var2, local_12);
        u_var4 = u_var5;
        if (((ctx.dx_reg | u_var4) != 0) && ((u_var4 + 8) != param_2)) {
            pass1_1030_13f6(
                u_var1,
                CONCAT22((u_var4 + 0x18), (u_var4 + 0x16)) & 0xffffff,
            );
            pass1_1030_13f6(u_var2, local_12);
        }
        local_12 = local_12 + 1;
    }
    return;
}

pub fn pass1_1028_e4ec(param_1: &mut  Struct514) {
    let pu_var1: &mut  u32;
    let plVar2: &mut  long;
    let mut u_var3: u32;
    let mut u_var4: u32;
    let lVar5: u32;
    
    
    let local_bx_11: &mut  Struct514;
    let mut u_var6: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    u_var4 = 0;
  // u_var6 = (param_1  >> 0x10);
    local_bx_11 = param_1;
    if (local_bx_11.field_0x10 == 0) {
        while {
            if (local_bx_11.field_0x8 == 0) {
                return;
            }
            unsafe {
                plVar2 = &local_bx_11.field_0x8;
                *plVar2 = *plVar2 + -1;
            }
            pass1_1030_1312(local_bx_11.field_0x4, local_bx_11.field_0x8);
            (ctx.dx_reg | u_var4) == 0
        } {}
    } else {
        while {
            u_var3 = local_bx_11.field_0xc;
            pu_var1 = &local_bx_11.field_0x8;
            let pu_var1_val = unsafe { *pu_var1 };
            if (u_var3 <= pu_var1_val && pu_var1_val != u_var3) {
                return;
            }
            lVar5 = local_bx_11.field_0x8;
            plVar2 = &local_bx_11.field_0x8;
            unsafe { *plVar2 = *plVar2 + 1 };
            pass1_1030_1312(local_bx_11.field_0x4, lVar5);
            (ctx.dx_reg | lVar5) == 0
        } {}
    }
    return;
}

pub fn pass1_1028_e0a0(param_1: u32, param_2: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 0x52);
    pass1_1030_4782(u_var1, (u_var1 >> 0x10), 1, param_2, (param_2 >> 0x10));
    return;
}

pub fn pass1_1028_e0bc(param_1: u32, param_2: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let in_ax: &mut  u32;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    let in_dx: &mut  Struct199;
    let pa_var5: &mut  Struct199;
    let pu_var6: &mut  u32;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x98, in_dx);
    pu_var3 = in_ax;
    pa_var5 = in_dx;
    pass1_1030_4bbe((param_1 + 0x52), param_2);
    i_var4 = 0x26;
    pu_var6 = in_ax;
    while (i_var4 != 0) {
        i_var4 = i_var4 + -1;
        pu_var2 = pu_var6;
        pu_var6 = pu_var6 + 1;
        pu_var1 = pu_var3;
        pu_var3 = pu_var3 + 1;
        unsafe { *pu_var2 = *pu_var1 };
    }
    return CONCAT22(in_dx, in_ax);
}

pub fn pass1_1028_e100(param_1: u32, param_2: u16) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut u_var3: i32;
    let mut in_eax: u32;
    let mut i_var4: i32;
    let in_dx: &mut  u16;
    let mut u_var5: i32;
    let pu_var6: &mut  u32;
    let pu_var7: &mut  u32;
    let mut u_var8: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    if (ctx.g_struct_ptr_1 == 0) {
        struct_fn_1000_160a();
        ctx.g_u16_ptr_1050_5f2e = in_dx;
    } else {
        in_eax = ctx.g_struct_ptr_1 & 0xffff;
    }
    alloc_mem_1000_1708(0xae, 0, 1, in_eax, ctx.g_u16_ptr_1050_5f2e);
    u_var3 = in_eax;
    local_6 = in_eax & 0xffff | ZEXT24(ctx.g_u16_ptr_1050_5f2e) << 0x10;
    u_var5 = ctx.g_u16_ptr_1050_5f2e | u_var3;
    if (u_var5 == 0) {
        local_6 = 0;
    } else {
        (u_var3 + 0xa4) = 0;
        (u_var3 + 0xa8) = 0;
        (u_var3 + 0xac) = 0;
        in_eax = local_6;
    }
    pu_var6 = in_eax;
    pass1_1030_4c06((param_1 + 0x52), param_2);
  // u_var8 = (local_6  >> 0x10);
    pu_var7 = local_6;
    i_var4 = 0x2b;
    while (i_var4 != 0) {
        i_var4 = i_var4 + -1;
        pu_var2 = pu_var7;
        pu_var7 = pu_var7 + 1;
        pu_var1 = pu_var6;
        pu_var6 = pu_var6 + 1;
        unsafe { *pu_var2 = *pu_var1 };
    }
    pu_var7 = pu_var6;
    return;
}

pub fn pass1_1028_e198(param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    let paVar1: &mut  Struct493;
    let mut in_dx: u16;

  // paVar1 = pass1_1028_e1ec(param_1, param_4, (param_4  >> 0x10));
    pass1_1030_5b1c(CONCAT22(in_dx, paVar1), param_2, param_3);
    return;
}

pub fn pass1_1028_e1bc() {
    let mut local_6: u32;

    return;
}

pub fn pass1_1028_e1ec(ctx: &mut AppContext, param_1: &mut Struct493, param_2: u16, param_3: u16) -> &mut Struct493 {
    let mut u_var1: u32;
    if param_3 == 0x0 {
        return param_3;
    }
    if param_3 == 0xff {
        return ctx.PTR_LOOP_1050_5166;
    }
    u_var1 = (param_1 + 10 + param_3 * 4);
    pass1_1030_1312(u_var1, param_2, param_3 & 0xff);
    return u_var1;
}

pub fn pass1_1028_d7de(param_1: &mut  Struct215, param_2: u8) -> &mut  Struct215 {
    pass1_1008_57c4(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_d81c(param_1: &mut  Struct841, param_2: u32) {
    let u_var1: u8;
    let mut u_var2: i32;
    let extraout_var: u32;
    let in_dx: &mut  Struct199;
    let paVar3: &mut  Struct199;
    let paVar4: &mut  Struct199;
    
    // let ctx.dx_reg: &mut  Struct199;
    // let ctx.dx_reg: &mut  Struct199;
    // let ctx.dx_reg: &mut  Struct199;
    // let ctx.dx_reg: &mut  Struct199;
    let extraout_dx_04: &mut  Struct199;
    let extraout_dx_05: &mut  Struct199;
    let extraout_dx_06: &mut  Struct199;
    let mut extraout_dx_07: u16;
    let mut u_var5: u16;
    let local_bx_4: &mut  Struct841;
    let mut u_var6: u16;
    let mut local_4: u16;

  // u_var6 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    param_1 = 0;
    local_bx_4.field_0x4 = param_2;
    &local_bx_4.field_0x52 = 0;
    ctx._PTR_LOOP_1050_65e2 = param_1;
    local_bx_4.field_0x32 = 0xec36;
    local_bx_4.field_0x34 = &ctx.PTR_LOOP_1050_1028;
    local_bx_4.field_0x36 = 0xecac;
    local_bx_4.field_0x38 = &ctx.PTR_LOOP_1050_1028;
    local_bx_4.field_0x3a = 0xed2c;
    local_bx_4.field_0x3c = &ctx.PTR_LOOP_1050_1028;
    local_bx_4.field_0x3e = 0xedc4;
    local_bx_4.field_0x40 = &ctx.PTR_LOOP_1050_1028;
    local_bx_4.field_0x42 = 0xee54;
    local_bx_4.field_0x44 = &ctx.PTR_LOOP_1050_1028;
    local_bx_4.field_0x46 = 0xef00;
    local_bx_4.field_0x48 = &ctx.PTR_LOOP_1050_1028;
    local_bx_4.field_0x4a = 0x10b0;
    local_bx_4.field_0x4c = 0x1030;
    local_bx_4.field_0x4e = 0x1120;
    local_bx_4.field_0x50 = 0x1030;
    process_struct_1000_179c(8, in_dx);
    u_var2 = param_2;
    paVar3 = (in_dx | u_var2);
    if (paVar3 != 0x0) {
        pass1_1030_615a((param_2 & 0xffff | ZEXT24(in_dx) << 0x10));
    }
    process_struct_1000_179c(0x56c, paVar3);
    paVar4 = (paVar3 | u_var2);
    if (paVar4 == 0x0) {
        u_var2 = 0;
        paVar4 = 0x0;
    } else {
        pass1_1030_44be(CONCAT22(paVar3, u_var2));
    }
    local_bx_4.field_0x52 = u_var2;
    local_bx_4.field_0x54 = paVar4;
    process_struct_1000_179c(4, paVar4);
    paVar3 = (paVar4 | u_var2);
    if (paVar3 != 0x0) {
        pass1_1008_bde0(CONCAT22(paVar4, u_var2));
        paVar3 = ctx.dx_reg;
    }
    u_var1 = pass1_1000_4906((param_1 & 0xffff0000 | &local_bx_4.field_0xa), 0, 0x24);
    u_var2 = CONCAT31(extraout_var, u_var1);
    process_struct_1000_179c(0x1c, paVar3);
    paVar4 = (paVar3 | u_var2);
    if (paVar4 == 0x0) {
        &local_bx_4.field_0xe = 0;
    } else {
        pass1_1030_11aa(u_var2, paVar3, 5, 0x15);
        local_bx_4.field_0xe = u_var2;
        local_bx_4.field_0x10 = ctx.dx_reg;
        paVar4 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x1c, paVar4);
    if ((paVar4 | u_var2) == 0) {
        u_var2 = 0;
        paVar3 = 0x0;
    } else {
        pass1_1030_11aa(u_var2, paVar4, 5, 10);
        paVar3 = ctx.dx_reg;
    }
    local_bx_4.field_0x12 = u_var2;
    local_bx_4.field_0x14 = paVar3;
    process_struct_1000_179c(0x1c, paVar3);
    if ((paVar3 | u_var2) == 0) {
        u_var2 = 0;
        paVar3 = 0x0;
    } else {
        pass1_1030_11aa(u_var2, paVar3, 5, 0x19);
        paVar3 = ctx.dx_reg;
    }
    local_bx_4.field_0x16 = u_var2;
    local_bx_4.field_0x18 = paVar3;
    process_struct_1000_179c(0x1c, paVar3);
    if ((paVar3 | u_var2) == 0) {
        u_var2 = 0;
        paVar3 = 0x0;
    } else {
        pass1_1030_11aa(u_var2, paVar3, 5, 10);
        paVar3 = ctx.dx_reg;
    }
    local_bx_4.field_0x1a = u_var2;
    local_bx_4.field_0x1c = paVar3;
    process_struct_1000_179c(0x1c, paVar3);
    if ((paVar3 | u_var2) == 0) {
        u_var2 = 0;
        paVar3 = 0x0;
    } else {
        pass1_1030_11aa(u_var2, paVar3, 100, 500);
        paVar3 = extraout_dx_04;
    }
    local_bx_4.field_0x1e = u_var2;
    local_bx_4.field_0x20 = paVar3;
    process_struct_1000_179c(0x1c, paVar3);
    if ((paVar3 | u_var2) == 0) {
        u_var2 = 0;
        paVar3 = 0x0;
    } else {
        pass1_1030_11aa(u_var2, paVar3, 0x19, 100);
        paVar3 = extraout_dx_05;
    }
    local_bx_4.field_0x22 = u_var2;
    local_bx_4.field_0x24 = paVar3;
    process_struct_1000_179c(0x1c, paVar3);
    if ((paVar3 | u_var2) == 0) {
        u_var2 = 0;
        paVar3 = 0x0;
    } else {
        pass1_1030_11aa(u_var2, paVar3, 100, 500);
        paVar3 = extraout_dx_06;
    }
    local_bx_4.field_0x26 = u_var2;
    local_bx_4.field_0x28 = paVar3;
    process_struct_1000_179c(0x1c, paVar3);
    if ((paVar3 | u_var2) == 0) {
        u_var2 = 0;
        u_var5 = 0;
    } else {
        pass1_1030_11aa(u_var2, paVar3, 5, 2);
        u_var5 = extraout_dx_07;
    }
    local_bx_4.field_0x2a = u_var2;
    local_bx_4.field_0x2c = u_var5;
    return;
}

pub fn pass1_1028_daba(param_1: &mut  Struct842) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let pu_var3: &mut  u32;
    let ppcVar4: fn();
    let pa_var5: &mut Struct44;
    let local_bx_43: &mut  Struct842;
    let mut u_var6: u16;
    let mut unaff_cs: u16;
    let mut local_e: u32;
    let mut local_a: u32;

    pa_var5 = _PTR_LOOP_1050_5740;
    if (ctx._PTR_LOOP_1050_5740 != 0x0) {
        pass1_1030_61b0(ctx._PTR_LOOP_1050_5740, (ctx._PTR_LOOP_1050_5740 >> 0x10));
        unaff_cs = 0x1000;
        error_check_1000_17ce(pa_var5);
    }
  // u_var6 = (param_1  >> 0x10);
    local_bx_43 = param_1;
    u_var1 = local_bx_43.field_0x52;
    u_var2 = local_bx_43.field_0x54;
    local_e = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0) {
        pass1_1030_4538(u_var1, u_var2);
        unaff_cs = 0x1000;
        error_check_1000_17ce(local_e);
    }
    if (ctx._PTR_LOOP_1050_5166 != 0x0) {
        ppcVar4 = *_PTR_LOOP_1050_5166;
        (**ppcVar4)(unaff_cs, _PTR_LOOP_1050_5166);
    }
    pa_var5 = ctx._PTR_LOOP_1050_06e0;
    ctx._PTR_LOOP_1050_65e2 = 0;
    if (ctx._PTR_LOOP_1050_06e0 != 0x0) {
        pass1_1008_c626(ctx._PTR_LOOP_1050_06e0);
        unaff_cs = 0x1000;
        error_check_1000_17ce(pa_var5);
    }
    pu_var3 = local_bx_43.field_0xe;
    u_var1 = local_bx_43.field_0x10;
    if ((u_var1 | pu_var3) != 0) {
        unsafe {
            ppcVar4 = *pu_var3;
            (**ppcVar4)(unaff_cs, pu_var3, u_var1, 1);
        }
    }
    pu_var3 = local_bx_43.field_0x12;
    u_var1 = local_bx_43.field_0x14;
    if ((u_var1 | pu_var3) != 0) {
        unsafe {
            ppcVar4 = *pu_var3;
            (**ppcVar4)(unaff_cs, pu_var3, u_var1, 1);
        }
    }
    pu_var3 = local_bx_43.field_0x16;
    u_var1 = local_bx_43.field_0x18;
    if ((u_var1 | pu_var3) != 0) {
        unsafe {
            ppcVar4 = *pu_var3;
            (**ppcVar4)(unaff_cs, pu_var3, u_var1, 1);
        }
    }
    pu_var3 = local_bx_43.field_0x1a;
    u_var1 = local_bx_43.field_0x1c;
    if ((u_var1 | pu_var3) != 0) {
        unsafe {
            ppcVar4 = *pu_var3;
            (**ppcVar4)(unaff_cs, pu_var3, u_var1, 1);
        }
    }
    pu_var3 = local_bx_43.field_0x1e;
    u_var1 = local_bx_43.field_0x20;
    if ((u_var1 | pu_var3) != 0) {
        unsafe {
            ppcVar4 = *pu_var3;
            (**ppcVar4)(unaff_cs, pu_var3, u_var1, 1);
        }
    }
    pu_var3 = local_bx_43.field_0x22;
    u_var1 = local_bx_43.field_0x24;
    if ((u_var1 | pu_var3) != 0) {
        unsafe {
            ppcVar4 = *pu_var3;
            (**ppcVar4)(unaff_cs, pu_var3, u_var1, 1);
        }
    }
    pu_var3 = local_bx_43.field_0x26;
    u_var1 = local_bx_43.field_0x28;
    if ((u_var1 | pu_var3) != 0) {
        unsafe {
            ppcVar4 = *pu_var3;
            (**ppcVar4)(unaff_cs, pu_var3, u_var1, 1);
        }
    }
    pu_var3 = local_bx_43.field_0x2a;
    u_var1 = local_bx_43.field_0x2c;
    if ((u_var1 | pu_var3) != 0) {
        unsafe {
            ppcVar4 = *pu_var3;
            (**ppcVar4)(unaff_cs, pu_var3, u_var1, 1);
        }
    }
    return;
}

pub fn pass1_1028_dc52(param_1: &mut  Struct374, param_2: Vec<u8>, param_3: u16, param_4: u16) {
    let local_bx_5: &mut  Struct374;
    let mut local_es_5: u16;
    let mut temp_5f1f4e4cff: u32;

  // local_es_5 = (param_1  >> 0x10);
    local_bx_5 = param_1;
    param_1 = ctx.s_1_1050_389a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_5.field_0x4 = (ctx._PTR_LOOP_1050_65e2 + (param_4 >> 8) * 4 + 10);
    local_bx_5.field_0x8 = 1;
    local_bx_5.field_0x10 = param_2;
    param_1 = 0x11a6;
    local_bx_5.field_0x2 = 0x1030;
    temp_5f1f4e4cff = local_bx_5.field_0x4;
    local_bx_5.field_0xc = (temp_5f1f4e4cff + 10);
    if (param_2 == 0x0) {
        local_bx_5.field_0x8 = local_bx_5.field_0xc;
    } else {
        local_bx_5.field_0x8 = 1;
    }
    return param_1;
}

pub fn pass1_1028_bab6(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = pass1_1028_bad4(param_1);
    pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    return;
}

pub fn pass1_1028_bad4(param_1: u32) {
    let mut in_ax: i32;
    let mut in_dx: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_baf6(param_1);
    return CONCAT22((in_ax + 10), (in_ax + 8));
}

pub fn pass1_1028_baf6(param_1: Vec<u8>) {
    let mut u_var1: u32;
    let mut local_4: u16;

    u_var1 = pass1_1028_bb24(param_1);
    if (u_var1 == 0) {
        return;
    }
    pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1 >> 0x10));
    return;
}

pub fn pass1_1028_bb24(param_1: Vec<u8>) {
    let u_var1: u8;
    let extraout_ah: u8;
    let mut in_dx: u16;
    let mut u_var2: i32;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var2 = (param_1  >> 0x10);
    if ((param_1 + 8) == 0) {
        return 0;
    }
    u_var1 = pass1_1028_b58e((param_1 & 0xffff | u_var2 << 0x10));
    return CONCAT22(
        (CONCAT11(extraout_ah, u_var1) + 10),
        (CONCAT11(extraout_ah, u_var1) + 8),
    );
}

pub fn pass1_1028_bb56(param_1: u32, param_2: u32) {
    pass1_1030_177a(param_1, param_2);
    return;
}

pub fn pass1_1028_bb6a(param_1: &mut  Struct830) {
    let local_bx_3: &mut  Struct830;
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    local_bx_3 = param_1;
    if ((local_bx_3.field_0x12 != 5) && (local_bx_3.field_0x12 != 6)) {
        return 0;
    }
    return CONCAT22(local_bx_3.field_0x16, local_bx_3.field_0x14 + 0xa4);
}

pub fn pass1_1028_bb96(param_1: u32, param_2: &mut  u32, param_3: u16) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut u_var3: u32;
    let mut i_var4: i32;
    let local_bx_5: &mut  Struct831;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut u_var7: u16;

  // u_var6 = (param_1  >> 0x10);
    local_bx_5 = param_1;
    if ((local_bx_5.field_0x12 == 5) || (local_bx_5.field_0x12 == 6)) {
        u_var3 = local_bx_5.field_0x14;
      // u_var7 = (u_var3  >> 0x10);
        pu_var5 = (u_var3 + 0xa4);
        i_var4 = 2;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = param_2;
            param_2 = param_2 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        pu_var5 = param_2;
        pass1_1028_c724(param_1);
        u_var3 = local_bx_5.field_0x14;
      // u_var6 = (u_var3  >> 0x10);
        i_var4 = u_var3;
        if ((i_var4 + 0xaa) == 0) {
            (i_var4 + 0xaa) = 1;
        }
    }
    return;
}

pub fn pass1_1028_bbf0(param_1: u16, param_2: u16, param_1_00: &mut  u32) {
    unsafe { *param_1_00 = 0 };
    return;
}

pub fn pass1_1028_bc02(param_1: &mut  u32) {
    let pp_var1: fn();

    unsafe {
        pp_var1 = (*param_1 + 0x40);
        (**pp_var1)();
    }
    return;
}

pub fn pass1_1028_bc1c(param_1: &mut  Struct832) {
    let local_bx_3: &mut  Struct832;
    let mut u_var1: u16;

  // u_var1 = (param_1  >> 0x10);
    local_bx_3 = param_1;
    if (local_bx_3.field_0x12 == 4) {
        return local_bx_3.field_0xe;
    }
    if (local_bx_3.field_0x12 == 7) {
        return local_bx_3.field_0x10;
    }
    return local_bx_3.field_0xc;
}

pub fn pass1_1028_bc4a(param_1: u32) {
    let mut u_var1: u16;
    let in_struct_1: &mut Struct44;
    let mut in_stack_0000fff0: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    in_struct_1 = pass1_1028_e0bc(
        ctx._PTR_LOOP_1050_65e2,
        CONCAT22(in_stack_0000fff0, (param_1 + 0xc)),
    );
    u_var1 = (in_struct_1 + 0x96);
    error_check_1000_17ce(in_struct_1);
    return u_var1;
}

pub fn pass1_1028_bc7e(param_1: u32) {
    pass1_1028_bdac(param_1, 4);
    return;
}

pub fn pass1_1028_bc90(param_1: &mut  u32, param_2: Vec<u8>, param_3: u32, param_4: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;

    i_var3 = pass1_1028_c7b6(param_1, param_2, param_4);
    if ((i_var3 == 5) || (i_var3 == 6)) {
      // u_var4 = (param_1  >> 0x10);
        unsafe { u_var2 = *param_1 };
        pp_var1 = (u_var2 + 0x60);
        i_var3 = (**pp_var1)();
        if (i_var3 != 0) {
            pp_var1 = (u_var2 + 0x5c);
            i_var3 = (**pp_var1)();
            if (i_var3 != 0) {
                pass1_1028_c23e(param_1, u_var4, param_2, param_3, param_4);
                if (i_var3 != 0) {
                    u_var4 = pass1_1028_c314(
                        param_1,
                        u_var4,
                        param_2,
                        param_3,
                        (param_3 >> 0x10),
                        param_4,
                    );
                    if (u_var4 != 0) {
                        return 1;
                    }
                }
            }
        }
    } else {
      ctx.PTR_LOOP_1050_50ca = 0x6a8;
    }
    return 0;
}

pub fn pass1_1028_bd38(param_1: &mut Struct44) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let pu_var3: Vec<u8>;
    let u_var4: u8;
    let mut u_var5: u32;
    let extraout_var: u32;
    let mut in_dx: u16;
    let mut u_var6: u16;
    let mut local_14: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var5 = (ctx._PTR_LOOP_1050_65e2 + 0x52);
    pass1_1030_4bbe(u_var5, (param_1 + 0xc));
    u_var6 = in_dx;
    u_var4 = pass1_1028_b58e(param_1);
    pu_var3 = *(CONCAT31(extraout_var, u_var4) + 0x2e);
    local_14 = 0x11;
    while {
        u_var1 = (local_14 * 4 + u_var5);
        u_var2 = (local_14 * 4 + u_var5 + 2);
        if ((u_var2 | u_var1) != 0) {
            pass1_1038_5770(pu_var3, CONCAT22(u_var2, u_var1), local_14);
        }
        local_14 = local_14 + 1;
        local_14 < 0x25
    } {}
    return;
}

pub fn pass1_1028_bdac(param_1: &mut  Struct833, param_2: i32) {
    let mut iVar1: i32;
    let ppc_var2: fn();
    let local_bx_7: &mut  Struct833;
    let mut u_var3: u16;
    let mut unaff_cs: u16;

  // u_var3 = (param_1  >> 0x10);
    local_bx_7 = param_1;
    if (local_bx_7.field_0x12 != param_2) {
        if (local_bx_7.field_0x12 == 6) {
            if (local_bx_7.field_0x18 == param_2) {
                local_bx_7.field_0x12 = local_bx_7.field_0x18;
                local_bx_7.field_0x18 = 0;
                return;
            }
        } else {
            if (param_2 != 6) {
                iVar1 = local_bx_7.field_0x12;
                if ((iVar1 == 4) || (iVar1 == 5)) {
                    unaff_cs = 0x1000;
                    error_check_1000_17ce(local_bx_7.field_0x14);
                }
                local_bx_7.field_0x12 = param_2;
                ppc_var2 = (param_1 + 0x3c);
                ppc_var2(unaff_cs, param_1);
                return;
            }
            local_bx_7.field_0x18 = local_bx_7.field_0x12;
            local_bx_7.field_0x12 = 6;
        }
    }
    return;
}

pub fn pass1_1028_be2a(param_1: &mut Struct44) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    if ((i_var3 + 0x12) != 6) {
        return;
    }
    u_var5 = pass1_1028_b4f2(param_1);
    if ((u_var5 + 0x200) != 0x8000002) {
        if ((i_var3 + 0x1c) == 0x8000002) {
            i_var3 = 6;
            // goto code_r0x1028be96;
        }
        pp_var1 = (param_1 + 100);
        i_var2 = (**pp_var1)();
        if (i_var2 == 0) {
            return;
        }
        i_var3 = pass1_1028_cb04(i_var3, u_var4);
        if (i_var3 == 0) {
            i_var3 = 6;
            // goto code_r0x1028be96;
        }
        pass1_1028_c952(param_1);
    }
    i_var3 = 5;
    // code_r0x1028be96:
    pass1_1028_bdac(param_1, i_var3);
    return;
}

pub fn pass1_1028_be9e(param_1: &mut Struct44) {
    let piVar1: &mut  i32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let local_bx_4: &mut  Struct834;
    let mut u_var4: u16;
    let mut u_var5: u32;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var4 = (param_1  >> 0x10);
    local_bx_4 = param_1;
    if (local_bx_4.field_0x12 == 4) {
        u_var5 = pass1_1028_b4f2(param_1);
        if ((u_var5 + 0x200) == 0x8000002) {
            u_var2 = local_bx_4.field_0x14;
            piVar1 = (u_var2 + 0x94);
            unsafe { *piVar1 = *piVar1 + -1 };
        } else {
            i_var3 = pass1_1028_cb04(local_bx_4, u_var4);
            if (i_var3 == 0) {
                return;
            }
            u_var2 = local_bx_4.field_0x14;
            piVar1 = (u_var2 + 0x94);
            unsafe { *piVar1 = *piVar1 + -1 };
            pass1_1028_c952(param_1);
        }
        u_var2 = local_bx_4.field_0x14;
        if ((u_var2 + 0x94) < 1) {
            pass1_1028_bdac(param_1, 5);
        }
    }
    return;
}

pub fn pass1_1028_bf22(param_1: &mut  Struct835) {
    let mut iVar1: i32;
    let mut in_dx: u16;
    let local_bx_3: &mut  Struct835;
    let mut unaff_bp: u16;
    let mut u_var2: u16;
    let mut u_var3: u32;

  // u_var2 = (param_1  >> 0x10);
    local_bx_3 = param_1;
    iVar1 = local_bx_3.field_0x12;
    if (iVar1 == 4) {
        u_var3 = pass1_1028_e0bc(
            ctx._PTR_LOOP_1050_65e2,
            CONCAT22(unaff_bp, local_bx_3.field_0xc),
        );
    } else {
        iVar1 = iVar1 + -5;
        if (iVar1 != 0) {
            if (iVar1 != 1) {
                &local_bx_3.field_0x14 = 0;
            }
            return;
        }
        pass1_1028_e100(ctx._PTR_LOOP_1050_65e2, local_bx_3.field_0xc);
        u_var3 = CONCAT22(in_dx, iVar1);
    }
    local_bx_3.field_0x14 = u_var3;
    &local_bx_3.field_0x16 = (u_var3 >> 0x10);
    return;
}

pub fn pass1_1028_bf76(param_1: &mut  Struct764) {
    let mut iVar1: i32;
    let BVar2: bool;
    let local_bx_23: &mut  Struct764;
    let mut u_var3: u16;
    let mut local_4: u16;

    iVar1 = pass1_fn_1008_612e();
  // u_var3 = (param_1  >> 0x10);
    local_bx_23 = param_1;
    BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, &local_bx_23.field_0xc, 0x28);
    if (BVar2 == 0) {
        if (iVar1 == 1) {
            &local_bx_23.field_0x10 = 0x48;
            return;
        }
        if (iVar1 != 2) {
            &local_bx_23.field_0x10 = 0x4a;
            return;
        }
        &local_bx_23.field_0x10 = 0x49;
        return;
    }
    if (iVar1 == 1) {
        &local_bx_23.field_0x10 = 0x70;
        return;
    }
    if (iVar1 != 2) {
        &local_bx_23.field_0x10 = 0x72;
        return;
    }
    &local_bx_23.field_0x10 = 0x71;
    return;
}

pub fn pass1_1028_c00a(param_1: &mut Struct44, param_2: libc::c_long) {
    let paVar1: &mut  Struct1115;
    let ppc_var2: fn(a: u16, b: u16, c: u16);
    let u_var3: u8;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let paVar6: &mut  Struct493;
    let extraout_var: u32;
    let pu_var7: Vec<u8>;
    let mut in_dx: i32;
    
    
    
    let mut u_var8: u16;
    let mut u_var9: i32;
    let mut u_var10: u16;
    let mut u_var11: u32;
    let mut u_var12: u32;
    let mut local_2e: u32;
    let mut local_2a: u32;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = pass1_1028_b58e(param_1);
    paVar1 = (CONCAT31(extraout_var, u_var3) + 0x2e);
    pu_var7 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 4);
    u_var4 = SUB42(pu_var7, 0);
    u_var10 = SUB42(&ctx.PTR_LOOP_1050_1038, 0);
    pass1_1038_4d6e(paVar1, pu_var7 & 0xffff | in_dx << 0x10);
    _local_12 = CONCAT22(ctx.dx_reg, u_var4);
    ppc_var2 = (*_local_12 + 0x10);
    u_var5 = u_var4;
    ppc_var2(&ctx.PTR_LOOP_1050_1038, u_var4, ctx.dx_reg);
    _local_16 = CONCAT22(ctx.dx_reg, u_var5);
    local_1a = 0;
    loop {
        if (_local_16 <= local_1a) {
            // LAB_1028_c0d6:
            if (_local_12 != 0x0) {
                ppc_var2 = *_local_12;
                ppc_var2(u_var10, u_var4, ctx.dx_reg, 1);
            }
            return;
        }
        ppc_var2 = (*_local_12 + 4);
        u_var11 = _local_16;
        ppc_var2(u_var10, u_var4, ctx.dx_reg, local_1a, (local_1a >> 0x10));
        u_var8 = ctx.dx_reg;
        paVar6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var11, ctx.dx_reg);
        u_var10 = 0x1030;
        u_var11 = pass1_1030_73a8(CONCAT22(u_var8, paVar6));
        u_var12 = pass16_funcs::pass1_1028_6302(u_var11);
      // u_var9 = (u_var12  >> 0x10);
        if ((param_2 <= u_var9) && (param_2 < u_var9 || (param_2 <= u_var12))) {
            pass16_funcs::pass1_1028_6356(u_var11, 0, param_2, param_2);
            // goto LAB_1028_c0d6;
        }
        pass16_funcs::pass1_1028_6356(u_var11, 0, u_var12, u_var9);
        param_2 = param_2 - u_var12;
        local_1a = local_1a + 1;
    }
}

pub fn pass1_1028_c0f0(param_1: &mut Struct44, param_2: libc::c_long) {
    let paVar1: &mut  Struct1115;
    let ppc_var2: fn();
    let u_var3: u8;
    let mut u_var4: u16;
    let pa_var5: &mut  Struct493;
    let extraout_var: u32;
    let pu_var6: Vec<u8>;
    let mut in_dx: i32;
    
    let mut u_var7: i32;
    
    
    // let mut ctx.dx_reg: u16;
    let mut u_var8: u16;
    let mut u_var9: i32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut u_var12: u32;
    let mut u_var13: u32;
    let mut local_1c: u32;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var3 = pass1_1028_b58e(param_1);
    paVar1 = (CONCAT31(extraout_var, u_var3) + 0x2e);
    pass1_1028_cb04(param_1, (param_1 >> 0x10));
  // u_var10 = (paVar1  >> 0x10);
    if (((paVar1 + 0x204) == 0) && ((paVar1 + 0x206) == 0)) {
        u_var7 = ctx.dx_reg;
        pu_var6 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 4);
        u_var4 = SUB42(pu_var6, 0);
        u_var11 = SUB42(&ctx.PTR_LOOP_1050_1038, 0);
        pass1_1038_4d6e(paVar1, pu_var6 & 0xffff | u_var7 << 0x10);
        _local_14 = CONCAT22(ctx.dx_reg, u_var4);
        ppc_var2 = (*_local_14 + 0x10);
        u_var10 = u_var4;
        ppc_var2(&ctx.PTR_LOOP_1050_1038, u_var4, ctx.dx_reg);
        _local_18 = CONCAT22(ctx.dx_reg, u_var10);
        local_1c = 0;
        while (local_1c < _local_18) {
            ppc_var2 = (*_local_14 + 4);
            u_var12 = _local_18;
            ppc_var2(
                u_var11,
                u_var4,
                ctx.dx_reg,
                local_1c,
                (local_1c >> 0x10),
            );
            u_var8 = ctx.dx_reg;
            pa_var5 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var12, ctx.dx_reg);
            u_var11 = 0x1030;
            u_var12 = pass1_1030_73a8(CONCAT22(u_var8, pa_var5));
            u_var13 = pass16_funcs::pass1_1028_6302(u_var12);
          // u_var9 = (u_var13  >> 0x10);
            u_var7 = u_var13;
            if ((param_2 <= u_var9) && (param_2 < u_var9 || (param_2 <= u_var7))) {
                param_2 = 0;
                break;
            }
            param_2 = CONCAT22(
                (param_2 - u_var9) - (param_2 < u_var7),
                param_2 - u_var7,
            );
            local_1c = local_1c + 1;
        }
        if (_local_14 != 0x0) {
            ppc_var2 = *_local_14;
            ppc_var2(u_var11, u_var4, ctx.dx_reg, 1);
        }
        if (param_2 != 0) {
            pass1_1030_7d7c(
                (CONCAT31(extraout_var, u_var3) & 0xffff | in_dx << 0x10),
                param_2,
                CONCAT22(0x1d, (param_2 >> 0x10)),
            );
        }
    }
    return;
}

pub fn pass1_1028_c1f8(param_1: Vec<u8>, param_2: Vec<u8>, param_3: Vec<u8>) {
    let mut in_ax: i32;
    let mut in_dx: u16;
    let pu_var1: &mut  u32;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1028_baf6(param_1);
    pu_var1 = pass1_1030_5b5c(in_ax, in_dx);
    unsafe { local_c = *pu_var1 };
    uStack8 = (pu_var1 + 4);
    pass1_1008_3e94(&local_c, param_2, param_3);
    return;
}

pub fn pass1_1028_c23e(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: libc::c_long,
    param_5: u32,
) -> i32 {
    let lVar1: u32;
    let ppc_var2: fn();
    let mut in_ax: i32;
    let paVar3: &mut  Struct493;
    let paVar4: &mut  Struct493;
    
    let mut u_var5: i32;
    let mut u_var6: i32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1030_627e(ctx._PTR_LOOP_1050_5740, param_1_00, param_5);
    _local_6 = CONCAT22(ctx.dx_reg, in_ax);
    u_var5 = ctx.dx_reg | in_ax;
    if (u_var5 != 0) {
        paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, in_ax, ctx.dx_reg);
        _local_a = CONCAT22(u_var5, paVar3);
        lVar1 = &paVar3[1].field_0xc;
        if (lVar1 != param_2_00) {
          // paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, lVar1, (lVar1  >> 0x10));
            _local_12 = CONCAT22(u_var5, paVar3);
            u_var6 = u_var5;
          // paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00  >> 0x10));
            _local_16 = CONCAT22(u_var6, paVar4);
            if (((_local_12 == 0x0) || ((u_var6 | paVar4) == 0))
                || (&paVar4[0x11].field_0x2 != &paVar3[0x11].field_0x2))
            {
                return 0;
            }
            ppc_var2 = (*_local_12 + 0x18);
            ppc_var2(0x1030, paVar3, u_var5, _local_6);
            ppc_var2 = (*_local_16 + 8);
            ppc_var2();
            pass1_1030_73ee(_local_a, param_2_00);
        }
    }
    return 0;
}

pub fn pass1_1028_c314(
    param_1: u16,
    param_2: u16,
    param_1_00: &mut  u16,
    param_4: u16,
    param_5: u16,
    param_2_00: u32,
) -> i32 {
    let mut in_dx: u16;
    let mut unaff_ss: u16;
    let pu_var1: &mut  u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut uStack8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

  // local_6 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00  >> 0x10));
    pu_var1 = pass1_1030_5b5c(local_6, in_dx);
    unsafe { local_c = *pu_var1 };
    uStack8 = (pu_var1 + 4);
    pass1_1008_3e94(
        param_1_00,
        CONCAT22(unaff_ss, &local_10),
        CONCAT22(unaff_ss, &local_e),
    );
    pass1_1008_3e94(
        &local_c,
        CONCAT22(unaff_ss, &local_14),
        CONCAT22(unaff_ss, &local_12),
    );
    if ((((1 < local_e) && (1 < local_10)) && (local_e < (local_12 - 1)))
        && (local_10 < (local_14 - 1)))
    {
        return 1;
    }
  ctx.PTR_LOOP_1050_50ca = 0x6b8;
    return 0;
}

pub fn pass1_1028_c3aa(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let paVar3: &mut  Struct493;
    let paVar4: &mut  Struct493;
    let mut i_var5: i32;
    let mut u_var6: u16;
    let pu_var7: Vec<u8>;
    let pu_var8: Vec<u8>;
    let mut u_var9: u32;
    let mut u_var10: u16;
    let mut u_var11: u16;
    
    let mut u_var12: i32;
    
    
    
    let mut u_var13: u16;
    let mut unaff_ss: u16;
    let mut u_var14: u32;
    let ppVar15: &mut  Struct2551;
    let u_var16: u8;
    let u_var17: u8;
    let mut u_var18: u16;
    let mut u_var19: u16;
    let mut local_34: u32;
    let mut local_30: u32;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let mut local_28: u32;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    u_var14 = pass1_1030_bcae(local_4, unaff_ss);
  // u_var10 = (u_var14  >> 0x10);
  // paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00  >> 0x10));
    u_var1 = &paVar3.field_0x10;
    u_var16 = param_1_00;
    u_var17 = (param_1_00 >> 8);
  // u_var18 = (param_1_00  >> 0x10);
    u_var11 = u_var10;
    u_var14 = param_5;
  // paVar4 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    local_18 = local_4;
    pass1_1030_bcde(
        local_18,
        unaff_ss,
        CONCAT22(u_var11, paVar4),
        CONCAT22(u_var18, CONCAT11(u_var17, u_var16)),
        u_var14,
    );
    if (local_18 < 0) {
      ctx.PTR_LOOP_1050_50ca = 0x6af;
        return 0;
    }
    if (0x1e < local_18) {
        u_var19 = 0x87;
        ppVar15 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x870009);
        i_var5 = ppVar15;
        pass1_1010_65d0(ppVar15, u_var19);
        if (i_var5 == 0) {
            u_var12 = ctx.dx_reg;
            pu_var8 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x15);
            u_var6 = SUB42(pu_var8, 0);
            u_var13 = SUB42(&ctx.PTR_LOOP_1050_1038, 0);
            pass1_1038_4d6e(
                CONCAT22(u_var10, paVar3),
                pu_var8 & 0xffff | u_var12 << 0x10,
            );
            _local_20 = CONCAT22(ctx.dx_reg, u_var6);
            ppc_var2 = (*_local_20 + 0x10);
            u_var11 = u_var6;
            u_var10 = u_var6;
            u_var18 = ctx.dx_reg;
            ppc_var2(&ctx.PTR_LOOP_1050_1038, u_var6, ctx.dx_reg);
            _local_24 = CONCAT22(ctx.dx_reg, u_var11);
            local_28 = 0;
            while (true) {
                if (_local_24 <= local_28) {
                    if (_local_20 != 0x0) {
                        ppc_var2 = *_local_20;
                        ppc_var2(
                            u_var13,
                            u_var6,
                            ctx.dx_reg,
                            1,
                            u_var10,
                            u_var18,
                            _local_20,
                            _local_20,
                        );
                    }
                  ctx.PTR_LOOP_1050_50ca = 0x6b6;
                  ctx.PTR_LOOP_1050_50cc = (local_18 - 0x1e);
                    return 0;
                }
                u_var16 = param_5;
                u_var17 = (param_5 >> 8);
                u_var9 = _local_24;
                u_var14 = param_1_00;
              // u_var11 = (param_5  >> 0x10);
                pass1_1030_1d58(_local_20);
                pu_var7 = local_4;
                u_var13 = 0x1030;
                pass1_1030_bcde(
                    pu_var7,
                    unaff_ss,
                    (u_var9 & 0xffff | ctx.dx_reg << 0x10),
                    u_var14,
                    CONCAT22(u_var11, CONCAT11(u_var17, u_var16)),
                );
                if ((0 < pu_var7) && (pu_var7 < 0x1f)) {
                    break;
                }
                if (pu_var7 < local_18) {
                    local_18 = pu_var7;
                }
                local_28 = local_28 + 1;
            }
            if (_local_20 == 0x0) {
                return 0;
            }
            ppc_var2 = *_local_20;
            ppc_var2(
                0x1030,
                u_var6,
                ctx.dx_reg,
                1,
                u_var10,
                u_var18,
                _local_20,
                _local_20,
                u_var9,
                ctx.dx_reg,
            );
            return 0;
        }
    }
    return 0;
}

pub fn pass1_1028_c522(
    param_1: u16,
    param_2: u16,
    param_1_00: u32,
    param_2_00: u32,
    param_5: u32,
) -> i32 {
    let mut u_var1: u32;
    let local_AX_25: &mut  Struct836;
    let paVar2: &mut  Struct493;
    let local_AX_79: &mut  Struct836;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut u_var4: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: [u8; 2];

    u_var4 = pass1_1030_bcae(local_4, unaff_ss);
  // u_var3 = (u_var4  >> 0x10);
  // local_AX_25 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2_00, (param_2_00  >> 0x10));
    u_var1 = local_AX_25.field_0x10;
  // paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    local_AX_79 = local_4;
    pass1_1030_bcde(
        local_AX_79,
        unaff_ss,
        CONCAT22(u_var3, paVar2),
        param_1_00,
        param_5,
    );
    if (local_AX_79 < 0) {
      ctx.PTR_LOOP_1050_50ca = 0x6af;
    } else {
        if (local_AX_79 < 0x1f) {
            return 0;
        }
      ctx.PTR_LOOP_1050_50ca = 0x6b6;
      ctx.PTR_LOOP_1050_50cc = &local_AX_79[-2].field_0xa;
    }
    return 0;
}

pub fn pass1_1028_c5a6(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4_00: u32,
    param_5: u32,
) -> i32 {
    let mut u_var1: u16;
    let paVar2: &mut  Struct493;
    let mut u_var3: i32;
    let mut u_var4: i32;
    let lVar5: u32;
    let mut u_var6: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_4: u16;

    lVar5 = pass1_1030_627e(ctx._PTR_LOOP_1050_5740, param_2_00, param_5);
  // u_var3 = (lVar5  >> 0x10);
    u_var4 = u_var3 | lVar5;
    if (lVar5 != 0) {
        paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, lVar5, u_var3);
        _local_a = CONCAT22(u_var4, paVar2);
        local_e = 0x7a;
        if (0 < (param_2_00 + 4)) {
            if (param_1_00 == 0x7b) {
                param_1_00 = 0x7e;
            } else {
                if (param_1_00 == 0x7c) {
                    param_1_00 = 0x7d;
                }
            }
            local_e = 0x7f;
        }
        if (_local_a != 0x0) {
            u_var6 = pass1_1030_73a8(_local_a);
            if ((u_var6 != 0)
                && ((
                    u_var1 = (u_var6 + 0xc),
                    u_var1 == local_e || (u_var1 == param_1_00),
                )))
            {
                return 1;
            }
        }
    }
    return 0;
}

pub fn pass1_1028_c64a(
    param_1: u32,
    param_2: &mut  u32,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_4_00: u32,
) -> i32 {
    let mut u_var1: u16;
    let mut unaff_ss: u16;
    let mut u_var2: u16;
    let mut u_var3: u16;
    let mut local_18: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 2];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    unsafe { _local_8 = *param_2 };
    uStack4 = (param_2 + 1);
    pass1_1008_3eb4(
        CONCAT22(unaff_ss, &local_8),
        CONCAT22(unaff_ss, local_e),
        CONCAT22(unaff_ss, &local_c),
        CONCAT22(unaff_ss, &local_a),
    );
    _local_8 = _local_8 & 0xffff | (local_c - 1) << 0x10;
    u_var2 = param_1;
  // u_var3 = (param_1  >> 0x10);
    u_var1 = pass1_1028_c5a6(
        u_var2,
        u_var3,
        0x7b,
        CONCAT22(unaff_ss, &local_8),
        param_4_00,
    );
    if (u_var1 == 0) {
        _local_8 = _local_8 & 0xffff | (local_c + 1) << 0x10;
        u_var1 = pass1_1028_c5a6(
            u_var2,
            u_var3,
            0x7b,
            CONCAT22(unaff_ss, &local_8),
            param_4_00,
        );
        if (u_var1 == 0) {
            local_8 = local_a - 1;
            local_6 = local_c;
            u_var1 = pass1_1028_c5a6(
                u_var2,
                u_var3,
                0x7c,
                CONCAT22(unaff_ss, &local_8),
                param_4_00,
            );
            if (u_var1 == 0) {
                _local_8 = CONCAT22(local_6, local_a + 1);
                u_var1 = pass1_1028_c5a6(
                    u_var2,
                    u_var3,
                    0x7c,
                    CONCAT22(unaff_ss, &local_8),
                    param_4_00,
                );
                if (u_var1 == 0) {
                    return u_var1;
                }
            }
        }
    }
    return 1;
}

pub fn pass1_1028_c724(param_1: u32) {
    let mut u_var1: i32;
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut u_var4: u16;

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    u_var2 = (i_var3 + 0x14);
    if ((u_var2 + 0xac) != 0) {
        return;
    }
    u_var2 = (i_var3 + 0x14);
    u_var1 = (u_var2 + 0xa6);
    if (u_var1 == 0xd) {
        u_var2 = (i_var3 + 0x14);
        (u_var2 + 0xac) = 1;
        // goto LAB_1028_c770;
    }
    if (u_var1 < 0xe) {
        if (u_var1 == 0) {}
        // goto LAB_1028_c770;
        if (u_var1 == 0x7) {
            u_var2 = (i_var3 + 0x14);
            (u_var2 + 0xac) = 10;
            // goto LAB_1028_c770;
        }
    }
    u_var2 = (i_var3 + 0x14);
    (u_var2 + 0xac) = 5;
    // LAB_1028_c770:
    u_var2 = (i_var3 + 0x14);
    if ((u_var2 + 0xac) == 0) {
        u_var2 = (i_var3 + 0x14);
        if ((u_var2 + 0xa8) != 0) {
            u_var2 = (i_var3 + 0x14);
            (u_var2 + 0xac) = 1;
        }
        return;
    }
    return;
}

pub fn pass1_1028_c7b6(param_1: u16, param_2: u16, param_3: u32, param_4: u32) -> i32 {
    let pu_var1: &mut  u32;
    let paVar2: &mut  Struct493;
    
    let mut u_var3: u16;
    let mut u_var4: i32;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u32;
    let mut local_a: u32;
    let mut local_6: u32;

    pu_var1 = &local_a;
    pass1_1030_64ce(ctx._PTR_LOOP_1050_5740, param_1, param_2, pu_var1, unaff_ss);
    unsafe { local_6 = *pu_var1 };
    u_var3 = (pu_var1 + 2);
    local_1e._3_1_ = (local_6 >> 0x18);
    if (local_1e._3_1_ == '\0') {
        return 0;
    }
    paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_6, u_var3);
    u_var5 = pass1_1030_73a8(CONCAT22(u_var3, paVar2));
  // u_var4 = (u_var5  >> 0x10);
    if ((u_var4 | u_var5) != 0) {
        match (u_var5 + 0xc) {
            1 | 2 | 3 | 4 | 5 | 6 => {}
            7 | 8 | 9 => {
                return 0;
            }
        }
        return 0;
    }
    return 0;
}

pub fn pass1_1028_c89c(param_1: &mut Struct44, param_2: u32, param_3: &mut  u32) {
    let u_var1: u8;
    let pu_var2: &mut  u32;
    let extraout_var: u32;
    let mut in_dx: i32;
    
    let mut unaff_ss: u16;
    let mut local_16: [u32; 2];
    let mut local_e: u32;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1028_b58e(param_1);
    _local_6 = CONCAT31(extraout_var, u_var1) & 0xffff | in_dx << 0x10;
    local_a = (CONCAT31(extraout_var, u_var1) + 8);
    pu_var2 = local_16;
    pass1_1030_64ce(ctx._PTR_LOOP_1050_5740, param_2, local_a, pu_var2, unaff_ss);
    unsafe { *param_3 = *pu_var2 };
    return;
}

pub fn pass1_1028_c8ee(param_1: u16, param_2: u16, param_1_00: i32, param_2_00: &mut  u16) {
    let mut unaff_ss: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3eb4(
        param_2_00,
        CONCAT22(unaff_ss, &local_8),
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    if (param_1_00 == 1) {
        local_8 = local_8 + 1;
    } else {
        if (param_1_00 == 2) {
            _local_6 = _local_6 & 0xffff0000 | (local_6 - 1);
        } else {
            if (param_1_00 == 3) {
                _local_6 = _local_6 & 0xffff0000 | (local_6 + 1);
            } else {
                if (param_1_00 == 4) {
                    _local_6 = _local_6 & 0xffff | (local_4 + 1) << 0x10;
                } else {
                    if (param_1_00 == 5) {
                        _local_6 = _local_6 & 0xffff | (local_4 - 1) << 0x10;
                    }
                }
            }
        }
    }
    pass1_1008_3e76(param_2_00, local_8, _local_6, (_local_6 >> 0x10));
    return;
}

pub unsafe fn pass1_1028_c952(param_1: &mut Struct44) {
    let var1: &mut  Struct1121;
    let u_var2: u8;
    let var3: &mut  Struct837;
    let var4: bool;
    let mut var5: i32;
    let var6: u32;
    let mut var7: u32;
    let mut var8: i32;
    let mut var9: i32;
    let mut var10: i32;
    let var11: &mut Struct44;
    let mut var12: u16;
    let mut var13: u16;
    let mut var14: u32;
    let mut var15: u32;
    let mut var16: u16;
    let mut var17: u32 = 0;

    // var11 = param_1;
    var7 = param_1.field_0x14;
    var3 = var7;
    var8 = (&param_1.field_0x14 + 2) | var3;
    if var8 != 0 {
        u_var2 = pass1_1028_b58e(param_1);
        var9 = CONCAT31(var6, u_var2);
        var1 = (var9 + 0x2e);
        var17._0_2_ = var1;
        if (((var9 + 0x30) | var17) != 0)
            &&// u_var10 = (paVar1  >> 0x10), (local_e + 0x206) == 0))
        {
            var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, &param_1.field_0xc, 0x32);
            if var4 == 0 {
                var4 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, &param_1.field_0xc, 0x33);
                if ((var4 != 0) && ((*ctx._PTR_LOOP_1050_65e2 % 5) == 0)) {
                    return;
                }
            } else {
                if ((*ctx._PTR_LOOP_1050_65e2 % 10) == 0) {
                    return;
                }
            }
          // u_var9 = (u_var5  >> 0x10);
            if ((var17 + 0x204) == 0) {
                var16 = 0;
                while var16 < 0x25 {
                    var14 = (&var3.field_0x0 + var16 * 4);
                    var8 = var14;
                    var10 = (&var3.field_0x2 + var16 * 4) | var8;
                    if var10 != 0 {
                        var7 = var14;
                        pass1_1038_540a(var17, var13, var16);
                        var14 = (var14 >> 0x10);
                        if (var7 & 0xffff | var10 << 0x10) < var14 {
                            var5 = var8 - var7;
                            var9 = (var14 - var10) - (var8 < var7);
                            pass1_1038_52b8(var1, CONCAT22(var9, var5), 0x21);
                            var14 = CONCAT22(
                                (var14 - var9) - (var8 < var5),
                                var8 - var5,
                            );
                        }
                        if (var14 | var14) != 0 {
                            pass1_1038_52b8(var1, var14, var16);
                        }
                    }
                    var16 = var16 + 1;
                }
            } else {
                var8 = var3.field_0x8c;
                var10 = var3.field_0x8e;
                if (var10 | var8) != 0 {
                    pass1_1038_52b8(var1, CONCAT22(var10, var8), 0x23);
                }
                var8 = var3.field_0x90;
                var10 = var3.field_0x92;
                if (var10 | var8) != 0 {
                    pass1_1038_52b8(var1, CONCAT22(var10, var8), 0x24);
                    return;
                }
            }
        }
    }
    return;
}

pub fn pass1_1028_cb04(param_1: &mut Struct44) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let paVar3: Struct939;
    let u_var4: u8;
    let mut u_var5: i32;
    let extraout_var: u32;
    let mut u_var6: u32;
    let lVar7: u32;
    let mut u_var8: u32;
    let mut in_dx: i32;
    let local_bx_92: &mut  Struct839;
    let mut unaff_si: u16;
    let mut u_var9: u16;
    let mut u_var10: u16;
    let mut bVar11: bool;
    let ppVar12: &mut  Struct2551;
    let mut local_36: u32;
    let mut local_32: u32;
    let mut local_26: u32;
    let mut local_1e: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32 = 0;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let temp_7ffdd6893f6: &mut  Struct840;

    lVar7 = (param_1 + 0x14);
    if lVar7 != 0 {
        u_var4 = pass1_1028_b58e(param_1);
        temp_7ffdd6893f6 = CONCAT31(extraout_var, u_var4);
        paVar3 = (CONCAT31(extraout_var, u_var4) & 0xffff | in_dx << 0x10);
        u_var1 = &temp_7ffdd6893f6.field_0x2e;
        u_var5 = temp_7ffdd6893f6.field_0x30;
        local_e._0_2_ = u_var1;
        local_12 = u_var5 | local_e;
        if local_12 != 0 {
          // u_var9 = (u_var1  >> 0x10);
            if (local_e + 0x206) != 0 {
                return;
            }
            local_bx_92 = lVar7;
          // u_var10 = (lVar7  >> 0x10);
            if (local_e + 0x204) != 0 {
                u_var1 = local_bx_92.field_0x8c;
                u_var6 = u_var1;
                pass1_1038_540a(local_e, u_var9, 0x23);
                local_26 = (u_var1 >> 0x10);
                if (ctx.dx_reg <= local_26)
                    && ((
                        u_var5 = u_var6,
                        local_26._0_2_ = u_var1,
                        ctx.dx_reg < local_26 || (u_var5 < local_26),
                    ))
                {
                    pass1_1030_7d7c(
                        paVar3,
                        local_26 - u_var5,
                        CONCAT22(0x23, (local_26 - ctx.dx_reg) - (local_26 < u_var5)),
                    );
                    ppVar12 = process_struct_1010_20ba(
                        ctx.g_struct_var_1050_0ed0,
                        CONCAT22(unaff_si, 0x2b),
                    );
                    pass1_1010_043a(ppVar12, (local_e + 4), 0x12);
                }
                u_var1 = local_bx_92.field_0x90;
                u_var6 = u_var1;
                pass1_1038_540a(local_e, u_var9, 0x24);
                local_26 = (u_var1 >> 0x10);
                if ((ctx.dx_reg <= local_26)
                    && ((
                        u_var5 = u_var6,
                        local_26._0_2_ = u_var1,
                        ctx.dx_reg < local_26 || (u_var5 < local_26),
                    )))
                {
                    pass1_1030_7d7c(
                        paVar3,
                        local_26 - u_var5,
                        CONCAT22(
                            0x24,
                            (local_26 - ctx.dx_reg) - (local_26 < u_var5),
                        ),
                    );
                }
                return;
            }
            pass1_1038_540a(local_e, u_var5, 0x21);
            local_16 = 0x11;
            local_10 = ctx.dx_reg;
            while (local_16 < 0x25) {
                u_var2 = (&local_bx_92.field_0x0 + local_16 * 4);
                u_var8 = u_var2;
                pass1_1038_540a(local_e, u_var9, local_16);
                u_var8 = u_var8 & 0xffff | ctx.dx_reg << 0x10;
                local_36 = (u_var2 >> 0x10);
                if (u_var8 < u_var2) {
                    if ((((local_16 == 0x23) || (local_16 == 0x24)) || (local_10 < local_36))
                        || ((
                            u_var5 = u_var2,
                            local_10 <= local_36 && (local_12 < u_var5),
                        )))
                    {
                        lVar7 = u_var2 - u_var8;
                        pass1_1030_7d7c(paVar3, lVar7, CONCAT22(local_16, (lVar7 >> 0x10)));
                        if (local_16 == 0x23) {
                            ppVar12 = process_struct_1010_20ba(
                                ctx.g_struct_var_1050_0ed0,
                                CONCAT22(unaff_si, 0x2b),
                            );
                            pass1_1010_043a(ppVar12, (local_e + 4), 0x12);
                        }
                    } else {
                        bVar11 = local_12 < u_var5;
                        local_12 = local_12 - u_var5;
                        local_10 = (local_10 - local_36) - bVar11;
                    }
                }
                local_16 = local_16 + 1;
            }
            return;
        }
    }
    return;
}

pub fn pass1_1028_ccd0(param_1: &mut Struct44, param_2: &mut  u16) {
    let pp_var1: fn();
    let u_var2: u8;
    let pu_var3: Vec<u8>;
    let extraout_var: u32;
    let mut i_var4: i32;
    let mut i_var5: i32;
    
    let mut u_var6: i32;
    
    let mut unaff_ss: u16;
    let mut u_var7: u16;
    let u_var8: u8;
    let u_var9: u8;
    let mut u_var10: u16;
    let mut u_var11: u16;
    let mut u_var12: u16;
    let mut in_stack_0000fe80: u16;
    let mut local_178: u16;
    let mut local_176: u16;
    let mut local_54: u16;
    let mut local_48: u16;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u32;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u32;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: [u8; 12];
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1008_3eb4(
        param_2,
        CONCAT22(unaff_ss, &local_8),
        CONCAT22(unaff_ss, &local_6),
        CONCAT22(unaff_ss, &local_4),
    );
    u_var6 = ctx.dx_reg;
    u_var2 = pass1_1028_b58e(param_1);
    _local_14 = CONCAT31(extraout_var, u_var2) & 0xffff | u_var6 << 0x10;
    local_18 = (CONCAT31(extraout_var, u_var2) + 0x2e);
    local_1c = (local_18 + 4);
    pass1_1028_c1f8(
        param_1,
        (param_1 >> 0x10),
        0xe0,
        unaff_ss,
        &local_1e,
        unaff_ss,
    );
    local_a = local_4 - 1;
    i_var4 = local_4 + 1;
    local_c = local_6 - 1;
    i_var5 = local_6 + 1;
    if (local_a < 0) {
        local_a = 0;
    }
    if (local_1e <= i_var4) {
        i_var4 = local_1e - 1;
    }
    if (local_c < 0) {
        local_c = 0;
    }
    if (local_20 <= i_var5) {
        i_var5 = local_20 - 1;
    }
    _local_10 = CONCAT22(i_var4, i_var5);
    zero_list_1008_6c90(local_2c, unaff_ss);
    pass1_1008_6cec(
        CONCAT22(unaff_ss, local_2c),
        local_8,
        _local_10,
        local_8,
        CONCAT22(local_a, local_c),
    );
    _local_30 = process_struct_1010_20ba(
        ctx.g_struct_var_1050_0ed0,
        CONCAT22(in_stack_0000fe80, 0x2f),
    );
  // u_var6 = (_local_30  >> 0x10);
    local_34 = (_local_30 + 0x20);
    pu_var3 = local_2c;
    pass1_1030_6522(ctx._PTR_LOOP_1050_5740, CONCAT22(unaff_ss, pu_var3), local_34);
    _local_38 = CONCAT22(u_var6, pu_var3);
    if ((u_var6 | pu_var3) != 0) {
        local_3c = 0;
        local_3e = 0;
        local_40 = local_c;
        while (local_40 <= local_10) {
            local_48 = local_a;
            while (u_var7 = local_3e, local_48 <= local_e) {
                i_var4 = local_3e >> 0xf;
                pp_var1 = (*_local_38 + 4);
                local_3e = local_3e + 1;
                (**pp_var1)(0x1030, _local_38, (_local_38 >> 0x10), u_var7, i_var4);
                local_3c = CONCAT22(ctx.dx_reg, u_var7);
                local_3c._3_1_ = (ctx.dx_reg >> 8);
                if (local_3c._3_1_ == '\0') {
                    local_54 = u_var7;
                    if (u_var7 == 7) {
                        pass1_1008_3e76(param_2, local_8, local_40, local_48);
                        u_var11 = local_34;
                      // u_var12 = (local_34  >> 0x10);
                        u_var8 = local_1c;
                        u_var9 = (local_1c >> 8);
                      // u_var10 = (local_1c  >> 0x10);
                        u_var7 = 6;
                    } else {
                        if (u_var7 == 8) {
                            pass1_1008_3e76(param_2, local_8, local_40, local_48);
                            u_var11 = local_34;
                          // u_var12 = (local_34  >> 0x10);
                            u_var8 = local_1c;
                            u_var9 = (local_1c >> 8);
                          // u_var10 = (local_1c  >> 0x10);
                            u_var7 = 7;
                        } else {
                            if (u_var7 != 9) {}
                            // goto LAB_1028_ce2c;
                            pass1_1008_3e76(param_2, local_8, local_40, local_48);
                            u_var11 = local_34;
                          // u_var12 = (local_34  >> 0x10);
                            u_var8 = local_1c;
                            u_var9 = (local_1c >> 8);
                          // u_var10 = (local_1c  >> 0x10);
                            u_var7 = 8;
                        }
                    }
                    pass1_1028_87f0(
                        CONCAT22(unaff_ss, &local_178),
                        0,
                        0,
                        u_var7,
                        param_2,
                        (param_2 >> 0x10),
                        CONCAT22(u_var10, CONCAT11(u_var9, u_var8)),
                        CONCAT22(u_var12, u_var11),
                    );
                    pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_178));
                    local_178 = ctx.s_1_1050_389a;
                    local_176 = &ctx.PTR_LOOP_1050_1008;
                }
                // LAB_1028_ce2c:
                local_48 = local_48 + 1;
            }
            local_40 = local_40 + 1;
        }
    }
    return;
}

pub fn pass1_1028_ced2(param_1: &mut  Struct833) {
    let u_var1: u8;
    let extraout_ah: u8;
    let extraout_ah_00: u8;
    let mut in_dx: u16;
    
    let mut u_var2: i32;
    let mut u8_var3: bool;
    let mut u8_var4: bool;
    let mut u_var5: u16;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_4: u16;

  // u_var2 = (param_1  >> 0x10);
    u8_var3 = (*(param_1 + 0x1a) & 2) == 0;
    if (u8_var3) {
        u_var6 = 0;
        u_var7 = 0x23;
        u_var5 = 1;
        u_var1 = pass1_1028_b58e((param_1 & 0xffff | u_var2 << 0x10));
        pass1_1030_7d7c(
            CONCAT22(in_dx, CONCAT11(extraout_ah, u_var1)),
            u_var5,
            CONCAT22(u_var7, u_var6),
        );
        in_dx = ctx.dx_reg;
    }
    u8_var4 = (*(param_1 + 0x1a) & 1) == 0;
    if (u8_var4) {
        u_var6 = 0;
        u_var7 = 0xe;
        u_var5 = 1;
        u_var1 = pass1_1028_b58e((param_1 & 0xffff | u_var2 << 0x10));
        pass1_1030_7d7c(
            CONCAT22(in_dx, CONCAT11(extraout_ah_00, u_var1)),
            u_var5,
            CONCAT22(u_var7, u_var6),
        );
    }
    if (u8_var4 || u8_var3) {
        pass1_1028_bdac(param_1, 6);
        return 0;
    }
    return 1;
}

pub fn pass1_1028_cf44(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b418(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_cfd2(param_1: &mut  u32, param_2: u32) {
    unsafe { *param_1 = param_2 };
    (param_1 + 4) = 0;
    return;
}

pub fn pass1_1028_cff2(param_1: u32) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut u_var4: u16;

  // u_var4 = (param_1  >> 0x10);
    pu_var1 = (param_1 + 4);
    u_var2 = (param_1 + 6);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
    }
    return;
}

pub fn pass1_1028_d01a(param_1: &mut  u32) {
    let pu_var1: &mut  u32;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let mut u_var4: i32;
    let mut in_dx: i32;
    
    
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    unsafe { pu_var1 = *param_1 };
    _local_e = pu_var1;
    while (true) {
        u_var4 = _local_e;
        pass1_1028_d728(pu_var1);
        _local_e = CONCAT22(in_dx, u_var4);
        if ((in_dx | u_var4) == 0) {
            break;
        }
        u_var3 = *_local_e;
        ppc_var2 = u_var3 + 2;
        ppc_var2();
        in_dx = ctx.dx_reg;
        if (_local_e != 0x0) {
            ppc_var2 = u_var3;
            ppc_var2();
            in_dx = ctx.dx_reg;
        }
    }
    return;
}

pub fn pass1_1028_d078(param_1: u32, param_2: u32) {
    let pp_var1: fn();
    let pu_var2: &mut  u32;
    
    let struct_a: &mut  Struct199;
    
    
    let mut i_var3: i32;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut u_var5: u32;
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

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    pu_var2 = (i_var3 + 4);
    struct_a = (i_var3 + 6);
    _local_e = CONCAT22(struct_a, pu_var2);
    local_12 = pu_var2;
    local_10 = struct_a;
    if ((struct_a | pu_var2) != 0) {
        unsafe {
            pp_var1 = *pu_var2;
            (**pp_var1)();
        }
        struct_a = ctx.dx_reg;
    }
    process_struct_1000_179c(0x1c, struct_a);
    local_12 = pu_var2;
    local_10 = struct_a;
    if ((struct_a | pu_var2) == 0) {
        pu_var2 = 0x0;
        local_4 = 0;
    } else {
        process_struct_1008_8e9e(CONCAT22(struct_a, pu_var2), 6, 0x24);
        local_4 = ctx.dx_reg;
    }
    (i_var3 + 4) = pu_var2;
    (i_var3 + 6) = local_4;
  // local_a = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_2, (param_2  >> 0x10));
    local_6 = local_a;
    if ((local_4 | local_a) == 0) {
        pass1_1018_dcf6(CONCAT22(unaff_ss, &local_16));
        u_var5 = pass1_1018_dd1e(&local_16, unaff_ss, 0, 0xa0000);
        pass1_1008_8faa((i_var3 + 4), u_var5);
        return;
    }
    pass1_1038_565e(CONCAT22(local_4, local_a));
    if ((ctx.dx_reg | local_a) != 0) {
        local_8 = ctx.dx_reg;
        pass1_1028_d172(param_1, CONCAT22(ctx.dx_reg, local_a));
    }
    return;
}

pub fn pass1_1028_d172(param_1: u32, param_2: u32) {
    let mut unaff_ss: u16;
    let lVar1: u32;
    let mut u_var2: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: [u8; 8];
    let mut local_6: u16;
    let mut local_4: u16;

    pass1_1018_dcf6(CONCAT22(unaff_ss, &local_6));
    pass1_1008_5784(CONCAT22(unaff_ss, local_e), param_2);
    while (true) {
        lVar1 = pass1_1008_5b12(CONCAT22(unaff_ss, local_e));
        if (lVar1 == 0) {
            break;
        }
        u_var2 = pass1_1018_dd1e(&local_6, unaff_ss, 0, *(lVar1 + 4) << 0x10);
        pass1_1008_8faa((param_1 + 4), u_var2);
    }
    return;
}

pub fn pass1_1028_d1dc(struct_a: &mut  Struct500, string_a: String) {
    let local_struct_1: &mut  Struct500;
    let pc_var1: String;
    let in_stack_0000fffa: String;

  // pc_var1 = (struct_a  >> 0x10);
    local_struct_1 = struct_a;
    struct_a.a = ctx.s_1_1050_389a;
    local_struct_1.b = &ctx.PTR_LOOP_1050_1008;
    local_struct_1.c = string_a;
    local_struct_1.d = 0;
    struct_a.a = 0x6ad2;
    local_struct_1.b = &ctx.PTR_LOOP_1050_1028;
    string_fn_1000_3f9c(
        &local_struct_1.field_0x8,
        pc_var1,
        s_ctor_1050_5160,
        &ctx.g_alloc_addr_1050_1050,
        in_stack_0000fffa,
    );
    return struct_a;
}

pub fn pass1_1028_d22e(param_1: &mut  u32, param_2: u32) {
    let mut u_var1: i32;
    let in_dx: &mut  Struct199;
    let mut u_var2: i32;
    let mut u_var3: u16;
    let mut local_4: u16;

  // u_var3 = (param_1  >> 0x10);
    unsafe { *param_1 = 0 };
    (param_1 + 4) = param_2;
    process_struct_1000_179c(0xc, in_dx);
    u_var1 = param_2;
    u_var2 = in_dx | u_var1;
    if (u_var2 == 0) {
        unsafe { *param_1 = 0 };
    } else {
        pass1_1028_d59c((param_2 & 0xffff | ZEXT24(in_dx) << 0x10));
        param_1 = u_var1;
        (param_1 + 2) = u_var2;
    }
    return;
}

pub fn pass1_1028_d282(param_1: &mut  u32) {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut local_6: u16;
    let mut local_4: u16;

    unsafe { u_var1 = *param_1 };
    u_var2 = (param_1 + 2);
    _local_6 = CONCAT22(u_var2, u_var1);
    if ((u_var2 | u_var1) != 0) {
        pass1_1028_d658(CONCAT22(u_var2, u_var1));
        error_check_1000_17ce(_local_6);
    }
    return;
}

pub fn pass1_1028_d2b0(param_1: &mut u32) {
    let mut unaff_ss: u16;
    let mut local_10c: u16;
    let mut local_10a: u16;

    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, 16000);
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, (s_0_023_1050_3a93 + 5));
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, 14000);
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, 13000);
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(
        &local_10c,
        unaff_ss,
        (s__P__P__P__P__P__P__P__P__P__P__P_1050_2e35 + 0xab),
    );
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, (s_fem133_wav_1050_2af7 + 1));
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, (s_fem36_wav_1050_270c + 4));
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, (s_noth_bmp_1050_2321 + 7));
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, (s_547_bmp_1050_1f3f + 1));
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, (s_42_flc_1050_1b54 + 4));
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, 6000);
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, 5000);
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, 4000);
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, 3000);
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    pass16_funcs::pass1_1028_9c62(&local_10c, unaff_ss, 1000);
    pass1_1028_d566(param_1, CONCAT22(unaff_ss, &local_10c));
    local_10c = ctx.s_1_1050_389a;
    local_10a = &ctx.PTR_LOOP_1050_1008;
    unsafe { pass1_1028_d6b2(*param_1) };
    return;
}

pub fn pass1_1028_d52c(param_1: &mut  u32, param_2: u32, param_3: &mut  u32) {
    let pp_var1: fn();
    let mut i_var2: i32;
    let mut u_var3: u16;

    unsafe { pp_var1 = (*param_3 + 8) };
    i_var2 = (**pp_var1)();
    if (i_var2 != 0) {
        unsafe { u_var3 = pass1_1028_d776(*param_1, param_2, param_3) };
        if (u_var3 != 0) {
            return 1;
        }
    }
    return 0;
}

pub fn pass1_1028_d566(param_1: &mut  u32, param_2: u32) {
    let mut iVar1: i32;
    let mut u_var2: u16;
    let fn_ptr_1: fn();

    fn_ptr_1 = (param_2 + 8);
    iVar1 = (**fn_ptr_1)();
    if (iVar1 != 0) {
        unsafe { u_var2 = pass1_1028_d742(*param_1, param_2) };
        if (u_var2 != 0) {
            return 1;
        }
    }
    return 0;
}

pub fn pass1_1028_d59c(param_1: &mut  u32) {
    let pu_var1: &mut  u16;
    let mut u_var2: i32;
    let pu_var3: &mut  u16;
    let in_dx: &mut  Struct199;
    let struct_a: &mut  Struct199;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut local_e: u16;
    let mut local_c: u16;

  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
    unsafe { *param_1 = 0 };
    (i_var4 + 4) = 0;
    (i_var4 + 8) = 0;
    pu_var3 = *ctx._g_bool_1050_5748;
    unsafe { *param_1 = pu_var3 };
    process_struct_1000_179c(0xc, in_dx);
    pu_var1 = (pu_var3 & 0xffff | ZEXT24(in_dx) << 0x10);
    struct_a = (in_dx | pu_var3);
    if (struct_a == 0x0) {
        (i_var4 + 4) = 0;
    } else {
        process_struct_1008_574a((pu_var3 & 0xffff | ZEXT24(in_dx) << 0x10));
        unsafe { *pu_var1 = 0xd804 };
        (pu_var3 + 2) = &ctx.PTR_LOOP_1050_1028;
        (i_var4 + 4) = pu_var1;
        pu_var3 = pu_var1;
    }
    u_var2 = pu_var3;
    process_struct_1000_179c(0xc, struct_a);
    _local_e = CONCAT22(struct_a, u_var2);
    if ((struct_a | u_var2) == 0) {
        (i_var4 + 8) = 0;
    } else {
        process_struct_1008_574a(CONCAT22(struct_a, u_var2));
        *_local_e = 0xd804;
        (u_var2 + 2) = &ctx.PTR_LOOP_1050_1028;
        (i_var4 + 8) = _local_e;
    }
    return;
}

pub fn pass1_1028_d658(param_1: u32) {
    let pu_var1: &mut  u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut i_var4: i32;
    let mut u_var5: u16;

  // u_var5 = (param_1  >> 0x10);
    i_var4 = param_1;
    pu_var1 = (i_var4 + 4);
    u_var2 = (i_var4 + 6);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
    }
    pu_var1 = (i_var4 + 8);
    u_var2 = (i_var4 + 10);
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
    }
    return;
}

pub fn pass1_1028_d69e(param_1: u32) {
    let mut u_var1: u32;

    u_var1 = (param_1 + 4);
    return (u_var1 + 8);
}

pub fn pass1_1028_d6b2(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let ppc_var3: fn();
    let mut u_var4: i32;
    let pu_var5: &mut  u32;
    let pu_var6: &mut  u32;
    
    
    let mut u_var7: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    pu_var2 = *ctx._PTR_LOOP_1050_65e2;
    pu_var6 = pu_var2;
    while (true) {
        u_var4 = pu_var6;
      // u_var7 = (param_1  >> 0x10);
        pass1_1020_c860(*(param_1 + 8));
        let pu_var1_val = unsafe { *pu_var1 };
        if (((ctx.dx_reg | u_var4) == 0)
            || (
                pu_var1 = (u_var4 + 0xc),
                pu_var2 <= pu_var1_val && pu_var1_val != pu_var2,
            ))
        {
            break;
        }
        ppc_var3 = ((param_1 + 8) + 0x10);
        pu_var5 = pu_var2;
        (**ppc_var3)();
        pu_var6 = (pu_var5 & 0xffff | ctx.dx_reg << 0x10);
        pass1_1028_d742(param_1, pu_var5 & 0xffff | ctx.dx_reg << 0x10);
        if (pu_var6 != 0x0) {
            unsafe {
                ppc_var3 = *pu_var6;
                (**ppc_var3)(0x1020, pu_var5, ctx.dx_reg, 1);
            }
        }
    }
    return;
}

pub fn pass1_1028_d728(param_1: u32) {
    let pp_var1: fn();

    pp_var1 = ((param_1 + 4) + 0x10);
    (**pp_var1)();
    return;
}

pub fn pass1_1028_d742(param_1: u32, param_2: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    pp_var1 = (param_2 + 0xc);
    u_var2 = (**pp_var1)();
    pass1_1020_c872((param_1 + 4), (u_var2 + 4), u_var2);
    return 1;
}

pub fn pass1_1028_d776(param_1: u32, param_2: u32, param_3: &mut  u32) {
    let pp_var1: fn();
    let mut u_var2: u32;

    unsafe { pp_var1 = (*param_3 + 0xc) };
    u_var2 = (**pp_var1)();
    pass1_1020_c872((param_1 + 8), param_2, u_var2);
    return 1;
}

pub fn pass1_1028_b316(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1028_b260(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_b34c() {
    let pc_var1: String;
    let mut c_var2: u8;
    let mut in_eax: u32;
    let mut in_DL: u8;
    let mut in_bx: i32;
    let local_bx_23: &mut  Struct829;
    let pu_var3: &mut  u16;
    let unaff_bp: &mut  u16;
    let mut unaff_si: i32;
    let mut unaff_es: u16;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let mut in_CF: u8;
    let mut in_ZF: bool;
    let mut in_SF: u8;
    let mut in_OF: u8;
    let in_stack_0000d730: &mut  Struct1;

    pu_var3 = &stack0xfffe;
    c_var2 = '\x0f';
    while {
        unaff_bp = unaff_bp + -1;
        pu_var3 = pu_var3 + -1;
        unsafe { *pu_var3 = *unaff_bp };
        c_var2 = c_var2 + -1;
        '\0' < c_var2
    } {}
    if (!in_ZF && in_OF == in_SF) {
        pc_var1 = (in_bx + unaff_si);
        unsafe { *pc_var1 = *pc_var1 - in_DL };
        pass1_1030_1628(in_stack_0000d730);
      // u_var4 = (in_stack_0000d730  >> 0x10);
        local_bx_23 = in_stack_0000d730;
        local_bx_23.field_0xc = 0;
        local_bx_23.field_0xe = 0;
        local_bx_23.field_0x10 = 0;
        local_bx_23.field_0x12 = 0;
        local_bx_23.field_0x18 = 0;
        local_bx_23.field_0x1a = 0;
        local_bx_23.field_0x1c = 0;
        in_stack_0000d730.field_0x0 = 0xcf6a;
        local_bx_23.field_0x2 = &ctx.PTR_LOOP_1050_1028;
        local_bx_23.field_0x16 = 0;
        local_bx_23.field_0x14 = 0;
        return;
    }
    0x872 = unaff_es;
    (in_bx + 0xc) = CONCAT11((in_eax >> 8) + in_eax + in_CF, in_eax);
    return;
}

pub fn pass1_1028_b354(param_1: &mut  Struct763) {
    let local_bx_15: &mut  Struct763;
    let mut u_var1: u16;

    pass1_1030_1628(param_1);
  // u_var1 = (param_1  >> 0x10);
    local_bx_15 = param_1;
    &local_bx_15.field_0xc = 0;
    &local_bx_15.field_0xe = 0;
    &local_bx_15.field_0x10 = 0;
    &local_bx_15.field_0x12 = 0;
    &local_bx_15.field_0x18 = 0;
    &local_bx_15.field_0x1a = 0;
    &local_bx_15.field_0x1c = 0;
    param_1.field_0x0 = 0xcf6a;
    local_bx_15.field_0x2 = &ctx.PTR_LOOP_1050_1028;
    &local_bx_15.field_0x16 = 0;
    &local_bx_15.field_0x14 = 0;
    return;
}

pub fn pass1_1028_b39e(param_1: &mut Struct764, param_2: u16, param_3: u32) {
    let local_bx_25: &mut  Struct764;
    let mut u_var1: i32;

    pass1_1030_165e(param_1, 0x7000000, param_3);
  // u_var1 = (param_1  >> 0x10);
    local_bx_25 = param_1;
    &local_bx_25.field_0xc = param_2;
    &local_bx_25.field_0xe = 0x42;
    &local_bx_25.field_0x10 = 0;
    &local_bx_25.field_0x12 = 0;
    &local_bx_25.field_0x18 = 0;
    &local_bx_25.field_0x1a = 0;
    &local_bx_25.field_0x1c = 0;
    param_1.field_0x0 = 0xcf6a;
    local_bx_25.field_0x2 = &ctx.PTR_LOOP_1050_1028;
    pass1_1028_bf76((param_1 & 0xffff | u_var1 << 0x10));
    &local_bx_25.field_0x14 = 0;
    if (0x4e < &local_bx_25.field_0xc) && (&local_bx_25.field_0xc < 0x70) {
        &local_bx_25.field_0xe = 0x6b;
    }
    return;
}

pub unsafe fn pass1_1028_b418(param_1: &mut Struct7) {
    let mut iVar1: i32;
    let mut b_var2: u8;
    let u_var3: u8;
    let mut u_var4: i32;
    let mut i_var5: i32;
    let mut u_var6: u16;

  // u_var6 = (param_1  >> 0x10);
    i_var5 = param_1;
    param_1.ptr_a_lo = 0xcf6a;
    (i_var5 + 2) = &ctx.PTR_LOOP_1050_1028;
    iVar1 = (i_var5 + 0x12);
    if ((iVar1 == 4) || (iVar1 == 5))
        || ((
            u_var4 = iVar1 - 6,
            u_var4 == 0
                && ((
                    iVar1 = (i_var5 + 0x18),
                    iVar1 == 4 || (u_var4 = iVar1 - 5, u_var4 == 0),
                )),
        ))
    {
        b_var2 = error_check_1000_17ce((i_var5 + 0x14));
        u_var4 = b_var2;
    }
    u_var3 = u_var4;
    pass1_1030_16b2(param_1);
    return u_var3;
}

pub fn pass1_1028_b46e(param_1: &mut  Struct781, param_2: Vec<u8>) {
    let mut u_var1: u16;
    let u_var2: u8;
    let extraout_var: u32;
    let mut u_var4: i32;
    let pa_var5: &mut  Struct1095;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut u_var3: u32;

    pa_var5 = pass1_1028_b4f2(param_1);
  // u_var4 = (pa_var5  >> 0x10);
    u_var6 = 0;
    u_var7 = 0;
    u_var2 = pass1_1028_b58e(param_1);
    u_var3 = CONCAT31(extraout_var, u_var2);
    pass1_1030_6d80(u_var3 & 0xffff | u_var4 << 0x10, CONCAT22(u_var7, u_var6));
    u_var1 = (u_var3 + 0x32);
    if (u_var1 != 0) {
        pass1_1030_6c4c(u_var3 & 0xffff | u_var4 << 0x10, 0);
        pass1_1038_387e(pa_var5, 0, u_var1, u_var3 & 0xffff | u_var4 << 0x10);
    }
    pass1_1030_7296((u_var3 & 0xffff | u_var4 << 0x10));
    (param_1 + 0x1c) = (param_2 + 0x200);
    return;
}

pub fn pass1_1028_b4f2(param_1: &mut Struct44) {
    let u_var1: u8;
    let extraout_ah: u8;
    let mut in_dx: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var1 = pass1_1028_b58e(param_1);
    return CONCAT22(
        (CONCAT11(extraout_ah, u_var1) + 0x30),
        (CONCAT11(extraout_ah, u_var1) + 0x2e),
    );
}

pub fn pass1_1028_b514(param_1: u32) {
    let mut iVar1: i32;
    let u_var2: u8;
    let extraout_ah: u8;
    let mut in_dx: u16;
    let mut i_var3: i32;
    let mut u_var4: i32;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var4 = (param_1  >> 0x10);
    i_var3 = param_1;
    iVar1 = (i_var3 + 0x12);
    if (((iVar1 == 4) || (iVar1 == 5))
        || (iVar1 == 6 && ((iVar1 = (i_var3 + 0x18), iVar1 == 4 || (iVar1 == 5)))))
    {
        error_check_1000_17ce((i_var3 + 0x14));
    }
    (i_var3 + 0x14) = 0;
    (i_var3 + 0x12) = 7;
    u_var2 = pass1_1028_b58e((param_1 & 0xffff | u_var4 << 0x10));
    _local_6 = CONCAT22(in_dx, CONCAT11(extraout_ah, u_var2));
    pass1_1030_7296(CONCAT22(in_dx, CONCAT11(extraout_ah, u_var2)));
    pass1_1030_72d0(_local_6);
    pass1_1030_730a(_local_6);
    return;
}

pub fn pass1_1028_b58e(param_1: &mut Struct44) {
    let mut u_var1: u32;
    let paVar2: &mut  Struct493;

    u_var1 = (param_1 + 8);
  // paVar2 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    return paVar2;
}

pub fn pass1_1028_b5a8(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;

  // u_var2 = (param_1  >> 0x10);
    if ((param_1 + 0x12) != 5) {
        return 0;
    }
    u_var1 = (param_1 + 0x14);
    return (u_var1 + 0x94);
}

pub fn pass1_1028_b5ca(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u16;

  // u_var2 = (param_1  >> 0x10);
    if ((param_1 + 0x12) != 5) {
        return 0;
    }
    u_var1 = (param_1 + 0x14);
    return (u_var1 + 0x9c);
}

pub fn pass1_1028_afce(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    // let ctx.ax_reg: &mut  Struct825;
    let mut i_var3: i32;
    let in_dx: &mut  Struct199;
    let local_bx_43: &mut  Struct826;
    let pu_var4: &mut  u32;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x116, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        local_bx_43 = param_1;
        ctx.ax_reg.field_0x4 = local_bx_43.field_0x4;
        pu_var4 = &local_bx_43.field_0x8;
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var4;
            pu_var4 = pu_var4 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
        ctx.ax_reg.field_0x108 = local_bx_43.field_0x108;
        ctx.ax_reg.field_0x10c = local_bx_43.field_0x10c;
        ctx.ax_reg.field_0x110 = local_bx_43.field_0x110;
        ctx.ax_reg.field_0x114 = local_bx_43.field_0x114;
        *_local_a = 0xb0ce;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_b0a2(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_b0de(param_1: &mut  Struct500, param_2: u32, param_3: u32) -> &mut  Struct500 {
    pass16_funcs::pass1_1028_6af2(param_1, param_2, param_3);
    param_1.a = 0xb1f4;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_b108(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    // let ctx.ax_reg: &mut  Struct827;
    let mut i_var3: i32;
    // let in_dx: &mut  Struct199;
    let local_bx_43: &mut  Struct828;
    let pu_var4: &mut  u32;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x110, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        local_bx_43 = param_1;
        ctx.ax_reg.field_0x4 = local_bx_43.field_0x4;
        pu_var4 = &local_bx_43.field_0x8;
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var3 = 0x40;
        while (i_var3 != 0) {
            i_var3 = i_var3 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var4;
            pu_var4 = pu_var4 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
        ctx.ax_reg.field_0x108 = local_bx_43.field_0x108;
        ctx.ax_reg.field_0x10c = local_bx_43.field_0x10c;
        *_local_a = 0x6e50;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
        *_local_a = 0xb1f4;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_b1c8(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_b1f4() -> &mut  u16 {
    let pc_var1: String;
    let pu_var2: Vec<u8>;
    let pb_var3: Vec<u8>;
    let mut cVar4: u8;
    let mut in_dx: u16;
    let mut in_bx: i32;
    let pu_var5: &mut  u16;
    let unaff_bp: &mut  u16;
    let mut unaff_si: i32;
    let mut unaff_DI: i32;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let in_stack_0000d731: &mut  Struct1;

    pu_var5 = &stack0xfffe;
    cVar4 = '\x0f';
    unsafe {
        while {
            unaff_bp = unaff_bp + -1;
            pu_var5 = pu_var5 + -1;
            *pu_var5 = *unaff_bp;
            cVar4 = cVar4 + -1;
            '\0' < cVar4
        } {}
        pc_var1 = (in_bx + unaff_si);
        *pc_var1 = *pc_var1 - in_dx;
        pu_var2 = (in_bx + unaff_si);
        *pu_var2 = *pu_var2;
        pb_var3 = (&ctx.PTR_LOOP_1050_1028 + in_bx + unaff_DI);
        *pb_var3 = *pb_var3 | (in_dx >> 8);
    }
    pass1_1030_1628(in_stack_0000d731);
  // u_var6 = (in_stack_0000d731  >> 0x10);
    (in_stack_0000d731 + 0xc) = 0;
    in_stack_0000d731.field_0x0 = 0xb33c;
    (in_stack_0000d731 + 2) = &ctx.PTR_LOOP_1050_1028;
    return in_stack_0000d731;
}

pub fn pass1_1028_b204(param_1: &mut  u16) {
    let mut u_var1: u16;

    pass1_1030_1628(param_1);
  // u_var1 = (param_1  >> 0x10);
    unsafe {
        (param_1 + 0xc) = 0;
        *param_1 = 0xb33c;
    }
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_b22c(param_1: &mut  u16, param_2: u16, param_3: u32) {
    let mut u_var1: u16;

    pass1_1030_165e(param_1, 0x6000000, param_3);
  // u_var1 = (param_1  >> 0x10);
    (param_1 + 0xc) = param_2;
    unsafe { *param_1 = 0xb33c };
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1028;
    return param_1;
}

pub fn pass1_1028_b260(param_1: u32) {
    let in_AL: u8;

    param_1 = 0xb33c;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1028;
    pass1_1030_16b2(param_1);
    return in_AL;
}

pub fn pass1_1028_aec0(param_1: u32) {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let paVar3: &mut  Struct493;
    let mut in_dx: u16;
    let mut u_var4: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

  // u_var4 = (param_1  >> 0x10);
    u_var1 = (param_1 + 0x108);
  // paVar3 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var1, (u_var1  >> 0x10));
    u_var2 = paVar3[0x10].field_0x16;
    pass1_1030_375a(u_var2, (u_var2 >> 0x10), 0, (param_1 + 0x114));
    return;
}

pub fn pass1_1028_ad9c(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    // let ctx.ax_reg: &mut  Struct822;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    // let in_dx: &mut  Struct199;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        ctx.ax_reg.field_0x4 = (param_1 + 4);
        pu_var3 = (param_1 + 8);
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
        *_local_a = 0xae56;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_ae2a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_ab68() {
    let mut iVar1: i32;
    let ppc_var2: fn();
    let b_var3: bool;
    let mut u_var4: u16;
    let mut unaff_ss: u16;
    let pu_var5: &mut  u32;
    let mut local_24: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT13((unaff_ss >> 8), CONCAT12(unaff_ss, &local_14)),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    // LAB_1028_ab7e:
    pu_var5 = pass1_1028_e4ec(CONCAT22(unaff_ss, &local_14));
  // u_var4 = (pu_var5  >> 0x10);
    if (pu_var5 == 0x0) {
        return 1;
    }
    iVar1 = (pu_var5 + 0xc);
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, iVar1, 0x11);
    if (b_var3 == 0) {}
    // goto code_r0x1028abad;
    // goto LAB_1028_abc0;
    // code_r0x1028abad:
    b_var3 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, iVar1, 0x12);
    if (b_var3 != 0) {
        // LAB_1028_abc0:
        if ((pu_var5 + 0x12) == 5) {
            unsafe { ppc_var2 = (*pu_var5 + 0x30) };
            ppc_var2(&ctx.PTR_LOOP_1050_1008);
        }
    }
    // goto LAB_1028_ab7e;
}

pub fn pass1_1028_abec(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    let mut in_ax: i32;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    let in_dx: &mut  Struct199;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, in_ax);
    if ((in_dx | in_ax) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        (in_ax + 4) = (param_1 + 4);
        pu_var3 = (param_1 + 8);
        pu_var5 = (in_ax + 8);
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1028;
        *_local_a = 0xaca6;
        (in_ax + 2) = &ctx.PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_ac7a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_a9f4() {
    let pp_var1: fn();
    let BVar2: bool;
    let mut unaff_ss: u16;
    let pu_var3: &mut  u32;
    let mut local_24: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x700,
    );
    while (true) {
        pu_var3 = pass1_1028_e4ec(CONCAT22(unaff_ss, &local_14));
        if (pu_var3 == 0x0) {
            break;
        }
        BVar2 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (pu_var3 + 0xc), 0xc);
        if (BVar2 != 0) {
            unsafe { pp_var1 = (*pu_var3 + 0x34) };
            (**pp_var1)(&ctx.PTR_LOOP_1050_1008, pu_var3);
        }
    }
    return 1;
}

pub fn pass1_1028_aa68(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    // let ctx.ax_reg: &mut  Struct821;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    // let in_dx: &mut  Struct199;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        ctx.ax_reg.field_0x4 = (param_1 + 4);
        pu_var3 = (param_1 + 8);
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
        *_local_a = 0xab22;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_aaf6(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_a89c() {
    let mut unaff_ss: u16;
    let mut u_var1: u32;
    let mut local_22: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        u_var1 = pass1_1028_e4ec(CONCAT22(unaff_ss, &local_14));
        if (u_var1 == 0) {
            break;
        }
        if ((u_var1 + 0x200) != 0x8000002) {
            pass1_1038_3fca(u_var1);
        }
    }
    return 1;
}

pub fn pass1_1028_a8f4(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    // let ctx.ax_reg: &mut  Struct820;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    // let in_dx: &mut  Struct199;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        ctx.ax_reg.field_0x4 = (param_1 + 4);
        pu_var3 = (param_1 + 8);
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
        *_local_a = 0xa9ae;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_a982(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_a73c() {
    let mut unaff_ss: u16;
    let paVar1: &mut  Struct1120;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;

    pass1_1028_dc52(
        CONCAT22(unaff_ss, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        paVar1 = pass1_1028_e4ec(CONCAT22(unaff_ss, &local_14));
        if (paVar1 == 0x0) {
            break;
        }
        pass1_1038_5464(paVar1);
        pass1_1038_56d6(paVar1, 0);
        pass1_1038_518c(paVar1);
    }
    return 1;
}

pub fn pass1_1028_a79c(param_1: u32) {
    let pu_var1: &mut  u32;
    let pu_var2: &mut  u32;
    // let ctx.ax_reg: &mut  Struct819;
    let pu_var3: &mut  u32;
    let mut i_var4: i32;
    // let in_dx: &mut  Struct199;
    let pu_var5: &mut  u32;
    let mut u_var6: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    process_struct_1000_179c(0x108, in_dx);
    _local_a = CONCAT22(in_dx, ctx.ax_reg);
    if ((in_dx | ctx.ax_reg) != 0) {
        *_local_a = ctx.s_1_1050_389a;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1008;
      // u_var6 = (param_1  >> 0x10);
        ctx.ax_reg.field_0x4 = (param_1 + 4);
        pu_var3 = (param_1 + 8);
        pu_var5 = &ctx.ax_reg.field_0x8;
        i_var4 = 0x40;
        while (i_var4 != 0) {
            i_var4 = i_var4 + -1;
            pu_var2 = pu_var5;
            pu_var5 = pu_var5 + 1;
            pu_var1 = pu_var3;
            pu_var3 = pu_var3 + 1;
            unsafe { *pu_var2 = *pu_var1 };
        }
        *_local_a = 0x6ad2;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
        *_local_a = 0xa856;
        ctx.ax_reg.field_0x2 = &ctx.PTR_LOOP_1050_1028;
    }
    return;
}

pub fn pass1_1028_a82a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    param_1.ptr_a_lo = ctx.s_1_1050_389a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn pass1_1028_a28a(param_1: u16, param_2: u16, param_1_00: &mut  Struct817) {
    let pp_var1: fn();
    let mut u_var2: u16;
    let mut u_var3: u16;
    let pu_var4: Vec<u8>;
    let mut u_var5: u32;
    let mut u_var6: u32;
    let mut in_dx: i32;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let local_bx_33: &mut  Struct817;
    let mut u_var9: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;

    pu_var4 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0xe);
    u_var2 = SUB42(pu_var4, 0);
    pass1_1038_4d6e(param_1_00, pu_var4 & 0xffff | in_dx << 0x10);
    _local_a = CONCAT22(in_dx, u_var2);
  // u_var9 = (param_1_00  >> 0x10);
    local_bx_33 = param_1_00;
    u_var6 = local_bx_33.field_0x1f6;
    pp_var1 = (*_local_a + 0x10);
    u_var5 = u_var6;
    u_var7 = in_dx;
    (**pp_var1)(&ctx.PTR_LOOP_1050_1038, u_var2, in_dx);
    u_var3 = u_var5;
    u_var8 = u_var7;
    pass1_1030_38b8(u_var6, (u_var6 >> 0x10));
    if ((u_var5 & 0xffff | u_var7 << 0x10) == 0) {
        u_var6 = 100;
        u_var8 = 0;
    } else {
        u_var6 = CONCAT22(u_var8, u_var3) / (u_var5 & 0xffff | u_var7 << 0x10);
      // u_var8 = (u_var6  >> 0x10);
    }
    u_var6 = u_var6 & 0xffff | u_var8 << 0x10;
    if (_local_a != 0x0) {
        pp_var1 = *_local_a;
        (**pp_var1)(0x1030, u_var2, in_dx, 1);
    }
    if (u_var6 < 100) {
        if (u_var6 < 0x55) {
            if (u_var6 < 0x4b) {
                if (u_var6 < 0x32) {
                    if (u_var6 < 0x19) {
                        local_bx_33.field_0x20a = 1;
                        local_bx_33.field_0x20c = 0xffff;
                        return;
                    }
                    local_bx_33.field_0x20a = 0;
                    local_bx_33.field_0x20c = 0;
                    return;
                }
                local_bx_33.field_0x20a = 0xfffb;
            } else {
                local_bx_33.field_0x20a = 0xfff6;
            }
        } else {
            local_bx_33.field_0x20a = 0xfff1;
        }
    } else {
        local_bx_33.field_0x20a = 0xffec;
    }
    local_bx_33.field_0x20c = 1;
    return;
}

pub fn pass1_1028_a3ae(param_1: u16, param_2: u16, param_1_00: u32) {
    
    let mut u_var1: u16;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let mut u_var4: i32;
    let in_edx: u32;
    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let ppVar5: &mut  Struct2551;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut local_146: u16;
    let mut local_144: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1a: u32;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u16;
    let mut local_4: u16;

    i32_var6 = param_1_00;
  // u_var7 = (param_1_00  >> 0x10);
    pass1_1038_3fb0(param_1_00);
    local_4 = in_edx;
    if (((i32_var6 + 0x204) != 0)
        && (
            u_var1 = pass1_1030_25b2(CONCAT22(local_4, in_ax), 0x82),
            u_var1 != 0,
        ))
    {
        return;
    }
    u_var3 = (i32_var6 + 0x1f6);
    local_a = u_var3;
    pass1_1030_38b8(u_var3, (u_var3 >> 0x10));
    u_var2 = u_var3;
    local_10 = in_edx;
    _local_e = u_var3 & 0xffff | in_edx << 0x10;
    pass1_1038_540a(param_1_00, 0x1e);
    u_var4 = local_10 | u_var2;
    local_12 = u_var2;
    if ((((u_var4 == 0) && ((i32_var6 + 0x200) != 0x8000002))
        && (pass1_1030_38b8(local_a, (local_a >> 0x10)), -1 < u_var4))
        && (0 < u_var4 || (u_var2 != 0)))
    {
        ppVar5 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x2b));
      // local_1e = (ppVar5  >> 0x10);
        local_20 = ppVar5;
        pass1_1010_043a(ppVar5, (i32_var6 + 4), 0x11);
    }
    local_1a = _local_e;
    u_var2 = local_12 * 10;
    u_var4 = (local_10 * 5
        + CARRY2(local_12, local_12) * 2
        + CARRY2(local_12 * 2, local_12 * 2)
        + CARRY2(local_12 * 4, local_12))
        * 2
        + CARRY2(local_12 * 5, local_12 * 5);
    _local_16 = CONCAT22(u_var4, u_var2);
    if ((u_var4 <= local_c) && (u_var4 < local_c || (u_var2 < _local_e))) {
        pass1_1028_ae66(
            CONCAT22(unaff_ss, &local_146),
            _local_e,
            CONCAT22(u_var4, u_var2),
            (i32_var6 + 4),
        );
        pass1_1030_835a(ctx._g_bool_1050_5748, CONCAT22(unaff_ss, &local_146));
        local_1a = _local_16;
        local_146 = ctx.s_1_1050_389a;
        local_144 = &ctx.PTR_LOOP_1050_1008;
    }
    local_1a = local_1a + 9;
    pass1_1038_52b8(param_1_00, local_1a / 10, 0x1e);
    return;
}

pub fn pass1_1028_a4ee(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let ppc_var2: fn();
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut u_var5: u16;
    let mut u_var6: i32;
    let pu_var7: Vec<u8>;
    let mut u_var8: u32;
    let mut u_var9: i32;
    let mut u_var10: i32;
    let mut u_var11: u16;
    let in_edx: u32;
    let mut u_var12: u16;
    let u_var13: u8;
    let mut u_var14: u16;
    let mut local_32: u16;
    let mut local_30: u32;
    let mut local_22: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_a: u32;
    let mut local_6: u32;

  // u_var12 = (param_2  >> 0x10);
    u_var1 = (param_2 + 0x1f6);
    u_var8 = *ctx._PTR_LOOP_1050_65e2;
    pu_var7 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x26);
    u_var9 = in_edx;
    u_var3 = pu_var7;
    u_var13 = 0x38;
    pass1_1038_4d6e(param_2, pu_var7 & 0xffff | in_edx << 0x10);
    _local_12 = CONCAT22(u_var9, u_var3);
    ppc_var2 = (*_local_12 + 0x10);
    u_var6 = u_var3;
    u_var10 = u_var9;
    ppc_var2(&ctx.PTR_LOOP_1050_1038, u_var3, u_var9);
    if ((u_var10 | u_var6) != 0) {
        u_var13 = 0x30;
        pass1_1030_3548(u_var1, CONCAT22(u_var10, u_var6));
    }
    if (_local_12 != 0x0) {
        ppc_var2 = *_local_12;
        ppc_var2(u_var13, u_var3, u_var9, 1);
    }
    u_var11 = (u_var8 % 0xc);
  // u_var14 = (param_1  >> 0x10);
    u_var4 = u_var11;
    if (u_var8 % 0xc == 0) {
        pass1_1030_387c(u_var1);
        pass16_funcs::pass1_1028_a61e(param_1, u_var14, u_var1, param_2);
    }
    pass1_1038_3fb0(param_2);
    if (((param_2 + 0x204) != 0)
        && (
            u_var5 = pass1_1030_25b2(CONCAT13((u_var11 >> 8), CONCAT12(u_var11, u_var4)), 0x80),
            u_var5 != 0,
        ))
    {
        return;
    }
  // u_var12 = (u_var1  >> 0x10);
    u_var6 = u_var1 + 0x180;
    u_var8 = u_var6;
    local_32 = 1;
    while {
        if ((local_32 * 2 + u_var6) != 0) {
            pass1_fn_1008_612e(1, 100);
            if (u_var8 <= (local_32 * 2 + u_var6)) {
                pass1_1028_a188(
                    param_1,
                    u_var14,
                    (local_32 * 2 + u_var1 + 0x174),
                    local_32,
                    param_2,
                );
            }
        }
        local_32 = local_32 + 1;
        local_32 < 6
    } {}
    return;
}

// WARNING: This function may have set the stack p i32er
