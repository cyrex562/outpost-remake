use crate::{defines::{
    AppContext, Struct304, Struct376, Struct44, Struct79, Struct80, Struct81, Struct83,
    Struct84, Struct85,
}, err_funcs::error_check_1000_17ce, mixed_fn_1010_830a, struct_ops2, util::{CONCAT11, CONCAT12, CONCAT13, CONCAT22, ZEXT24}};
use crate::app_context::AppContext;
use crate::big_funcs::call_big_fn_1010_c7e2;
use crate::bool_funcs::check_flag_1000_1ab0;
use crate::draw::draw1::load_cursor_1020_7f7a;
use crate::draw::draw2::process_struct_1020_1738;
use crate::err_funcs::{_SHI_INVOKEERRORHANDLER1, error_check_1000_0dc6, error_check_1000_16ee, invoke_error_handler_1000_1e61};
use crate::func_ptr_funcs::call_fn_ptr_1000_5586;
use crate::list_funcs::zero_list_1008_6c90;
use crate::mem_funcs::{Address, alloc_mem_1000_010c, alloc_mem_1000_131c, alloc_mem_1000_167a, alloc_mem_1000_1708, alloc_mem_1008_909c, free_mem_1000_1b9a, get_mem_sz_1000_1532};
use crate::other_funcs::{return_1000_214a, return_one_1000_2146, zero_list_1008_3e38};
use crate::pass::pass10_funcs::pass1_1040_c54a;
use crate::pass::pass12_funcs::pass1_1008_c6fa;
use crate::pass::pass14_funcs::{pass1_1008_3e54, pass1_1008_3e76, pass1_1008_3e94, pass1_1008_4cdc, pass1_1008_4d84, pass1_1008_5134, pass1_1008_5236, pass1_1008_52fc, pass1_1008_5784, pass1_1008_57c4, pass1_1008_5b12, pass1_1008_687a, pass1_fn_1008_60e8};
use crate::pass::pass17_funcs::pass1_1030_1cd8;
use crate::pass::pass19_funcs::pass1_1040_5d12;
use crate::pass::pass20_funcs::{pass1_1010_c864, pass1_1010_cc56, pass1_1010_cf36, pass1_1010_d24a, pass1_1010_d448, pass1_1010_d5ae, pass1_1010_d710};
use crate::pass::pass4_funcs::{pass1_1028_dc52, pass1_1028_e1ec, pass1_1028_e4ec};
use crate::pass::pass4_funcs::pass1_1030_1312;
use crate::pass::pass5_funcs::pass1_1030_ce2e;
use crate::pass::pass6_funcs::{pass1_1038_4d28, pass1_1038_540a};
use crate::pass::pass7_funcs::{pass1_1018_209c, pass1_1018_4790, pass1_1018_47c8, pass1_1018_48b0, pass1_1018_4920, pass1_1018_4b78, pass1_1018_4cda, pass1_1018_4dce};
use crate::pass::pass8_funcs::{pass1_1008_ec72, pass1_1010_038e, pass1_1010_041a, pass1_1010_0538, pass1_1010_37d4, pass1_1010_383a, pass1_1010_65d0};
use crate::pass::pass8_funcs::pass1_1010_1d80;
use crate::pass::pass_funcs::{pass1_1000_4906, pass1_fn_1000_2fa4, pass1_fn_1000_52be};
use crate::prog_structs::prog_structs_1::{Struct104, Struct393, Struct552};
use crate::prog_structs::prog_structs_11::{Struct355, Struct475, Struct95};
use crate::prog_structs::prog_structs_12::{Struct102, Struct1049, Struct460, Struct94};
use crate::prog_structs::prog_structs_13::{Struct342, Struct880};
use crate::prog_structs::prog_structs_14::Struct893;
use crate::prog_structs::prog_structs_15::Struct1169;
use crate::prog_structs::prog_structs_16::{Struct1023, Struct151, Struct493};
use crate::prog_structs::prog_structs_17::{Struct1026, Struct1055, Struct1175};
use crate::prog_structs::prog_structs_18::{Struct1042, Struct180, Struct195, Struct391, Struct568};
use crate::prog_structs::prog_structs_2::{Struct1054, Struct199, Struct296, Struct413, Struct660};
use crate::prog_structs::prog_structs_20::{Struct1073, Struct388, Struct508, Struct514};
use crate::prog_structs::prog_structs_21::Struct297;
use crate::prog_structs::prog_structs_23::{Struct1022, Struct1037, Struct210, Struct341};
use crate::prog_structs::prog_structs_24::{Struct103, Struct1144, Struct1172, Struct182, Struct2111, Struct354, Struct384};
use crate::prog_structs::prog_structs_25::{Struct152, Struct402, Struct64, Struct65, Struct79, Struct80, Struct81, Struct83, Struct84, Struct85, Struct882};
use crate::prog_structs::prog_structs_26::{Struct392, Struct96, Struct97};
use crate::prog_structs::prog_structs_27::{Struct1029, Struct186, Struct298, Struct344};
use crate::prog_structs::prog_structs_28::{Struct1034, Struct1040, Struct1043, Struct1044, Struct1046, Struct1047, Struct1048, Struct1056, Struct1057, Struct1071, Struct1074, Struct346, Struct357, Struct377, Struct434, Struct464};
use crate::prog_structs::prog_structs_29::{Struct1025, Struct1027, Struct1030, Struct1031, Struct1032, Struct1033, Struct1038, Struct1039, Struct105, Struct114, Struct214, Struct216, Struct310, Struct314, Struct336, Struct375, Struct412};
use crate::prog_structs::prog_structs_30::{Struct106, Struct154, Struct190, Struct304, Struct435};
use crate::prog_structs::prog_structs_31::{Struct107, Struct127, Struct155, Struct188, Struct191, Struct303, Struct316, Struct352, Struct405, Struct436, Struct437, Struct438, Struct474};
use crate::prog_structs::prog_structs_4::Struct217;
use crate::prog_structs::prog_structs_5::Struct881;
use crate::prog_structs::prog_structs_6::Struct1041;
use crate::prog_structs::prog_structs_7::{Struct372, Struct376, Struct44, Struct629};
use crate::prog_structs::prog_structs_8::{Struct108, Struct302, Struct649, Struct68};
use crate::prog_structs::prog_structs_9::{Struct1019, Struct1072, Struct209, Struct636};
use crate::string_ops1::{big_switch_statement_1020_bd80, big_switch_statement_1020_c222, copy_string_1000_3d3e, load_str_1010_84ac, load_string_1010_847e, pass1_1020_c0ca};
use crate::sys1::get_sys_metrics_1018_4b1e;
use crate::sys2::process_struct_1040_8478;
use crate::ui_funcs::ui2::pass1_1038_af40;
use crate::util::{CARRY1, CONCAT31, LOCK, SBORROW1, SBORROW2, SUB42};
use crate::winapi_funcs::{GetSystemMetrics16, swi};

// WARNING: Variable defined which should be unmapped: local_4

pub fn set_struct_1000_0782(ctx: &mut AppContext, param_1: u16) -> u16 {
    // let in_ax: *mut Struct81;
    // let local_BX__1: *mut Struct79;
    // let mut unaff_bp: u16;
    // let mut unaff_si: u16;
    // let mut in_stack_00000000: u16;
    let mut local_4: u16;

    local_BX__1.field_0xe = 0;
    local_BX__1.field_0x10 = (local_BX__1 + 1);
    local_BX__1.field_0x8 = 0x9a0;
    struct_fn_1000_07ac(ctx, ctx.si_reg, ctx.ax_reg, ctx.bp_reg, param_1);
    return 1;
}

pub fn struct_fn_1000_07ac(ctx: &mut AppContext,
                           param_1: u16,
                           param_2: u16,
                           param_3: u16,
                           param_4: u16) {
    // let mut in_ax: i32;
    let mut pstruct_var1: Address<Struct81>;
    let mut i_var2: u32;
    // let mut in_dx: i32;
    // let local_BX__1: *mut Struct80;
    let mut u_var3: u32 = 0;
    let mut local_a: u16;
    let mut local_8: u16;
    // let mut temp_79f3d97cb82: Address<Struct81>;
    let mut temp_79f3d97cb82: u32;

    temp_79f3d97cb82 = ctx.bx_reg.field_0x10;
    ctx.bx_reg.field_0xe = temp_79f3d97cb82;
    // u_var3 = ctx.bx_reg + (ctx.dx_reg - temp_79f3d97cb82);
    i_var2 = temp_79f3d97cb82 + (u_var3 - u_var3 % ctx.ax_reg);
    ctx.bx_reg.field_0x10 = i_var2;
    while temp_79f3d97cb82 < (i_var2 - ctx.ax_reg) {
        pstruct_var1 = (&temp_79f3d97cb82.field_0x0 + ctx.ax_reg);
        (temp_79f3d97cb82).field_0x0 = pstruct_var1;
        temp_79f3d97cb82 = pstruct_var1.clone();
    }
    (temp_79f3d97cb82).field_0x0 = 0x0;
    return;
}

pub fn set_struct_1000_09ca(ctx: &mut AppContext) -> u32 {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    // let mut in_ax: i32;
    // let local_BX__1: *mut Struct84;
    // let local_bx_17: *mut Struct83;
    let temp_5f585a33fd: *mut Struct85;

    let mut struct_var_3: Struct84 = get_struct_from_address<Struct84>(ctx.bx_reg);


    pu_var1 = &local_BX__1.field_0x14;
    local_bx_17 = ((local_BX__1 + (in_ax - pu_var1) + -6 & 0xfffc) + pu_var1);
    (local_bx_17).field_0x0 = 1;
    local_BX__1.field_0xe = pu_var1;
    local_bx_17.field_0x4 = local_bx_17;
    local_bx_17.field_0x2 = local_bx_17;
    local_BX__1.field_0x10 = local_bx_17;
    if ((local_BX__1.field_0xc & 7) == 2) {
        local_BX__1.field_0x12 = 8;
    } else {
        u_var2 = (local_BX__1).field_0x0;
        temp_5f585a33fd = (u_var2 + 0x18);
        // local_BX__1.field_0x12 =
        //     (&temp_5f585a33fd[-2].field_0x1 &
        //      ~-(temp_5f585a33fd + 1 < &BYTE_1050_0008)) +
        //     8;
    }
    local_bx_17[-1].field_0x4 = (local_bx_17 - pu_var1);
    pu_var1 = (local_bx_17 - pu_var1) | 2;
    local_BX__1.field_0x18 = local_bx_17;
    local_BX__1.field_0x16 = local_bx_17.field_0x2;
    (local_bx_17.field_0x2 + 1) = pu_var1;
    local_bx_17.field_0x2 = pu_var1;
    local_BX__1.field_0x8 = 0xe08;
    return pu_var1 & 0xfffc;
}

pub fn struct_fn_1000_115c(param_1: u16, param_2: u16) -> u16 {
    let paVar1: *mut Struct127;
    let pu_var2: *mut u32;
    let pbVar3: Vec<u8>;
    let paVar4: *mut Struct127;
    let in_bx: *mut u32;
    let pu_var5: *mut u32;
    let mut local_8: u16;
    let mut local_4: u16;

    unsafe {
        ctx.ax_reg = (*in_bx & 0x7ffc);
    }
    // paVar4 = ((((ctx.ax_reg + 1) & 0xfffc) - 8 &
    //                           ~-(((ctx.ax_reg + 1) & 0xfffc) < 8)) +
    //                          8);
    if (ctx.ax_reg < paVar4) {
        pu_var5 = (ctx.ax_reg + in_bx);
        unsafe {
            if (((*pu_var5 & 1) != 0) || (ctx.ax_reg + (*pu_var5 & 0xfffc) < paVar4)) {
                return 0;
            }
            if (pu_var5 == PTR_LOOP_1050_000e) {
                PTR_LOOP_1050_000e = pu_var5[1];
            }
            (pu_var5[2] + 2) = pu_var5[1];
            (pu_var5[1] + 4) = pu_var5[2];
            local_4 = (ctx.ax_reg + ((*pu_var5 & 0xfffc) - paVar4));
            if (local_4 < s_version__d__d_1050_0012._0_2_) {
                pu_var2 = in_bx;
                *pu_var2 = *pu_var2 + (*pu_var5 & 0xfffc);
                pbVar3 = (pu_var5 + (*pu_var5 & 0xfffc));
                *pbVar3 = *pbVar3 | 2;
                return 1;
            }
        }
    } else {
        local_4 = (ctx.ax_reg + -paVar4);
        if (local_4 < s_version__d__d_1050_0012._0_2_) {
            return 1;
        }
        unsafe {
            pu_var5 = (ctx.ax_reg + in_bx);
            if ((*pu_var5 & 1) == 0) {
                local_4 = local_4 + (*pu_var5 & 0xfffc);
                if (pu_var5 == PTR_LOOP_1050_000e) {
                    PTR_LOOP_1050_000e = pu_var5[1];
                }
                (pu_var5[2] + 2) = pu_var5[1];
                (pu_var5[1] + 4) = pu_var5[2];
            }
        }
        if (PTR_DAT_0005_0000_1050_0004 < local_4) {
            PTR_DAT_0005_0000_1050_0004 = local_4;
        }
    }
    unsafe {
        *in_bx = *in_bx & 0x8003 | paVar4;
        (paVar4 + in_bx) = local_4 | 2;
        paVar4 = paVar4 + in_bx;
        *(paVar4 + 4) = ctx.PTR_LOOP_1050_0010;
        (paVar4 + 2) = (ctx.PTR_LOOP_1050_0010 + 2);
        ((ctx.PTR_LOOP_1050_0010 + 2) + 4) = paVar4;
        (ctx.PTR_LOOP_1050_0010 + 2) = paVar4;
        (paVar4 + local_4 + -2) = local_4;
        paVar1 = (paVar4 + local_4);
        *paVar1 = (Struct127)(*paVar1 & 0xfd);
    }
    return 1;
}
.
pub fn struct_fn_1000_160a(ctx: &mut AppContext) -> Address<Struct94> {
    // let mut paVar1: Address<Struct94> = return_one_1000_2146();
    let mut pstruct_var1: Address<Struct94> = Address::new();
    if pstruct_var1.full_addr == 0x0 {
        return pstruct_var1;
    }
    if (ctx.g_u16_ptr_1050_5f2e | ctx._g_Struct94_ptr_1) == 0 {
        ctx.WORD_1050_5f30 = 1;
        ctx.u16_1050_5f32 = 1;
        ctx.__g_Struct94_ptr_1 = init_struct_1000_18ec(None);
        if ctx.__g_Struct94_ptr_1 != 0x0 {
            if ctx.PTR_LOOP_1050_5f42 != 0x0 {
                process_struct_1000_1a54(
                    ctx.PTR_LOOP_1050_5f42,
                    ctx.__g_Struct94_ptr_1,
                    (ctx.__g_Struct94_ptr_1 >> 0x10),
                );
            }
            if ctx.u16_1050_5f44 != 0xffff {
                check_structs_1000_1afe(ctx.u16_1050_5f44, ctx.__g_Struct94_ptr_1);
            }
        }
    }
    return ctx._g_Struct94_ptr_1;
}

pub fn process_struct_1000_179c(param_1: u16, struct_a: &mut Struct103) {
    let u16_ptr_1: *mut u16;
    let mut u16_ptr_2: u16;
    let mut local_4: u16;

    u16_ptr_1 = _g_Struct94_ptr_1;
    u16_ptr_2 = ctx.g_u16_ptr_1050_5f2e;
    if ((ctx.g_u16_ptr_1050_5f2e | _g_Struct94_ptr_1) == 0) {
        u16_ptr_1 = struct_fn_1000_160a();
        u16_ptr_2 = struct_a;
    }
    alloc_mem_1000_1708(param_1, 0, 0, u16_ptr_1, u16_ptr_2);
    return;
}

pub fn init_struct_1000_18ec(param_1: Option<&mut Address<Struct94>>) {
    init_struct_1000_1902(param_1, 0, 0, 0xc);
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn init_struct_1000_1902(param_1: Option<&mut Address<Struct94>>, param_2: u16, param_3: u16, param_4: u16) {
    let paVar1: *mut Struct94;
    let u_var2: u8;
    let local_struct: *mut Struct94;
    let mut u_var3: i32;
    let b_var4: bool;
    let extraout_var: u32;
    let mut i_var5: i32;
    let alloc_addr: Vec<u8>;
    
    let mut u_var6: u16;
    let pa_var7: *mut Struct94;
    // let mut unaff_cs: u16;
    let mut u_var8: u32;
    let mut local_4: u16;

    if ((param_1 & 0x8000) != 0) && (_SHI_INVOKEERRORHANDLER1 != -0x6f70) {
        param_1 = (param_1 | 1);
    }
    while {
        local_struct = (param_1 & 0xfffb | 0x1000);
        alloc_mem_1000_131c(local_struct, 0x100, 0);
        if (alloc_addr | local_struct) != 0 {
            break;
        }
        pa_var7 = local_struct;
        invoke_error_handler_1000_1e61(unaff_cs, 2, 0);
        pa_var7 != 0x0
    } {}
    if ((alloc_addr | local_struct) != 0) {
        local_struct.field_0x2e = &ctx.PTR_s__1050_1f7e_1050_5f1a;
        local_struct.field_0x30 = &ctx.g_alloc_addr_1050_1050;
        local_struct.Struct94_ptr_0x2a = g_Struct94_1050_5f1e;
        local_struct.alloc_addr = g_alloc_addr_1050_5f20;
        i_var5 = 5;
        pa_var7 = local_struct;
        g_Struct94_1050_5f1e = local_struct;
        g_alloc_addr_1050_5f20 = alloc_addr;
        while (i_var5 != 0) {
            i_var5 = i_var5 + -1;
            paVar1 = pa_var7;
            pa_var7 = &pa_var7.field_0x2;
            paVar1.field_0x0 = 0;
        }
        local_struct.field_0xa = 0;
        local_struct.field_0xe = 0;
        local_struct.field_0xc = 0;
        local_struct.field_0x12 = 0;
        local_struct.field_0x10 = 0;
        local_struct.field_0x14 = 0xbead;
        u_var3 = param_1 & 0xfffd;
        local_struct.field_0x16 = u_var3;
        local_struct.field_0x18 = 0;
        local_struct.field_0x1a = 0x2000;
        local_struct.field_0x1c = 0x800;
        get_mem_sz_1000_1532(alloc_addr);
        local_struct.field_0x1e = u_var3;
        local_struct.field_0x20 = ctx.dx_reg;
        local_struct.field_0x24 = 0;
        local_struct.field_0x22 = 0;
        local_struct.field_0x26 = 0xfffe;
        local_struct.field_0x28 = 0xffff;
        local_struct.field_0x32 = 0;
        local_struct.field_0x34 = 0;
        local_struct.field_0x40 = 0;
        local_struct.field_0x3e = 0;
        u_var6 = ctx.dx_reg;
        b_var4 = check_structs_1000_1afe(param_4, CONCAT22(alloc_addr, local_struct));
        if (b_var4 != 0) {
            if ((param_3 | param_2) != 0) {
                u_var8 = CONCAT22(alloc_addr, local_struct);
                u_var2 = pass1_fn_1000_52be(param_2, param_3, param_4, 0);
                alloc_mem_1000_010c(1, CONCAT31(extraout_var, u_var2), u_var6, u_var8);
            }
            return;
        }
        free_mem_1000_1b9a(0, CONCAT22(alloc_addr, local_struct));
    }
    return;
}

pub fn process_struct_1000_1a54(param_1: u16, param_2: *mut Struct94, param_3: u16) -> u32 {
    let mut u_var1: i32;
    let mut u_var2: i32;
    let mut unaff_si: u16;
    let mut unaff_cs: u16;

    if (param_2.field_0x14 != 0xbead) {
        invoke_error_handler_1000_1e61(unaff_cs, 10, 0);
        return 0;
    }
    u_var1 = check_flag_1000_1ab0(unaff_si);
    if (u_var1 < param_2.field_0x18 + 0x14) {
        u_var2 = 0;
    } else {
        u_var2 = param_2.field_0x1a;
        param_2.field_0x1a = u_var1;
        param_2.field_0x1c = u_var1 >> 2;
    }
    return u_var2;
}

pub fn check_structs_1000_1afe(param_1: *mut Struct96, param_2: *mut Struct95) -> bool {
    let paVar1: *mut Struct96;
    let local_SI_22: *mut Struct95;
    let mut u_var2: u16;
    let mut unaff_cs: u16;

    if (param_1 == 0x0) {
        paVar1 = 0x0;
    } else {
        paVar1 = ((param_1 + 1) & 0xfffe);
    }
    u_var2 = (param_2 >> 0x10);
    local_SI_22 = param_2;
    if (local_SI_22.field_0x14 == -0x4153) {
        if ((paVar1 < param_1) || ((local_SI_22.field_0x1a - 0x14) < paVar1)) {
            invoke_error_handler_1000_1e61(unaff_cs, 3, local_SI_22);
        } else {
            if (local_SI_22.field_0x2 == 0) {
                local_SI_22.field_0x18 = paVar1;
                return 1;
            }
        }
        return 0;
    }
    invoke_error_handler_1000_1e61(unaff_cs, 10, 0);
    return 0;
}

pub fn process_struct_1000_2cb0(param_1: *mut Struct151) {
    let pu8_var1: Vec<u8>;
    let mut b_var2: u8;

    b_var2 = param_1.field_0xa;
    if (((b_var2 & 0x83) != 0) && ((b_var2 & 8) != 0)) {
        error_check_1000_16ee(param_1.field_0x6, param_1.field_0x8);
        pu8_var1 = &param_1.field_0xa;
        unsafe {
            *pu8_var1 = *pu8_var1 & 0xf7;
        }
        param_1.field_0x6 = 0;
        param_1.field_0x8 = 0;
        (param_1).field_0x0 = 0;
        param_1.field_0x2 = 0;
        param_1.field_0x4 = 0;
    }
    return;
}

pub unsafe fn process_struct_1000_2ce8(ctx: &mut AppContext, param_1: *mut Struct152, param_2: u16) {
    let pu8_var1: Vec<u8>;
    let pvVar2: &mut Vec<u8>;

    pvVar2 = alloc_mem_1000_167a(ctx, 0x200, param_2);
    if param_2 == 0 {
        pu8_var1 = &param_1.field_0xa;
        unsafe {
            *pu8_var1 = *pu8_var1 | 4;
        }
        param_1.field_0xf2 = 1;
        param_2 = &ctx.g_alloc_addr_1050_1050;
        pvVar2 = &param_1.field_0xf1;
    } else {
        pu8_var1 = &param_1.field_0xa;
        unsafe {
            *pu8_var1 = *pu8_var1 | 8;
        }
        param_1.field_0xf2 = 0x200;
    }
    param_1.field_0x2 = param_2;
    (param_1).field_0x0 = pvVar2;
    param_1.field_0x8 = param_2;
    param_1.field_0x6 = pvVar2;
    param_1.field_0x4 = 0;
    return;
}

pub fn process_struct_1000_2e74(param_1: *mut Struct154) -> bool {
    let pu8_var1: Vec<u8>;
    let pvVar2: &mut Vec<u8>;
    let pvVar3: &mut Vec<u8>;
    let mut ppvVar4: Vec<u8>;
    let pu8_var5: Vec<u8>;

    if (PTR_LOOP_1050_61ec != 0x0) {
        pu8_var5 = &param_1.field_0xf0;
        ppvVar4 = 0x5ff2;
        if ((param_1 == 0x621c) || (ppvVar4 = &PTR_LOOP_1050_5ff6, param_1 == 0x6228)) {
            let pb_var5_val = unsafe { *pu8_var5 };
            if (((param_1.field_0xa & 0xc) == 0) && ((pb_var5_val & 1) == 0)) {
                pvVar2 = *ppvVar4;
                pvVar3 = ppvVar4[1];
                if ((pvVar2 | pvVar3) == 0) {
                    pvVar2 = alloc_mem_1000_167a(0x200, pvVar3);
                    if (pvVar3 == 0) {
                        return 0;
                    }
                    *ppvVar4 = pvVar2;
                    ppvVar4[1] = pvVar3;
                }
                param_1.field_0x6 = pvVar2;
                param_1.field_0x8 = pvVar3;
                (param_1).field_0x0 = pvVar2;
                param_1.field_0x2 = pvVar3;
                param_1.field_0x4 = 0x200;
                param_1.field_0xf2 = 0x200;
                pu8_var1 = &param_1.field_0xa;
                unsafe {
                    *pu8_var1 = *pu8_var1 | 2;
                    *pu8_var5 = 0x11;
                }
                return 1;
            }
        } else {
            if (u16_1050_5f8a <= param_1.field_0xb) {
                pu8_var1 = pu8_var5;
                unsafe {
                    *pu8_var1 = *pu8_var1 | 0x10;
                }
            }
        }
    }
    return 0;
}

pub fn process_struct_1000_2f00(param_1: u16, param_2: *mut Struct155) {
    if (((param_2.field_0xf0 & 0x10) != 0) && ((*(param_2.field_0xb + 0x5f90) & 0x40) != 0)) {
        pass1_fn_1000_2fa4(param_2);
        if (param_1 != 0) {
            param_2.field_0xf0 = 0;
            param_2.field_0xf2 = 0;
            (param_2).field_0x0 = 0;
            param_2.field_0x2 = 0;
            param_2.field_0x6 = 0;
            param_2.field_0x8 = 0;
        }
    }
    return;
}

pub fn set_struct_1008_0000(param_1: &mut Struct97) {
    let local_bx_4: *mut Struct97;
    let mut in_stack_00000006: u16;

    // Segment:    2
    // Offset:     000060e0
    // Length:     efe0
    // Min Alloc:  efe0
    // Flags:      0d50
    //     Code
    //     Moveable
    //     Preload
    //     Impure (Non-shareable)
    //
    *_param_1 = 0x52a;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1.field_0x4 = 0;
    param_1.field_0x8 = 0;
    *_param_1 = 0x51e;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn process_struct_1008_3ab8(in_Struct180: *mut Struct180) -> *mut Struct180 {
    let local_bx_15: *mut Struct180;
    let mut local_es_15: u16;

    pass1_1008_687a(in_Struct180, 0);
    local_es_15 = (in_Struct180 >> 0x10);
    local_bx_15 = in_Struct180;
    local_bx_15.field_0xde = 0;
    in_Struct180 = s_0_000_1050_3b46;
    local_bx_15.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    copy_string_1000_3d3e(
        (in_Struct180 & 0xffff0000 | &local_bx_15.field_0x5b),
        s_SOLDefaultWindowClass_1050_01fe,
    );
    return in_Struct180;
}

pub fn set_struct_1008_4016(param_1: *mut Struct102) {
    let local_bx_12: *mut Struct102;
    let mut in_stack_00000006: u16;

    set_struct_1008_56b4(param_1);
    param_1.field_0x6 = 0;
    param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x10 = 0;
    param_1.field_0x14 = 0;
    param_1.field_0x18 = 0;
    param_1.field_0x1c = 0;
    *_param_1 = &PTR_LOOP_1050_48de;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn process_struct_1008_41bc(param_1: *mut Struct182) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let local_Struct182: *mut Struct182;
    let mut local_es_5: u16;
    let temp_862134eb919: *mut u32;
    let mut temp_5f500f05be: u32;
    // fn_ptr_1: *mut Vec<u8>;

    local_es_5 = (param_1 >> 0x10);
    local_Struct182 = param_1;
    param_1 = &PTR_LOOP_1050_48de;
    local_Struct182.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    pu_var1 = local_Struct182.field_0xa;
    u_var2 = local_Struct182.field_0xc;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            fn_ptr_1 = *pu_var1;
            (**fn_ptr_1)();
        }
    }
    if (local_Struct182.field_0x6 != 0) {
        temp_5f500f05be = local_Struct182.field_0x6;
        error_check_1000_0dc6(temp_5f500f05be, (temp_5f500f05be >> 0x10));
    }
    param_1 = ctx.s_1_1050_389a;
    local_Struct182.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn process_struct_1008_4544(param_1: *mut Struct104) {
    let mut bVar1: bool;
    let local_Struct104: *mut Struct186;
    let mut u_var2: i32;
    let mut local_8: u16;

    u_var2 = (param_1 >> 0x10);
    local_Struct104 = param_1;
    if (local_Struct104.field_0x6 == 0) {
        process_struct_1008_47cc((param_1 & 0xffff | u_var2 << 0x10));
    }
    if (local_Struct104.field_0x6 == 0) {
        bVar1 = false;
    } else {
        if (local_Struct104.field_0xa == 0) {
            process_struct_1008_4834((param_1 & 0xffff | u_var2 << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return;
    }
    return;
}

pub fn process_struct_1008_4772(in_struct_104_ptr: *mut Struct104) {
    let mut bVar1: bool;
    let local_bx_4: *mut Struct104;
    let mut u_var2: i32;
    let mut local_4: u16;

    u_var2 = (in_struct_104_ptr >> 0x10);
    local_bx_4 = in_struct_104_ptr;
    if (&local_bx_4.a == 0) {
        process_struct_1008_47cc((in_struct_104_ptr & 0xffff | u_var2 << 0x10));
    }
    if (&local_bx_4.a == 0) {
        bVar1 = false;
    } else {
        if (&local_bx_4.field_0xa == 0) {
            process_struct_1008_4834((in_struct_104_ptr & 0xffff | u_var2 << 0x10));
        }
        bVar1 = true;
    }
    if (!bVar1) {
        return 0;
    }
    return CONCAT22(local_bx_4.c, local_bx_4.b);
}

pub fn process_struct_1008_47cc(in_Struct104: *mut Struct103) -> u8 {
    let mut u_var1: u32;
    let mut u_var2: u32;
    let local_AX_23: *mut Struct106;
    let in_eax: u32;
    let local_Struct104: *mut Struct104;
    let local_bx_53: *mut Struct105;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut uStack14: u32;
    let temp_5f4c330c45: *mut Struct107;

    u_var3 = (in_Struct104 >> 0x10);
    local_Struct104 = in_Struct104;
    if (&local_Struct104.a != 0) {
        u_var1 = &local_Struct104.a;
        temp_5f4c330c45 = &local_Struct104.field_0x8;
        local_AX_23 = u_var1;
        local_AX_23 = &local_AX_23.field_0xe;
        &local_Struct104.b = u_var1 & 0xffff0000 | ZEXT24(local_AX_23);
        local_Struct104.d = &local_AX_23.field_0x436;
        local_Struct104.e = (temp_5f4c330c45 + (-(0xfbd7 < local_AX_23) & 0x6c));
        u_var2 = &local_Struct104.b;
        u_var4 = (u_var2 >> 0x10);
        local_bx_53 = u_var2;
        uStack14 = local_bx_53.field_0xe;
        in_eax = (uStack14 * local_bx_53.field_0x4 + 0x1f) / 0x20 << 2;
        local_Struct104.f = in_eax;
    }
    return in_eax;
}

// WARNING: Could not reconcile some variable overlaps

pub fn process_struct_1008_4834(in_Struct108: *mut Struct103) -> u8 {
    let pp_var1: fn();
    let local_AX_43: *mut u32;
    let mut u_var2: u32;
    
    let struct_a: *mut Struct199;
    
    let local_Struct108: *mut Struct108;
    let mut local_es_4: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    local_es_4 = (in_Struct108 >> 0x10);
    local_Struct108 = in_Struct108;
    local_AX_43 = local_Struct108.field_0xa;
    struct_a = local_Struct108.field_0xc;
    if ((struct_a | local_AX_43) != 0) {
        unsafe {
            pp_var1 = *local_AX_43;
        }
        (**pp_var1)();
        struct_a = ctx.dx_reg;
    }
    process_struct_1000_179c(0x14, struct_a);
    _local_a = CONCAT22(struct_a, local_AX_43);
    if ((struct_a | local_AX_43) != 0) {
        u_var2 = local_Struct108.field_0x10;
        u_var2 = u_var2 & 0xffff0000 | (u_var2 + 0x28);
        process_struct_1008_4c98(_local_a, u_var2, 0x100);
        local_Struct108.field_0xa = u_var2;
        local_Struct108.field_0xc = ctx.dx_reg;
        return u_var2;
    }
    &local_Struct108.field_0xa = 0;
    return local_AX_43;
}

pub fn get_struct_field_1008_48aa(param_1: u32) -> u16 {
    return (param_1 + 0xe);
}

pub unsafe fn process_struct_1008_48b8(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    process_struct_1008_41bc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn process_struct_1008_48fe(
    ctx: &mut AppContext,
    param_1: &mut String,
    param_2: u16,
    param_3: &String,
) {
    // let mut local_DX__1: u16;
    // let local_bx_4: *mut Struct188;
    // let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    *param_1 = ctx.s_1_1050_389a._type.clone();
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = 0;
    local_bx_4.field_0x8 = 0;
    local_bx_4.field_0xc = 0xffff;
    local_bx_4.field_0xe = 0;
    local_bx_4.field_0x12 = 0;
    local_bx_4.field_0x16 = 0;
    local_bx_4.field_0x1a = 0;
    local_bx_4.field_0x1e = 0;
    local_bx_4.field_0x22 = param_2;
    *param_1 = ctx.PTR_LOOP_1050_4c4c.clone();
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    pass1_fn_1008_60e8(param_3);
    local_bx_4.field_0x8 = param_2;
    local_bx_4.field_0xa = local_DX__1;
}

pub fn process_struct_1008_4c58(ctx: &mut AppContext, param_1: *mut Struct190) {
    let local_struct: *mut Struct190;
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_struct = param_1;
    param_1 = ctx.s_1_1050_389a;
    local_struct.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_struct.field_0x4 = 0;
    local_struct.field_0xc = 0;
    local_struct.field_0xe = 0;
    local_struct.field_0x12 = 1;
    param_1 = 0x4f1c;
    local_struct.field_0x2 = &ctx.PTR_LOOP_1050_1008;
}

pub fn process_struct_1008_4c98(
    ctx: &mut AppContext,
    param_1: *mut Struct191,
    param_2: u32,
    param_3: u16,
) {
    let local_bx_4: *mut Struct191;
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = param_2;
    local_bx_4.field_0xc = param_3;
    local_bx_4.field_0xe = 0;
    local_bx_4.field_0x12 = 0;
    param_1 = 0x4f1c;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub unsafe fn process_struct_1008_4ef6(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    pass1_1008_4cdc(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub unsafe fn process_struct_1008_50c2(
    param_1: *mut Struct195,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) {
    let local_bx_23: *mut Struct195;
    let mut local_es_23: u16;

    param_1.field_0x0 = param_4;
    local_es_23 = (param_1 >> 0x10);
    local_bx_23 = param_1;
    local_bx_23.field_0x2 = (param_4 + 2);
    local_bx_23.field_0x4 = param_3;
    local_bx_23.field_0x8 = param_2;
    local_bx_23.field_0xc = param_5;
    local_bx_23.field_0x10 = 0;
    pass1_1008_52fc((param_1 & 0xffff | local_es_23 << 0x10));
    return;
}

pub fn set_struct_1008_56b4(ctx: &mut AppContext, param_1: *mut Struct103) -> *mut Struct103 {
    let local_bx_4: *mut Struct103;
    let mut in_stack_00000006: u16;

    _param_1.ptr_1_lo = ctx.s_1_1050_389a;
    param_1.ptr_1_hi = &ctx.PTR_LOOP_1050_1008;
    param_1.field_0x4 = 0;
    _param_1.ptr_1_lo = s__s__d_1050_573a;
    param_1.ptr_1_hi = &ctx.PTR_LOOP_1050_1008;
    return _param_1;
}

pub fn process_struct_1008_574a(ctx: &mut AppContext, param_1: *mut Struct199) -> *mut Struct199 {
    let local_bx_4: *mut Struct199;
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1.field_0x0 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = 0;
    local_bx_4.field_0x8 = 0;
    local_bx_4.field_0xa = 1;
    param_1.field_0x0 = (s__s__s__1050_5bc0 + 4);
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return local_bx_4;
}

pub unsafe fn process_struct_1008_8d8a(
    in_struct_2: *mut Struct209,
    in_struct_1: u32,
    param_3: u32,
) {
    let mut u_var1: i32;
    let mut c_var2: u8;
    let mut u_var3: i32;
    let struct_a: *mut Struct199;
    let paVar4: *mut Struct199;
    let mut local_DX_187: u16;
    let struct_2: *mut Struct209;
    let mut local_es_4: u16;
    let mut u_var5: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    local_es_4 = (in_struct_2 >> 0x10);
    struct_2 = in_struct_2;
    u_var1 = struct_2.field_0x2e;
    if (u_var1 < 0x28) {
        if ((u_var1 < 0x25) && (u_var1 != 0x23)) {
            if (0x23 < u_var1) {
                return;
            }
            c_var2 = u_var1;
            if (((c_var2 != 0xb) && (c_var2 != 0xf)) && (c_var2 != '!')) {
                return;
            }
        }
    } else {
        if (u_var1 < 0x46) {
            if (u_var1 < 0x43) {
                if (u_var1 < 0x33) {
                    return;
                }
                if ((u_var1 != 0x34 && 0 < (u_var1 - 0x33)) && (u_var1 != 0x37)) {
                    return;
                }
            }
        } else {
            if (u_var1 != 0x49) {
                if ((u_var1 - 0x49) < 0x2a) {
                    return;
                }
                if (5 < (u_var1 - 0x73)) {
                    return;
                }
            }
        }
    }
    if (&struct_2.field_0x3a == 0) {
        u_var5 = process_struct_1008_4772(in_struct_1);
        struct_a = (u_var5 >> 0x10);
        u_var1 = u_var5;
        paVar4 = struct_a;
        u_var3 = u_var1;
        process_struct_1000_179c(0x14, struct_a);
        _local_a = CONCAT22(paVar4, u_var3);
        if ((paVar4 | u_var3) == 0) {
            &struct_2.field_0x3a = 0;
        } else {
            u_var5 = in_struct_2 & 0xffff0000 | &struct_2.field_0x28;
            process_struct_1008_50c2(_local_a, (u_var1 + 8), (u_var1 + 4), u_var5, param_3);
            struct_2.field_0x3a = u_var5;
            &struct_2.field_0x3c = local_DX_187;
        }
        pass1_1008_5134(&struct_2.field_0x3a);
        return;
    }
    pass1_1008_5236(&struct_2.field_0x3a);
    return;
}

pub fn process_struct_1008_8e9e(in_struct_1: *mut Struct210, param_2: u32, param_3: u32) {
    let struc_1: *mut Struct210;
    let mut u_var1: u16;

    u_var1 = (in_struct_1 >> 0x10);
    struc_1 = in_struct_1;
    in_struct_1 = ctx.s_1_1050_389a;
    struc_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    struc_1.field_0x4 = 0;
    struc_1.field_0x6 = 0;
    struc_1.field_0xa = 0;
    struc_1.field_0xe = param_3;
    struc_1.field_0x12 = 0;
    struc_1.field_0x16 = param_2;
    struc_1.field_0x1a = 1;
    in_struct_1 = 0x9170;
    struc_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    if (struc_1.field_0xe < 7) {
        struc_1.field_0xe = 6;
    }
    alloc_mem_1008_909c(in_struct_1);
    struc_1.field_0x6 = 0;
    return;
}

pub fn modify_struct_1008_9174(param_1: *mut u16, param_2: u32, param_3: u32) {
    let local_bx_4: *mut Struct214;
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    unsafe {
        *param_1 = ctx.s_1_1050_389a;
    }
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = param_3;
    local_bx_4.field_0x8 = param_2;
    local_bx_4.field_0xc = param_2;
    local_bx_4.field_0x10 = 0;
    unsafe {
        *param_1 = 0x9412;
    }
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn process_struct_1008_9262(param_1: i32, param_2: u16, param_3: u32, param_4: u32) {
    let pp_var1: fn();
    let mut in_ax: i32;
    let in_dx: *mut Struct199;
    let lVar2: u32;
    let mut local_8: u16;
    let mut local_4: u16;

    process_struct_1000_179c(0x12, in_dx);
    if ((in_dx | in_ax) == 0) {
        lVar2 = 0;
    } else {
        lVar2 = modify_struct_1008_9174(CONCAT22(in_dx, in_ax), param_3, param_4);
    }
    if (lVar2 != 0) {
        pp_var1 = ((param_1 + 6) + 4);
        (**pp_var1)();
    }
    return;
}

pub fn set_struct_1008_9584(param_1: *mut Struct216, param_2: u32) {
    let local_bx_4: *mut Struct216;
    let mut local_es_4: u16;

    local_es_4 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = param_2;
    param_1 = 0x9d2e;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x8 = 0;
    local_bx_4.field_0xac = 0x2000000;
    local_bx_4.field_0xb0 = 0;
    local_bx_4.field_0xb4 = 0x8000;
    local_bx_4.field_0xb6 = 0x8000;
    local_bx_4.field_0xb8 = 0x8000;
    local_bx_4.field_0xba = 0x8000;
    local_bx_4.field_0xbc = 0;
    local_bx_4.field_0xbe = 0;
    local_bx_4.field_0xc2 = 0;
    local_bx_4.field_0xc4 = 0;
    local_bx_4.field_0xc6 = 0;
    local_bx_4.field_0xc8 = (s_572_bmp_1050_2007 + 1);
    local_bx_4.field_0xca = 0;
    param_1 = 0x380a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    *&local_bx_4.field_0x5b = 0;
    *&local_bx_4.field_0xa = 0;
    return;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn process_struct_1008_9d36(param_1: Vec<u8>, param_2: Vec<u8>, param_3: u16) {
    let mut u_var1: u16;
    let pu_var2: Vec<u8>;
    
    let mut u_var3: u16;
    let mut u_var4: u32;
    let mut local_4: u16;

    pass1_1018_4cda(param_1, CONCAT22(param_3, param_2));
    (param_1 + 0x1c) = ctx.s_1_1050_389a;
    (param_1 + 0x1e) = &ctx.PTR_LOOP_1050_1008;
    (param_1 + 0x1c) = (ctx.s_18_2_1050_3aa5 + 3);
    (param_1 + 0x1e) = &ctx.PTR_LOOP_1050_1008;
    (param_1 + 0x20) = 0;
    zero_list_1008_3e38(CONCAT22(param_2, param_1 + 0x52));
    CONCAT22(param_2, param_1) = 0x9fb2;
    (param_1 + 2) = &ctx.PTR_LOOP_1050_1008;
    (param_1 + 0x1c) = 0x9fca;
    (param_1 + 0x1e) = &ctx.PTR_LOOP_1050_1008;
    PTR_LOOP_1050_4230 = param_1;
    PTR_LOOP_1050_4232 = param_2;
    pass1_1000_4906(CONCAT22(param_2, param_1 + 0x22), 0, 0x30);
    pass1_1018_4dce(CONCAT22(param_2, param_1), 0x1c0);
    local_4 = 0;
    while {
        u_var1 = local_4 + 0x1c0;
        mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, u_var1);
        (param_1 + local_4 * 4 + 0x22) = u_var1;
        (param_1 + local_4 * 4 + 0x24) = ctx.dx_reg;
        local_4 = local_4 + 1;
        local_4 < 0xc
    } {}
    u_var4 = process_struct_1008_4772((param_1 + 0x22));
    u_var3 = (u_var4 >> 0x10);
    pass1_1008_3e76(
        CONCAT22(param_2, param_1 + 0x52),
        0,
        (0x1e0 - (u_var4 + 8)) / 2 - 0x32,
        (0x280 - (u_var4 + 4)) / 2,
    );
    if (CONCAT22(param_2, param_1) == 0) {
        pu_var2 = 0x0;
        param_2 = 0x0;
    } else {
        pu_var2 = param_1 + 0x1c;
    }
    process_struct_1008_9262(
        _PTR_LOOP_1050_0388,
        (_PTR_LOOP_1050_0388 >> 0x10),
        0x50,
        CONCAT22(param_2, pu_var2),
    );
    return;
}

pub fn process_struct_1008_9fd2(param_1: *mut Struct217, param_2: u32) {
    let paVar1: *mut Struct199;
    let paVar2: *mut Struct199;
    let struct_a: *mut Struct199;
    let struct_a_00: *mut Struct199;
    let mut local_DX_149: u16;
    let mut u_var3: u16;
    let mut local_4: u16;

    u_var3 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var3, param_1), (param_2 >> 0x10));
    paVar1 = 0x0;
    &param_1.field_0xa = 0;
    &param_1.field_0x410 = 0;
    param_1.field_0x414 = 0;
    param_1.field_0x416 = 0;
    param_1.field_0x418 = 0;
    param_1.field_0x41a = 0;
    param_1.field_0x41c = 0;
    param_1.field_0x41e = 0;
    CONCAT22(u_var3, param_1) = 0xad92;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    paVar2 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    struct_a_00 = (paVar2 | paVar1);
    if (struct_a_00 == 0x0) {
        &param_1.field_0xa = 0;
    } else {
        paVar1 = process_struct_1008_574a(CONCAT22(paVar2, paVar1));
        param_1.field_0xa = paVar1;
        param_1.field_0xc = struct_a_00;
    }
    process_struct_1000_179c(0xc, struct_a_00);
    local_DX_149 = struct_a_00 | paVar1;
    if (local_DX_149 == 0) {
        paVar2 = 0x0;
        local_DX_149 = 0;
    } else {
        paVar2 = process_struct_1008_574a(CONCAT22(struct_a_00, paVar1));
    }
    param_1.field_0x410 = paVar2;
    param_1.field_0x412 = local_DX_149;
    return;
}

pub unsafe fn process_struct_1008_c882(param_1: u32) {
    let pu_var1: *mut u16;
    let mut switch_var: u16;
    let ppc_var2: fn();
    let mut u_var3: u32;
    let pu_var4: *mut u32;
    let pa_var5: *mut Struct199;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let puVar8: Vec<u8>;
    
    let mut u_var9: i32;
    let pa_var10: *mut Struct199;
    let mut u_var11: u16;
    let ctx.dx_reg: *mut Struct199;
    let struct_a: *mut Struct199;
    let paVar12: *mut Struct199;
    
    let mut iVar13: i32;
    let mut unaff_si: u16;
    let mut u_var14: u16;
    let uVar15: u8;
    let ppVar16: *mut Struct2111;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;

    u_var14 = (param_1 >> 0x10);
    iVar13 = param_1;
    pu_var4 = (iVar13 + 10);
    pa_var5 = (iVar13 + 0xc);
    if ((pa_var5 | pu_var4) != 0) {
        unsafe {
            ppc_var2 = *pu_var4;
        }
        ppc_var2();
        pa_var5 = ctx.dx_reg;
    }
    process_struct_1000_179c(0xc, pa_var5);
    u_var9 = pa_var5 | pu_var4;
    if (u_var9 == 0) {
        pa_var5 = 0x0;
        u_var9 = 0;
    } else {
        pa_var5 = process_struct_1008_574a(CONCAT22(pa_var5, pu_var4));
    }
    (iVar13 + 10) = pa_var5;
    (iVar13 + 0xc) = u_var9;
    ppVar16 = struct_ops2::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_si, 0x35));
    pa_var10 = (ppVar16 >> 0x10);
    pa_var5 = pa_var10;
    puVar8 = pass1_1008_c6fa(ctx._PTR_LOOP_1050_06e0, 0x44);
    local_10 = 0;
    struct_a = pa_var5;
    while (true) {
        pu_var1 = ((puVar8 & 0xffff) + 4);
        let pu_var1_val = unsafe { *pu_var1 };
        if (pu_var1_val == local_10 || pu_var1_val < local_10) {
            break;
        }
        u_var3 = (puVar8 & 0xffff | ZEXT24(pa_var5) << 0x10);
        switch_var = (u_var3 + local_10 * 2);
        if ((switch_var * 2 + ppVar16 + 10) != 0) {
            u_var7 = switch_var;
            big_switch_statement_1020_bd80(switch_var);
            pass1_fn_1008_60e8(u_var7, struct_a);
            uVar15 = 0;
            paVar12 = struct_a;
            u_var6 = u_var7;
            process_struct_1000_179c(0x14, struct_a);
            if ((paVar12 | u_var6) == 0) {
                u_var6 = 0;
                u_var11 = 0;
            } else {
                uVar15 = 0x18;
                pass1_1018_47c8(
                    CONCAT22(paVar12, u_var6),
                    1,
                    CONCAT13((ZEXT24(struct_a) >> 8), CONCAT12(struct_a, u_var7)),
                    switch_var,
                    0,
                );
                u_var11 = ctx.dx_reg;
            }
            u_var3 = (iVar13 + 10);
            ppc_var2 = ((iVar13 + 10) + 4);
            ppc_var2(uVar15, u_var3, (u_var3 >> 0x10), u_var6, u_var11);
            struct_a = ctx.dx_reg;
        }
        local_10 = local_10 + 1;
    }
    return;
}

pub unsafe fn process_struct_1008_cbc4(param_1: *mut Struct296, param_2: Vec<u8>) {
    let mut u_var1: u32;
    let u_var2: u8;
    let mut u_var3: u16;
    let mut u8_var4: bool;
    let struct_a_lo: *mut Struct199;
    let pa_var5: *mut Struct199;
    let mut u_var6: i32;
    let pa_var7: *mut Struct514;
    let struct_c: *mut Struct199;
    let pa_var8: *mut Struct199;
    let struct_a: *mut Struct199;
    let pu_var9: Vec<u8>;
    let struct_b: *mut Struct296;
    let struct_b_hi: *mut Struct296;
    let mut u_var10: u32;
    let mut local_40: u16;
    let mut local_3e: u16;
    let mut local_3c: u16;
    let mut local_3a: u16;
    let mut local_38: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_28: u16;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let mut temp_5fe001fdd0: u32;
    let temp_5ff61a9dbc: Vec<u8>;
    let fn_ptr_a: fn();

    struct_b_hi = (param_1 >> 0x10);
    struct_b = param_1;
    struct_a_lo = struct_b.field_0x1e;
    pa_var5 = struct_b.field_0x20;
    if ((pa_var5 | struct_a_lo) != 0) {
        fn_ptr_a = struct_a_lo;
        (**fn_ptr_a)();
        pa_var5 = struct_c;
    }
    process_struct_1000_179c(0xc, pa_var5);
    pa_var8 = (pa_var5 | struct_a_lo);
    if (pa_var8 == 0x0) {
        pa_var5 = 0x0;
        pa_var8 = 0x0;
    } else {
        pa_var5 = process_struct_1008_574a(CONCAT22(pa_var5, struct_a_lo));
    }
    struct_b.field_0x1e = pa_var5;
    struct_b.field_0x20 = pa_var8;
    local_6 = (param_2 + 0x200);
    pass1_1028_dc52(
        CONCAT13((ctx.stack_seg_reg >> 8), CONCAT12(ctx.stack_seg_reg, &local_18)),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    local_1e = 0;
    while (true) {
        ctx.ax_reg = &local_18;
        pass1_1028_e4ec(CONCAT22(ctx.stack_seg_reg, ctx.ax_reg));
        if ((ctx.dx_reg | ctx.ax_reg) == 0) {
            break;
        }
        if (ctx.ax_reg.field_0x200 == local_6) {
            local_1e = local_1e + 1;
        }
    }
    u8_var4 = false;
    if (1 < local_1e) {
        local_10 = local_c;
        local_e = local_a;
        if (local_8 != 0) {
            local_10 = 1;
            local_e = 0;
        }
        while (true) {
            ctx.ax_reg = &local_18;
            pass1_1028_e4ec(CONCAT22(ctx.stack_seg_reg, ctx.ax_reg));
            if ((ctx.dx_reg | ctx.ax_reg) == 0) {
                break;
            }
            if ((ctx.ax_reg.field_0x200 == local_6) && (ctx.ax_reg.field_0x4 != 0x4000001)) {
                u_var10 = pass1_1038_4d28(CONCAT22(ctx.dx_reg, ctx.ax_reg));
                pa_var8 = (u_var10 >> 0x10);
                u_var6 = u_var10;
                pass1_fn_1008_60e8(u_var6, pa_var8);
                u_var2 = u_var6;
                ctx.cs_reg = '\0';
                pa_var5 = pa_var8;
                process_struct_1000_179c(0x12, pa_var8);
                if ((pa_var5 | u_var6) == 0) {
                    u_var6 = 0;
                    u_var3 = 0;
                } else {
                    temp_5fe001fdd0 = ctx.ax_reg.field_0x4;
                    ctx.cs_reg = '\x18';
                    pass1_1018_4920(
                        u_var6,
                        pa_var5,
                        1,
                        0,
                        u_var2,
                        pa_var8,
                        temp_5fe001fdd0,
                        (temp_5fe001fdd0 >> 0x10),
                    );
                    u_var3 = ctx.dx_reg;
                }
                u_var1 = &struct_b.field_0x1e;
                fn_ptr_a = (&struct_b.field_0x1e + 4);
                (**fn_ptr_a)(ctx.cs_reg, u_var1, (u_var1 >> 0x10), u_var6, u_var3);
                u8_var4 = true;
            }
        }
    }
    if (!u8_var4) {
        load_str_1010_84ac(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x43d,
        );
        _local_40 = CONCAT22(struct_a, ctx.ax_reg);
        ctx.cs_reg._0_1_ = 0;
        pa_var5 = struct_a;
        pa_var7 = ctx.ax_reg;
        process_struct_1000_179c(0x12, struct_a);
        if ((pa_var5 | pa_var7) == 0) {
            pa_var7 = 0x0;
            pu_var9 = 0x0;
        } else {
            ctx.cs_reg._0_1_ = 0x18;
            pass1_1018_4920(pa_var7, pa_var5, 0, 0, ctx.ax_reg, struct_a, 0, 0);
            pu_var9 = ctx.dx_reg;
        }
        u_var1 = &struct_b.field_0x1e;
        fn_ptr_a = (&struct_b.field_0x1e + 4);
        (**fn_ptr_a)(
            ctx.cs_reg._0_1_,
            u_var1,
            (u_var1 >> 0x10),
            pa_var7,
            pu_var9,
            _local_40,
            pa_var7,
            pu_var9,
        );
    }
    return;
}

// WARNING: Variable defined which should be unmapped: local_36
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn process_struct_1008_cda2(ctx: &mut AppContext, param_1: *mut Struct298, param_2: u32) {
    let plVar1: *mut long;
    let mut u_var2: u32;
    let ppc_var3: fn();
    let pu_var4: *mut u32;
    let pa_var5: *mut Struct199;
    let mut u_var6: u16;
    let mut u_var7: u16;
    let puVar8: Vec<u8>;
    let mut u_var9: i32;
    let mut u_var10: u32;
    
    let mut u_var11: i32;
    let paVar12: *mut Struct199;
    
    
    
    let struct_a: *mut Struct199;
    
    let local_struct_1: *mut Struct297;
    let mut u_var13: u16;
    let mut u_var14: u16;
    let uVar15: u8;
    let mut unaff_ss: u16;
    let mut local_36: u16;
    let mut local_34: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: [u8; 8];
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u32;
    let mut local_1e: u32;
    let mut local_1a: u32;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_e: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u32;
    let mut local_4: u16;

    u_var13 = (param_1 >> 0x10);
    local_struct_1 = param_1;
    pu_var4 = local_struct_1.field_0x1a;
    pa_var5 = local_struct_1.field_0x1c;
    local_e = CONCAT22(pa_var5, pu_var4);
    local_12 = pu_var4;
    local_10 = pa_var5;
    if ((pa_var5 | pu_var4) != 0) {
        unsafe {
            ppc_var3 = *pu_var4;
            ppc_var3();
        }
        pa_var5 = ctx.dx_reg;
    }
    process_struct_1000_179c(0xc, pa_var5);
    u_var11 = pa_var5 | pu_var4;
    local_12 = pu_var4;
    local_10 = pa_var5;
    if (u_var11 == 0) {
        pa_var5 = 0x0;
        u_var11 = 0;
    } else {
        pa_var5 = process_struct_1008_574a(CONCAT22(pa_var5, pu_var4));
    }
    local_struct_1.field_0x1a = pa_var5;
    local_struct_1.field_0x1c = u_var11;
    local_4 = 0;
    u_var14 = (param_2 >> 0x10);
    local_8 = (param_2 + 0x210);
    u_var11 = (param_2 + 0x212) | local_8;
    u_var10 = u_var11;
    if (u_var11 != 0) {
        local_1a = (local_8 + 10);
        local_1e = 0;
        while (u_var10 = local_1a, local_1e < local_1a) {
            pass1_1030_1312(local_8, (local_8 >> 0x10), local_1e, (local_1e >> 0x10));
            local_22 = u_var10 & 0xffff | ctx.dx_reg << 0x10;
            if ((ctx.dx_reg | u_var10) != 0) {
                local_24 = 1;
                while (local_24 < 0x15) {
                    local_26 = pass1_1030_ce2e(local_22, (local_22 >> 0x10), local_24);
                    if (local_26 != 0) {
                        pass1_1008_5784(CONCAT22(unaff_ss, local_2e), &local_struct_1.field_0x1a);
                        while {
                            puVar8 = local_2e;
                            pass1_1008_5b12(CONCAT22(unaff_ss, puVar8));
                            _local_32 = CONCAT22(ctx.dx_reg, puVar8);
                            pa_var5 = (ctx.dx_reg | puVar8);
                            if (pa_var5 == 0x0) {
                                break;
                            }
                            (puVar8 + 0xe) != local_24
                        } {}
                        if (_local_32 == 0) {
                            u_var6 = big_switch_statement_1020_c222(local_24);
                            pass1_fn_1008_60e8(u_var6, pa_var5);
                            uVar15 = 0;
                            paVar12 = pa_var5;
                            u_var7 = u_var6;
                            process_struct_1000_179c(0x10, pa_var5);
                            local_e = CONCAT22(paVar12, u_var7);
                            if ((paVar12 | u_var7) == 0) {
                                u_var7 = 0;
                                u_var14 = 0;
                            } else {
                                uVar15 = 0x18;
                                u_var7 = local_26;
                                pass1_1018_48b0(
                                    local_e,
                                    CONCAT13(
                                        (local_26 >> 0xf),
                                        CONCAT12(
                                            (local_26 >> 0xf),
                                            local_26 & 0xff | (local_26 >> 8) << 8,
                                        ),
                                    ),
                                    CONCAT22(pa_var5, u_var6),
                                    local_24,
                                );
                                u_var14 = ctx.dx_reg;
                            }
                            u_var2 = &local_struct_1.field_0x1a;
                            // WARNING: Load size is inaccurate
                            ppc_var3 = (*local_struct_1.field_0x1a + 4);
                            ppc_var3(uVar15, u_var2, (u_var2 >> 0x10), u_var7, u_var14);
                        } else {
                            plVar1 = (puVar8 + 8);
                            unsafe {
                                *plVar1 = *plVar1 + local_26;
                            }
                        }
                        local_4 = 1;
                    }
                    local_24 = local_24 + 1;
                }
            }
            local_1e = local_1e + 1;
        }
    }
    u_var11 = u_var10;
    local_a = 0;
    if (local_4 == 0) {
        load_str_1010_84ac(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x43f,
        );
        _local_36 = CONCAT22(struct_a, u_var11);
        uVar15 = 0;
        pa_var5 = struct_a;
        u_var9 = u_var11;
        process_struct_1000_179c(0x10, struct_a);
        local_12 = u_var9;
        local_10 = pa_var5;
        if ((pa_var5 | u_var9) == 0) {
            u_var9 = 0;
            u_var14 = 0;
        } else {
            uVar15 = 0x18;
            pass1_1018_48b0(CONCAT22(pa_var5, u_var9), 0, CONCAT22(struct_a, u_var11), 0);
            u_var14 = ctx.dx_reg;
        }
        u_var2 = &local_struct_1.field_0x1a;
        // WARNING: Load size is inaccurate
        ppc_var3 = (*local_struct_1.field_0x1a + 4);
        ppc_var3(
            uVar15,
            u_var2,
            (u_var2 >> 0x10),
            u_var9,
            u_var14,
            _local_36,
            u_var9,
            u_var14,
        );
    }
    return;
}

pub unsafe fn process_struct_1008_d3ae(
    ctx: &mut AppContext,
    in_struct_1: *mut Struct298,
    in_struct_a_2: *mut Struct298,
) -> u8 {
    let pu_var1: *mut u32;
    let mut u_var2: u32;
    let mut u8_var3: bool;
    let u_var4: u8;
    let pa_var5: *mut Struct199;
    let mut u_var6: u16;
    let mut u_var7: i32;
    let mut u_var8: i32;
    let struct_a: *mut Struct199;
    let local_struct_1: *mut Struct298;
    let mut u_var9: u16;
    let u_var10: u8;
    let local_struct_2: *mut Struct199;
    let mut u_var11: u32;
    let mut u_var12: u32;
    let paVar13: *mut Struct568;
    let paVar14: *mut Struct568;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_18: u16;
    let mut local_16: u16;
    let mut local_12: u16;
    let mut local_10: u16;
    let mut local_c: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let fn_ptr_2: fn();

    u_var9 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    pu_var1 = local_struct_1.astruct99_0xa;
    u_var8 = local_struct_1.field_0xc;
    local_struct_2 = CONCAT22(u_var8, pu_var1);
    if ((u_var8 | pu_var1) != 0) {
        unsafe {
            fn_ptr_2 = *pu_var1;
        }
        local_struct_2 = (**fn_ptr_2)();
    }
    process_struct_1000_179c(0xc, (local_struct_2 >> 0x10));
    u_var8 = (local_struct_2 >> 0x10) | local_struct_2;
    if (local_struct_2 == 0x0) {
        pa_var5 = 0x0;
        u_var8 = 0;
    } else {
        pa_var5 = process_struct_1008_574a(local_struct_2);
    }
    local_struct_1.astruct99_0xa = pa_var5;
    local_struct_1.field_0xc = u_var8;
    u8_var3 = false;
    local_6 = 0x21;
    while (u_var4 = pa_var5, 0x10 < local_6) {
        u_var11 = pass1_1038_540a(in_struct_a_2, local_6);
        struct_a = (u_var11 >> 0x10);
        pa_var5 = (struct_a | u_var11);
        if (u_var11 != 0) {
            u8_var3 = true;
            u_var6 = pass1_1020_c0ca(local_6);
            u_var7 = pass1_fn_1008_60e8(u_var6, struct_a);
            u_var10 = 0;
            u_var8 = u_var7;
            pa_var5 = struct_a;
            process_struct_1000_179c(0x10, struct_a);
            if ((pa_var5 | u_var8) == 0) {
                u_var12 = 0;
            } else {
                u_var10 = 0x18;
                u_var12 = pass1_1018_4790(
                    CONCAT22(pa_var5, u_var8),
                    u_var11,
                    CONCAT22(struct_a, u_var7),
                    local_6,
                );
            }
            u_var2 = &local_struct_1.astruct99_0xa;
            fn_ptr_2 = (&local_struct_1.astruct99_0xa + 4);
            pa_var5 = fn_ptr_2(u_var10, u_var2, (u_var2 >> 0x10), u_var12);
        }
        local_6 = local_6 - 1;
    }
    if (!u8_var3) {
        paVar13 = load_str_1010_84ac(
            ctx._g_struct_73_1050_14cc,
            (ctx._g_struct_73_1050_14cc >> 0x10),
            0x43e,
        );
        u_var10 = 0;
        paVar14 = paVar13;
        process_struct_1000_179c(0x10, (paVar13 >> 0x10));
        if (paVar14 == 0x0) {
            u_var12 = 0;
        } else {
            u_var10 = 0x18;
            u_var12 = pass1_1018_4790(paVar14, 0, paVar13, 0);
        }
        u_var2 = &local_struct_1.astruct99_0xa;
        fn_ptr_2 = (&local_struct_1.astruct99_0xa + 4);
        u_var4 = (**fn_ptr_2)(u_var10, u_var2, (u_var2 >> 0x10), u_var12);
    }
    return u_var4;
}

pub fn process_struct_1008_dcdc(in_struct_1: *mut Struct302) {
    let local_struct_1: *mut Struct302;
    let mut u_var1: u16;

    u_var1 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    in_struct_1 = ctx.s_1_1050_389a;
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_struct_1.field_0x4 = 0;
    local_struct_1.field_0x8 = 0;
    local_struct_1.field_0xc = 0;
    local_struct_1.field_0xe = 0;
    local_struct_1.field_0x12 = 0;
    in_struct_1 = 0xdd4a;
    local_struct_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    return;
}

pub fn process_struct_1008_dd4e(param_1: *mut Struct303, param_2: u32) {
    let mut u_var1: i32;
    let paVar2: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut u_var3: i32;
    let mut u_var4: u16;
    let mut local_4: u16;

    u_var4 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var4, param_1), (param_2 >> 0x10));
    u_var1 = 0;
    &param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x1e = 0;
    CONCAT22(u_var4, param_1) = 0xeaac;
    param_1.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    paVar2 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    u_var3 = paVar2 | u_var1;
    if (u_var3 == 0) {
        &param_1.field_0xa = 0;
    } else {
        paVar2 = process_struct_1008_574a(CONCAT22(paVar2, u_var1));
        param_1.field_0xa = paVar2;
        param_1.field_0xc = u_var3;
    }
    return;
}

pub unsafe fn process_struct_1008_ddca(ctx: &mut AppContext, in_struct_1: *mut Struct304) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_bx_5: *mut Struct304;
    let mut u_var4: u16;

    u_var4 = (in_struct_1 >> 0x10);
    local_bx_5 = in_struct_1;
    in_struct_1 = 0xeaac;
    local_bx_5.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    pu_var1 = local_bx_5.field_0xe;
    u_var2 = local_bx_5.field_0x10;
    if ((u_var2 | pu_var1) != 0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pu_var1 = local_bx_5.field_0x12;
    u_var2 = local_bx_5.field_0x14;
    if ((u_var2 | pu_var1) != 0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    pu_var1 = local_bx_5.field_0xa;
    u_var2 = local_bx_5.field_0xc;
    if ((u_var2 | pu_var1) != 0) {
        ppc_var3 = *pu_var1;
        (**ppc_var3)();
    }
    error_check_1000_17ce(local_bx_5.field_0x1e);
    pass1_1010_1d80(in_struct_1);
}

pub fn process_string_14f0_01d8(param_1: u8, param_2: u8, param_3: u8, param_4: u16) {
    let pu8_var1: Vec<u8>;
    let pc_var2: *mut code;
    let mut u8_var3: u8;
    let mut i_var4: i32;
    let local_SP: *mut u16;
    let local_BP__1: *mut u16;
    let unaff_si: String;
    let mut unaff_DI: i32;
    
    let mut local_DS__1: u16;
    let mut local_7790: u16;
    let mut uStack9930: u16;
    let puStack64: *mut u16;
    let temp_87f6a135679: Vec<u8>;
    let temp_87fd3b67608: String;
    let temp_2030414afc22f7: String;
    let mut char_2: u8;
    let string_1: String;

    local_SP = &stack0xfffe;
    local_SP = &stack0xfffe;
    local_SP = (register0x00000010 + -2);
    char_2 = '\x1e';
    while {
        local_BP__1 = local_BP__1 + -1;
        local_SP = local_SP + -1;
        unsafe {
            *local_SP = *local_BP__1;
        }
        char_2 = char_2 + -1;
        '\0' < char_2
    } {}
    uStack9930 = (local_SP + unaff_si);
    unaff_si[param_4] = param_2;
    string_1 = unaff_si + param_4;
    unsafe {
        *string_1 = *string_1 + param_1;
        string_1 = unaff_si + param_4;
        *string_1 = *string_1 + param_1;
        string_1 = unaff_si;
        *string_1 = *string_1 + param_2;
        pu8_var1 = (unaff_si + param_4);
        *pu8_var1 = *pu8_var1 >> 2 | *pu8_var1 << 6;
        temp_87fd3b67608 = (&local_7790 + unaff_DI);
        temp_87fd3b67608 =
            temp_87fd3b67608 + (*pu8_var1 < '\0') * ((unaff_si & 3) - (temp_87fd3b67608 & 3));
        string_1 = unaff_si + param_4;
        *string_1 = *string_1 + param_1;
        string_1 = unaff_si + param_4;
        *string_1 = *string_1 + param_1;
        string_1 = unaff_si + param_4;
        *string_1 = *string_1 + param_1;
        string_1 = unaff_si + param_4;
        *string_1 = *string_1 + param_1;
        u8_var3 = param_1 | 0xc4;
        string_1 = unaff_si + param_4;
        *string_1 = *string_1 + param_2;
        i_var4 = CONCAT11(0x28, param_4);
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si;
        *string_1 = *string_1 + param_2;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1;
        temp_87f6a135679 = &stack0x007e + unaff_si;
        *temp_87f6a135679 = *temp_87f6a135679;
        pu8_var1 = (unaff_si + i_var4);
        *pu8_var1 = *pu8_var1 | u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si;
        *string_1 = *string_1 + param_2;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1;
        unaff_si[i_var4 + 0x80] = '(';
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        u8_var3 = param_1 | 0xc4;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        pu8_var1 = (unaff_si + i_var4 + 0xf00);
        *pu8_var1 = *pu8_var1 | 0x28;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        LOCK();
        string_1 = unaff_si + i_var4 + 0xc00;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + param_3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        u8_var3 = param_1 | 0xc4;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + param_3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        string_1 = unaff_si + i_var4;
        *string_1 = *string_1 + u8_var3;
        pc_var2 = swi(3);
        (*pc_var2)();
    }
    return;
}

pub fn process_struct_1040_c630(param_1: *mut Struct336) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut i_var3: i32;
    let mut local_DX_67: u16;
    let mut local_DX_87: u16;
    let local_bx_6: *mut Struct336;
    let mut local_es_6: u16;
    let mut local_CS__1: u16;
    let mut temp_5f342bbd88: u32;

    local_es_6 = (param_1 >> 0x10);
    local_bx_6 = param_1;
    temp_5f342bbd88 = local_bx_6.field_0x42;
    if ((temp_5f342bbd88 + 0x12) != 0x71) {
        local_bx_6.field_0x36 = 5;
        local_bx_6.field_0x26 = 5;
        local_bx_6.field_0x28 = 5;
        i_var3 = local_bx_6.field_0x36;
        local_bx_6.field_0x30 = i_var3;
        local_bx_6.field_0x2e = i_var3;
        if (PTR_LOOP_1050_5f02 == 0x0) {
            mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0xff);
            _PTR_LOOP_1050_5f04 = CONCAT22(local_DX_67, i_var3);
            local_CS__1 = 0x1010;
            mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x100);
            _PTR_LOOP_1050_5f08 = CONCAT22(local_DX_87, i_var3);
        }
        PTR_LOOP_1050_5f02 = PTR_LOOP_1050_5f02 + 1;
        local_bx_6.field_0x8 = _PTR_LOOP_1050_5f04;
        local_bx_6.field_0xc = _PTR_LOOP_1050_5f08;
        struct_ops2::process_struct_1040_9618(param_1);
        local_bx_6.field_0x20 = 0;
        local_bx_6.field_0x1e = 200;
        local_bx_6.field_0x22 = 0xa0;
        local_bx_6.field_0x24 = local_bx_6.field_0x2c + local_bx_6.field_0x36;
        local_bx_6.field_0x2e = local_bx_6.field_0x36 * 3 + local_bx_6.field_0x2a;
        local_bx_6.field_0x30 = local_bx_6.field_0x36;
        local_bx_6.field_0x32 = local_bx_6.field_0x22 - local_bx_6.field_0x36;
        local_bx_6.field_0x3c = 0x25;
        u_var2 = param_1;
        pp_var1 = (u_var2 + 4);
        (**pp_var1)(local_CS__1, param_1);
        pp_var1 = (u_var2 + 8);
        (**pp_var1)(local_CS__1, param_1, local_es_6);
    }
    return;
}

pub fn process_struct_1040_bf3e(param_1: *mut Struct341, param_2: u16) -> *mut Struct341 {
    let local_bx_4: *mut Struct341;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1 = ctx.s_1_1050_389a;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    param_1 = (ctx.s_18_2_1050_3aa5 + 3);
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x4 = param_2;
    param_1 = ctx.s_0_020_1050_3ab0;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.field_0x6 = 0;
    param_1 = 0xc53e;
    local_bx_4.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    return param_1;
}

pub fn process_struct_1040_b7ee(param_1: *mut u16, param_2: u32, param_3: u16) {
    let local_bx_23: *mut Struct342;
    let mut u_var1: u16;
    let mut u_var2: u16;

    process_struct_1040_b0bc(param_1, 0, CONCAT22(param_3, 0xfab));
    u_var1 = (param_1 >> 0x10);
    local_bx_23 = param_1;
    local_bx_23.field_0x94 = 0;
    local_bx_23.field_0x98 = 0;
    local_bx_23.field_0xb0 = 0;
    local_bx_23.field_0xb4 = 0;
    local_bx_23.field_0xb6 = 0;
    unsafe {
        *param_1 = 0xbeba;
    }
    local_bx_23.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    if (param_2 != 0) {
        u_var2 = (param_2 >> 0x10);
        local_bx_23.field_0xb0 = (param_2 + 6);
        local_bx_23.field_0xb4 = (param_2 + 0x14);
    }
    return;
}

pub fn process_struct_1010_02e0(param_1: *mut Struct316, param_2: u16, param_3: u16) {
    let mut u_var1: i32;
    let paVar2: *mut Struct199;
    let struct_a: *mut Struct199;
    let mut u_var3: i32;
    let mut local_4: u16;

    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    u_var1 = 0;
    &param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x10 = 0;
    param_1.field_0x18 = 0;
    CONCAT22(param_2, param_1) = 0xe98;
    param_1.field_0x2 = 0x1010;
    paVar2 = struct_a;
    process_struct_1000_179c(0xc, struct_a);
    u_var3 = paVar2 | u_var1;
    if (u_var3 == 0) {
        &param_1.field_0xa = 0;
    } else {
        paVar2 = process_struct_1008_574a(CONCAT22(paVar2, u_var1));
        param_1.field_0xa = paVar2;
        param_1.field_0xc = u_var3;
    }
    return;
}

pub fn modify_struct_1010_0f24(param_1: *mut Struct314, param_2: *mut Struct314, param_3: Vec<u8>) {
    let mut unaff_bp: u16;
    let ppVar1: *mut Struct2111;

    process_struct_1010_2cd2(param_1, param_2, param_3);
    param_1.field_0x60 = 0;
    param_1.field_0x64 = 0;
    param_1.field_0x68 = 0;
    &param_1.field_0x6a = 0;
    CONCAT22(param_2, param_1) = (s_648_bmp_1050_1919 + 1);
    param_1.field_0x2 = 0x1010;
    ppVar1 = struct_ops2::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 3));
    param_1.field_0x6a = ppVar1;
    param_1.field_0x6c = (ppVar1 >> 0x10);
    return;
}

pub fn modify_struct_1010_195e(param_1: *mut Struct314, param_2: *mut Struct314, param_3: Vec<u8>) {
    let mut unaff_bp: u16;
    let ppVar1: *mut Struct2111;

    modify_struct_1010_0f24(param_1, param_2, param_3);
    (param_1 + 1) = 0;
    CONCAT22(param_2, param_1) = s_35_flc_1050_1b2a;
    param_1.field_0x2 = 0x1010;
    ppVar1 = struct_ops2::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(unaff_bp, 3));
    (param_1 + 1) = ppVar1;
    param_1[1].field_0x2 = (ppVar1 >> 0x10);
    return CONCAT22(param_2, param_1);
}

pub fn process_struct_1010_1d48(param_1: *mut Struct375, param_2: u16) -> *mut Struct375 {
    let local_bx_4: *mut Struct375;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    param_1.ptr_1_lo = ctx.s_1_1050_389a;
    local_bx_4.ptr_1_hi = &ctx.PTR_LOOP_1050_1008;
    local_bx_4.u32_x04 = 0;
    local_bx_4.u16_x08 = param_2;
    param_1.ptr_1_lo = (s_573_bmp_1050_200f + 5);
    local_bx_4.ptr_1_hi = 0x1010;
    return param_1;
}

pub fn process_struct_1010_1df2(param_1: *mut Struct377, param_2: u16, param_3: u32) {
    let in_ax: *mut Struct199;
    let in_dx: *mut Struct199;
    let struct_a: *mut Struct199;
    let local_bx_4: *mut Struct377;
    let mut u_var1: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;
    let fn_ptr_1: fn();

    u_var1 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    struct_a = in_dx;
    if (&local_bx_4.field_0x4 == 0) {
        process_struct_1000_179c(0xc, in_dx);
        struct_a = (in_dx | in_ax);
        if (struct_a == 0x0) {
            &local_bx_4.field_0x4 = 0;
        } else {
            in_ax = process_struct_1008_574a(CONCAT22(in_dx, in_ax));
            local_bx_4.field_0x4 = in_ax;
            &local_bx_4.field_0x6 = struct_a;
        }
    }
    process_struct_1000_179c(10, struct_a);
    _local_a = CONCAT22(struct_a, in_ax);
    if ((struct_a | in_ax) == 0) {
        local_6 = 0;
    } else {
        *_local_a = ctx.s_1_1050_389a;
        in_ax.field_0x2 = &ctx.PTR_LOOP_1050_1008;
        in_ax.field_0x4 = param_3;
        in_ax.field_0x8 = param_2;
        *_local_a = (s_573_bmp_1050_200f + 1);
        in_ax.field_0x2 = 0x1010;
        local_6 = _local_a;
    }
    fn_ptr_1 = (&local_bx_4.field_0x4 + 4);
    (**fn_ptr_1)(0x1000, &local_bx_4.field_0x4, local_6);
    return;
}

pub fn process_struct_1010_2bfc(param_1: *mut Struct384, param_2: u32) -> *mut Struct384 {
    let mut local_EAX__1: u32;
    let mut local_register2: u16;
    let mut local_6: u16;
    let mut uStack4: u16;

    local_6 = param_2;
    uStack4 = (param_2 >> 0x10);
    process_struct_1010_1d48(CONCAT22(local_6, param_1), uStack4);
    param_1.field_0xa = 0;
    param_1.field_0xc = 0;
    param_1.field_0xe = 0;
    param_1.field_0x10 = 0;
    CONCAT22(local_6, param_1) = s_add39_wav_1050_2cc2;
    param_1.field_0x2 = 0x1010;
    return (ZEXT24(param_1) | local_EAX__1 & 0xffff0000);
}

pub fn process_struct_1010_2cd2(
    param_1: *mut Struct314,
    param_2: *mut Struct314,
    param_3: Vec<u8>,
) -> u8 {
    let mut unaff_ss: u16;
    let ppVar1: *mut Struct2111;
    let p_uvar2: *mut u16;
    let pu_var3: *mut u16;
    let mut u_var4: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x18 = 0;
    param_1.field_0x22 = 0;
    param_1.field_0x24 = 0;
    param_1.field_0x26 = 0;
    param_1.field_0x28 = 0;
    param_1.field_0x52 = 0;
    param_1.field_0x56 = 0;
    param_1.field_0x5a = 0;
    param_1.field_0x5e = 0;
    param_1.field_0x5c = 0;
    CONCAT22(param_2, param_1) = 0x36da;
    param_1.field_0x2 = 0x1010;
    pu_var3 = &local_4;
    pu_var2 = &local_6;
    u_var4 = unaff_ss;
    ppVar1 = struct_ops2::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(pu_var2, 0x48));
    pass1_1008_3e94(
        (ppVar1 + 0xe),
        CONCAT22(unaff_ss, pu_var2),
        CONCAT22(u_var4, pu_var3),
    );
    &param_1.field_0xe = 0x19001db;
    param_1.field_0xa = 0x140 - (param_1.field_0xe / 2 - local_4);
    param_1.field_0xc = 0xf0 - (param_1.field_0x10 / 2 - local_6);
    param_1.field_0x1a = 0xa006e;
    param_1.field_0x1e = 0xa012c;
    pass1_1000_4906(CONCAT22(param_2, &param_1.field_0x2a), 0, 0x28);
    return param_1;
}

pub fn process_struct_1010_35a4(param_1: *mut Struct388, param_2: u32) {
    let pp_var1: fn();
    let mut u_var2: u32;
    let mut u_var3: i32;
    let paVar4: *mut Struct114;
    let in_dx: *mut Struct199;
    
    let mut u_var5: u16;
    let ctx.dx_reg: *mut Struct199;
    let struct_a: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let local_bx_5: *mut Struct388;
    let mut u_var6: u16;
    let mut local_12: u16;
    let mut local_c: u32;
    let mut local_8: u32;

    u_var6 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    u_var2 = local_bx_5.field_0x56;
    u_var2 = (u_var2 + 8);
    local_8 = *(u_var2 + local_bx_5.field_0x5a * 4);
    local_c = param_2;
    if (param_2 != 0) {
        u_var6 = 0x1000;
        process_struct_1000_179c(0x4a, in_dx);
        u_var3 = param_2;
        if ((in_dx | u_var3) == 0) {
            u_var3 = 0;
            u_var5 = 0;
        } else {
            u_var6 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
            pass1_1040_c54a(param_2 & 0xffff | ZEXT24(in_dx) << 0x10, 1, local_8);
            u_var5 = ctx.dx_reg;
        }
        pp_var1 = (param_1 + 0x18);
        (**pp_var1)(u_var6, param_1, 1, u_var3, u_var5);
        struct_a = ctx.dx_reg;
        while ((local_c & 0xf) != 0) {
            u_var2 = (local_8 + 8);
            local_8 = *(((local_c & 0xf) - 1) * 4 + u_var2);
            u_var6 = 0x1000;
            paVar4 = local_8;
            process_struct_1000_179c(0x4a, struct_a);
            u_var3 = paVar4;
            if ((struct_a | u_var3) == 0) {
                u_var3 = 0;
            } else {
                u_var6 = SUB42(&ctx.PTR_LOOP_1050_1040, 0);
                pass1_1040_c54a(paVar4 & 0xffff | ZEXT24(struct_a) << 0x10, 1, local_8);
            }
            pp_var1 = (param_1 + 0x18);
            (**pp_var1)(u_var6, param_1, 1, u_var3);
            local_c = local_c >> 4;
            struct_a = ctx.dx_reg;
        }
    }
    return;
}

pub fn process_struct_1010_3680(param_1: u16, param_2: u16, param_3: u16, param_4: u16) {
    let mut in_ax: i32;
    let in_dx: *mut Struct199;
    let mut local_4: u16;

    process_struct_1000_179c(0x4a, in_dx);
    if ((in_dx | in_ax) != 0) {
        pass1_1040_c54a(CONCAT22(in_dx, in_ax), 1, CONCAT22(param_2_00, param_1_00));
        return;
    }
    return;
}

pub fn process_struct_1010_38f8(param_1: *mut Struct405, param_2: u16) -> u16 {
    let mut in_ax: i32;
    let mut u_var1: u16;
    let in_dx: *mut Struct199;
    let local_bx_25: *mut Struct405;
    let mut u_var2: u16;
    let pu_var3: *mut u16;
    let mut local_4: u16;

    if (param_2 != 0) {
        u_var1 = param_2 << 2;
        process_struct_1000_179c(u_var1, in_dx);
        u_var2 = (param_1 >> 0x10);
        local_bx_25 = param_1;
        local_bx_25.field_0x8 = u_var1;
        local_bx_25.field_0xa = in_dx;
        return CONCAT22(in_dx, local_bx_25.field_0x8);
    }
    process_struct_1000_179c(0x1a, in_dx);
    if ((in_dx | in_ax) != 0) {
        pu_var3 = pass1_1010_37d4(CONCAT22(in_dx, in_ax));
        return pu_var3;
    }
    return 0x0;
}

pub fn process_struct_1010_394a(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut in_ax: i32;
    let local_struct_1: *mut Struct402;
    let mut local_4: u16;

    if (param_1_00 != 0) {
        process_struct_1000_179c(param_1_00 << 2, local_struct_1);
        return;
    }
    process_struct_1000_179c(0x16, local_struct_1);
    if ((local_struct_1 | in_ax) != 0) {
        pass1_1010_383a(CONCAT22(local_struct_1, in_ax));
        return;
    }
    return;
}

pub fn process_struct_1010_3b7a(param_1: *mut Struct64, param_2: u32) {
    let mut u_var1: u16;

    u_var1 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var1, param_1), (param_2 >> 0x10));
    param_1.field_0xa = ctx.s_1_1050_389a;
    &param_1.field_0xc = &ctx.PTR_LOOP_1050_1008;
    param_1.field_0xa = (ctx.s_18_2_1050_3aa5 + 3);
    &param_1.field_0xc = &ctx.PTR_LOOP_1050_1008;
    &param_1.str_field_0xe = 0;
    &param_1.field_0x12 = 0;
    (&param_1.field_0x12 + 2) = 0;
    &param_1.field_0x16 = 0;
    CONCAT22(u_var1, param_1) = 0x3d6a;
    param_1.field_0x2 = 0x1010;
    param_1.field_0xa = 0x3d7a;
    &param_1.field_0xc = 0x1010;
    return;
}

pub fn process_struct_1010_3e3c(in_struct_1: *mut Struct393, param_2: u16) {
    let u_var1: u8;
    let extraout_var: u32;
    
    let local_struct_1: *mut Struct393;
    let local_struct_2: *mut Struct393;
    let mut uStack16: u16;
    let mut uStack10: u16;
    let mut u_var2: i32;

    local_struct_2 = in_struct_1;
    uStack16 = (in_struct_1 >> 0x10);
    get_sys_metrics_1018_4b1e(in_struct_1, 6, param_2);
    local_struct_2.field_0x20 = ctx.s_1_1050_389a;
    local_struct_2.field_0x22 = &ctx.PTR_LOOP_1050_1008;
    local_struct_2.field_0x20 = (ctx.s_18_2_1050_3aa5 + 3);
    local_struct_2.field_0x22 = &ctx.PTR_LOOP_1050_1008;
    local_struct_2.field_0x24 = 0;
    &local_struct_2.field_0x66 = 0;
    local_struct_2.field_0x6a = 4;
    local_struct_2.field_0x6c = 0;
    local_struct_2.field_0x70 = 0;
    local_struct_2.field_0x74 = 0;
    pass1_1008_3e54(
        (in_struct_1 & 0xffff0000 | &local_struct_2.field_0x76),
        0,
        3,
        5,
    );
    local_struct_2.field_0x7c = 0;
    in_struct_1.field_0x0 = &PTR_LOOP_1050_4a46;
    local_struct_2.u16_0x2 = 0x1010;
    local_struct_2.field_0x20 = &PTR_LOOP_1050_4a82;
    local_struct_2.field_0x22 = 0x1010;
    u_var1 = pass1_1000_4906(
        (in_struct_1 & 0xffff0000 | &local_struct_2.field_0x26),
        0,
        0x40,
    );
    u_var2 = CONCAT31(extraout_var, u_var1);
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1a1);
    local_struct_2.field_0x66 = u_var2;
    local_struct_2.field_0x68 = ctx.dx_reg;
    pass1_1018_4b78(in_struct_1, uStack10);
    return;
}

pub unsafe fn pass1_1010_3f00(param_1: *mut Struct376) {
    let pu_var1: *mut u32;
    let mut u_var2: i32;
    let ppc_var3: fn();
    let local_AX_159: *mut Struct391;
    let local_bx_5: *mut Struct392;
    let mut u_var4: u16;
    let mut local_10: u32;
    let mut local_4: u16;

    u_var4 = (param_1 >> 0x10);
    local_bx_5 = param_1;
    param_1.ptr_a_lo = &PTR_LOOP_1050_4a46;
    local_bx_5.field_0x2 = 0x1010;
    local_bx_5.field_0x20 = &PTR_LOOP_1050_4a82;
    local_bx_5.field_0x22 = 0x1010;
    local_4 = 0;
    while {
        pu_var1 = (&local_bx_5.field_0x26 + local_4 * 4);
        u_var2 = (&local_bx_5.field_0x26 + local_4 * 4 + 2);
        if ((u_var2 | pu_var1) != 0) {
            unsafe {
                ppc_var3 = *pu_var1;
                (**ppc_var3)();
            }
        }
        local_4 = local_4 + 1;
        local_4 < 0x10
    } {}
    pu_var1 = local_bx_5.field_0x66;
    u_var2 = local_bx_5.field_0x68;
    if ((u_var2 | pu_var1) != 0) {
        unsafe {
            ppc_var3 = *pu_var1;
            (**ppc_var3)();
        }
    }
    error_check_1000_17ce(local_bx_5.field_0x70);
    if (param_1 == 0x0) {
        local_AX_159 = 0x0;
        u_var4 = 0;
    } else {
        local_AX_159 = &local_bx_5.field_0x20;
    }
    local_10 = CONCAT22(u_var4, local_AX_159);
    local_10 = ctx.s_1_1050_389a;
    local_AX_159.field_0x2 = &ctx.PTR_LOOP_1050_1008;
    pass1_1010_1d80(param_1);
    return;
}

pub fn process_struct_1010_4a8a(param_1: *mut Struct375, param_2: *mut Struct375, param_3: u16) {
    let local_AX_19: Vec<u8>;
    let local_DX_102: Vec<u8>;
    let ppVar1: *mut Struct2111;
    let local_6: Vec<u8>;

    process_struct_1010_1d48(CONCAT22(param_2, param_1), param_3);
    local_AX_19 = 0x0;
    param_1.u16_x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x1e = 0;
    param_1.field_0x20 = 1;
    param_1.field_0x22 = 0;
    param_1.field_0x24 = 0;
    &param_1.field_0x26 = 0;
    param_1.field_0x2a = 0;
    param_1.field_0x2c = 1;
    param_1.field_0x2e = 0;
    param_1.field_0x30 = 0;
    &param_1.field_0x32 = 0;
    CONCAT22(param_2, param_1) = (s_SCForceMorale__s_for_colony__08l_1050_5024 + 6);
    param_1.ptr_1_hi = 0x1010;
    mixed_fn_1010_830a(ctx._g_struct_73_1050_14cc, 0x1b3);
    *&param_1.u16_x16 = local_AX_19;
    *(&param_1.u16_x16 + 2) = local_DX_102;
    ppVar1 = struct_ops2::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(local_6, 3));
    param_1.field_0x26 = ppVar1;
    &param_1.field_0x28 = (ppVar1 >> 0x10);
    process_struct_1008_4772(param_1.u16_x16);
    param_1.u32_x0e = 0x13c;
    param_1.ptr_2_lo = 0;
    param_1.field_0x10 = 0;
    param_1.ptr_2_hi = 0;
    return;
}

pub fn process_struct_1010_4d5c(
    param_1: *mut Struct412,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
    param_6: u16,
) {
    let mut u_var1: u32;
    let p_uvar2: *mut u16;
    let in_dx: *mut Struct199;
    let local_bx_4: *mut Struct412;
    let local_SI_55: *mut Struct413;
    let mut u_var3: u16;

    u_var3 = (param_1 >> 0x10);
    local_bx_4 = param_1;
    if (&local_bx_4.field_0x1a == 0) {
        pu_var2 = (local_bx_4.field_0x30 << 3);
        process_struct_1000_179c(pu_var2, in_dx);
        local_bx_4.field_0x1a = pu_var2;
        &local_bx_4.field_0x1c = in_dx;
    }
    u_var1 = &local_bx_4.field_0x1a;
    local_SI_55 = (param_6 * 8);
    (local_SI_55 + u_var1) = param_5;
    u_var1 = &local_bx_4.field_0x1a;
    (local_SI_55 + u_var1 + 2) = param_4;
    u_var1 = &local_bx_4.field_0x1a;
    (local_SI_55 + u_var1 + 4) = param_3;
    u_var1 = &local_bx_4.field_0x1a;
    (local_SI_55 + u_var1 + 6) = param_2;
    return;
}

pub fn process_struct_1010_5d9c(param_1: u32, button_state: u16) {
    let ppVar1: *mut Struct2111;
    let mut in_stack_0000fffa: u16;

    (param_1 + 0x1e) = button_state;
    if (button_state == 0) {
        ppVar1 = struct_ops2::process_struct_1010_20ba(
            ctx._g_Struct372_1050_0ed0,
            CONCAT22(in_stack_0000fffa, 0x2e),
        );
        pass1_1018_209c(ppVar1);
    }
    return;
}

pub fn process_struct_1010_60fa(param_1: *mut Struct434) {
    let local_bx_3: *mut Struct434;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    local_bx_3.field_0x7e = 1;
    local_bx_3.field_0x7c = local_bx_3.field_0x20;
    local_bx_3.field_0x20 = 1;
    return;
}

pub fn process_struct_1010_6118(param_1: *mut Struct435) {
    let local_bx_3: *mut Struct435;
    let mut u_var1: u16;

    u_var1 = (param_1 >> 0x10);
    local_bx_3 = param_1;
    if (local_bx_3.field_0x7e != 0) {
        local_bx_3.field_0x20 = local_bx_3.field_0x7c;
    }
    return;
}

pub fn modify_struct_1010_6326(param_1: *mut Struct436, param_2: u32) {
    let mut u_var1: u16;

    u_var1 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var1, param_1), (param_2 >> 0x10));
    param_1.field_0xa = 0;
    param_1.field_0xe = 0;
    param_1.field_0x12 = 0;
    param_1.field_0x16 = 0;
    param_1.field_0x1a = 0;
    param_1.field_0x1e = 0;
    param_1.field_0x22 = 0;
    CONCAT22(u_var1, param_1) = 0x66f0;
    param_1.field_0x2 = 0x1010;
    return;
}

pub fn modify_struct_1010_6700(param_1: *mut Struct437, param_2: u32) {
    let extraout_var: u32;
    let mut u_var1: u16;

    u_var1 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var1, param_1), (param_2 >> 0x10));
    param_1.field_0x148 = 0x33;
    CONCAT22(u_var1, param_1) = 0x6aac;
    param_1.field_0x2 = 0x1010;
    pass1_1000_4906(CONCAT22(u_var1, &param_1.field_0xa), 0, 0x114);
    param_1.field_0x32 = 1;
    param_1.field_0x40 = 1;
    param_1.field_0x46 = 1;
    param_1.field_0x4e = 1;
    param_1.field_0x54 = 1;
    param_1.field_0x5e = 1;
    param_1.field_0x68 = 1;
    param_1.field_0x6c = 1;
    param_1.field_0x74 = 1;
    param_1.field_0x78 = 1;
    param_1.field_0x7a = 1;
    param_1.field_0x7e = 1;
    param_1.field_0x82 = 1;
    param_1.field_0xa2 = 1;
    param_1.field_0xa4 = 1;
    param_1.field_0xa6 = 1;
    param_1.field_0xa8 = 1;
    param_1.field_0xae = 1;
    param_1.field_0xb2 = 1;
    param_1.field_0xb8 = 1;
    param_1.field_0xbe = 1;
    param_1.field_0xc0 = 1;
    param_1.field_0xc4 = 1;
    param_1.field_0xd4 = 1;
    param_1.field_0xda = 1;
    param_1.field_0xe2 = 1;
    param_1.field_0xfe = 1;
    param_1.field_0x100 = 1;
    param_1.field_0x102 = 1;
    param_1.field_0x104 = 1;
    param_1.field_0x106 = 1;
    param_1.field_0x108 = 1;
    pass1_1000_4906(CONCAT22(u_var1, &param_1.field_0x11e), 0, 0x2a);
    param_1.field_0x120 = 1;
    param_1.field_0x122 = 1;
    param_1.field_0x124 = 1;
    param_1.field_0x126 = 1;
    param_1.field_0x128 = 1;
    param_1.field_0x12c = 1;
    param_1.field_0x138 = 1;
    return (extraout_var & 0xffff00) << 8 | ZEXT24(param_1);
}

pub fn process_struct_1010_6abc(param_1: *mut Struct438, param_2: u32) {
    let pp_var1: fn();
    let ppVar2: *mut Struct2111;
    let mut u_var3: u16;
    let mut u_var4: u16;
    let mut in_stack_0000fff6: u16;

    u_var4 = param_2;
    process_struct_1010_1d48(CONCAT22(u_var4, param_1), (param_2 >> 0x10));
    param_1.field_0xa = ctx.s_1_1050_389a;
    param_1.field_0xc = &ctx.PTR_LOOP_1050_1008;
    param_1.field_0xa = (ctx.s_18_2_1050_3aa5 + 3);
    param_1.field_0xc = &ctx.PTR_LOOP_1050_1008;
    param_1.field_0xe = 0;
    param_1.field_0x10 = 0;
    &param_1.field_0x14 = 0;
    param_1.field_0x1c = 0;
    param_1.field_0x20 = 0;
    &param_1.field_0x22 = 0;
    CONCAT22(u_var4, param_1) = 0x7e28;
    param_1.field_0x2 = 0x1010;
    param_1.field_0xa = 0x7e38;
    param_1.field_0xc = 0x1010;
    ppVar2 = struct_ops2::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(in_stack_0000fff6, 3));
    param_1.field_0x14 = ppVar2;
    &param_1.field_0x16 = (ppVar2 >> 0x10);
    u_var3 = *&param_1.field_0x14;
    pp_var1 = (&param_1.field_0x14 + 4);
    (**pp_var1)();
    ppVar2 = struct_ops2::process_struct_1010_20ba(ctx._g_Struct372_1050_0ed0, CONCAT22(u_var3, 0x2f));
    param_1.field_0x22 = ppVar2;
    &param_1.field_0x24 = (ppVar2 >> 0x10);
    pp_var1 = (&param_1.field_0x22 + 4);
    (**pp_var1)();
    return;
}

pub fn process_struct_1010_9298(
    in_struct_1: *mut Struct314,
    param_2: *mut Struct314,
    param_3: u16,
) {
    let u_var1: u8;
    let extraout_AH: u8;
    let in_dx: *mut Struct199;
    let mut u_var2: i32;

    u_var1 = process_struct_1010_2cd2(in_struct_1, param_2, param_3);
    u_var2 = CONCAT11(extraout_AH, u_var1);
    CONCAT22(param_2, in_struct_1) = 0x9566;
    in_struct_1.field_0x2 = 0x1010;
    process_struct_1000_179c(0x20c, in_dx);
    in_struct_1.field_0x5c = u_var2;
    in_struct_1.field_0x5e = in_dx;
    pass1_1000_4906(CONCAT22(in_dx, in_struct_1.field_0x5c), 0, 0x20c);
    return CONCAT22(param_2, in_struct_1);
}

pub fn process_struct_1040_b082(param_1: *mut u16, param_2: u32) {
    let local_bx_21: *mut Struct344;
    let mut u_var1: u16;

    struct_ops2::process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        param_2,
        (param_2 >> 0x10),
    );
    u_var1 = (param_1 >> 0x10);
    local_bx_21 = param_1;
    local_bx_21.field_0x8e = 0;
    local_bx_21.field_0x90 = 0;
    unsafe {
        *param_1 = 0xb772;
    }
    local_bx_21.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    return;
}

pub fn process_struct_1040_b0bc(param_1: *mut Struct346, param_2: u32, param_3: u32) {
    let local_bx_21: *mut Struct346;
    let mut u_var1: u16;

    struct_ops2::process_struct_1040_7728(
        param_1,
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        param_3,
        (param_3 >> 0x10),
    );
    u_var1 = (param_1 >> 0x10);
    local_bx_21 = param_1;
    local_bx_21.field_0x8e = 0;
    local_bx_21.field_0x90 = param_2;
    param_1 = 0xb772;
    local_bx_21.field_0x2 = &ctx.PTR_LOOP_1050_1040;
    return;
}

pub fn process_struct_1008_e3ec(in_struct_a: *mut Struct310, param_2: u32, param_3: u32) {
    let mut u_var1: u32;
    let local_AX_88: *mut u32;
    let p_uvar2: *mut u16;
    let struct_c_lo: *mut Struct199;
    
    let local_DX_78: Vec<u8>;
    
    let paVar3: *mut Struct199;
    let ctx.dx_reg: *mut Struct199;
    let local_DX_273: Vec<u8>;
    let pu_var4: Vec<u8>;
    
    let struct_b: *mut Struct310;
    let struct_b_hi: *mut Struct310;
    
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u16;
    let mut local_18: u32;
    let mut local_14: u16;
    let mut local_12: u16;
    let mut local_c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;
    let mut temp_5fd014a0be: u32;
    let struct_c_hi: *mut Struct199;
    let fn_ptr_a: fn();

    struct_b_hi = (in_struct_a >> 0x10);
    struct_b = in_struct_a;
    local_AX_88 = struct_b.field_0xe;
    paVar3 = struct_b.field_0x10;
    if ((paVar3 | local_AX_88) != 0) {
        unsafe {
            fn_ptr_a = *local_AX_88;
            (**fn_ptr_a)();
        }
        paVar3 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x18, paVar3);
    if ((paVar3 | local_AX_88) == 0) {
        local_AX_88 = 0x0;
        pu_var4 = 0x0;
    } else {
        pass1_1030_1cd8(CONCAT13((paVar3 >> 8), CONCAT12(paVar3, local_AX_88)), 5, 5);
        pu_var4 = local_DX_78;
    }
    struct_b.field_0xe = local_AX_88;
    struct_b.field_0x10 = pu_var4;
    pass1_1028_dc52(
        CONCAT22(ctx.stack_seg_reg, &local_14),
        (&ctx.PTR_LOOP_1050_0000 + 1),
        0,
        0x400,
    );
    while (true) {
        pu_var2 = &local_14;
        pass1_1028_e4ec(CONCAT22(ctx.stack_seg_reg, pu_var2));
        if ((ctx.dx_reg | pu_var2) == 0) {
            break;
        }
        if ((pu_var2 + 0x100) != 0x8000002) {
            temp_5fd014a0be = &struct_b.field_0xe;
            fn_ptr_a = (&struct_b.field_0xe + 0xc);
            (**fn_ptr_a)(0x28, temp_5fd014a0be, (temp_5fd014a0be >> 0x10));
        }
    }
    param_3 = &struct_b.field_0xe;
    struct_c_hi = struct_b.field_0x14;
    struct_c_lo = &struct_b.field_0x12;
    paVar3 = (struct_c_hi | struct_c_lo);
    if (paVar3 != 0x0) {
        fn_ptr_a = struct_c_lo;
        (**fn_ptr_a)(&PTR_LOOP_1050_1028, struct_c_lo);
        paVar3 = ctx.dx_reg;
    }
    process_struct_1000_179c(0x18, paVar3);
    if ((paVar3 | struct_c_lo) == 0) {
        struct_c_lo = 0x0;
        pu_var4 = 0x0;
    } else {
        pass1_1030_1cd8(CONCAT13((paVar3 >> 8), CONCAT12(paVar3, struct_c_lo)), 5, 5);
        pu_var4 = local_DX_273;
    }
    struct_b.field_0x12 = struct_c_lo;
    struct_b.field_0x14 = pu_var4;
    local_c = local_8;
    local_a = local_6;
    if (local_4 != 0) {
        local_c = 1;
        local_a = 0;
    }
    while (true) {
        pu_var2 = &local_14;
        pass1_1028_e4ec(CONCAT22(ctx.stack_seg_reg, pu_var2));
        if ((ctx.dx_reg | pu_var2) == 0) {
            break;
        }
        u_var1 = &struct_b.field_0x12;
        fn_ptr_a = (&struct_b.field_0x12 + 0xc);
        (**fn_ptr_a)(0x28, u_var1, (u_var1 >> 0x10));
    }
    param_2 = &struct_b.field_0x12;
    return;
}

// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn process_struct_1008_e586(param_1: Vec<u8>, param_2: u16, param_1_00: u32) {
    let paVar1: *mut Struct493;
    let mut in_dx: i32;
    let struct_a: *mut Struct199;
    let in_string_2: String;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    paVar1 = pass1_1028_e1ec(ctx._PTR_LOOP_1050_65e2, param_1_00, (param_1_00 >> 0x10));
    local_6 = CONCAT22(in_dx, paVar1);
    struct_a = (in_dx | paVar1);
    if (struct_a == 0x0) {
        return 0;
    }
    process_struct_1000_179c(0x80, struct_a);
    _local_a = CONCAT22(struct_a, paVar1);
    in_string_2 = pass1_1038_4d28(local_6);
    copy_string_1000_3d3e(_local_a, in_string_2);
    return CONCAT22(struct_a, paVar1);
}

pub unsafe fn process_struct_1008_ea86(param_1: &mut Struct44, param_2: u8) -> &mut Struct44 {
    process_struct_1008_ddca(param_1);
    if ((param_2 & 1) != 0) {
        error_check_1000_17ce(param_1);
    }
    return param_1;
}

pub fn process_struct_1008_ed1e(param_1: u16, param_2: u16, param_1_00: i32) {
    let mut in_ax: i32;
    let in_dx: *mut Struct199;
    let mut local_4: u16;

    if (param_1_00 != 0) {
        process_struct_1000_179c(param_1_00 << 2, in_dx);
        return;
    }
    process_struct_1000_179c(0x1a, in_dx);
    if ((in_dx | in_ax) != 0) {
        pass1_1008_ec72(CONCAT22(in_dx, in_ax));
        return;
    }
    return;
}

pub fn process_struct_1010_9348(in_struct_1: *mut Struct460, param_2: u16) {
    let local_struct_1: *mut Struct460;
    let mut local_es_21: u16;

    (param_2 * 8 + 0x319e) = param_2;
    local_es_21 = (in_struct_1 >> 0x10);
    local_struct_1 = in_struct_1;
    local_struct_1.b = param_2 * 8 + 0x3198;
    local_struct_1.c = &ctx.g_alloc_addr_1050_1050;
    local_struct_1.a = param_2;
    return;
}
