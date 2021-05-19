use crate::err_funcs::error_check_1000_17ce;
use crate::file_funcs::file1::{write_to_file_1008_7c2a, write_to_file_1008_7cac, write_to_file_1008_7e1c};
use crate::mem_funcs::alloc_mem_1000_1708;
use crate::pass::{pass13_funcs, pass14_funcs, pass_funcs};
use crate::pass::pass17_funcs::pass1_1030_8344;
use crate::pass::pass18_funcs::pass1_1038_3608;
use crate::pass::pass8_funcs::pass1_1010_1d80;
use crate::prog_structs::prog_structs_10::Struct252;
use crate::prog_structs::prog_structs_12::{Struct235, Struct266};
use crate::prog_structs::prog_structs_13::Struct279;
use crate::prog_structs::prog_structs_16::Struct248;
use crate::prog_structs::prog_structs_17::Struct284;
use crate::prog_structs::prog_structs_19::Struct271;
use crate::prog_structs::prog_structs_1::Struct283;
use crate::prog_structs::prog_structs_2::{Struct199, Struct293};
use crate::prog_structs::prog_structs_20::Struct253;
use crate::prog_structs::prog_structs_21::Struct240;
use crate::prog_structs::prog_structs_23::Struct267;
use crate::prog_structs::prog_structs_24::{Struct2111, Struct236, Struct249};
use crate::prog_structs::prog_structs_25::Struct272;
use crate::prog_structs::prog_structs_26::{Struct241, Struct254};
use crate::prog_structs::prog_structs_27::{pass1_struct_2, Struct268, Struct285};
use crate::prog_structs::prog_structs_28::{Struct237, Struct255, Struct260, Struct273, Struct290};
use crate::prog_structs::prog_structs_29::{Struct225, Struct226, Struct227, Struct228, Struct229, Struct238, Struct242, Struct243, Struct256, Struct269, Struct286, Struct291};
use crate::prog_structs::prog_structs_30::{Struct230, Struct231, Struct232, Struct233, Struct234, Struct239, Struct244, Struct245, Struct246, Struct247, Struct257, Struct261, Struct274, Struct287, Struct292};
use crate::prog_structs::prog_structs_31::{Struct250, Struct251, Struct258, Struct259, Struct262, Struct263, Struct264, Struct275, Struct276, Struct277, Struct278, Struct280, Struct281, Struct282, Struct288, Struct289};
use crate::prog_structs::prog_structs_4::Struct270;
use crate::prog_structs::prog_structs_7::Struct44;
use crate::prog_structs::prog_structs_8::Struct265;
use crate::struct_funcs::{process_struct_1000_179c, process_struct_1008_574a, process_struct_1010_1d48, struct_fn_1000_160a};
use crate::util::{CONCAT22, CONCAT31, SUB42};

pub unsafe fn pass1_1008_b38c(param_1: u32) {
    let pp_var1: fn();
    let in_ax: *mut Struct199;
    let mut i_var2: i32;
    let ppVar3: *mut pass1_struct_2;
    let in_dx: *mut Struct199;
    let mut u_var4: i32;
    let mut u_var5: u16;
    let mut i32_var6: i32;
    let mut u_var7: u16;
    let mut u_var8: u32;
    let lVar9: u32;
    let mut u_var10: u32;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    u_var7 = (param_1 >> 0x10);
    i32_var6 = param_1;
    if ((i32_var6 + 0x12) == 0) {
        process_struct_1000_179c(0xc, in_dx);
        u_var4 = in_dx | in_ax;
        if (u_var4 == 0) {
            (i32_var6 + 0x12) = 0;
        } else {
            in_ax = process_struct_1008_574a(CONCAT22(in_dx, in_ax));
            (i32_var6 + 0x12) = in_ax;
            (i32_var6 + 0x14) = u_var4;
        }
        lVar9 = CONCAT22(u_var4, in_ax);
        local_4 = 0x6d9;
        while (u_var5 = (lVar9 >> 0x10), local_4 < 0x6e7) {
            if (local_4 == 0x6e3) {
                ppVar3 = pass1_1030_8344(
                    ctx._g_bool_1050_5748,
                    (ctx._g_bool_1050_5748 >> 0x10),
                    0x8000001,
                );
                lVar9 = CONCAT22(u_var5, ppVar3);
                if (&ppVar3.field_0x136 != 0) {
                    // goto LAB_1008_b44a;
                }
            } else {
                // LAB_1008_b44a:
                process_struct_1000_179c(10, (lVar9 >> 0x10));
                if (lVar9 == 0) {
                    u_var10 = 0;
                } else {
                    u_var10 = pass13_funcs::pass1_1008_b11e(lVar9);
                }
                u_var5 = (u_var10 >> 0x10);
                i_var2 = u_var10;
                u_var8 = load_str_1010_84ac(
                    ctx._g_struct_73_1050_14cc,
                    (ctx._g_struct_73_1050_14cc >> 0x10),
                    local_4,
                );
                (i_var2 + 4) = u_var8;
                (i_var2 + 6) = (u_var8 >> 0x10);
                (i_var2 + 8) = local_4 - 0x6d8;
                u_var8 = (i32_var6 + 0x12);
                pp_var1 = ((i32_var6 + 0x12) + 8);
                lVar9 = (**pp_var1)(0x1010, u_var8, (u_var8 >> 0x10), u_var10);
            }
            local_4 = local_4 + 1;
        }
    }
    return CONCAT22((i32_var6 + 0x14), (i32_var6 + 0x12));
}

pub unsafe fn pass1_1008_b47a(param_1: u32) {
    let mut u_var1: u32;
    let mut i_var2: i32;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    if ((param_1 + 0x1e) != 0) {
        u_var1 = (param_1 + 0x1e);
        u_var3 = (u_var1 >> 0x10);
        i_var2 = u_var1;
        return CONCAT22((i_var2 + 6), (i_var2 + 4));
    }
    return 0;
}

pub unsafe fn pass1_1008_b4a0(param_1: u32, param_2: libc::c_long) {
    let mut u_var1: u32;

    let mut u_var2: u16;
    let mut in_dx: u16;
    let mut u_var3: u16;
    let mut i_var4: i32;
    let mut u_var5: u16;
    let mut u_var6: u32;
    let lVar7: u32;

    i_var4 = param_1;
    u_var5 = (param_1 >> 0x10);
    if (param_2 == 0) {
        (i_var4 + 0x16) = 0;
    } else {
        pass1_1008_b9ce(param_1, param_2);
        (i_var4 + 0x16) = in_ax;
        (i_var4 + 0x18) = in_dx;
    }
    u_var1 = (i_var4 + 0x16);
    if ((u_var1 + 8) != 0) {
        pass13_funcs::pass1_1008_b200(param_1);
        u_var6 = pass1_1008_b38c(param_1);
        u_var3 = (u_var6 >> 0x10);
        u_var2 = u_var6;
        u_var1 = (i_var4 + 0x16);
        pass1_1008_b85c(param_1, (u_var1 + 10));
        (i_var4 + 0x1a) = u_var2;
        (i_var4 + 0x1c) = u_var3;
        u_var1 = (i_var4 + 0x16);
        lVar7 = pass1_1008_b8ac(param_1, (u_var1 + 0xe));
        (i_var4 + 0x1e) = lVar7;
        (i_var4 + 0x20) = (lVar7 >> 0x10);
        return;
    }
    (i_var4 + 0x1a) = 0;
    (i_var4 + 0x1e) = 0;
    return;
}

pub unsafe fn pass1_1008_b544(param_1: u32, param_2: i32) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let mut u_var4: u32;
    let mut u_var5: u32;
    let ppVar6: *mut pass1_struct_2;
    let mut in_dx: u16;
    let mut i_var7: i32;
    let mut u_var8: u16;
    let mut unaff_cs: u16;

    i_var7 = param_1;
    u_var8 = (param_1 >> 0x10);
    if (param_2 != 0) {
        if ((i_var7 + 0x1a) != 0) {
            u_var4 = (i_var7 + 0x16);
            (u_var4 + 8) = 1;
            u_var4 = (i_var7 + 0x1a);
            u_var5 = (i_var7 + 0x16);
            (u_var5 + 10) = (u_var4 + 8);
            u_var4 = (i_var7 + 0x1e);
            u_var5 = (i_var7 + 0x16);
            (u_var5 + 0xe) = (u_var4 + 8);
            u_var4 = (i_var7 + 0x16);
            ppVar6 = pass1_1030_8344(
                ctx._g_bool_1050_5748,
                (ctx._g_bool_1050_5748 >> 0x10),
                (u_var4 + 10),
            );
            unaff_cs = SUB42(&PTR_LOOP_1050_1038, 0);
            pass1_1038_3608(CONCAT22(in_dx, ppVar6));
        }
    }
    (i_var7 + 0x1e) = 0;
    (i_var7 + 0x1a) = 0;
    (i_var7 + 0x16) = 0;
    pu_var1 = (i_var7 + 0xe);
    u_var2 = (i_var7 + 0x10);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)(unaff_cs, pu_var1, u_var2, 1);
    }
    (i_var7 + 0xe) = 0;
    pu_var1 = (i_var7 + 0x12);
    u_var2 = (i_var7 + 0x14);
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)(unaff_cs, pu_var1, u_var2, 1);
    }
    (i_var7 + 0x12) = 0;
    return;
}

pub unsafe fn pass1_1008_b61a(param_1: u32, param_2: u32) {
    let mut u_var1: u16;
    let mut u_var2: u32;

    u_var2 = pass1_1008_b8fa(param_1, param_2);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x1a) = u_var2;
    (param_1 + 0x1c) = (u_var2 >> 0x10);
    return;
}

pub unsafe fn pass1_1008_b63a(param_1: Vec<u8>, param_2: Vec<u8>) {
    let mut in_dx: u16;
    let mut u_var1: u16;

    pass1_1008_b964(param_1, param_2);
    u_var1 = (param_1 >> 0x10);
    (param_1 + 0x1e) = in_ax;
    (param_1 + 0x20) = in_dx;
    return;
}

pub unsafe fn pass1_1008_b820(param_1: u32) {
    let ppVar1: *mut pass1_struct_2;
    let mut in_dx: u16;
    let mut u_var2: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    ppVar1 = pass1_1030_8344(
        ctx._g_bool_1050_5748,
        (ctx._g_bool_1050_5748 >> 0x10),
        0x8000001,
    );
    if (&ppVar1.field_0x152 == 0) {
        return 0;
    }
    u_var2 = (param_1 >> 0x10);
    return CONCAT22((param_1 + 0xc), (param_1 + 10));
}

pub unsafe fn pass1_1008_b85c(param_1: u32, param_2: libc::c_long) {
    let pu_var1: Vec<u8>;

    let mut unaff_ss: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass14_funcs::pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0xe));
    while {
        pu_var1 = local_a;
        pass14_funcs::pass1_1008_5b12(CONCAT22(unaff_ss, pu_var1));
        if ((ctx.dx_reg | pu_var1) == 0) {
            return;
        }
        (pu_var1 + 8) != param_2
    } {}
    return;
}

pub unsafe fn pass1_1008_b8ac(param_1: u32, param_2: i32) -> libc::c_long {
    let mut unaff_ss: u16;
    let lVar1: u32;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    pass14_funcs::pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x12));
    while {
        lVar1 = pass14_funcs::pass1_1008_5b12(CONCAT22(unaff_ss, local_a));
        if (lVar1 == 0) {
            return 0;
        }
        (lVar1 + 8) != param_2
    } {}
    return lVar1;
}

pub unsafe fn pass1_1008_b8fa(param_1: u32, param_2: u32) {
    let pu_var1: Vec<u8>;

    let mut unaff_ss: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    if (param_2 == 0) {
        return;
    }
    pass14_funcs::pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0xe));
    while {
        pu_var1 = local_a;
        pass14_funcs::pass1_1008_5b12(CONCAT22(unaff_ss, pu_var1));
        if ((ctx.dx_reg | pu_var1) == 0) {
            return;
        }
        pass_funcs::pass1_1000_3d7a((pu_var1 + 4), param_2, 0);
        pu_var1 != 0x0
    } {}
    return;
}

pub unsafe fn pass1_1008_b964(param_1: u32, param_2: libc::c_long) {
    let pu_var1: Vec<u8>;

    let mut unaff_ss: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    if (param_2 == 0) {
        return;
    }
    pass14_funcs::pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 0x12));
    while {
        pu_var1 = local_a;
        pass14_funcs::pass1_1008_5b12(CONCAT22(unaff_ss, pu_var1));
        if ((ctx.dx_reg | pu_var1) == 0) {
            return;
        }
        pass_funcs::pass1_1000_3d7a((pu_var1 + 4), param_2, 0);
        pu_var1 != 0x0
    } {}
    return;
}

pub unsafe fn pass1_1008_b9ce(param_1: u32, param_2: libc::c_long) {
    let pu_var1: Vec<u8>;

    let mut unaff_ss: u16;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: [u8; 8];

    if (param_2 == 0) {
        return;
    }
    pass14_funcs::pass1_1008_5784(CONCAT22(unaff_ss, local_a), (param_1 + 10));
    while {
        pu_var1 = local_a;
        pass14_funcs::pass1_1008_5b12(CONCAT22(unaff_ss, pu_var1));
        if ((ctx.dx_reg | pu_var1) == 0) {
            return;
        }
        pass_funcs::pass1_1000_3d7a((pu_var1 + 4), param_2, 0);
        pu_var1 != 0x0
    } {}
    return;
}

pub unsafe fn pass1_1008_ba38(param_1: u32, param_2: u32) {
    let mut u_var1: u32;
    let u_var2: u8;
    let BVar3: bool;
    let pu_var4: Vec<u8>;
    let extraout_var: u32;

    let mut i_var5: i32;
    let mut u_var6: u16;
    let mut unaff_ss: u16;
    let mut local_2a: u32;
    let mut local_1e: u16;
    let mut local_14: [u8; 8];
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;

    u_var2 = write_to_file_1008_7cac(param_2, 0x16);
    if (CONCAT31(extraout_var, u_var2) != 0) {
        u_var6 = (param_1 >> 0x10);
        i_var5 = param_1;
        local_c = (i_var5 + 0x22);
        BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_c), 2);
        if (BVar3 != 0) {
            if ((i_var5 + 10) == 0) {
                local_c = 0;
            } else {
                u_var1 = (i_var5 + 10);
                local_c = (u_var1 + 8);
            }
            local_1e = local_c;
            BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_1e), 2);
            if (BVar3 != 0) {
                pass14_funcs::pass1_1008_5784(CONCAT22(unaff_ss, local_14), (i_var5 + 10));
                while {
                    pu_var4 = local_14;
                    pass14_funcs::pass1_1008_5b12(CONCAT22(unaff_ss, pu_var4));
                    _local_a = CONCAT22(ctx.dx_reg, pu_var4);
                    if ((ctx.dx_reg | pu_var4) == 0) {
                        return;
                    }
                    u_var1 = (pu_var4 + 4);
                    write_to_file_1008_7c2a(param_2, u_var1, (u_var1 >> 0x10));
                    if (pu_var4 == 0x0) {
                        break;
                    }
                    local_6 = (_local_a + 8);
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_6), 2);
                    if (BVar3 == 0) {
                        break;
                    }
                    local_2a = (_local_a + 10);
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_2a), 4);
                    if (BVar3 == 0) {
                        break;
                    }
                    local_6 = (_local_a + 0xe);
                    BVar3 = write_to_file_1008_7e1c(param_2, CONCAT22(unaff_ss, &local_6), 2);
                    BVar3 != 0
                } {}
            }
        }
        ctx.g_u16_1050_0310 = 0x6d0;
    }
    return;
}

pub unsafe fn pass1_1008_bd02(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass13_funcs::pass1_1008_afde(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_bd28(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass13_funcs::pass1_1008_b08c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_bd4e(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass13_funcs::pass1_1008_b08c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_bd74(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass13_funcs::pass1_1008_b08c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn pass1_1008_bd9a(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass13_funcs::pass1_1008_b08c(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// void  pass1_1008_bde0(param_1: u32)
pub unsafe fn pass1_1008_bde0(param_1: u32) {
    let mut u_var1: u32;
    let in_dx: *mut u16;
    let local_bx_64: *mut Struct225;
    let local_bx_92: *mut Struct226;
    let local_bx_120: *mut Struct227;
    let local_bx_148: *mut Struct228;
    let local_bx_176: *mut Struct229;
    let local_bx_204: *mut Struct230;
    let local_bx_232: *mut Struct231;
    let local_bx_260: *mut Struct232;
    let local_bx_288: *mut Struct233;
    let local_bx_316: *mut Struct234;
    let local_bx_344: *mut Struct235;
    let local_bx_372: *mut Struct236;
    let local_bx_400: *mut Struct237;
    let local_bx_428: *mut Struct238;
    let local_bx_456: *mut Struct239;
    let local_bx_484: *mut Struct240;
    let local_bx_512: *mut Struct241;
    let local_bx_540: *mut Struct242;
    let local_bx_568: *mut Struct243;
    let local_bx_596: *mut Struct244;
    let local_bx_624: *mut Struct245;
    let local_bx_654: *mut Struct246;
    let local_bx_685: *mut Struct247;
    let local_bx_716: *mut Struct248;
    let local_bx_747: *mut Struct249;
    let local_bx_778: *mut Struct250;
    let local_bx_809: *mut Struct251;
    let local_bx_840: *mut Struct252;
    let local_bx_871: *mut Struct253;
    let local_bx_902: *mut Struct254;
    let local_bx_933: *mut Struct255;
    let local_bx_964: *mut Struct256;
    let local_bx_995: *mut Struct257;
    let local_bx_1026: *mut Struct258;
    let local_bx_1057: *mut Struct259;
    let local_bx_1088: *mut Struct260;
    let local_bx_1119: *mut Struct261;
    let local_bx_1150: *mut Struct262;
    let local_bx_1181: *mut Struct263;
    let local_bx_1212: *mut Struct264;
    let local_bx_1243: *mut Struct265;
    let local_bx_1274: *mut Struct266;
    let local_bx_1305: *mut Struct267;
    let local_bx_1336: *mut Struct268;
    let local_bx_1367: *mut Struct269;
    let local_bx_1398: *mut Struct270;
    let local_bx_1429: *mut Struct271;
    let local_bx_1460: *mut Struct272;
    let local_bx_1491: *mut Struct273;
    let local_bx_1522: *mut Struct274;
    let local_bx_1553: *mut Struct275;
    let local_bx_1584: *mut Struct276;
    let local_bx_1615: *mut Struct277;
    let local_bx_1646: *mut Struct278;
    let local_bx_1677: *mut Struct279;
    let local_bx_1708: *mut Struct280;
    let local_bx_1739: *mut Struct281;
    let local_bx_1770: *mut Struct282;
    let local_bx_1801: *mut Struct283;
    let local_bx_1832: *mut Struct284;
    let local_bx_1863: *mut Struct285;
    let local_bx_1894: *mut Struct286;
    let local_bx_1925: *mut Struct287;
    let local_bx_1956: *mut Struct288;
    let local_bx_1987: *mut Struct289;
    let local_bx_2018: *mut Struct290;
    let local_bx_2049: *mut Struct291;
    let local_bx_2080: *mut Struct292;
    let mut u_var2: u16;

    ctx._PTR_LOOP_1050_06e0 = param_1;
    if (ctx.__g_Struct94_ptr_1 == 0) {
        u_var1 = param_1;
        struct_fn_1000_160a();
        _g_Struct94_ptr_1 = u_var1;
        ctx.g_u16_ptr_1050_5f2e = in_dx;
    } else {
    }
    alloc_mem_1000_1708(0x1aa, 0x10000, _g_Struct94_ptr_1, ctx.g_u16_ptr_1050_5f2e);
    param_1 = _g_Struct94_ptr_1;
    (param_1 + 2) = ctx.g_u16_ptr_1050_5f2e;
    u_var2 = (*param_1 >> 0x10);
    local_bx_64 = param_1;
    local_bx_64.field_0x6 = 0x6e4;
    local_bx_64.field_0x8 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 10) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_92 = param_1;
    local_bx_92.field_0xc = 0x6ea;
    local_bx_92.field_0xe = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x10) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_120 = param_1;
    local_bx_120.field_0x12 = 0x6ee;
    local_bx_120.field_0x14 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x16) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_148 = param_1;
    local_bx_148.field_0x18 = 0x6f2;
    local_bx_148.field_0x1a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x1c) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_176 = param_1;
    local_bx_176.field_0x1e = 0x6f6;
    local_bx_176.field_0x20 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x22) = 4;
    u_var2 = (*param_1 >> 0x10);
    local_bx_204 = param_1;
    local_bx_204.field_0x24 = 0x6fe;
    local_bx_204.field_0x26 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x28) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_232 = param_1;
    local_bx_232.field_0x2a = 0x702;
    local_bx_232.field_0x2c = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x2e) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_260 = param_1;
    local_bx_260.field_0x30 = 0x708;
    local_bx_260.field_0x32 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x34) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_288 = param_1;
    local_bx_288.field_0x36 = 0x70e;
    local_bx_288.field_0x38 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x3a) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_316 = param_1;
    local_bx_316.field_0x3c = 0x714;
    local_bx_316.field_0x3e = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x40) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_344 = param_1;
    local_bx_344.field_0x42 = 0x71a;
    local_bx_344.field_0x44 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x46) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_372 = param_1;
    local_bx_372.field_0x48 = 0x71e;
    local_bx_372.field_0x4a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x4c) = 7;
    u_var2 = (*param_1 >> 0x10);
    local_bx_400 = param_1;
    local_bx_400.field_0x4e = 0x72c;
    local_bx_400.field_0x50 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x52) = 6;
    u_var2 = (*param_1 >> 0x10);
    local_bx_428 = param_1;
    local_bx_428.field_0x54 = 0x738;
    local_bx_428.field_0x56 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x58) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_456 = param_1;
    local_bx_456.field_0x5a = 0x73e;
    local_bx_456.field_0x5c = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x5e) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_484 = param_1;
    local_bx_484.field_0x60 = 0x744;
    local_bx_484.field_0x62 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 100) = 4;
    u_var2 = (*param_1 >> 0x10);
    local_bx_512 = param_1;
    local_bx_512.field_0x66 = 0x74c;
    local_bx_512.field_0x68 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x6a) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_540 = param_1;
    local_bx_540.field_0x6c = 0x750;
    local_bx_540.field_0x6e = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x70) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_568 = param_1;
    local_bx_568.field_0x72 = 0x756;
    local_bx_568.field_0x74 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x76) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_596 = param_1;
    local_bx_596.field_0x78 = 0x75a;
    local_bx_596.field_0x7a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x7c) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_624 = param_1;
    local_bx_624.field_0x7e = 0x75e;
    local_bx_624.field_0x80 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x82) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_654 = param_1;
    local_bx_654.field_0x84 = 0x764;
    local_bx_654.field_0x86 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x88) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_685 = param_1;
    local_bx_685.field_0x8a = 0x76a;
    local_bx_685.field_0x8c = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x8e) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_716 = param_1;
    local_bx_716.field_0x90 = 0x770;
    local_bx_716.field_0x92 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x94) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_747 = param_1;
    local_bx_747.field_0x96 = 0x774;
    local_bx_747.field_0x98 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x9a) = 4;
    u_var2 = (*param_1 >> 0x10);
    local_bx_778 = param_1;
    local_bx_778.field_0x9c = 0x77c;
    local_bx_778.field_0x9e = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xa0) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_809 = param_1;
    local_bx_809.field_0xa2 = 0x780;
    local_bx_809.field_0xa4 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xa6) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_840 = param_1;
    local_bx_840.field_0xa8 = 0x782;
    local_bx_840.field_0xaa = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xac) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_871 = param_1;
    local_bx_871.field_0xae = 0x786;
    local_bx_871.field_0xb0 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xb2) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_902 = param_1;
    local_bx_902.field_0xb4 = 0x78a;
    local_bx_902.field_0xb6 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xb8) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_933 = param_1;
    local_bx_933.field_0xba = 0x78e;
    local_bx_933.field_0xbc = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xbe) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_964 = param_1;
    local_bx_964.field_0xc0 = 0x792;
    local_bx_964.field_0xc2 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xc4) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_995 = param_1;
    local_bx_995.field_0xc6 = 0x796;
    local_bx_995.field_0xc8 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xca) = 4;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1026 = param_1;
    local_bx_1026.field_0xcc = 0x79e;
    local_bx_1026.field_0xce = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xd0) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1057 = param_1;
    local_bx_1057.field_0xd2 = 0x7a0;
    local_bx_1057.field_0xd4 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xd6) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1088 = param_1;
    local_bx_1088.field_0xd8 = 0x7a4;
    local_bx_1088.field_0xda = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xdc) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1119 = param_1;
    local_bx_1119.field_0xde = 0x7a6;
    local_bx_1119.field_0xe0 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xe2) = 6;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1150 = param_1;
    local_bx_1150.field_0xe4 = 0x7b2;
    local_bx_1150.field_0xe6 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xe8) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1181 = param_1;
    local_bx_1181.field_0xea = 0x7b4;
    local_bx_1181.field_0xec = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xee) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1212 = param_1;
    local_bx_1212.field_0xf0 = 0x7ba;
    local_bx_1212.field_0xf2 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xf4) = 0x2d;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1243 = param_1;
    local_bx_1243.field_0xf6 = 0x814;
    local_bx_1243.field_0xf8 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0xfa) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1274 = param_1;
    local_bx_1274.field_0xfc = 0x81a;
    local_bx_1274.field_0xfe = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x100) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1305 = param_1;
    local_bx_1305.field_0x102 = 0x81c;
    local_bx_1305.field_0x104 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x106) = 0x4b;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1336 = param_1;
    local_bx_1336.field_0x108 = 0x8b2;
    local_bx_1336.field_0x10a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x10c) = 6;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1367 = param_1;
    local_bx_1367.field_0x10e = 0x8be;
    local_bx_1367.field_0x110 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x112) = 4;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1398 = param_1;
    local_bx_1398.field_0x11a = 0x8c6;
    local_bx_1398.field_0x11c = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x11e) = 0x35;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1429 = param_1;
    local_bx_1429.field_0x120 = 0x930;
    local_bx_1429.field_0x122 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x124) = 0x2e;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1460 = param_1;
    local_bx_1460.field_0x114 = 0x98c;
    local_bx_1460.field_0x116 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x118) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1491 = param_1;
    local_bx_1491.field_0x126 = 0x98e;
    local_bx_1491.field_0x128 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x12a) = 9;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1522 = param_1;
    local_bx_1522.field_0x12c = 0x9a0;
    local_bx_1522.field_0x12e = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x130) = 0x1a;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1553 = param_1;
    local_bx_1553.field_0x132 = 0x9d4;
    local_bx_1553.field_0x134 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x136) = 8;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1584 = param_1;
    local_bx_1584.field_0x138 = 0x9e4;
    local_bx_1584.field_0x13a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x13c) = 0x4a;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1615 = param_1;
    local_bx_1615.field_0x144 = 0xa78;
    local_bx_1615.field_0x146 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x148) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1646 = param_1;
    local_bx_1646.field_0x14a = 0xa7c;
    local_bx_1646.field_0x14c = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x14e) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1677 = param_1;
    local_bx_1677.field_0x150 = 0xa7e;
    local_bx_1677.field_0x152 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x154) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1708 = param_1;
    local_bx_1708.field_0x156 = 0xa80;
    local_bx_1708.field_0x158 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x15a) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1739 = param_1;
    local_bx_1739.field_0x15c = 0xa86;
    local_bx_1739.field_0x15e = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x160) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1770 = param_1;
    local_bx_1770.field_0x168 = 0xa8a;
    local_bx_1770.field_0x16a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x16c) = 0x1b;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1801 = param_1;
    local_bx_1801.field_0x16e = 0xac0;
    local_bx_1801.field_0x170 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x172) = 0x16;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1832 = param_1;
    local_bx_1832.field_0x174 = 0xaec;
    local_bx_1832.field_0x176 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x178) = 0x3e;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1863 = param_1;
    local_bx_1863.field_0x17a = 0xb68;
    local_bx_1863.field_0x17c = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x17e) = 0x46;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1894 = param_1;
    local_bx_1894.field_0x180 = 0xbf4;
    local_bx_1894.field_0x182 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x184) = 1;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1925 = param_1;
    local_bx_1925.field_0x186 = 0xbf6;
    local_bx_1925.field_0x188 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x18a) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1956 = param_1;
    local_bx_1956.field_0x18c = 0xbfc;
    local_bx_1956.field_0x18e = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 400) = 3;
    u_var2 = (*param_1 >> 0x10);
    local_bx_1987 = param_1;
    local_bx_1987.field_0x192 = 0xc02;
    local_bx_1987.field_0x194 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x196) = 10;
    u_var2 = (*param_1 >> 0x10);
    local_bx_2018 = param_1;
    local_bx_2018.field_0x198 = 0xc16;
    local_bx_2018.field_0x19a = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x19c) = 0x24;
    u_var2 = (*param_1 >> 0x10);
    local_bx_2049 = param_1;
    local_bx_2049.field_0x19e = 0xc5e;
    local_bx_2049.field_0x1a0 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x1a2) = 2;
    u_var2 = (*param_1 >> 0x10);
    local_bx_2080 = param_1;
    local_bx_2080.field_0x1a4 = 0xc62;
    local_bx_2080.field_0x1a6 = &ctx.g_alloc_addr_1050_1050;
    (param_1 + 0x1a8) = 0x44;
    return;
}

pub unsafe fn pass1_1008_c626(param_1: *mut u32) {
    ctx._PTR_LOOP_1050_06e0 = 0;
    let param_1_val = unsafe { *param_1 };
    error_check_1000_17ce(param_1_val);
    return;
}

pub unsafe fn pass1_1008_c646(param_1: u16, param_2: u32) -> i32 {
    let pi_var1: *mut i32;
    let mut u_var2: i32;
    let mut u_var3: u32;
    let pu_var4: Vec<u8>;
    let mut in_dx: i32;
    let mut unaff_si: u16;
    let ppVar5: *mut Struct2111;
    let mut local_12: u32;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var4 = pass1_1008_c6fa(CONCAT22(param_2, param_1), (param_2 >> 0x10));
    ppVar5 = process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x35));
    local_12 = 0;
    loop {
        pi_var1 = (pu_var4 + 4);
        let pi_var1_val = unsafe { *pi_var1 };
        if (pi_var1_val == local_12 || pi_var1_val < local_12) {
            // LAB_1008_c6a5:
            return local_12._2_2_;
        }
        u_var3 = (pu_var4 & 0xffff | in_dx << 0x10);
        u_var2 = (u_var3 + local_12 * 2);
        if ((u_var2 * 2 + ppVar5 + 10) != 0) {
            local_12 = u_var2 << 0x10;
            // goto LAB_1008_c6a5;
        }
        local_12 = (local_12 + 1);
    }
}

pub unsafe fn pass1_1008_c6ae(param_1: u32, param_2: i32, param_3: u16) -> bool {
    let pu_var1: *mut u16;
    let mut u_var2: u32;
    let pu_var3: Vec<u8>;
    let mut in_dx: i32;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    pu_var3 = pass1_1008_c6fa(param_1, param_3);
    local_8 = 0;
    while (true) {
        pu_var1 = (pu_var3 + 4);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_8 || pu_var1_val < local_8) {
            return 0;
        }
        u_var2 = (pu_var3 & 0xffff | in_dx << 0x10);
        if ((u_var2 + local_8 * 2) == param_2) {
            break;
        }
        local_8 = local_8 + 1;
    }
    return 1;
}

pub unsafe fn pass1_1008_c6fa(param_1: Vec<u8>, param_2: u16) {
    let mut in_eax: u32;

    if ((0 < param_2) && (param_2 < 0x47)) {
        return (in_eax & 0xffff0000 | (param_2 * 6 + param_1));
    }
    return (in_eax & 0xffff0000);
}

pub unsafe fn pass1_1008_c72a(param_1: i32, param_2: u16, param_3: u16) {
    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    (param_1 + 10) = 0;
    (param_1 + 0xe) = 0;
    CONCAT22(param_2, param_1) = 0xca4a;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn pass1_1008_c75c(param_1: *mut Struct293) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_5: *mut Struct293;
    let mut u_var4: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1 = 0xca4a;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    pu_var1 = local_bx_5.field_0xa;
    u_var2 = local_bx_5.field_0xc;
    if ((u_var2 | pu_var1) != 0) {
        unsafe { ppc_var3 = *pu_var1 };
        (**ppc_var3)();
    }
    pass1_1010_1d80(param_1);
    return;
}
