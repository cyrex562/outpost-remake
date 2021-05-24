use crate::app_context::AppContext;
use crate::bad_funcs::bad1::halt_baddata;
use crate::err_ops::{error_check_1000_0dc6, error_check_1000_17ce};
use crate::file_ops::{close, file2, read, write};
use crate::file_ops::read::read_from_file_1008_6f7a;
use crate::list_funcs::set_array_val_1008_72a8;
use crate::mem_funcs::Address;
use crate::mem_funcs::alloc_mem::{alloc_mem_1000_07fc, alloc_mem_1000_0a48, alloc_mem_1000_1708};
use crate::mem_funcs::mem_ops_1::get_fn_ptr_at_address;
use crate::other_funcs::mixed_fn_1010_830a;
use crate::other_funcs::{modify_list_1008_3f62, set_param_3_with_switch_1008_72bc, switch_statement_1008_738c, switch_statement_1008_73ea, zero_list_1008_3e38};
use crate::pass::pass12_funcs::pass1_1008_c6ae;
use crate::pass::pass13_funcs::pass1_1008_b0bc;
use crate::pass::pass14_funcs::{pass1_1008_3e76, pass1_1008_3eb4, pass1_1008_3f92, pass1_1008_4b8e, pass1_1008_5784, pass1_1008_5b12, pass1_1008_6eee, pass1_1008_7006, pass1_1008_7056, pass1_1008_766e, pass1_fn_1008_60e8, pass1_fn_1008_612e};
use crate::pass::pass15_funcs::{pass1_1020_8712, pass1_1020_ba3e, pass1_1020_bb16, pass1_1020_bb8a, pass1_1020_c444, pass1_1020_c4a8, process_struct_1020_847a};
use crate::pass::pass16_funcs::{pass1_1028_780c, pass1_1028_90e6};
use crate::pass::pass17_funcs::{pass1_1030_1cd8, pass1_1030_2068, pass1_1030_2958, pass1_1030_560e, pass1_1030_5d0a, pass1_1030_61fe, pass1_1030_6222, pass1_1030_66de, pass1_1030_671c, pass1_1030_6740, pass1_1030_67cc, pass1_1030_73ee, pass1_1030_7e5a, pass1_1030_835a};
use crate::pass::pass18_funcs::pass1_1038_30aa;
use crate::pass::pass20_funcs::{pass1_1010_ed22, pass1_1018_0196};
use crate::pass::pass2_funcs::pass1_1010_ebf8;
use crate::pass::pass3_funcs::pass1_1028_21ba;
use crate::pass::pass4_funcs::{pass1_1028_b204, pass1_1028_e1ec, pass1_1028_e2ac, pass1_1030_0000, pass1_1030_145a, pass1_1030_14b4, ret_1030_154c};
use crate::pass::pass6_funcs::pass1_1038_79b2;
use crate::pass::pass7_funcs::{pass1_1018_2678, pass1_1018_268e, pass1_1018_26c2};
use crate::pass::pass8_funcs::pass1_1010_043a;
use crate::string_ops::misc::{get_string_index_1000_3da4, process_string_1008_7e4a, string_fn_1000_3f9c};
use crate::struct_ops::struct_ops_2::{process_struct_1000_179c, process_struct_1008_41bc, process_struct_1008_48fe, process_struct_1008_4c98, process_struct_1008_574a, process_struct_1008_dcdc, struct_fn_1000_160a};
use crate::struct_ops::struct_ops_2::pass1_1038_6520;
use crate::structs::prog_structs_1::{Struct104, Struct393};
use crate::structs::prog_structs_10::Struct933;
use crate::structs::prog_structs_11::Struct934;
use crate::structs::prog_structs_12::Struct235;
use crate::structs::prog_structs_16::{Struct1143, Struct493};
use crate::structs::prog_structs_17::Struct407;
use crate::structs::prog_structs_2::{Struct131, Struct199, Struct390, Struct7};
use crate::structs::prog_structs_20::{Struct120, Struct506};
use crate::structs::prog_structs_23::{Struct1167, Struct1168, Struct935};
use crate::structs::prog_structs_24::{Struct103, Struct2111, Struct8};
use crate::structs::prog_structs_25::Struct9;
use crate::structs::prog_structs_26::{Struct1140, Struct1141, Struct1142, Struct121};
use crate::structs::prog_structs_28::{FileObject, Struct936, Struct961};
use crate::structs::prog_structs_29::{Struct425, Struct763};
use crate::structs::prog_structs_31::Struct962;
use crate::structs::prog_structs_5::Struct1174;
use crate::structs::prog_structs_6::Struct473;
use crate::structs::prog_structs_9::Struct844;
use crate::typedefs::HFILE16;
use crate::util::{CARRY2, CONCAT11, CONCAT12, CONCAT13, CONCAT22, CONCAT31, SBORROW2, SUB21, SUB41, SUB42, ZEXT24};
use crate::winapi::{_hread, _hwrite16, _lclose16, _lcreat16, _llseek16, _lopen16};

pub unsafe fn file_fn_1008_6414(ctx: &mut AppContext, param_1: &mut  Struct7, param_2: u32) {
    let pp_var1: fn();
    let in_struct_a: &mut  Struct103;
    let paVar2: &mut  Struct7;
    let struct_a: &mut  Struct199;
    let paVar3: &mut  Struct199;


    let local_Struct117: &mut  Struct7;
    let mut local_es_4: u16;

    let mut local_2a: u16;
    let mut local_28: u16;
    let local_26: u8;

    //// ocal_es_4 = (param_1  >> 0x10);
    local_Struct117 = param_1;
    // param_1 = 0;
    local_Struct117.field_0x4 = 0;
    paVar2 = &local_26;
    process_struct_1008_48fe(ctx, paVar2,
        1,
        param_2,
    );
    paVar3 = struct_a;
    process_struct_1000_179c(0x1e, struct_a);
    if ((paVar3 | paVar2) == 0) {
        // param_1 = 0;
    } else {
        in_struct_a = CONCAT22(paVar3, paVar2);
        paVar2 = &local_26;
        pass1_1008_3f92(in_struct_a, CONCAT22(ctx.stack_seg_reg, &local_26));
        *param_1 = paVar2;
        local_Struct117.field_0x2 = ctx.dx_reg;
    }
    pp_var1 = (param_1 + 0x14);
    (**pp_var1)(0x1000, param_1);
    local_Struct117.field_0x4 = paVar2;
    local_Struct117.field_0x6 = ctx.dx_reg;
    close::close_file_1008_496c(ctx, &local_26);
    return;
}

pub fn pass1_1030_d72e(param_1: u32, param_2: &HFILE16) {

    let b_var1: bool;
    let mut i_var2: i32;
    let mut unaff_ss: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    read::read_from_file_1028_b81a(param_1, param_2);
    if (ctx.ax_reg == 0) {
        return;
    }
    local_a = 0;
    while (true) {
        if (9 < local_a) {
            if ((3 < PTR_LOOP_1050_0312)
                && (
                b_var1 = read::read_file_1008_7dee(param_2, &PTR_LOOP_1050_5812, 2),
                b_var1 == 0,
                ))
            {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            return;
        }
        b_var1 = read::read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_8), 4);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return;
        }
        b_var1 = read::read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
        if (b_var1 == 0) {
            break;
        }
        i_var2 = local_a * 0xc + param_1;
        (i_var2 + 0x20) = local_8;
        (i_var2 + 0x22) = local_6;
        set_param_3_with_switch_1008_72bc(param_2, (param_2 >> 0x10), local_4);
        (i_var2 + 0x24) = local_8;
        if (PTR_LOOP_1050_0312 < 2) {
            i_var2 = local_a * 0xc + param_1;
            (i_var2 + 0x26) = 3;
            (i_var2 + 0x28) = 0;
        } else {
            b_var1 = read::read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
            if (b_var1 == 0) {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            b_var1 = read::read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_8), 4);
            if (b_var1 == 0) {
                ctx.g_u16_1050_0310 = 0x6d2;
                return;
            }
            i_var2 = local_a * 0xc + param_1;
            (i_var2 + 0x26) = local_4;
            (i_var2 + 0x28) = _local_8;
        }
        local_a = local_a + 1;
    }
    ctx.g_u16_1050_0310 = 0x6d2;
    return;
}

pub fn pass1_1030_c84e(param_1: &mut FileObject, param_2: &HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    b_var1 = write::write_to_file_1028_b5ec(param_1, param_2);
    if (b_var1 != 0) {
        local_c = (param_1 + 0x20);
        b_var1 = write::write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return b_var1;
        }
        b_var1 = 1;
    }
    return b_var1;
}

pub unsafe fn pass1_1030_b836(struct_b: &mut  Struct962, file_a: &HFILE16) {
    let pp_var1: fn();
    // let local_AX_11: &mut  Struct962;
    let BVar2: bool;
    let mut u_var3: u16;
    let mut u_var4: u32;

    let struct_a: &mut  Struct199;

    let mut u_var5: u16;
    // let ctx.dx_reg: &mut  Struct199;
    let mut u_var6: u16;
    // let mut unaff_ss: u16;
    let pHVar7: &HFILE16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_4: u16;

    local_AX_11 = struct_b;
    local_AX_11 = &local_AX_11.field_0x4;
    BVar2 = read::read_file_1008_7dee(file_a, (struct_b & 0xffff0000 | ZEXT24(local_AX_11)), 4);
    if (BVar2 != 0) {
        u_var4 = struct_b & 0xffff0000 | &local_AX_11.field_0x8;
        read::read_file_1008_7bc8(ctx, file_a, u_var4);
        if ((u_var4 != 0)
            && (
            struct_a = ctx.dx_reg,
            BVar2 = read::read_file_1008_7dee(file_a, CONCAT22(unaff_ss, &local_4), 2),
            BVar2 != 0,
            ))
        {
          // u_var6 = (struct_b  >> 0x10);
            local_AX_11.field_0xe = local_4;
            BVar2 = read::read_file_1008_7dee(file_a, CONCAT22(unaff_ss, &local_12), 2);
            if (BVar2 == 0) {
                return;
            }
            while (true) {
                if (local_12 == 0) {
                    return;
                }
                u_var3 = local_12;
                pHVar7 = file_a;
                local_12 = local_12 - 1;
                process_struct_1000_179c(0x2a, struct_a);
                if ((struct_a | u_var3) == 0) {
                    u_var3 = 0;
                    u_var5 = 0;
                } else {
                    pass1_1038_6520(CONCAT22(struct_a, u_var3));
                    u_var5 = ctx.dx_reg;
                }
                read::read_from_file_1038_774e(u_var3, u_var5, pHVar7);
                if (u_var3 == 0) {
                    break;
                }
                pp_var1 = (local_AX_11.field_0x10 + 4);
                (**pp_var1)();
                struct_a = ctx.dx_reg;
            }
            return;
        }
    }
    ctx.g_u16_1050_0310 = 0x6d2;
    return;
}

pub fn pass1_1030_227a(param_1: &mut FileObject, param_2: &HFILE16) {
    let mut i_var1: i32;
    let BVar2: bool;

    i_var1 = write::write_to_file_1030_1978(param_1, param_2);
    if (i_var1 != 0) {
        i_var1 = param_1;
        BVar2 = write::write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (i_var1 + 0x10)), 0x106);
        if (BVar2 != 0) {
            BVar2 =
                write::write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (i_var1 + 0x116)), 0x86);
            if (BVar2 != 0) {
                BVar2 =
                    write::write_to_file_1008_7e1c(param_2, (param_1 & 0xffff0000 | (i_var1 + 0x19c)), 10);
                if (BVar2 != 0) {
                    BVar2 = write::write_to_file_1008_7e1c(
                        param_2,
                        (param_1 & 0xffff0000 | (i_var1 + 0x1a6)),
                        0x106,
                    );
                    if (BVar2 != 0) {
                        BVar2 = write::write_to_file_1008_7e1c(
                            param_2,
                            (param_1 & 0xffff0000 | (i_var1 + 0x2ac)),
                            0x106,
                        );
                        if (BVar2 != 0) {
                            return;
                        }
                    }
                }
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub unsafe fn pass1_1030_1b18(ctx: &mut AppContext, param_1: u32, param_2: &mut HFILE16) -> u16 {
    let pi_var1: &mut  i32;
    let mut u_var2: u32;
    let mut u_var3: i32;

    let mut u_var4: u16;
    let b_var5: bool;
    let mut u_var6: i32;
    let in_dx: &mut  u16;
    let struct_a: &mut  Struct199;
    let mut i_var7: i32;
    let mut i_var8: i32;
    let mut u_var9: u16;
    let mut local_18: u16;

    pass1_1030_19b4(param_1, param_2);
    if (ctx.ax_reg != 0) {
        if (ctx.g_struct_ptr_1 == 0) {
            struct_fn_1000_160a();
            ctx.g_u16_ptr_1050_5f2e = in_dx;
        } else {
        }
        u_var4 = alloc_mem_1000_1708();
      // u_var9 = (param_1  >> 0x10);
        i_var7 = param_1;
        (i_var7 + 0x10) = u_var4;
        (i_var7 + 0x12) = ctx.g_u16_ptr_1050_5f2e;
        struct_a = (i_var7 + 0x12);
        b_var5 = read::read_file_1008_7dee(param_2, CONCAT22(struct_a, (i_var7 + 0x10)), 2);
        if (b_var5 != 0) {
            pi_var1 = (i_var7 + 0x10);
            let pi_var1_val = unsafe { *pi_var1 };
            if (pi_var1_val == 0) {
                return 1;
            }
            u_var3 = pi_var1_val * 2;
            u_var6 = u_var3;
            process_struct_1000_179c(u_var3, struct_a);
            u_var2 = (i_var7 + 0x10);
          // u_var4 = (u_var2  >> 0x10);
            i_var8 = u_var2;
            (i_var8 + 2) = u_var6;
            (i_var8 + 4) = struct_a;
            u_var2 = (i_var7 + 0x10);
            b_var5 = read::read_file_1008_7dee(param_2, *(u_var2 + 2), u_var3);
            if (b_var5 != 0) {
                return 1;
            }
        }
        ctx.g_u16_1050_0310 = 0x6d2;
    }
    return 0;
}

pub unsafe fn pass1_1030_19b4(ctx: &mut AppContext, param_1: &mut Struct933, param_2: &HFILE16) {
    let u_var1: u8;
    let extraout_var: u32;
    let mut u_var2: u32;

    u_var1 = read::pass1_1030_1730(param_1, param_2);
    if u_var1 != 0 {
        u_var2 = param_1.field_0xc;
        read::read_file_1008_76e4(ctx, param_2, &mut u_var2);
        if u_var2 == 0 {
            ctx.g_u16_1050_0310 = 0x6d2;
            return;
        }
    }
    return;
}

pub unsafe fn pass1_1028_e628(param_1: u32, param_2: u16, param_3: u16, param_3_00: i32, param_5: i32) {
    let pc_var1: String;
    let pi_var2: &mut  i32;
    let mut u_var3: u32;
    let mut cVar4: u8;
    let pu_var5: &mut  u32;
    let mut u_var6: u32;
    let pa_var7: &mut  Struct844;
    let mut u_var8: u32;
    let lVar9: u32;
    let pp_var10: fn();
    let pu_var11: &mut  u16;
    let u_var12: u8;
    let u_var13: u8;
    let u_var14: u8;
    let uVar15: u8;
    let uVar16: u8;
    let uVar17: u8;
    let paVar18: &mut  Struct493;
    let pu_var19: &mut  u16;
    let mut u_var20: u16;
    let BVar21: bool;
    let mut u_var22: i32;
    let pu_var23: Vec<u8>;
    let mut u_var24: u32;

    let mut i_var25: i32;
    // let ctx.dx_reg: &mut  Struct199;
    // let ctx.dx_reg: &mut  Struct199;
    // let ctx.dx_reg: &mut  Struct199;

    // let mut extraout_dx_04: u16;
    // let mut extraout_dx_05: u16;
    // let mut extraout_dx_06: u16;
    // let mut extraout_dx_07: u16;
    // let extraout_dx_08: &mut  Struct199;
    // let mut extraout_dx_09: u16;
    // let mut extraout_dx_10: u16;
    let paVar26: &mut  Struct199;
    let mut u_var27: u16;
    // let mut unaff_si: u16;
    // let unaff_DI: Vec<u8>;
    // let mut unaff_es: u16;
    let mut u_var28: u16;
    // let mut unaff_cs: u16;
    let mut u_var29: u16;
    let mut uVar30: u16;
    // let mut unaff_ss: u16;
    let mut bVar31: bool;
    let mut bVar32: bool;
    let ppVar33: &mut  Struct2111;
    let pu_var34: &mut  u16;
    let paVar35: &mut  Struct763;
    let mut local_154: u16;
    let mut local_152: u16;
    let mut local_14c: u16;
    let mut local_14a: u16;
    let mut uStack80: u16;
    let mut uStack78: u16;
    let uStack76: u8;
    let uStack75: u8;
    let mut local_4a: u16;
    let mut local_48: u16;
    let mut local_46: u16;
    let mut local_44: u16;
    let mut local_42: u16;
    let mut local_40: u16;
    let mut uVar36: u16;
    let mut local_3a: u16;
    let mut local_38: u32;
    let mut uStack52: u16;
    let mut uStack50: u16;
    let mut local_30: u32;
    let mut local_2c: u16;
    let mut local_2a: u32;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u32;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u32;
    let mut local_6: u32;

    uVar36 = SUB42(&ctx.g_alloc_addr_1050_1050, 0);
    local_42._0_1_ = SUB21(&local_6, 0);
    local_42._1_1_ = (&local_6 >> 8);
    local_46._0_1_ = param_2;
    u_var12 = local_46;
    local_46._1_1_ = (param_2 >> 8);
    uVar15 = local_46._1_1_;
    local_44 = param_3;
    local_4a._0_1_ = 0x3f;
    local_4a._1_1_ = 0xe6;
    local_48 = unaff_cs;
    local_3a = unaff_si;
    local_38._0_2_ = unaff_DI;
    BVar21 = read::read_file_1008_7dee(CONCAT22(param_3, param_2), CONCAT22(unaff_ss, &local_6), 4);
    u_var29 = local_44;
    uVar16 = local_46._1_1_;
    u_var13 = local_46;
    if (BVar21 == 0) {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    local_a = 0;
    if (((param_3_00 == 0) && ((param_5 - 0x100) == '\0'))
        && (
            paVar26 = (param_5 - 0x100 >> 7),
            paVar26 < (&PTR_LOOP_1050_000e + 1),
        ))
    {
        uVar30 = &PTR_LOOP_1050_1028;
        local_46._0_1_ = param_1;
        u_var14 = local_46;
        local_46._1_1_ = (param_1 >> 8);
        uVar17 = local_46._1_1_;
      // local_44 = (param_1  >> 0x10);
        u_var28 = local_44;
        u_var27 = param_1;
        match paVar26 {
            0x0 => {
                pa_var7 = (u_var27 + 0xe);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
              // local_40 = (pa_var7  >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0x73;
                local_46._1_1_ = 0xe6;
                pass1_1030_145a(pa_var7, local_6);
                local_1e._2_2_ = 0;
                local_1a = 0;
                while (CONCAT22(local_1a, local_1e._2_2_) < local_6) {
                    u_var29 = 0x1000;
                    local_40 = 0xe6d5;
                    u_var24 = local_6;
                    process_struct_1000_179c(0x14, paVar26);
                    local_20 = u_var24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        u_var22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        u_var29 = 0x1030;
                        local_42._0_1_ = 0xeb;
                        local_42._1_1_ = 0xe6;
                        u_var22 = local_20;
                        pass1_1030_5d0a();
                        local_40 = extraout_dx_04;
                    }
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = 0x97;
                    local_46._1_1_ = 0xe6;
                    pp_var10 = (CONCAT22(local_40, u_var22) + 0x10);
                    local_44 = u_var29;
                    local_18 = u_var22;
                    local_16 = local_40;
                    (**pp_var10)();
                    if (u_var22 == 0) {
                        return;
                    }
                    u_var24 = (local_18 + 4);
                    u_var22 = (local_18 + 6);
                    local_e = u_var24;
                  // local_c = (u_var24  >> 0x10);
                    paVar26 = (u_var22 & 0xff);
                    local_40 = local_16;
                    local_42._0_1_ = local_18;
                    local_42._1_1_ = (local_18 >> 8);
                    u_var8 = (u_var27 + 0xe);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                  // local_44 = (u_var8  >> 0x10);
                    local_4a._0_1_ = 0xc0;
                    local_4a._1_1_ = 0xe6;
                    local_48 = u_var29;
                    pass1_1030_14b4(
                        u_var8,
                        local_18,
                        local_16,
                        u_var24 & 0xffff | (u_var22 & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(local_1a, local_1e._2_2_) + 1;
                    local_1e._2_2_ = lVar9;
                  // local_1a = (lVar9  >> 0x10);
                }
            }
            0x1 => {
                // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
            }
            0x2 => {
                pa_var7 = (u_var27 + 0x12);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
              // local_40 = (pa_var7  >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0xf;
                local_46._1_1_ = 0xe7;
                pass1_1030_145a(pa_var7, local_6);
                local_2a._2_2_ = 0;
                local_26 = 0;
                while (CONCAT22(local_26, local_2a._2_2_) < local_6) {
                    u_var29 = 0x1000;
                    local_40 = 0xe771;
                    u_var24 = local_6;
                    process_struct_1000_179c(0x1c, paVar26);
                    local_20 = u_var24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        u_var22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        u_var29 = 0x1030;
                        local_42._0_1_ = 0x87;
                        local_42._1_1_ = 0xe7;
                        u_var22 = local_20;
                        pass1_1030_2958((u_var24 & 0xffff | ZEXT24(paVar26) << 0x10));
                        local_40 = extraout_dx_05;
                    }
                    _local_24 = CONCAT22(local_40, u_var22);
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = 0x33;
                    local_46._1_1_ = 0xe7;
                    pp_var10 = (*_local_24 + 0x10);
                    local_44 = u_var29;
                    (**pp_var10)();
                    if (u_var22 == 0) {
                        return;
                    }
                  // local_40 = (_local_24  >> 0x10);
                    uVar30 = _local_24;
                    u_var24 = (uVar30 + 4);
                    u_var22 = (uVar30 + 6);
                    local_e = u_var24;
                  // local_c = (u_var24  >> 0x10);
                    paVar26 = (u_var22 & 0xff);
                    local_42._0_1_ = SUB41(_local_24, 0);
                    local_42._1_1_ = (_local_24 >> 8);
                    u_var8 = (u_var27 + 0x12);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                  // local_44 = (u_var8  >> 0x10);
                    local_4a._0_1_ = 0x5c;
                    local_4a._1_1_ = 0xe7;
                    local_48 = u_var29;
                    pass1_1030_14b4(
                        u_var8,
                        uVar30,
                        local_40,
                        u_var24 & 0xffff | (u_var22 & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(local_26, local_2a._2_2_) + 1;
                    local_2a._2_2_ = lVar9;
                  // local_26 = (lVar9  >> 0x10);
                }
            }
            0x3 => {
                local_42._0_1_ = 0;
                local_42._1_1_ = 0;
                local_40 = 0x500;
                u_var22 = u_var27 + 0x114;
                local_46._0_1_ = u_var22;
                u_var12 = local_46;
                local_46._1_1_ = (u_var22 >> 8);
                u_var13 = local_46._1_1_;
                local_48 = 0;
                uStack76 = ctx._PTR_LOOP_1050_65e2;
                uStack75 = (ctx._PTR_LOOP_1050_65e2 >> 8);
                local_4a._0_1_ = (ctx._PTR_LOOP_1050_65e2 >> 0x10);
                local_4a._1_1_ = (ctx._PTR_LOOP_1050_65e2 >> 0x18);
                uStack78 = SUB42(&PTR_LOOP_1050_1028, 0);
                uStack80 = 0x970b;
                local_16 = u_var22;
                pass1_1028_e2ac(ctx._PTR_LOOP_1050_65e2);
                local_46._0_1_ = local_16;
                local_46._1_1_ = (local_16 >> 8);
                local_4a._0_1_ = _PTR_LOOP_1050_5740;
                local_4a._1_1_ = (_PTR_LOOP_1050_5740 >> 8);
              // local_48 = (_PTR_LOOP_1050_5740  >> 0x10);
                uStack76 = 0x28;
                uStack75 = 0x10;
                u_var29 = 0x1030;
                uStack78 = 0x9728;
                local_44 = paVar26;
                local_42._0_1_ = u_var12;
                local_42._1_1_ = u_var13;
                local_40 = u_var28;
                local_14 = paVar26;
                pass1_1030_61fe(
                    _PTR_LOOP_1050_5740,
                    CONCAT22(paVar26, local_16),
                    param_1 & 0xffff0000 | u_var22,
                    (u_var27 + 0x108),
                );
                if (((u_var27 + 0x11a) == 10) || ((u_var27 + 0x11a) == 0x37)) {
                    if ((u_var27 + 0x11a) == 0x37) {
                        u_var3 = (u_var27 + 0x11e);
                        paVar26 = (u_var27 + 0x120);
                        local_38._0_2_ = u_var3;
                        local_38._2_2_ = (u_var3 >> 0x10);
                        u_var3 = (u_var27 + 0x10c);
                        local_2a._0_2_ = u_var3;
                        local_2a._2_2_ = (u_var3 >> 0x10);
                        (local_38 + 0x20) = u_var3;
                    }
                    local_42._0_1_ = 0;
                    local_42._1_1_ = 0;
                    local_40 = 0x400;
                    i_var25 = u_var27 + 0x114;
                    local_46._0_1_ = i_var25;
                    local_46._1_1_ = (i_var25 >> 8);
                    local_48 = 0;
                    uStack76 = ctx._PTR_LOOP_1050_65e2;
                    uStack75 = (ctx._PTR_LOOP_1050_65e2 >> 8);
                    local_4a._0_1_ = (ctx._PTR_LOOP_1050_65e2 >> 0x10);
                    local_4a._1_1_ = (ctx._PTR_LOOP_1050_65e2 >> 0x18);
                    uStack78 = 0x1030;
                    uStack80 = 0x9788;
                    local_44 = u_var28;
                    pass1_1028_e2ac(ctx._PTR_LOOP_1050_65e2);
                    (u_var27 + 0x10c) = i_var25;
                    (u_var27 + 0x10e) = paVar26;
                    u_var20 = (u_var27 + 0x10c);
                    local_42._0_1_ = u_var20;
                    local_42._1_1_ = (u_var20 >> 8);
                    local_46._0_1_ = local_6;
                    local_46._1_1_ = (local_6 >> 8);
                  // local_44 = (local_6  >> 0x10);
                    local_48 = 0x1030;
                    u_var29 = 0x1018;
                    local_4a._0_1_ = 0xaa;
                    local_4a._1_1_ = 0x97;
                    local_40 = paVar26;
                    pass1_1018_0196(local_6, CONCAT22(paVar26, u_var20), (u_var27 + 0x108));
                    paVar26 = ctx.dx_reg;
                    if ((u_var27 + 0x11a) == 10) {
                        local_42._0_1_ = local_6;
                        local_42._1_1_ = (local_6 >> 8);
                      // local_40 = (local_6  >> 0x10);
                        local_44 = 0x1018;
                        u_var29 = 0x1010;
                        local_46._0_1_ = 0xc4;
                        local_46._1_1_ = 0x97;
                        pass1_1010_ed22(local_6, (u_var27 + 0x10c));
                        paVar26 = ctx.dx_reg;
                    }
                }
                u_var3 = (u_var27 + 0x10c);
                local_42._0_1_ = ctx._PTR_LOOP_1050_65e2;
                local_42._1_1_ = (ctx._PTR_LOOP_1050_65e2 >> 8);
              // local_40 = (ctx._PTR_LOOP_1050_65e2  >> 0x10);
                local_46._0_1_ = 0xd7;
                local_46._1_1_ = 0x97;
                local_44 = u_var29;
              // paVar18 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, u_var3, (u_var3  >> 0x10));
                (u_var27 + 0x110) = paVar18;
                (u_var27 + 0x112) = paVar26;
                if ((paVar26 | (u_var27 + 0x110)) != 0) {
                    local_40 = local_16;
                    local_42._0_1_ = 0;
                    local_42._1_1_ = 0;
                    u_var20 = (u_var27 + 0x110);
                    local_46._0_1_ = u_var20;
                    local_46._1_1_ = (u_var20 >> 8);
                    local_4a._0_1_ = 7;
                    local_4a._1_1_ = 0x98;
                    pp_var10 = ((u_var27 + 0x110) + 8);
                    local_48 = u_var29;
                    local_44 = paVar26;
                    (**pp_var10)();
                    paVar26 = ctx.dx_reg;
                }
                local_42._0_1_ = ctx._PTR_LOOP_1050_65e2;
                local_42._1_1_ = (ctx._PTR_LOOP_1050_65e2 >> 8);
              // local_40 = (ctx._PTR_LOOP_1050_65e2  >> 0x10);
                local_46._0_1_ = 0x15;
                local_46._1_1_ = 0x98;
                local_44 = u_var29;
                local_1a = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, local_16, local_14);
                local_42._0_1_ = local_1a;
                local_42._1_1_ = (local_1a >> 8);
                local_46._0_1_ = 0x2b;
                local_46._1_1_ = 0x98;
                local_44 = u_var29;
                local_40 = paVar26;
                local_18 = paVar26;
                pass1_1030_73ee(CONCAT22(paVar26, local_1a), (u_var27 + 0x10c));
                local_42._0_1_ = ctx._PTR_LOOP_1050_06e0;
                local_42._1_1_ = (ctx._PTR_LOOP_1050_06e0 >> 8);
              // local_40 = (ctx._PTR_LOOP_1050_06e0  >> 0x10);
                local_44 = 0x1030;
                u_var29 = &ctx.PTR_LOOP_1050_1008;
                local_46._0_1_ = 0x3f;
                local_46._1_1_ = 0x98;
                local_20 = ctx.dx_reg;
                BVar21 = pass1_1008_c6ae(ctx._PTR_LOOP_1050_06e0, (u_var27 + 0x11a), 0x31);
                if ((BVar21 == 0) && ((u_var27 + 0x122) == 0)) {
                  // uVar36 = (*(local_1a + 0xc)  >> 0x10);
                    local_3a = (local_1a + 0x10);
                    local_38._0_2_ = &stack0xffc2;
                    if (local_3a < 1) {
                        u_var20 = 5;
                    } else {
                        u_var20 = 6;
                    }
                    (local_1a + 0x14) = u_var20;
                    local_20 = local_18;
                }
                u_var3 = (local_1a + 0x16);
                local_1e._0_2_ = u_var3;
                local_1e._2_2_ = (u_var3 >> 0x10);
                u_var8 = &PTR_LOOP_1050_65e2;
                local_42._0_1_ = u_var8;
                local_42._1_1_ = (u_var8 >> 8);
              // local_40 = (u_var8  >> 0x10);
                local_44 = &ctx.PTR_LOOP_1050_1008;
                local_46._0_1_ = 0x9b;
                local_46._1_1_ = 0x98;
                paVar18 = pass1_1028_e1ec(u_var8, local_1e, local_1e._2_2_);
                _local_24 = CONCAT22(paVar18, local_24);
                if (CONCAT22(local_1e._2_2_, local_1e) != 0) {
                    local_42._0_1_ = SUB21(&local_14c, 0);
                    local_42._1_1_ = (&local_14c >> 8);
                    local_44 = &ctx.PTR_LOOP_1050_1008;
                    local_46._0_1_ = 0xb7;
                    local_46._1_1_ = 0x98;
                    pass1_1030_e4fa(
                        CONCAT22(unaff_ss, &local_14c),
                        CONCAT22(local_1e._2_2_, local_1e),
                    );
                    pu_var5 = &g_bool_1050_5748;
                    local_42._0_1_ = SUB41(pu_var5, 0);
                    local_42._1_1_ = (pu_var5 >> 8);
                  // local_40 = (pu_var5  >> 0x10);
                    local_44 = 0x1030;
                    u_var29 = 0x1030;
                    local_46._0_1_ = 199;
                    local_46._1_1_ = 0x98;
                    pass1_1030_835a(pu_var5, CONCAT22(unaff_ss, &local_14c));
                    local_14c = ctx.s_1_1050_389a;
                    local_14a = &ctx.PTR_LOOP_1050_1008;
                }
                u_var3 = (u_var27 + 0x11e);
                local_42._0_1_ = u_var3;
                local_42._1_1_ = (u_var3 >> 8);
              // local_40 = (u_var3  >> 0x10);
                local_46._0_1_ = 0xec;
                local_46._1_1_ = 0x98;
                pp_var10 = ((u_var27 + 0x11e) + 4);
                local_44 = u_var29;
                (**pp_var10)();
                u_var3 = (u_var27 + 0x11e);
                u_var8 = (u_var3 + 4);
                local_46._0_1_ = u_var8;
                local_46._1_1_ = (u_var8 >> 8);
              // local_44 = (u_var8  >> 0x10);
                local_4a._0_1_ = local_1a;
                local_4a._1_1_ = (local_1a >> 8);
                local_48 = local_18;
                uStack76 = u_var29;
                uStack75 = (u_var29 >> 8);
                uStack78 = 0x9902;
                pass1_1030_7e5a(CONCAT22(local_18, local_1a), u_var8);
                return;
            }
            0x4 => {
                pa_var7 = (u_var27 + 0x16);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
              // local_40 = (pa_var7  >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0xa7;
                local_46._1_1_ = 0xe7;
                pass1_1030_145a(pa_var7, local_6);
                local_2a._2_2_ = 0;
                local_26 = 0;
                while (CONCAT22(local_26, local_2a._2_2_) < local_6) {
                    u_var29 = 0x1000;
                    local_40 = 0xe828;
                    u_var24 = local_6;
                    process_struct_1000_179c(0x1e, paVar26);
                    local_20 = u_var24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        u_var22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        u_var29 = 0x1030;
                        local_42._0_1_ = 0x40;
                        local_42._1_1_ = 0xe8;
                        u_var22 = local_20;
                        pass1_1030_560e((u_var24 & 0xffff | ZEXT24(paVar26) << 0x10));
                        local_40 = extraout_dx_06;
                    }
                    _local_24 = CONCAT22(local_40, u_var22);
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = 0xcb;
                    local_46._1_1_ = 0xe7;
                    pp_var10 = (*_local_24 + 0x10);
                    local_44 = u_var29;
                    (**pp_var10)();
                    if (u_var22 == 0) {
                        return;
                    }
                  // uVar36 = (_local_24  >> 0x10);
                    u_var8 = (_local_24 + 4);
                    local_e = u_var8;
                  // local_c = (u_var8  >> 0x10);
                    u_var6 = (_local_24 + 0x10);
                    local_1e._2_2_ = u_var6;
                  // local_1a = (u_var6  >> 0x10);
                    local_42._0_1_ = u_var6;
                    local_42._1_1_ = (u_var6 >> 8);
                    local_44 = 0;
                    local_48 = _PTR_LOOP_1050_5740;
                    local_46._0_1_ = (_PTR_LOOP_1050_5740 >> 0x10);
                    local_46._1_1_ = (_PTR_LOOP_1050_5740 >> 0x18);
                    local_4a._0_1_ = u_var29;
                    local_4a._1_1_ = (u_var29 >> 8);
                    uStack76 = 0xf8;
                    uStack75 = 0xe7;
                    local_40 = local_1a;
                    pass1_1030_6222(_PTR_LOOP_1050_5740, 0, u_var6, u_var8);
                    paVar26 = (local_c & 0xff);
                    local_42._0_1_ = SUB41(_local_24, 0);
                    local_42._1_1_ = (_local_24 >> 8);
                  // local_40 = (_local_24  >> 0x10);
                    u_var8 = (u_var27 + 0x16);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                  // local_44 = (u_var8  >> 0x10);
                    local_48 = 0x1030;
                    local_4a._0_1_ = 0x13;
                    local_4a._1_1_ = 0xe8;
                    pass1_1030_14b4(
                        u_var8,
                        _local_24,
                        local_40,
                        CONCAT22(local_c, local_e) & 0xffffff,
                    );
                    lVar9 = CONCAT22(local_26, local_2a._2_2_) + 1;
                    local_2a._2_2_ = lVar9;
                  // local_26 = (lVar9  >> 0x10);
                }
            }
            0x5 => {
                (paVar26).field_0x0 = "\x02";
                paVar26.field_0x2 = &PTR_LOOP_1050_1028;
                return;
            }
            0x6 => {
                pa_var7 = (u_var27 + 0x1a);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
              // local_40 = (pa_var7  >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0x61;
                local_46._1_1_ = 0xe8;
                pass1_1030_145a(pa_var7, local_6);
                local_30 = 0;
                while (local_30 < local_6) {
                    u_var29 = 0x1000;
                    local_40 = 0xe8c4;
                    u_var24 = local_6;
                    process_struct_1000_179c(0x21e, paVar26);
                    local_20 = u_var24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        u_var22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        u_var29 = &PTR_LOOP_1050_1038;
                        local_42._0_1_ = 0xda;
                        local_42._1_1_ = 0xe8;
                        u_var22 = local_20;
                        pass1_1038_30aa();
                        local_40 = extraout_dx_07;
                    }
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = 0x85;
                    local_46._1_1_ = 0xe8;
                    pp_var10 = (CONCAT22(local_40, u_var22) + 0x10);
                    local_44 = u_var29;
                    local_2c = u_var22;
                    local_2a._0_2_ = local_40;
                    (**pp_var10)();
                    if (u_var22 == 0) {
                        return;
                    }
                    u_var24 = (local_2c + 4);
                    u_var22 = (local_2c + 6);
                    local_e = u_var24;
                  // local_c = (u_var24  >> 0x10);
                    paVar26 = (u_var22 & 0xff);
                    local_40 = local_2a;
                    local_42._0_1_ = local_2c;
                    local_42._1_1_ = (local_2c >> 8);
                    u_var8 = (u_var27 + 0x1a);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                  // local_44 = (u_var8  >> 0x10);
                    local_4a._0_1_ = 0xae;
                    local_4a._1_1_ = 0xe8;
                    local_48 = u_var29;
                    pass1_1030_14b4(
                        u_var8,
                        local_2c,
                        local_2a,
                        u_var24 & 0xffff | (u_var22 & 0xff) << 0x10,
                    );
                    local_30 = local_30 + 1;
                }
            }
            // default:
            _ => {
                pa_var7 = (u_var27 + 0x1e);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
              // local_40 = (pa_var7  >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0xf9;
                local_46._1_1_ = 0xe8;
                pass1_1030_145a(pa_var7, local_6);
                local_42._0_1_ = _PTR_LOOP_1050_5740;
                local_42._1_1_ = (_PTR_LOOP_1050_5740 >> 8);
              // local_40 = (_PTR_LOOP_1050_5740  >> 0x10);
                local_44 = 0x1030;
                local_46._0_1_ = 7;
                local_46._1_1_ = 0xe9;
                pass1_1030_66de(_PTR_LOOP_1050_5740, local_6);
                local_30 = 0;
                while (true) {
                    if (local_6 <= local_30) {
                        local_40 = 0x1030;
                        local_42._0_1_ = 0xd5;
                        local_42._1_1_ = 0xe9;
                        ret_1030_154c();
                        local_40 = 0x1030;
                        local_42._0_1_ = 0xdf;
                        local_42._1_1_ = 0xe9;
                        pass1_1030_6740(_PTR_LOOP_1050_5740);
                        return;
                    }
                    local_14 = _PTR_LOOP_1050_5744;
                  // local_12 = (_PTR_LOOP_1050_5744  >> 0x10);
                    local_40 = 0x1030;
                    u_var29 = 0x1000;
                    local_42._0_1_ = 0xaf;
                    local_42._1_1_ = 0xe9;
                    pu_var23 = _PTR_LOOP_1050_5744;
                    alloc_mem_1000_07fc(_PTR_LOOP_1050_5744);
                    local_20 = pu_var23;
                    local_1e._0_2_ = extraout_dx_08;
                    if ((extraout_dx_08 | local_20) == 0) {
                        u_var22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        u_var29 = 0x1030;
                        local_42._0_1_ = 0xc4;
                        local_42._1_1_ = 0xe9;
                        u_var22 = local_20;
                        pass1_1030_67cc((pu_var23 & 0xffff | ZEXT24(extraout_dx_08) << 0x10));
                        local_40 = extraout_dx_09;
                    }
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = 0x2d;
                    local_46._1_1_ = 0xe9;
                    pp_var10 = (CONCAT22(local_40, u_var22) + 0x10);
                    local_44 = u_var29;
                    local_2c = u_var22;
                    local_2a._0_2_ = local_40;
                    (**pp_var10)();
                    if (u_var22 == 0) {
                        break;
                    }
                    u_var8 = (local_2c + 4);
                    local_e = u_var8;
                  // local_c = (u_var8  >> 0x10);
                    u_var24 = (local_2c + 8);
                    local_2a._2_2_ = u_var24;
                  // local_26 = (u_var24  >> 0x10);
                    u_var3 = (local_2c + 0xc);
                    local_38._2_2_ = u_var3;
                  // uStack52 = (u_var3  >> 0x10);
                    uStack50 = (local_2c + 0x10);
                    u_var22 = &local_38 + 2;
                    _local_24 = (_local_24 & 0xffff0000 | u_var22);
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                    local_44 = local_c;
                    local_4a._0_1_ = _PTR_LOOP_1050_5740;
                    local_4a._1_1_ = (_PTR_LOOP_1050_5740 >> 8);
                  // local_48 = (_PTR_LOOP_1050_5740  >> 0x10);
                    uStack76 = u_var29;
                    uStack75 = (u_var29 >> 8);
                    uStack78 = 0xe977;
                    pass1_1030_671c(
                        _PTR_LOOP_1050_5740,
                        u_var8,
                        CONCAT22(unaff_ss, u_var22),
                        u_var24,
                    );
                    local_42._0_1_ = local_2c;
                    local_42._1_1_ = (local_2c >> 8);
                    local_40 = local_2a;
                    u_var8 = (u_var27 + 0x1e);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                  // local_44 = (u_var8  >> 0x10);
                    local_48 = 0x1030;
                    local_4a._0_1_ = 0x92;
                    local_4a._1_1_ = 0xe9;
                    pass1_1030_14b4(
                        u_var8,
                        local_2c,
                        local_2a,
                        CONCAT22(local_c, local_e) & 0xffffff,
                    );
                    local_30 = local_30 + 1;
                }
                return;
            }
            0x9 => {
                local_6 = local_6 & 0xffff;
                local_42._0_1_ = param_3;
                local_42._1_1_ = (param_3 >> 8);
                local_40 = param_3_00;
                local_44 = param_2;
                local_46._0_1_ = (param_1 >> 0x10);
                local_46._1_1_ = (param_1 >> 0x18);
                local_c = *(u_var27 + 0x2e);
                local_a = *(u_var27 + 0x30);
                local_4a._0_1_ = 0x28;
                local_4a._1_1_ = 0x10;
                uStack76 = 0x2d;
                uStack75 = 0xe3;
                local_48 = u_var27;
                (*local_c)();
                return;
            }
            0xa => {
                pa_var7 = (u_var27 + 0x22);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
              // local_40 = (pa_var7  >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0xf3;
                local_46._1_1_ = 0xe9;
                pass1_1030_145a(pa_var7, local_6);
                local_38._2_2_ = 0;
                uStack52 = 0;
                while (CONCAT22(uStack52, local_38._2_2_) < local_6) {
                    local_40 = 0xea55;
                    u_var24 = local_6;
                    process_struct_1000_179c(0xe, paVar26);
                    local_20 = u_var24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        i_var25 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        local_42._0_1_ = 0x6b;
                        local_42._1_1_ = 0xea;
                        pu_var34 = pass1_1028_b204((u_var24 & 0xffff | ZEXT24(paVar26) << 0x10));
                      // local_40 = (pu_var34  >> 0x10);
                        i_var25 = pu_var34;
                    }
                    local_30 = CONCAT22(local_40, i_var25);
                    local_42._0_1_ = i_var25;
                    local_42._1_1_ = (i_var25 >> 8);
                    local_44 = 0x1000;
                    local_46._0_1_ = 0x17;
                    local_46._1_1_ = 0xea;
                    pp_var10 = (local_30 + 0x10);
                    (**pp_var10)();
                    if (i_var25 == 0) {
                        return;
                    }
                  // local_40 = (local_30  >> 0x10);
                    u_var29 = local_30;
                    u_var24 = (u_var29 + 4);
                    u_var22 = (u_var29 + 6);
                    local_e = u_var24;
                  // local_c = (u_var24  >> 0x10);
                    paVar26 = (u_var22 & 0xff);
                    local_42._0_1_ = local_30;
                    local_42._1_1_ = (local_30 >> 8);
                    u_var8 = (u_var27 + 0x22);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                  // local_44 = (u_var8  >> 0x10);
                    local_48 = 0x1000;
                    local_4a._0_1_ = 0x40;
                    local_4a._1_1_ = 0xea;
                    pass1_1030_14b4(
                        u_var8,
                        u_var29,
                        local_40,
                        u_var24 & 0xffff | (u_var22 & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(uStack52, local_38._2_2_) + 1;
                    local_38._2_2_ = lVar9;
                  // uStack52 = (lVar9  >> 0x10);
                }
            }
            0xb => {
                if (paVar26 < (&PTR_LOOP_1050_000e + 1)) {
                    pc_var1 = (unaff_si + 0x23);
                    pv_var1_val = unsafe { *pc_var1 };
                    cVar4 = pv_var1_val;
                    unsafe { *pc_var1 = *pc_var1 << 6 };
                    uVar36 = 0x2b;
                    pi_var2 = (&paVar26.field_0x0 + unaff_si);
                    let pi_var_2_val = unsafe { pi_var2 };
                    unsafe {
                        *pi_var2 = *pi_var2 + (-0x6600 - ((cVar4 << 5) < '\0'));
                    }
                } else {
                    uVar36 = 0x7af0;
                    local_46._0_1_ = u_var13;
                    local_46._1_1_ = uVar16;
                    local_44 = u_var29;
                    pass1_1028_780c(local_3a, local_38, CONCAT22(uStack52, local_38._2_2_));
                    if (param_3_00 == 0) {}
                    // goto code_r0x10287b17;
                }
                local_42._0_1_ = 0;
                local_42._1_1_ = 4;
                local_40 = 0x1d;
                local_46._0_1_ = 0x2b;
                local_46._1_1_ = 0;
                local_44 = 1;
                local_4a._0_1_ = SUB41(ctx.g_struct_var_1050_0ed0, 0);
                local_4a._1_1_ = (ctx.g_struct_var_1050_0ed0 >> 8);
              // local_48 = (ctx.g_struct_var_1050_0ed0  >> 0x10);
                uStack76 = 0x28;
                uStack75 = 0x10;
                uStack78 = 0x7b0a;
                ppVar33 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, 0x1002b);
              // local_44 = (ppVar33  >> 0x10);
                local_48 = ppVar33;
                local_46._0_1_ = (ppVar33 >> 0x10);
                local_46._1_1_ = (ppVar33 >> 0x18);
                local_4a._0_1_ = 0x10;
                local_4a._1_1_ = 0x10;
                uVar30 = 0x1010;
                uStack76 = 0x17;
                uStack75 = 0x7b;
                pass1_1010_043a(
                    ppVar33,
                    CONCAT13(local_42._1_1_, CONCAT12(local_42, local_44)),
                    local_40,
                );
                paVar26 = ctx.dx_reg;
                // code_r0x10287b17:
                local_42._0_1_ = 2;
                local_42._1_1_ = 0;
                local_40 = 0x400;
                local_46._0_1_ = ctx._PTR_LOOP_1050_65e2;
                local_46._1_1_ = (ctx._PTR_LOOP_1050_65e2 >> 8);
              // local_44 = (ctx._PTR_LOOP_1050_65e2  >> 0x10);
                local_4a._0_1_ = 0x27;
                local_4a._1_1_ = 0x7b;
                local_48 = uVar30;
                paVar18 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, 2, 0x400);
                local_42._0_1_ = SUB21(paVar18, 0);
                local_42._1_1_ = (paVar18 >> 8);
                local_4a._0_1_ = 0x38;
                local_4a._1_1_ = 0x7b;
                local_48 = uVar30;
                local_46._0_1_ = u_var14;
                local_46._1_1_ = uVar17;
                local_44 = u_var28;
                local_40 = paVar26;
                pass1_1028_780c(u_var27, u_var28, CONCAT22(paVar26, paVar18));
                local_40 = 2;
                local_44 = ctx.g_struct_var_1050_0ed0;
                local_42._0_1_ = (ctx.g_struct_var_1050_0ed0 >> 0x10);
                local_42._1_1_ = (ctx.g_struct_var_1050_0ed0 >> 0x18);
                local_46._0_1_ = uVar30;
                local_46._1_1_ = (uVar30 >> 8);
                local_48 = 0x7b44;
                local_a = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(uVar36, 2));
                local_c = u16_1050_13ae;
                if (2 < u16_1050_13ae) {
                    local_40 = 0x2f;
                    local_44 = ctx.g_struct_var_1050_0ed0;
                    local_42._0_1_ = (ctx.g_struct_var_1050_0ed0 >> 0x10);
                    local_42._1_1_ = (ctx.g_struct_var_1050_0ed0 >> 0x18);
                    local_46._0_1_ = 0x10;
                    local_46._1_1_ = 0x10;
                    u_var29 = 0x1010;
                    local_48 = 0x7b63;
                    ppVar33 = process_struct_1010_20ba(
                        ctx.g_struct_var_1050_0ed0,
                        CONCAT22(uVar36, 0x2f),
                    );
                  // local_48 = (ppVar33  >> 0x10);
                    local_4a._0_1_ = SUB41(ppVar33, 0);
                    local_4a._1_1_ = (ppVar33 >> 8);
                    local_46._0_1_ = 1;
                    local_46._1_1_ = 0;
                    while (CONCAT11(local_46._1_1_, local_46) < 9) {
                        if ((CONCAT11(local_4a._1_1_, local_4a)
                            + 0x34
                            + CONCAT11(local_46._1_1_, local_46) * 4)
                            == local_6)
                        {
                            local_40 = 100;
                            paVar26 = (&ctx.PTR_LOOP_1050_0000 + 1);
                            local_30 = CONCAT22(local_30._2_2_, 1);
                            local_42._0_1_ = 1;
                            local_42._1_1_ = 0;
                            u_var28 = &ctx.PTR_LOOP_1050_1008;
                            local_46._0_1_ = 0xd7;
                            local_46._1_1_ = 0x7b;
                            local_44 = u_var29;
                            pass1_fn_1008_612e();
                            pu_var19 = (CONCAT11(local_46._1_1_, local_46) - 7);
                            if (pu_var19 == 0x0) {
                                bVar32 = SBORROW2(paVar26, 0x32);
                                pu_var11 = &paVar26[-5].field_0xa;
                                bVar31 = paVar26 == (s_New_failed_in_Op__Op_1050_0020 + 0x12);
                                // LAB_1028_7b74:
                                if ((!bVar31 && bVar32) == (pu_var11 < 0)) {
                                    local_30 = local_30 & 0xffff0000;
                                }
                            } else {
                                pu_var19 = (CONCAT11(local_46._1_1_, local_46) - 8);
                                if (pu_var19 == 0x0) {
                                    bVar32 = SBORROW2(paVar26, 0x19);
                                    pu_var11 = (&paVar26[-3].field_0xa + 1);
                                    bVar31 = pu_var11 == 0x0;
                                    // goto LAB_1028_7b74;
                                }
                            }
                            local_1e._0_2_ = paVar26;
                            if (local_30 != 0) {
                                local_40 = CONCAT11(local_46._1_1_, local_46);
                                local_44 = &local_154;
                                local_42._0_1_ = unaff_ss;
                                local_42._1_1_ = (unaff_ss >> 8);
                                local_46._0_1_ = 8;
                                local_46._1_1_ = 0x10;
                                local_48 = 0x7bfc;
                                pass1_1028_90e6(
                                    CONCAT13(local_42._1_1_, CONCAT12(local_42, local_44)),
                                    local_40,
                                );
                                pu_var19 = &local_154;
                                local_42._0_1_ = SUB21(pu_var19, 0);
                                local_42._1_1_ = (pu_var19 >> 8);
                                local_46._0_1_ = SUB41(ctx._g_bool_1050_5748, 0);
                                local_46._1_1_ = (ctx._g_bool_1050_5748 >> 8);
                              // local_44 = (ctx._g_bool_1050_5748  >> 0x10);
                                local_48 = &ctx.PTR_LOOP_1050_1008;
                                u_var28 = 0x1030;
                                local_4a._0_1_ = 0xc;
                                local_4a._1_1_ = 0x7c;
                                pass1_1030_835a(
                                    ctx._g_bool_1050_5748,
                                    CONCAT22(unaff_ss, pu_var19),
                                );
                                local_154 = ctx.s_1_1050_389a;
                                local_152 = &ctx.PTR_LOOP_1050_1008;
                            }
                            local_42._0_1_ = 0;
                            local_42._1_1_ = 0;
                            local_40 = 10;
                            u_var29 = &ctx.PTR_LOOP_1050_1008;
                            local_46._0_1_ = 0x23;
                            local_46._1_1_ = 0x7c;
                            local_44 = u_var28;
                            pass1_fn_1008_612e();
                            local_18 = pu_var19;
                            if (CONCAT11(local_46._1_1_, local_46) == 7) {
                                local_40 = 7;
                                pu_var19 = pu_var19 + 0x37;
                                i_var25 = pu_var19 >> 0xf;
                            } else {
                                if (CONCAT11(local_46._1_1_, local_46) != 8) {}
                                // goto LAB_1028_7ba0;
                                local_40 = 8;
                                pu_var19 = pu_var19 + 0x32;
                                i_var25 = (pu_var19 >> 0xf) + (0xff9b < pu_var19);
                            }
                            local_44 = CONCAT11(local_42._1_1_, local_42) + pu_var19;
                            local_40 = local_40
                                + i_var25
                                + CARRY2(CONCAT11(local_42._1_1_, local_42), pu_var19);
                            local_42._0_1_ = local_40;
                            local_42._1_1_ = (local_40 >> 8);
                            u_var28 = CONCAT11(local_4a._1_1_, local_4a);
                            local_46._0_1_ = local_48;
                            local_46._1_1_ = (local_48 >> 8);
                            local_4a._0_1_ = 8;
                            local_4a._1_1_ = 0x10;
                            u_var29 = 0x1010;
                            uStack76 = 0xa0;
                            uStack75 = 0x7b;
                            local_48 = u_var28;
                            pass1_1010_ebf8(
                                CONCAT13(local_46._1_1_, CONCAT12(local_46, u_var28)),
                                local_44,
                                local_40,
                                local_40,
                            );
                        }
                        // LAB_1028_7ba0:
                        i_var25 = CONCAT11(local_46._1_1_, local_46) + 1;
                        local_46._0_1_ = i_var25;
                        local_46._1_1_ = (i_var25 >> 8);
                    }
                }
                return;
            }
            0xc => {
                pa_var7 = (u_var27 + 0x26);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
              // local_40 = (pa_var7  >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0x8b;
                local_46._1_1_ = 0xea;
                pass1_1030_145a(pa_var7, local_6);
                local_38._2_2_ = 0;
                uStack52 = 0;
                while (CONCAT22(uStack52, local_38._2_2_) < local_6) {
                    local_42._0_1_ = SUB21(&local_30, 0);
                    local_42._1_1_ = (&local_30 >> 8);
                    local_44 = param_3;
                    local_48 = 0x1030;
                    local_4a._0_1_ = 0xd;
                    local_4a._1_1_ = 0xeb;
                    local_46._0_1_ = u_var12;
                    local_46._1_1_ = uVar15;
                    u_var29 = read::read_file_1008_7dee(
                        CONCAT22(param_3, param_2),
                        CONCAT22(unaff_ss, &local_30),
                        2,
                    );
                    if (u_var29 == 0) {
                        ctx.g_u16_1050_0310 = 0x6d2;
                        return;
                    }
                    local_40 = param_2;
                    local_42._0_1_ = 8;
                    local_42._1_1_ = 0x10;
                    local_44 = 0xeaa2;
                    switch_statement_1008_73ea(param_2, param_3, local_30);
                    local_42._0_1_ = 8;
                    local_42._1_1_ = 0x10;
                    local_44 = 0xeaaf;
                    local_40 = u_var27;
                    local_2c = u_var29;
                    paVar35 = pass1_1030_0000(u_var27, u_var28, u_var29);
                  // local_40 = (paVar35  >> 0x10);
                    u_var29 = paVar35;
                    local_42._0_1_ = SUB41(paVar35, 0);
                    local_42._1_1_ = (paVar35 >> 8);
                    local_44 = 0x1030;
                    local_46._0_1_ = 0xc5;
                    local_46._1_1_ = 0xea;
                    pp_var10 = (paVar35 + 0x10);
                    local_2a._2_2_ = u_var29;
                    local_26 = local_40;
                    (**pp_var10)();
                    if (u_var29 == 0) {
                        return;
                    }
                    u_var24 = (local_2a._2_2_ + 4);
                    local_e = u_var24;
                  // local_c = (u_var24  >> 0x10);
                    local_40 = local_26;
                    local_42._0_1_ = local_2a._2_2_;
                    local_42._1_1_ = (local_2a._2_2_ >> 8);
                    u_var8 = (u_var27 + 0x26);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                  // local_44 = (u_var8  >> 0x10);
                    local_48 = 0x1030;
                    local_4a._0_1_ = 0xee;
                    local_4a._1_1_ = 0xea;
                    pass1_1030_14b4(
                        u_var8,
                        local_2a._2_2_,
                        local_26,
                        u_var24 & 0xffff | (*(local_2a._2_2_ + 6) & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(uStack52, local_38._2_2_) + 1;
                    local_38._2_2_ = lVar9;
                  // uStack52 = (lVar9  >> 0x10);
                }
            }
            0xd => {
                local_a = ZEXT24(paVar26) << 0x10;
                u_var3 = &PTR_LOOP_1050_000c;
                local_10 = u_var3;
              // local_e = (u_var3  >> 0x10);
                local_c = &ctx.PTR_LOOP_1050_0010;
                local_18 = &local_10;
                local_42._0_1_ = SUB21(&local_14, 0);
                local_42._1_1_ = (&local_14 >> 8);
                local_46._0_1_ = SUB21(&local_16, 0);
                local_46._1_1_ = (&local_16 >> 8);
                local_4a._0_1_ = SUB21(&local_10, 0);
                local_4a._1_1_ = (&local_10 >> 8);
                uStack76 = 0x28;
                uStack75 = 0x10;
                uStack78 = SUB42(s_523a_bmp_1050_2116 + 7, 0);
                pass1_1008_3eb4(
                    CONCAT22(unaff_ss, &local_10),
                    CONCAT22(unaff_ss, &local_16),
                    CONCAT22(unaff_ss, &local_14),
                    CONCAT22(unaff_ss, &local_12),
                );
                local_18 = local_14 - 1;
                local_42._0_1_ = SUB21(&local_10, 0);
                local_42._1_1_ = (&local_10 >> 8);
                local_48 = &ctx.PTR_LOOP_1050_1008;
                local_4a._0_1_ = 0x39;
                local_4a._1_1_ = 0x21;
                local_46._0_1_ = u_var14;
                local_46._1_1_ = uVar17;
                local_e = local_18;
                BVar21 = pass1_1028_21ba(u_var27, local_44, CONCAT22(unaff_ss, &local_10), local_6);
                if (BVar21 == 0) {
                    local_18 = local_14 + 1;
                    local_42._0_1_ = SUB21(&local_10, 0);
                    local_42._1_1_ = (&local_10 >> 8);
                    local_48 = &ctx.PTR_LOOP_1050_1008;
                    local_4a._0_1_ = 0x59;
                    local_4a._1_1_ = 0x21;
                    local_46._0_1_ = u_var14;
                    local_46._1_1_ = uVar17;
                    local_44 = u_var28;
                    local_e = local_18;
                    BVar21 =
                        pass1_1028_21ba(u_var27, u_var28, CONCAT22(unaff_ss, &local_10), local_6);
                    if (BVar21 == 0) {
                        local_e = local_14;
                        local_18 = local_12 - 1;
                        local_42._0_1_ = SUB21(&local_10, 0);
                        local_42._1_1_ = (&local_10 >> 8);
                        local_48 = &ctx.PTR_LOOP_1050_1008;
                        local_4a._0_1_ = 0x82;
                        local_4a._1_1_ = 0x21;
                        local_46._0_1_ = u_var14;
                        local_46._1_1_ = uVar17;
                        local_44 = u_var28;
                        local_10 = local_18;
                        BVar21 = pass1_1028_21ba(
                            u_var27,
                            u_var28,
                            CONCAT22(unaff_ss, &local_10),
                            local_6,
                        );
                        if (BVar21 == 0) {
                            local_18 = local_12 + 1;
                            local_42._0_1_ = SUB21(&local_10, 0);
                            local_42._1_1_ = (&local_10 >> 8);
                            local_48 = &ctx.PTR_LOOP_1050_1008;
                            local_4a._0_1_ = 0xa2;
                            local_4a._1_1_ = 0x21;
                            local_46._0_1_ = u_var14;
                            local_46._1_1_ = uVar17;
                            local_44 = u_var28;
                            local_10 = local_18;
                            BVar21 = pass1_1028_21ba(
                                u_var27,
                                u_var28,
                                CONCAT22(unaff_ss, &local_10),
                                local_6,
                            );
                            if (BVar21 == 0) {
                                return;
                            }
                        }
                    }
                }
                local_42._0_1_ = SUB41(_PTR_LOOP_1050_5a64, 0);
                local_42._1_1_ = (_PTR_LOOP_1050_5a64 >> 8);
              // local_40 = (_PTR_LOOP_1050_5a64  >> 0x10);
                local_44 = &ctx.PTR_LOOP_1050_1008;
                local_46._0_1_ = 0xb4;
                local_46._1_1_ = 0x21;
                pass1_1038_79b2(_PTR_LOOP_1050_5a64, local_a);
                return;
            }
            0xe => {
                pa_var7 = (u_var27 + 0x2a);
                local_42._0_1_ = SUB41(pa_var7, 0);
                local_42._1_1_ = (pa_var7 >> 8);
              // local_40 = (pa_var7  >> 0x10);
                local_44 = &PTR_LOOP_1050_1028;
                local_46._0_1_ = 0x31;
                local_46._1_1_ = 0xeb;
                pass1_1030_145a(pa_var7, local_6);
                local_38._2_2_ = 0;
                uStack52 = 0;
                while (CONCAT22(uStack52, local_38._2_2_) < local_6) {
                    u_var29 = 0x1000;
                    local_40 = 0xeb94;
                    u_var24 = local_6;
                    process_struct_1000_179c(0x3b2, paVar26);
                    local_20 = u_var24;
                    local_1e._0_2_ = paVar26;
                    if ((paVar26 | local_20) == 0) {
                        u_var22 = 0;
                        local_40 = 0;
                    } else {
                        local_40 = 0x1000;
                        u_var29 = 0x1030;
                        local_42._0_1_ = 0xaa;
                        local_42._1_1_ = 0xeb;
                        u_var22 = local_20;
                        pass1_1030_2068((u_var24 & 0xffff | ZEXT24(paVar26) << 0x10));
                        local_40 = extraout_dx_10;
                    }
                    local_30 = CONCAT22(local_40, u_var22);
                    local_42._0_1_ = u_var22;
                    local_42._1_1_ = (u_var22 >> 8);
                    local_46._0_1_ = 0x55;
                    local_46._1_1_ = 0xeb;
                    pp_var10 = (local_30 + 0x10);
                    local_44 = u_var29;
                    (**pp_var10)();
                    if (u_var22 == 0) {
                        return;
                    }
                  // local_40 = (local_30  >> 0x10);
                    uVar30 = local_30;
                    u_var24 = (uVar30 + 4);
                    u_var22 = (uVar30 + 6);
                    local_e = u_var24;
                  // local_c = (u_var24  >> 0x10);
                    paVar26 = (u_var22 & 0xff);
                    local_42._0_1_ = local_30;
                    local_42._1_1_ = (local_30 >> 8);
                    u_var8 = (u_var27 + 0x2a);
                    local_46._0_1_ = u_var8;
                    local_46._1_1_ = (u_var8 >> 8);
                  // local_44 = (u_var8  >> 0x10);
                    local_4a._0_1_ = 0x7e;
                    local_4a._1_1_ = 0xeb;
                    local_48 = u_var29;
                    pass1_1030_14b4(
                        u_var8,
                        uVar30,
                        local_40,
                        u_var24 & 0xffff | (u_var22 & 0xff) << 0x10,
                    );
                    lVar9 = CONCAT22(uStack52, local_38._2_2_) + 1;
                    local_38._2_2_ = lVar9;
                  // uStack52 = (lVar9  >> 0x10);
                }
            }
        }
        local_40 = 0x1030;
        local_42._0_1_ = 0xfb;
        local_42._1_1_ = 0xe6;
        ret_1030_154c();
    }
    return;
}

pub fn pass1_1028_b282(param_1: &mut FileObject, param_2: &HFILE16) -> bool {
    let b_var1: bool;
    let mut unaff_ss: u16;
    let mut local_c: u16;

    b_var1 = write::write_to_file_1030_16d6(param_1, param_2);
    if (b_var1 != 0) {
        local_c = (param_1 + 0xc);
        b_var1 = write::write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
        if (b_var1 == 0) {
            ctx.g_u16_1050_0310 = 0x6d0;
            return b_var1;
        }
        b_var1 = 1;
    }
    return b_var1;
}

pub fn pass1_1028_b2c8(param_1: u32, param_2: &HFILE16) -> bool {
    let u_var1: u8;
    let extraout_AH: u8;
    let BVar2: bool;
    let mut u_var3: u16;
    let mut unaff_ss: u16;
    let mut local_4: u16;

    u_var1 = read::pass1_1030_1730(param_1, param_2);
    BVar2 = CONCAT11(extraout_AH, u_var1);
    if (BVar2 != 0) {
        BVar2 = read::read_file_1008_7dee(param_2, CONCAT22(unaff_ss, &local_4), 2);
        if (BVar2 == 0) {
            ctx.g_u16_1050_0310 = 0x6d2;
            return BVar2;
        }
      // u_var3 = set_param_3_with_switch_1008_72bc(param_2, (param_2  >> 0x10), local_4);
        (param_1 + 0xc) = u_var3;
        BVar2 = 1;
    }
    return BVar2;
}

pub fn pass1_1028_4a5a(param_1: u32, in_file_1: &HFILE16) {

    let b_var1: bool;

    read::read_from_file_1028_b81a(param_1, in_file_1);
    if ((ctx.ax_reg != 0)
        && (
        b_var1 = read::read_file_1008_7dee(in_file_1, (param_1 & 0xffff0000 | (param_1 + 0x20)), 10),
        b_var1 == 0,
        ))
    {
        ctx.g_u16_1050_0310 = 0x6d2;
        return;
    }
    return;
}

pub unsafe fn pass1_1020_8a9c(param_1: &mut  Struct393) {
    let mut u_var1: u32;
    let mut u_var2: i32;
    let mut u_var3: u16;

    let mut unaff_si: u16;
    let mut unaff_ss: u16;
    let ppVar4: &mut  Struct2111;
    let pa_var5: &mut  Struct104;
    let local_struct_1: &mut  Struct393;
    let local_struct_1_hi: &mut  Struct393;
    let mut local_4c: u16;
    let mut local_4a: u16;
    let mut local_48: [u8; 30];
    let mut local_2a: [u8; 36];
    let mut local_6: u16;
    let mut local_4: u16;
    let local_struct_1_1: &mut  Struct393;

    local_struct_1 = param_1;
    local_struct_1_1 = local_struct_1;
  // local_struct_1_hi = (param_1  >> 0x10);
    process_struct_1020_847a(param_1, 2);
    u_var2 = &local_struct_1.field_0x14 + 2;
    zero_list_1008_3e38((param_1 & 0xffff0000 | u_var2));
    _local_4c = param_1 & 0xffff0000 | ZEXT24(&local_struct_1.field_0x1c);
    zero_list_1008_3e38((param_1 & 0xffff0000 | ZEXT24(&local_struct_1.field_0x1c)));
    &local_struct_1.field_0x22 = 0;
    param_1.field_0x0 = 0x8e92;
    local_struct_1.u16_0x2 = 0x1020;
    ppVar4 = process_struct_1010_20ba(ctx.g_struct_var_1050_0ed0, CONCAT22(unaff_si, 0x29));
  // u_var3 = (ppVar4  >> 0x10);
    local_struct_1.field_0x22 = ppVar4;
    local_struct_1.field_0x24 = u_var3;
    pass1_1018_2678(
        local_struct_1.field_0x22,
        u_var3,
        param_1 & 0xffff0000 | u_var2,
    );
    pa_var5 = pass1_1018_268e(&local_struct_1.field_0x22);
  // local_4 = (pa_var5  >> 0x10);
    u_var3 = pa_var5;
    u_var1 = &local_struct_1.field_0x8;
    local_6 = u_var3;
    pass1_1020_8712(
        (param_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        u_var1,
        (u_var1 >> 0x10),
        pa_var5,
        param_1 & 0xffff0000 | u_var2,
    );
    u_var1 = &local_struct_1.field_0x22;
  // local_struct_1 = (u_var1  >> 0x10);
    pass1_1018_26c2(u_var1, local_struct_1, _local_4c);
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 2);
    process_struct_1008_48fe(
        CONCAT22(unaff_ss, local_2a),
        1,
        CONCAT22(ctx.dx_reg, u_var3),
    );
    pass1_1008_3f92(CONCAT22(unaff_ss, local_48), CONCAT22(unaff_ss, local_2a));
    u_var1 = &local_struct_1_1.field_0x8;
    pass1_1020_8712(
        (param_1 & 0xffff | ZEXT24(local_struct_1_hi) << 0x10),
        (u_var1 + 8),
        (u_var1 >> 0x10),
        CONCAT22(unaff_ss, local_48),
        _local_4c,
    );
    process_struct_1008_41bc(CONCAT22(unaff_ss, local_48));
    close::close_file_1008_496c(ctx, local_2a);
    return;
}
