extern crate libc;

mod bad_funcs;
mod big_funcs;
mod cleanup;
mod defines;
mod draw;
mod err_funcs;
mod file_funcs;
mod func_ptr_funcs;
mod globals;
mod list_funcs;
mod mem_funcs;
mod other_funcs;
mod pass2_funcs;
mod pass3_funcs;
mod pass4_funcs;
mod pass5_funcs;
mod pass6_funcs;
mod pass7_funcs;
mod pass8_funcs;
mod pass_funcs;
mod sound_funcs;
mod string_funcs;
mod struct_funcs;
mod support;
mod sys_funcs;
mod ui_funcs;
mod util;

use defines::code;
use func_ptr_funcs::{call_fn_ptr_1000_24cd, call_fn_ptr_1000_24db};
use mem_funcs::alloc_mem_1000_167a;
use other_funcs::{big_fn_1010_b038, empty_fn_1000_55ac};
use pass_funcs::{
    pass1_1008_57a4, pass1_1008_5b12, pass1_1008_6978, pass1_fn_1000_25a8, pass1_fn_1000_2913,
};
use sound_funcs::mci_fn_1020_08b6;
use string_funcs::process_string_1000_28dc;
use struct_funcs::{process_struct_1000_179c, process_struct_1010_20ba, process_struct_1018_e91e};
use sys_funcs::{dos3_call_1000_23ea, get_dos_env_1000_27d6, get_module_file_name_1000_262c};
use util::CONCAT22;

pub unsafe fn entry(
    context: &mut AppContext,
    param_1: &mut String,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: u16,
) -> *mut i32 {
    let pi32_a: *mut i32;
    let fn_ptr_b: *mut code;
    let mut u16_c: u16;
    let mut fn_ptr_d: u16;
    let mut string_e: u16;
    let mut offset_f: u16;
    let pi32_g: *mut i32;
    let mut i32_h: i32;
    let pu8_j: *mut u8;
    let pc_k: *mut libc::c_char;
    let pu8_m: *mut u8;
    let mut string_n: u16;
    let pu8_p: *mut u8;
    let mut b_q: bool;
    let mut win_version_r: u32;
    let mut u32_s: u32;
    let mut u32_t: u32;
    let mut i32_v: i32;
    let mut fn_ptr_w: u32;
    let mut string_x: u32;

    u32_s = CONCAT22(param_5, context.g_u16_ptr_1050_5f84);
    loop {
        InitTask16(0);
        g_u16_ptr_1050_5f84 = u32_s;
        if ((param_2 != 0)
            && (
                b_q = param_4 < 0xff00,
                param_4 = param_4 + 0x100,
                context.PTR_LOOP_1050_5f7e = pu8_p,
                b_q,
            ))
        {
            context.PTR_LOOP_1050_5f48 = param_4;
            context.PTR_LOOP_1050_5f4a = pu8_j;
            context.g_h_instance = pu8_m;
            contextPTR_LOOP_1050_5f4e = param_3;
            context.PTR_LOOP_1050_5f50 = pu8_p;
            LockSegment16(0xffff);
            PTR_LOOP_1050_5f52 = (u32_s >> 0x10);
            g_u16_ptr_1050_5f84 = u32_s;
            win_version_r._0_2_ = GetVersion16();
            PTR_LOOP_1050_5f52 = (u32_s >> 0x10);
            g_u16_ptr_1050_5f84 = u32_s;
            PTR_LOOP_1050_5f80 = CONCAT11(win_version_r, (win_version_r >> 8));
            fn_ptr_d = swi(0x21);
            u32_t = u32_s;
            u32_s = (*fn_ptr_d)();
            PTR_LOOP_1050_5f52 = (u32_t >> 0x10);
            g_u16_ptr_1050_5f84 = u32_t;
            _u8_1050_5f82 = CONCAT11(u32_s, (u32_s >> 8));
            u8_1050_5f87 = '\0';
            WaitEvent16(0);
            g_u16_ptr_1050_5f84 = u32_s;
            u16_c = InitApp16(g_h_instance);
            g_u16_ptr_1050_5f84 = u32_s;
            if (u16_c != 0) {
                break;
            }
        }
        param_2 = call_fn_ptr_1000_24db();
        g_u16_ptr_1050_5f84 = u32_s;
    }
    dos3_call_1000_23ea(u16_c, param_3);
    get_module_file_name_1000_262c();
    get_dos_env_1000_27d6();
    empty_fn_1000_55ac();
    // fn_ptr_d = func_0x100023be(offset);
    call_fn_ptr_1000_24cd(fn_ptr_d);
    pass1_fn_1000_25a8();
    pass1_fn_1000_2913();
    string_e = process_string_1000_28dc((s_version__d__d_1050_0012 + 3));
    if (string_e != 0) {
        offset_f = 9;
        if (*string_e == 'M') {
            offset_f = 0xf;
        }
        string_e = string_e + offset_f;
        i32_h = 0x22;
        string_n = string_e;
        while {
            if i32_h == 0 {
                break;
            }
            i32_h = i32_h - 1;
            string_x = string_n;
            string_n = string_n + 1;
            *string_x != '\r'
        } {}
        *(string_n - 1) = 0;
    }
    // TODO: make sure we're referencing
    // param_1 = CONCAT22(0x1050, string_1);
    param_1 = string_e;
    FatalAppExit16(param_1, 0);
    FatalExit(0xff);
    pc_k = (s___NMSG___1050_63f6 + 8);
    loop {
        pi32_a = pc_k;
        pc_k = (pc_k + 2);
        let var_name = unsafe { *pi32_a };
        i32_h = var_name;
        pi32_g = pc_k;
        if ((i32_h == i32_v) || (pi32_g = (i32_h + 1), pi32_g == 0x0)) {
            return pi32_g;
        }
        i32_h = -1;
        while {
            if (i32_h == 0) {
                break;
            }

            i32_h = i32_h + -1;
            pi32_a = pc_k;
            pc_k = (pc_k + 1);
            let val = unsafe { *pi32_a };
            val != '\0'
        } {}
    }
}

fn CONCAT11(a: u8, b: u8) -> u16 {
    a << 8 | b
}

// WARNING: Removing unreachable block (ram,0x10002400)
// WARNING: Removing unreachable block (ram,0x10002422)

// WARNING: Removing unreachable block (ram,0x10002513)
// WARNING: Removing unreachable block (ram,0x10002557)

pub unsafe fn exit_1000_25cc() -> *mut i32 {
    let piVar1: *mut i32;
    let pc_var2: *mut libc::c_char;
    let i_var3: i32;
    let piVar4: *mut i32;
    let pi_var5: *mut i32;
    let pcVar6: *mut libc::c_char;
    let mut in_stack_00000008: i32;
    let pcVar7: *mut libc::c_char;

    pcVar7 = &dos_alloc_addr_1050_0002;
    pass1_fn_1000_25a8();
    pass1_fn_1000_2913();
    pcVar7 = process_string_1000_28dc(pcVar7);
    if (pcVar7 != 0x0) {
        i_var3 = 9;
        let val = unsafe { *pcVar7 };
        if (val == 'M') {
            i_var3 = 0xf;
        }
        pcVar7 = pcVar7 + i_var3;
        i_var3 = 0x22;
        pcVar6 = pcVar7;
        while {
            if i_var3 == 0 {
                break;
            }
            i_var3 = i_var3 + -1;
            pc_var2 = pcVar6;
            pcVar6 = pcVar6 + 1;
            let val = unsafe { *pc_var2 };
            val != '\r'
        } {}
        pcVar6[-1] = '\0';
    }
    // FatalAppExit16(CONCAT22(0x1050, pcVar7), 0);
    FatalAppExit16(pcVar7, 0);
    FatalExit(0xff);
    pi_var5 = (s___NMSG___1050_63f6 + 8);
    loop {
        piVar1 = pi_var5;
        pi_var5 = pi_var5 + 1;
        let val = unsafe { *piVar1 };
        i_var3 = val;
        piVar4 = pi_var5;
        if ((i_var3 == in_stack_00000008) || (piVar4 = (i_var3 + 1), piVar4 == 0x0)) {
            return piVar4;
        }
        i_var3 = -1;
        while {
            if (i_var3 == 0) {
                break;
            }
            i_var3 = i_var3 + -1;
            piVar1 = pi_var5;
            pi_var5 = (pi_var5 + 1);
            let val = unsafe { *piVar1 };
            val != '\0'
        } {}
    }
}

/*
Unable to decompile 'bad_1000_25f2'
// Cause:
Low-level Error: Overlapping input varnodes
*/

// WARNING (jumptable): Unable to track spacebase fully for stack
// WARNING (jumptable): Heritage AFTER dead removal. Example location: r0x10505fc2 : 0x1000270c

// WARNING: Restarted to delay deadcode elimination for space: ram

pub unsafe fn exit_1000_2950(param_1: i32) -> *mut i32 {
    let piVar1: *mut i32;
    let pc_var2: *mut libc::c_char;
    let pu_var3: *mut u8;
    let pcVar4: *mut libc::c_char;
    let mut i_var5: i32;
    let pi_var6: *mut i32;
    let mut in_AX: u16;
    let pi_var7: *mut i32;
    let in_CX: *mut libc::c_char;
    let mut in_dx: i32;
    let pcVar8: *mut libc::c_char;

    pu_var3 = PTR_LOOP_1050_6066;
    PTR_LOOP_1050_6066 = &PTR_LOOP_1050_1000;
    pi_var7 = alloc_mem_1000_167a(in_AX, in_dx);
    PTR_LOOP_1050_6066 = pu_var3;
    if ((in_dx | pi_var7) != 0) {
        return pi_var7;
    }
    pass1_fn_1000_25a8();
    pass1_fn_1000_2913();
    pcVar4 = process_string_1000_28dc(in_CX);
    if (pcVar4 != 0x0) {
        i_var5 = 9;
        let val = unsafe { *pcVar4 };
        if (val == 'M') {
            i_var5 = 0xf;
        }
        pcVar4 = pcVar4 + i_var5;
        i_var5 = 0x22;
        pcVar8 = pcVar4;
        while {
            if (i_var5 == 0) {
                break;
            }

            i_var5 = i_var5 + -1;
            pc_var2 = pcVar8;
            pcVar8 = pcVar8 + 1;
            let val = unsafe { *pc_var2 };
            val != '\r'
        } {}
        pcVar8[-1] = '\0';
    }
    // FatalAppExit16(CONCAT22(0x1050, pcVar4), 0);
    FatalAppExit16(pcVar4, 0);
    FatalExit(0xff);
    pi_var7 = (s___NMSG___1050_63f6 + 8);
    loop {
        piVar1 = pi_var7;
        pi_var7 = pi_var7 + 1;
        let val = unsafe { *piVar1 };
        i_var5 = val;
        pi_var6 = pi_var7;
        if ((i_var5 == param_1) || (pi_var6 = (i_var5 + 1), pi_var6 == 0x0)) {
            return pi_var6;
        }
        i_var5 = -1;
        while {
            if i_var5 == 0 {
                break;
            }
            i_var5 = i_var5 + -1;
            piVar1 = pi_var7;
            pi_var7 = (pi_var7 + 1);
            let val = unsafe { *piVar1 };
            val != '\0'
        } {}
    }
}

pub fn exported_stub_1000_29dc() -> u16 {
    let mut unaff_ss: u16;

    if (___EXPORTEDSTUB != 0xb8) {
        return unaff_ss;
    }
    return uRam100029ed;
}

pub fn ___EXPORTEDSTUB() -> u16 {
    return 0;
}

pub fn return0_1000_29ef() -> u16 {
    return 0;
}

pub fn return_1000_39e1() {
    return;
}

// WARNING: Removing unreachable block (ram,0x10003afe)
// WARNING: Removing unreachable block (ram,0x10003a40)
// WARNING: Removing unreachable block (ram,0x10003b7e)
pub fn fatal_app_exit_1000_3e9e() {
    FatalAppExit16(s_ABNORMAL_PROGRAM_TERMINATION_1050_6544, 0);
    return;
}

// WARNING: Instruction at (ram,0x10083e27) overlaps instruction at (ram,0x10083e24)
//
// WARNING: Removing unreachable block (ram,0x10083e1f)

pub fn loop_1010_11c6(param_1: &mut Struct365) {
    let pi32_1: *mut i32;
    let ppc_var2: *mut fn();
    let mut uVar3: u32;
    let mut u_var4: u32;
    let local_AX__1: *mut Struct366;
    let mut i_var5: i32;
    let local_AX_179: *mut Struct367;
    let paVar6: *mut Struct367;
    let mut u_var7: i32;
    let mut u_var8: u16;
    let in_dx: *mut Struct199;
    let mut u_var9: u16;
    let struct_a: *mut Struct199;
    let struct_a_00: *mut Struct199;
    let pa_var10: *mut Struct199;
    let pa_var11: *mut Struct199;
    let paVar12: *mut Struct199;
    let extraout_dx: *mut Struct199;
    let mut u_var13: u16;
    let extraout_dx_00: *mut Struct199;
    let mut extraout_dx_01: u16;
    let mut extraout_dx_02: u16;
    let pp_var14: *mut *mut libc::c_char;
    let mut pustruct_a: &mut Struct365;
    let mut iVar15: i32;
    let mut iVar16: i32;
    let mut uVar17: u16;
    let mut uVar18: u16;
    let pu16_19: *mut u16;
    let paVar20: *mut Struct367;
    let mut u_var21: u16;
    let mut local_36: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2c: u16;
    let mut local_2a: u16;
    let local_28: *mut Struct368;
    let mut local_26: u16;
    let mut local_24: u16;
    let mut local_22: u16;
    let mut local_20: u16;
    let mut local_1e: u16;
    let mut local_1c: u16;
    let mut local_1a: u32;
    let mut local_16: u32;
    let mut local_e: u32;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u16;
    let mut local_4: u16;

    if (u16_1050_0ecc == 0xffff) {
        return;
    }
    process_struct_1000_179c(0x1a, in_dx);
    if ((in_dx | local_AX__1) == 0) {
        i_var5 = 0;
        u_var9 = 0;
    } else {
        pu16_19 = pass1_1010_37d4(CONCAT22(in_dx, local_AX__1));
        u_var9 = (pu16_19 >> 0x10);
        i_var5 = pu16_19;
    }
    _local_a = &WORD_1050_0ece;
    local_e = 0;
    while (true) {
        uVar17 = (param_1 >> 0x10);
        pustruct_a = param_1;
        pu16_19 = &pustruct_a.field_0x68;
        let val = unsafe { *pu16_19 };
        if (val == local_e || val < local_e) {
            break;
        }
        u_var4 = pustruct_a.field_0x64;
        uVar3 = (u_var4 + local_e * 4);
        pp_var14 = (uVar3 + u16_1050_0ecc * 8);
        _local_32 = (uVar3 & 0xffff0000 | ZEXT24(pp_var14));
        iVar16 = unsafe { process_string_1000_475e(_local_a, *pp_var14) };
        if (iVar16 != 0) {
            _local_a = *_local_32;
            local_e = local_e & 0xffff | (local_e._2_2_ + 1) << 0x10;
        }
        local_e = local_e & 0xffff0000 | (local_e + 1);
    }
    (i_var5 + 0x10) = local_e._2_2_;
    process_struct_1010_38f8(CONCAT22(u_var9, i_var5), local_e._2_2_);
    local_AX_179 = 0x0;
    struct_a_00 = struct_a;
    process_struct_1000_179c(0x400, struct_a);
    pa_var10 = struct_a_00;
    paVar6 = local_AX_179;
    process_struct_1000_179c(0x400, struct_a_00);
    _local_26 = CONCAT22(pa_var10, paVar6);
    local_1c = 0;
    pass1_1000_4906(CONCAT22(struct_a_00, local_AX_179), 0, 0x400);
    pass1_1000_4906(CONCAT22(pa_var10, paVar6), 0, 0x400);
    local_2a = 0;
    local_e = 0;
    loop {
        pi32_1 = (i_var5 + 0x10);
        let val = unsafe { *pi32_1 };
        if (val == local_e || val < local_e) {
            return;
        }
        u_var4 = pustruct_a.field_0x64;
        uVar18 = (u_var4 >> 0x10);
        iVar15 = u_var4;
        iVar16 = (iVar15 + local_1c * 4);
        paVar12 = (iVar15 + local_1c * 4 + 2);
        iVar15 = iVar16 + (u16_1050_0ecc * 6 + 0xeba) * 8;
        local_16 = CONCAT22(paVar12, iVar15);
        u_var7 = iVar16 + (u16_1050_0ecc * 6 + 0xebc) * 8;
        pa_var11 = paVar12;
        process_struct_1000_179c(0x1a, paVar12);
        if ((pa_var11 | u_var7) == 0) {
            u_var4 = (i_var5 + 8);
            (u_var4 + local_e * 4) = 0;
        } else {
            pu16_19 = pass1_1010_37d4(CONCAT22(pa_var11, u_var7));
            u_var4 = (i_var5 + 8);
            uVar18 = (u_var4 >> 0x10);
            iVar16 = u_var4;
            (iVar16 + local_e * 4) = pu16_19;
            (iVar16 + local_e * 4 + 2) = (pu16_19 >> 0x10);
        }
        local_2a = local_2a + 1;
        u_var4 = (i_var5 + 8);
        uVar18 = (u_var4 >> 0x10);
        iVar16 = u_var4;
        u_var4 = (iVar16 + local_e * 4);
        ppc_var2 = ((iVar16 + local_e * 4) + 0x1c);
        ppc_var2(0x1000, u_var4, (u_var4 >> 0x10), local_2a, iVar15, paVar12);
        // local_16
        loop {
            pu16_19 = &pustruct_a.field_0x68;
            let val = unsafe { *pu16_19 };
            if (val == local_1c || val < local_1c) {
                break;
            }
            iVar15 = local_1c * 4;
            u_var4 = pustruct_a.field_0x64;
            u_var4 = (u_var4 + iVar15);
            iVar16 =
                process_string_1000_475e(*local_16, *(u_var4 + (u16_1050_0ecc * 6 + 0xeba) * 8));
            if (iVar16 != 0) {
                break;
            }
            u_var4 = pustruct_a.field_0x64;
            uVar18 = (u_var4 >> 0x10);
            iVar16 = u_var4;
            paVar12 = (iVar16 + iVar15 + 2);
            u_var7 = (iVar16 + iVar15) + (u16_1050_0ecc * 6 + 0xebc) * 8;
            local_1a = CONCAT22(paVar12, u_var7);
            process_struct_1000_179c(0x1a, paVar12);
            if ((paVar12 | u_var7) == 0) {
                uVar18 = 0;
                u_var13 = 0;
            } else {
                pu16_19 = pass1_1010_37d4(CONCAT22(paVar12, u_var7));
                u_var13 = (pu16_19 >> 0x10);
                uVar18 = SUB42(pu16_19, 0);
            }
            (local_AX_179 + local_e._2_2_ * 4) = uVar18;
            (local_AX_179 + local_e._2_2_ * 4 + 2) = u_var13;
            u_var4 = pustruct_a.field_0x64;
            uVar18 = (u_var4 >> 0x10);
            iVar16 = u_var4;
            local_2a = local_2a + 1;
            u_var4 = (local_AX_179 + local_e._2_2_ * 4);
            ppc_var2 = ((local_AX_179 + local_e._2_2_ * 4) + 0x1c);
            ppc_var2(
                0x1000,
                u_var4,
                (u_var4 >> 0x10),
                local_2a,
                (iVar16 + local_1c * 4) + (u16_1050_0ecc * 6 + 0xebc) * 8,
                (iVar16 + local_1c * 4 + 2),
            );
            local_28 = 0x0;
            paVar12 = extraout_dx;
            loop {
                pu16_19 = &pustruct_a.field_0x68;
                let var = unsafe { *pu16_19 };
                if (val == local_1c || val < local_1c) {
                    break;
                }
                u_var4 = pustruct_a.field_0x64;
                u_var4 = (u_var4 + local_1c * 4);
                iVar16 = process_string_1000_475e(
                    *local_1a,
                    *(u_var4 + (u16_1050_0ecc * 6 + 0xebc) * 8),
                );
                if (iVar16 != 0) {
                    break;
                }
                u_var4 = pustruct_a.field_0x64;
                u_var4 = (u_var4 + local_1c * 4);
                u_var7 = process_string_1000_475e(
                    *local_16,
                    *(u_var4 + (u16_1050_0ecc * 6 + 0xeba) * 8),
                );
                if (u_var7 != 0) {
                    break;
                }
                process_struct_1000_179c(0x1a, paVar12);
                if ((paVar12 | u_var7) == 0) {
                    uVar18 = 0;
                    u_var13 = 0;
                } else {
                    pu16_19 = pass1_1010_37d4(CONCAT22(paVar12, u_var7));
                    u_var13 = (pu16_19 >> 0x10);
                    uVar18 = SUB42(pu16_19, 0);
                }
                (paVar6 + local_28 * 4) = uVar18;
                (paVar6 + local_28 * 4 + 2) = u_var13;
                u_var4 = pustruct_a.field_0x64;
                uVar18 = (u_var4 >> 0x10);
                iVar16 = u_var4;
                local_2a = local_2a + 1;
                u_var4 = (paVar6 + local_28 * 4);
                ppc_var2 = ((paVar6 + local_28 * 4) + 0x1c);
                ppc_var2(
                    0x1000,
                    u_var4,
                    (u_var4 >> 0x10),
                    local_2a,
                    (iVar16 + local_1c * 4) + (u16_1050_0ecc * 6 + 0xebe) * 8,
                    (iVar16 + local_1c * 4 + 2),
                );
                local_1c = local_1c + 1;
                local_28 = &local_28.field_0x1;
                paVar12 = extraout_dx_00;
            }
            u_var4 = (local_AX_179 + local_e._2_2_ * 4);
            (u_var4 + 0x10) = local_28;
            u_var8 = local_28 << 2;
            paVar20 = paVar6;
            paVar12 = pa_var10;
            u_var21 = u_var8;
            process_struct_1010_38f8((local_AX_179 + local_e._2_2_ * 4), local_28);
            pass1_fn_1000_48a8(
                CONCAT22(extraout_dx_01, u_var8),
                CONCAT22(paVar12, paVar20),
                u_var21,
            );
            pass1_1000_4906(_local_26, 0, 0x400);
            local_e = local_e & 0xffff | (local_e._2_2_ + 1) << 0x10;
        }
        u_var4 = (i_var5 + 8);
        u_var4 = (u_var4 + local_e * 4);
        (u_var4 + 0x10) = local_e._2_2_;
        u_var8 = local_e._2_2_ << 2;
        u_var4 = (i_var5 + 8);
        paVar20 = local_AX_179;
        paVar12 = struct_a_00;
        u_var21 = u_var8;
        process_struct_1010_38f8((u_var4 + local_e * 4), local_e._2_2_);
        pass1_fn_1000_48a8(
            CONCAT22(extraout_dx_02, u_var8),
            CONCAT22(paVar12, paVar20),
            u_var21,
        );
        pass1_1000_4906(CONCAT22(struct_a_00, local_AX_179), 0, 0x400);
        local_e = (local_e + 1);
    }
}

/*
Unable to decompile 'window_msg_func_1010_7300'
// Cause: Exception while decompiling 1010:7300: The pipe is being closed

*/

pub fn mixed_fn_1010_830a(param_1: u32, param_2: u16) -> u32 {
    let mut u_var1: u32;
    let local_bx_20: *mut Struct449;
    let mut i_var2: i32;
    let mut unaff_ss: u16;
    let in_struct_a: *mut Struct103;
    let mut u_var3: u16;
    let mut local_32: u16;
    let mut local_30: u16;
    let mut local_2e: u16;
    let mut local_2c: u16;
    let mut local_a: u16;
    let mut local_8: u16;
    let mut local_6: u32;

    local_6 = 0;
    local_bx_20 = (param_2 * 0x10);
    uVar3 = (param_1 >> 0x10);
    if (local_bx_20.field_0x10 == 1) {
        u_var1 = &local_bx_20.field_0x12;
        _local_a = set_error_mode_1010_8b14(param_1, u_var1, (u_var1 >> 0x10));
        if ((local_bx_20.field_0x12 == _local_a) && (local_bx_20.field_0x14 == (_local_a >> 0x10)))
        {
            msg_box_1010_8bb4(param_1, _local_a);
            return 0;
        }
        in_struct_a = process_struct_1008_48fe(CONCAT22(unaff_ss, &local_2e), 1, _local_a);
        process_struct_1000_179c(0x1e, (in_struct_a >> 0x10));
        if (in_struct_a == 0x0) {
            local_6 = 0;
        } else {
            local_6 = pass1_1008_3f92(in_struct_a, CONCAT22(unaff_ss, &local_2e));
        }
        close_file_1008_496c(&local_2e);
        _local_2e = local_6;
    } else {
        if ((param_2 * 0x10 + 0x10) == 2) {
            _local_2e = pass1_1010_878c(param_1, (param_2 * 0x10 + 0x16));
            if ((param_1 + 0x67c) == 0) {
                return 0;
            }
            i_var2 = param_2 * 0x10;
            pass1_1008_6562(
                (param_1 + 0x67c),
                CONCAT22((i_var2 + 0x1c), (i_var2 + 0x1e)),
                (i_var2 + 0x1a),
                _local_2e,
                (_local_2e >> 0x10),
            );
        } else {
            i_var2 = param_2 * 0x10;
            if ((i_var2 + 0x10) == 3) {
                u_var1 = (i_var2 + 0x12);
                _local_2e = set_error_mode_1010_8b14(param_1, u_var1, (u_var1 >> 0x10));
                if (((i_var2 + 0x12) == _local_2e) && ((i_var2 + 0x14) == (_local_2e >> 0x10))) {
                    msg_box_1010_8bb4(param_1, _local_2e);
                    _local_2e = _local_2e;
                }
            } else {
                _local_2e = local_6;
                if ((param_2 * 0x10 + 0x10) == 4) {
                    u_var1 = (param_2 * 0x10 + 0x12);
                    _local_2e = set_error_mode_1010_8b14(param_1, u_var1, (u_var1 >> 0x10));
                }
            }
        }
    }
    local_6 = _local_2e;
    return local_6;
}

/*
Unable to decompile 'win_fn_1018_6086'
// Cause:
Low-level Error: Symbol $$undef00000006 extends beyond the end of the address space
*/

// WARNI

/*
Unable to decompile 'load_accelerators_1020_41c8'
// Cause:
Low-level Error: Symbol $$undef0000000c extends beyond the end of the address space
*/

// WARNING: Instruction at (ram,0x10207c29) overlaps instruction at (ram,0x10207c28)
//

pub fn infinite_loop_1020_7bba() {
    let pc_var1: *mut libc::c_char;
    let pb_var2: *mut u8;
    let paVar3: *mut Struct676;
    let pu_var4: *mut u16;
    let mut cVar5: u8;
    let mut bVar6: u8;
    let mut in_AL: u8;
    let mut bVar7: u8;
    let mut bVar8: u8;
    let mut paVar9: *mut Struct676;
    let mut iVar10: u16;
    let in_CX: *mut Struct677;
    let mut u_var11: i32;
    let local_CX_56: *mut Struct677;
    let mut in_dx: i32;
    let mut u_var12: i32;
    let mut in_BX: i32;
    let local_bx_59: *mut Struct678;
    let local_bx_110: *mut Struct679;
    let ppu_var13: *mut *mut u16;
    let unaff_BP: *mut *mut u16;
    let unaff_si: *mut Struct676;
    let local_SI_28: *mut Struct676;
    let paVar14: *mut Struct676;
    let unaff_DI: *mut Struct676;
    let paVar15: *mut Struct676;
    let mut unaff_es: u16;
    let mut uVar16: u16;
    let mut uVar17: u16;
    let mut unaff_ss: u16;
    let mut bVar18: bool;
    let mut in_af: u8;
    let mut in_TF: u8;
    let mut in_IF: u8;
    let mut in_NT: u8;
    let mut in_stack_00000034: u16;
    let mut cStack0035: u8;
    let mut uStack2269: u16;
    let mut uStack2257: i32;
    let ppuStack2255: *mut *mut u16;
    let paStack2251: *mut Struct676;
    let puStack2249: *mut u16;
    let mut uStack2245: u32;
    let mut uStack2241: i32;
    let paStack2239: *mut Struct677;
    let paStack2237: *mut Struct676;
    let apaStack2235: *mut Struct676;
    let ppuStack34: *mut *mut u16;
    let puStack2: *mut u16;

    loop {
        ppuStack34 = &puStack2;
        ppu_var13 = &puStack2;
        ppuStack2255 = &puStack2;
        cVar5 = '\x0f';
        pu_var4 = unaff_BP;
        while {
            pu_var4 = pu_var4 + -1;
            ppu_var13 = ppu_var13 + -1;
            unsafe { *ppu_var13 = *pu_var4 };
            cVar5 = cVar5 + -1;
            '\0' < cVar5
        } {}
        local_SI_28 = in_AL;
        pb_var2 = (&local_SI_28.field_0x0 + in_BX);
        bVar8 = in_dx;
        unsafe { *pb_var2 = *pb_var2 | bVar8 };
        let val = unsafe { *pb_var2 };
        if ((POPCOUNT(val) & 1) == 0) {
            bVar18 = CARRY1(bVar8, bVar8);
            u_var12 = in_dx & 0xff00 | (bVar8 * 0x2);
            paVar15 = unaff_DI;
            paVar9 = unaff_si;
            if ((POPCOUNT(bVar8 * 0x2) & 1) != 0) {
                return code_r0x10207be9();
            }
        } else {
            pb_var2 = &local_SI_28.field_0x37;
            unsafe { *pb_var2 = *pb_var2 + bVar8 };
            pb_var2 = (&local_SI_28.field_0x0 + in_BX);
            unsafe { *pb_var2 = *pb_var2 | bVar8 };
            uStack2245 = CONCAT22(in_BX, apaStack2235);
            puStack2249 = CONCAT22(&puStack2, local_SI_28);
            let val = unsafe { *pb_var2 };
            paVar14 = ((in_NT & 1) * 0x4000
                | (in_IF & 1) * 0x200
                | (in_TF & 1) * 0x100
                | (val < '\0') * 0x80
                | (val == 0) * 0x40
                | (in_af & 1) * 0x10
                | ((POPCOUNT(val) & 1) == 0) * 4);
            pb_var2 = (&local_SI_28.field_0x0 + in_BX);
            unsafe { *pb_var2 = *pb_var2 | bVar8 };
            paVar3 = unaff_DI;
            paVar15 = &unaff_DI.field_0x1;
            bVar7 = unaff_si;
            bVar18 = bVar7 < unsafe { *paVar3 };
            uStack2241 = in_dx;
            paStack2239 = in_CX;
            paStack2237 = unaff_si;
            let val = unsafe { *paVar3 };
            if ((bVar7 - val) < '\0') {
                cStack0035 = cStack0035 + in_BX + bVar18;
                pb_var2 = (&local_SI_28.field_0x0 + in_BX);
                unsafe { *pb_var2 = *pb_var2 | bVar8 };
                local_SI_28 = &local_SI_28[-1].field_0x37;
                pb_var2 = (&local_SI_28.field_0x0 + in_BX);
                unsafe { *pb_var2 = *pb_var2 | bVar8 };
                pb_var2 = (&local_SI_28.field_0x0 + in_BX);
                unsafe { *pb_var2 = *pb_var2 | bVar8 };
                u_var12 = in_dx - 1;
                uStack2257 = (in_NT & 1) * 0x4000
                    | SBORROW2(in_dx, 1) * 0x800
                    | (in_IF & 1) * 0x200
                    | (in_TF & 1) * 0x100
                    | (u_var12 < 0) * 0x80
                    | (u_var12 == 0) * 0x40
                    | (in_af & 1) * 0x10
                    | ((POPCOUNT(u_var12 & 0xff) & 1) == 0) * 4;
                pb_var2 = (&local_SI_28.field_0x0 + in_BX);
                unsafe { *pb_var2 = *pb_var2 | u_var12 };
                in_af = 9 < (bVar7 & 0xf) | in_af;
                pb_var2 = (&local_SI_28.field_0x0 + in_BX);
                unsafe { *pb_var2 = *pb_var2 | u_var12 };
                pc_var1 = &stack0x001e + local_SI_28;
                bVar8 = in_CX & 0x1f;
                unsafe { cVar5 = *pc_var1 };
                unsafe { *pc_var1 = *pc_var1 >> bVar8 };
                bVar18 = (in_CX & 0x1f) != 0 && (cVar5 >> bVar8 - 1 & 1) != 0;
                // code_r0x10207be9:
                pc_var1 = &stack0x0068 + local_SI_28;
                unsafe { *pc_var1 = *pc_var1 + in_CX + bVar18 };
                pb_var2 = (&local_SI_28.field_0x0 + in_BX);
                unsafe { *pb_var2 = *pb_var2 | u_var12 };
                u_var11 = (&local_SI_28.field_0x0 + in_BX) * 0x10;
                pb_var2 = &stack0x0006 + paVar15;
                local_CX_56._1_1_ = (u_var11 >> 8);
                unsafe { local_CX_56._1_1_ = local_CX_56._1_1_ + *pb_var2 };
                local_CX_56 = (u_var11 & 0xff | local_CX_56._1_1_ << 8);
                pc_var1 = &stack0x0000 + &local_SI_28.field_0x35;
                local_bx_59._1_1_ = (in_BX >> 8);
                unsafe {
                    *pc_var1 = *pc_var1 + local_bx_59._1_1_ + CARRY1(local_CX_56._1_1_, *pb_var2)
                };
                in_dx = u_var12;
            } else {
                pc_var1 = ((register0x00000010 + -2) + local_SI_28);
                unsafe { *pc_var1 = *pc_var1 + bVar7 + bVar18 };
                local_CX_56 = ((&local_SI_28.field_0x0 + in_BX) * 0x10);
                paStack2251 = unaff_DI;
                let val = unsafe { *pc_var1 };
                if ((POPCOUNT(val) & 1) == 0) {
                    // code_r0x10207c14:
                    pb_var2 = (&paVar14.field_0x0 + in_BX);
                    unsafe { *pb_var2 = *pb_var2 | in_dx };
                    pb_var2 = (&paVar14.field_0x0 + in_BX);
                    unsafe { *pb_var2 = *pb_var2 & in_dx };
                    uVar16 = (uStack2245 >> 0x10);
                    uStack2269 = (uStack2245 + 8);
                    uVar17 = (puStack2249 >> 0x10);
                    local_bx_110 = puStack2249;
                    unsafe { *puStack2249 = s_1_1050_389a };
                    local_bx_110.field_0x2 = &PTR_LOOP_1050_1008;
                    unsafe { *puStack2249 = (s_18_2_1050_3aa5 + 3) };
                    local_bx_110.field_0x2 = &PTR_LOOP_1050_1008;
                    local_bx_110.field_0x4 = uStack2269;
                    unsafe { *puStack2249 = s_0_020_1050_3ab0 };
                    local_bx_110.field_0x2 = &PTR_LOOP_1050_1008;
                    local_bx_110.field_0x6 = uStack2245;
                    local_bx_110.field_0xa = 0;
                    local_bx_110.field_0xe = 0;
                    local_bx_110.field_0x10 = 0;
                    local_bx_110.field_0x12 = 0;
                    unsafe { *puStack2249 = 0x7f72 };
                    local_bx_110.field_0x2 = 0x1020;
                    local_bx_110.field_0xa = (uStack2245 + 0xe4);
                    apaStack2235[0] = unaff_si;
                    puStack2 = unaff_BP;
                    iVar10 = GetSystemMetrics16(4);
                    (puStack2249 + 0xe) = iVar10;
                    iVar10 = GetSystemMetrics16(5);
                    (puStack2249 + 0x10) = iVar10;
                    iVar10 = GetSystemMetrics16(6);
                    (puStack2249 + 0x12) = iVar10;
                    return;
                }
            }
            pb_var2 = (&local_SI_28.field_0x0 + in_BX);
            bVar7 = in_dx;
            unsafe { *pb_var2 = *pb_var2 | bVar7 };
            let val = unsafe { *pb_var2 };
            puStack2249 = CONCAT22(
                in_dx,
                (in_NT & 1) * 0x4000
                    | (in_IF & 1) * 0x200
                    | (in_TF & 1) * 0x100
                    | (val < '\0') * 0x80
                    | (val == 0) * 0x40
                    | (in_af & 1) * 0x10
                    | ((POPCOUNT(val) & 1) == 0) * 4,
            );
            pb_var2 = (&local_SI_28.field_0x0 + in_BX);

            unsafe { *pb_var2 = *pb_var2 | bVar7 };
            let val = unsafe { *pb_var2 };
            paVar14 = ((in_NT & 1) * 0x4000
                | (in_IF & 1) * 0x200
                | (in_TF & 1) * 0x100
                | (val < '\0') * 0x80
                | (val == 0) * 0x40
                | (in_af & 1) * 0x10
                | ((POPCOUNT(val) & 1) == 0) * 4);
            pb_var2 = (&local_SI_28.field_0x0 + in_BX);
            unsafe { *pb_var2 = *pb_var2 | bVar7 };
            bVar8 = (0x79);
            pb_var2 = (&local_SI_28.field_0x0 + in_BX);
            unsafe { *pb_var2 = *pb_var2 & bVar7 };
            in_CX = local_CX_56;
            u_var12 = in_dx;
            unaff_DI = paVar15;
            paVar9 = bVar8;
            paStack2251 = local_SI_28;
            let val = unsafe { *pb_var2 };
            if (-1 < val) {
                // goto code_r0x10207c14;
            }
        }
        paVar14 = local_SI_28;
        pb_var2 = (&paVar14.field_0x0 + in_BX);
        bVar8 = u_var12;
        unsafe { *pb_var2 = *pb_var2 | bVar8 };
        in_CX = in_CX + -1;
        in_dx = u_var12;
        let val = unsafe { *pb_var2 };
        if (in_CX == 0x0 || val == 0) {
            // goto code_r0x10207c14;
        }
        pb_var2 = (&paVar14.field_0x0 + in_BX);
        unsafe { *pb_var2 = *pb_var2 | bVar8 };
        bVar6 = paVar9 - 1;
        bVar7 = 9 < (bVar6 & 0xf) | in_af;
        bVar6 = bVar6 + bVar7 * '\x06' & 0xf;
        pb_var2 = (&paVar14.field_0x0 + in_BX);
        unsafe { *pb_var2 = *pb_var2 | bVar8 };
        in_af = 9 < bVar6 | bVar7;
        in_AL = bVar6 + in_af * '\x06' & 0xf;
        pb_var2 = (&paVar14.field_0x0 + in_BX);
        unsafe { *pb_var2 = *pb_var2 | bVar8 };
        unaff_BP = &puStack2;
        unaff_si = paVar14;
    }
}

pub fn init_globals_1020_9826() {
    let pu_var1: *mut u16;
    let mut c_var2: u8;
    let pcVar3: *mut code;
    let mut in_eax: u32;
    let mut i_var4: i32;
    let i32_k: *mut u8;
    let unaff_BP: *mut u16;
    let unaff_si: *mut u8;
    let unaff_DI: *mut u8;
    let pu_var5: *mut u16;
    let mut unaff_ss: u16;
    let mut in_OF: u8;

    pu_var5 = &stack0xfffe;
    c_var2 = 0x2;
    while {
        unaff_BP = unaff_BP + -1;
        pu_var5 = pu_var5 + -1;
        unsafe { *pu_var5 = *unaff_BP };
        c_var2 = c_var2 + -1;
        '\0' < c_var2
    } {}
    pcVar3 = swi(4);
    if (in_OF == 0x1) {
        unsafe { (*pcVar3)() };
    }
    PTR_LOOP_1050_45e2 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_45e8 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4600 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4606 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_451e = 0xffff;
    PTR_LOOP_1050_45ae = 0xffff;
    PTR_LOOP_1050_45b4 = 0xffff;
    PTR_LOOP_1050_45ba = 0xffff;
    PTR_LOOP_1050_45cc = 0xffff;
    PTR_LOOP_1050_45d2 = 0xffff;
    PTR_LOOP_1050_45f6 = 0xffff;
    PTR_LOOP_1050_45fc = 0xffff;
    PTR_LOOP_1050_460e = 0xffff;
    PTR_LOOP_1050_4614 = 0xffff;
    _PTR_LOOP_1050_4616 = 0;
    _PTR_LOOP_1050_461c = 0;
    _PTR_LOOP_1050_4622 = 0;
    _PTR_LOOP_1050_4628 = 0;
    _PTR_LOOP_1050_462e = 0;
    _PTR_LOOP_1050_4634 = 0;
    PTR_LOOP_1050_4518 = 0x0;
    PTR_LOOP_1050_453c = 0x0;
    PTR_LOOP_1050_4542 = 0x0;
    PTR_LOOP_1050_456c = 0x0;
    PTR_LOOP_1050_45d8 = 0x0;
    PTR_LOOP_1050_45de = 0x0;
    PTR_LOOP_1050_45f0 = 0x0;
    PTR_LOOP_1050_461a = 0x0;
    PTR_LOOP_1050_4620 = 0x0;
    PTR_LOOP_1050_4626 = 0x0;
    PTR_LOOP_1050_462c = 0x0;
    PTR_LOOP_1050_4632 = 0x0;
    PTR_LOOP_1050_4638 = 0x0;
    _PTR_LOOP_1050_463a = 0;
    _PTR_LOOP_1050_4640 = 0;
    _PTR_LOOP_1050_4646 = 0;
    _PTR_LOOP_1050_464c = 0;
    _PTR_LOOP_1050_4652 = 0;
    _PTR_LOOP_1050_4658 = 0;
    PTR_LOOP_1050_465e = s_f_1050_4448;
    PTR_LOOP_1050_4660 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4664 = s_f_1050_4448;
    PTR_LOOP_1050_4666 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4662 = u16_1050_4452;
    PTR_LOOP_1050_4668 = u16_1050_4452;
    PTR_LOOP_1050_466a = s_f_1050_4448;
    PTR_LOOP_1050_466c = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_466e = u16_1050_4452;
    PTR_LOOP_1050_4672 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4678 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4674 = u16_1050_4462;
    PTR_LOOP_1050_467a = u16_1050_4462;
    PTR_LOOP_1050_467e = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4680 = u16_1050_4462;
    PTR_LOOP_1050_4684 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4686 = u16_1050_4462;
    PTR_LOOP_1050_468a = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_468c = u16_1050_4462;
    PTR_LOOP_1050_468e = s_f_1050_4448;
    PTR_LOOP_1050_4690 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4692 = u16_1050_4452;
    PTR_LOOP_1050_4694 = s_f_1050_4448;
    PTR_LOOP_1050_4696 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4698 = u16_1050_4452;
    PTR_LOOP_1050_469a = s_f_1050_4448;
    PTR_LOOP_1050_469c = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_469e = u16_1050_4452;
    PTR_LOOP_1050_46a0 = s_f_1050_4448;
    PTR_LOOP_1050_46a2 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_46a4 = u16_1050_4452;
    PTR_LOOP_1050_46a8 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_46aa = u16_1050_4462;
    PTR_LOOP_1050_46ae = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_46b0 = u16_1050_4462;
    PTR_LOOP_1050_46b4 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_46b6 = u16_1050_4462;
    PTR_LOOP_1050_46ba = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_46bc = u16_1050_4462;
    PTR_LOOP_1050_46c0 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_46c2 = u16_1050_4462;
    PTR_LOOP_1050_46c6 = 0x0;
    PTR_LOOP_1050_46c4 = 0x0;
    PTR_LOOP_1050_46cc = 0x0;
    PTR_LOOP_1050_46ca = 0x0;
    PTR_LOOP_1050_46d2 = 0x0;
    PTR_LOOP_1050_46d0 = 0x0;
    PTR_LOOP_1050_46d8 = 0x0;
    PTR_LOOP_1050_46d6 = 0x0;
    PTR_LOOP_1050_46de = 0x0;
    PTR_LOOP_1050_46dc = 0x0;
    PTR_LOOP_1050_46e4 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_46e6 = u16_1050_4462;
    PTR_LOOP_1050_46e8 = s_f_1050_4448;
    PTR_LOOP_1050_46ea = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_46ec = u16_1050_4452;
    PTR_LOOP_1050_46ee = s_f_1050_4448;
    PTR_LOOP_1050_46f0 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_46f2 = u16_1050_4452;
    _PTR_LOOP_1050_46f4 = 0;
    _PTR_LOOP_1050_46fa = 0;
    PTR_LOOP_1050_46f8 = 0xffff;
    PTR_LOOP_1050_46fe = 0xffff;
    _PTR_LOOP_1050_4700 = 0;
    _PTR_LOOP_1050_4706 = 0;
    PTR_LOOP_1050_470c = s_f_1050_4448;
    PTR_LOOP_1050_470e = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4710 = u16_1050_4452;
    PTR_LOOP_1050_4712 = s_f_1050_4448;
    PTR_LOOP_1050_4714 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4716 = u16_1050_4452;
    _PTR_LOOP_1050_4718 = 0;
    _PTR_LOOP_1050_471e = 0;
    _PTR_LOOP_1050_4724 = 0;
    _PTR_LOOP_1050_472a = 0;
    _PTR_LOOP_1050_4730 = 0;
    _PTR_LOOP_1050_4736 = 0;
    _PTR_LOOP_1050_473c = 0;
    _PTR_LOOP_1050_4742 = 0;
    _PTR_LOOP_1050_4748 = 0;
    _PTR_LOOP_1050_474e = 0;
    _PTR_LOOP_1050_4754 = 0;
    _PTR_LOOP_1050_475a = 0;
    _PTR_LOOP_1050_4760 = 0;
    PTR_LOOP_1050_463e = 0x0;
    PTR_LOOP_1050_4644 = 0x0;
    PTR_LOOP_1050_464a = 0x0;
    PTR_LOOP_1050_4650 = 0x0;
    PTR_LOOP_1050_4656 = 0x0;
    PTR_LOOP_1050_465c = 0x0;
    PTR_LOOP_1050_46c8 = 0x0;
    PTR_LOOP_1050_46ce = 0x0;
    PTR_LOOP_1050_46d4 = 0x0;
    PTR_LOOP_1050_46da = 0x0;
    PTR_LOOP_1050_46e0 = 0x0;
    PTR_LOOP_1050_4704 = 0x0;
    PTR_LOOP_1050_470a = 0x0;
    PTR_LOOP_1050_471c = 0x0;
    PTR_LOOP_1050_4722 = 0x0;
    PTR_LOOP_1050_4728 = 0x0;
    PTR_LOOP_1050_472e = 0x0;
    PTR_LOOP_1050_4734 = 0x0;
    PTR_LOOP_1050_473a = 0x0;
    PTR_LOOP_1050_4740 = 0x0;
    PTR_LOOP_1050_4746 = 0x0;
    PTR_LOOP_1050_474c = 0x0;
    PTR_LOOP_1050_4752 = 0x0;
    PTR_LOOP_1050_4758 = 0x0;
    PTR_LOOP_1050_475e = 0x0;
    PTR_LOOP_1050_4764 = 0x0;
    _PTR_LOOP_1050_4766 = 0;
    _PTR_LOOP_1050_476c = 0;
    _PTR_LOOP_1050_4772 = 0;
    _PTR_LOOP_1050_4778 = 0;
    _PTR_LOOP_1050_477e = 0;
    _PTR_LOOP_1050_4784 = 0;
    _PTR_LOOP_1050_478a = 0;
    _PTR_LOOP_1050_4790 = 0;
    _PTR_LOOP_1050_4796 = 0;
    _PTR_LOOP_1050_479c = 0;
    _PTR_LOOP_1050_47a2 = 0;
    _PTR_LOOP_1050_47a8 = 0;
    _PTR_LOOP_1050_47ae = 0;
    _PTR_LOOP_1050_47b4 = 0;
    PTR_LOOP_1050_476a = 0x0;
    PTR_LOOP_1050_4770 = 0x0;
    PTR_LOOP_1050_4776 = 0x0;
    PTR_LOOP_1050_477c = 0x0;
    PTR_LOOP_1050_4782 = 0x0;
    PTR_LOOP_1050_4788 = 0x0;
    PTR_LOOP_1050_478e = 0x0;
    PTR_LOOP_1050_4794 = 0x0;
    PTR_LOOP_1050_479a = 0x0;
    PTR_LOOP_1050_47a0 = 0x0;
    PTR_LOOP_1050_47a6 = 0x0;
    PTR_LOOP_1050_47ac = 0x0;
    PTR_LOOP_1050_47b2 = 0x0;
    PTR_LOOP_1050_47b8 = 0x0;
    i_var4 = 0x1b;
    pu_var5 = 0x47ba;
    _PTR_LOOP_1050_45d4 = in_eax;
    _PTR_LOOP_1050_45da = in_eax;
    PTR_LOOP_1050_45e0 = unaff_si;
    PTR_LOOP_1050_45e4 = unaff_DI;
    PTR_LOOP_1050_45e6 = unaff_si;
    PTR_LOOP_1050_45ea = unaff_DI;
    _PTR_LOOP_1050_45ec = in_eax;
    _PTR_LOOP_1050_45f2 = in_eax;
    _PTR_LOOP_1050_45f8 = in_eax;
    PTR_LOOP_1050_45fe = unaff_si;
    PTR_LOOP_1050_4602 = unaff_DI;
    PTR_LOOP_1050_4604 = unaff_si;
    PTR_LOOP_1050_4608 = unaff_DI;
    _PTR_LOOP_1050_460a = in_eax;
    _PTR_LOOP_1050_4610 = in_eax;
    PTR_LOOP_1050_4670 = i32_k;
    PTR_LOOP_1050_4676 = i32_k;
    PTR_LOOP_1050_467c = i32_k;
    PTR_LOOP_1050_4682 = i32_k;
    PTR_LOOP_1050_4688 = i32_k;
    PTR_LOOP_1050_46a6 = i32_k;
    PTR_LOOP_1050_46ac = i32_k;
    PTR_LOOP_1050_46b2 = i32_k;
    PTR_LOOP_1050_46b8 = i32_k;
    PTR_LOOP_1050_46be = i32_k;
    PTR_LOOP_1050_46e2 = i32_k;
    while (i_var4 != 0) {
        i_var4 = i_var4 + -1;
        pu_var1 = pu_var5;
        pu_var5 = pu_var5 + 1;
        unsafe { *pu_var1 = 0 };
    }
    _PTR_LOOP_1050_4850 = 0;
    _PTR_LOOP_1050_4856 = 0;
    PTR_LOOP_1050_484e = PTR_LOOP_1050_4468;
    PTR_LOOP_1050_4860 = PTR_LOOP_1050_4468;
    PTR_LOOP_1050_485c = 0x4464;
    PTR_LOOP_1050_485e = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4862 = 0x4464;
    PTR_LOOP_1050_4864 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4866 = PTR_LOOP_1050_4468;
    PTR_LOOP_1050_4868 = 0x4464;
    PTR_LOOP_1050_486a = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_486c = PTR_LOOP_1050_4468;
    PTR_LOOP_1050_486e = 0x4464;
    PTR_LOOP_1050_4870 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4872 = PTR_LOOP_1050_4468;
    _PTR_LOOP_1050_4874 = 0;
    _PTR_LOOP_1050_487a = 0;
    PTR_LOOP_1050_4880 = s__1050_4436;
    PTR_LOOP_1050_4882 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4886 = s__1050_4436;
    PTR_LOOP_1050_4888 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4884 = PTR_LOOP_1050_443a;
    PTR_LOOP_1050_488a = PTR_LOOP_1050_443a;
    PTR_LOOP_1050_488c = s__1050_4436;
    PTR_LOOP_1050_488e = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4890 = PTR_LOOP_1050_443a;
    PTR_LOOP_1050_4892 = 0x4482;
    PTR_LOOP_1050_4894 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4898 = 0x4482;
    PTR_LOOP_1050_489a = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4896 = PTR_LOOP_1050_4486;
    PTR_LOOP_1050_489c = PTR_LOOP_1050_4486;
    PTR_LOOP_1050_489e = 0x4482;
    PTR_LOOP_1050_48a0 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48a2 = PTR_LOOP_1050_4486;
    PTR_LOOP_1050_48a6 = 0x0;
    PTR_LOOP_1050_48a4 = 0x0;
    PTR_LOOP_1050_48aa = 0x4488;
    PTR_LOOP_1050_48ac = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48b0 = 0x4488;
    PTR_LOOP_1050_48b2 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48ae = PTR_LOOP_1050_448c;
    PTR_LOOP_1050_48b4 = PTR_LOOP_1050_448c;
    PTR_LOOP_1050_48b6 = 0x4488;
    PTR_LOOP_1050_48b8 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48ba = PTR_LOOP_1050_448c;
    PTR_LOOP_1050_48bc = 0x446a;
    PTR_LOOP_1050_48be = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48c2 = 0x446a;
    PTR_LOOP_1050_48c4 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48c0 = PTR_LOOP_1050_446e;
    PTR_LOOP_1050_48c6 = PTR_LOOP_1050_446e;
    PTR_LOOP_1050_48c8 = 0x446a;
    PTR_LOOP_1050_48ca = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48cc = PTR_LOOP_1050_446e;
    PTR_LOOP_1050_48ce = 0x447a;
    PTR_LOOP_1050_48d0 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48d4 = 0x447a;
    PTR_LOOP_1050_48d6 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48d2 = u16_1050_4480;
    PTR_LOOP_1050_48d8 = u16_1050_4480;
    PTR_LOOP_1050_48da = s__1050_4436;
    PTR_LOOP_1050_48dc = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48de = PTR_LOOP_1050_443a;
    PTR_LOOP_1050_48e0 = s__1050_4436;
    PTR_LOOP_1050_48e2 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48e4 = PTR_LOOP_1050_443a;
    PTR_LOOP_1050_48e6 = 0x447a;
    PTR_LOOP_1050_48e8 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48ea = u16_1050_4480;
    _PTR_LOOP_1050_48ec = 0;
    _PTR_LOOP_1050_48f2 = 0;
    PTR_LOOP_1050_48f8 = 0x447a;
    PTR_LOOP_1050_48fa = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_48fc = u16_1050_4480;
    PTR_LOOP_1050_48fe = 0x447a;
    PTR_LOOP_1050_4900 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4902 = u16_1050_4480;
    _PTR_LOOP_1050_4904 = 0;
    _PTR_LOOP_1050_490a = 0;
    PTR_LOOP_1050_485a = 0xffff;
    PTR_LOOP_1050_48f0 = 0xffff;
    PTR_LOOP_1050_48f6 = 0xffff;
    PTR_LOOP_1050_4908 = 0xffff;
    PTR_LOOP_1050_490e = 0xffff;
    _PTR_LOOP_1050_4910 = 0;
    _PTR_LOOP_1050_4916 = 0;
    PTR_LOOP_1050_4854 = 0x0;
    PTR_LOOP_1050_4878 = 0x0;
    PTR_LOOP_1050_487e = 0x0;
    PTR_LOOP_1050_48a8 = 0x0;
    PTR_LOOP_1050_4914 = 0x0;
    PTR_LOOP_1050_491a = 0x0;
    PTR_LOOP_1050_491c = 0x4488;
    PTR_LOOP_1050_491e = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4920 = PTR_LOOP_1050_448c;
    PTR_LOOP_1050_4922 = 0x4488;
    PTR_LOOP_1050_4924 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4926 = PTR_LOOP_1050_448c;
    _PTR_LOOP_1050_4928 = 0;
    _PTR_LOOP_1050_492e = 0;
    _PTR_LOOP_1050_4934 = 0;
    PTR_LOOP_1050_493a = 0x446a;
    PTR_LOOP_1050_493c = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4940 = 0x446a;
    PTR_LOOP_1050_4942 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_493e = PTR_LOOP_1050_446e;
    PTR_LOOP_1050_4944 = PTR_LOOP_1050_446e;
    _PTR_LOOP_1050_4946 = 0;
    _PTR_LOOP_1050_494c = 0;
    _PTR_LOOP_1050_4952 = 0;
    _PTR_LOOP_1050_4958 = 0;
    _PTR_LOOP_1050_495e = 0;
    _PTR_LOOP_1050_4964 = 0;
    _PTR_LOOP_1050_496a = 0;
    _PTR_LOOP_1050_4970 = 0;
    _PTR_LOOP_1050_4976 = 0;
    _PTR_LOOP_1050_497c = 0;
    _PTR_LOOP_1050_4982 = 0;
    _PTR_LOOP_1050_4988 = 0;
    _PTR_LOOP_1050_498e = 0;
    _PTR_LOOP_1050_4994 = 0;
    PTR_LOOP_1050_499a = s_f_1050_4448;
    PTR_LOOP_1050_499c = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49a0 = s_f_1050_4448;
    PTR_LOOP_1050_49a2 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_499e = u16_1050_4452;
    PTR_LOOP_1050_49a4 = u16_1050_4452;
    PTR_LOOP_1050_49a6 = s_f_1050_4448;
    PTR_LOOP_1050_49a8 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49aa = u16_1050_4452;
    PTR_LOOP_1050_49ac = 0x4470;
    PTR_LOOP_1050_49ae = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49b2 = 0x4470;
    PTR_LOOP_1050_49b4 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49b0 = PTR_PTR_DAT_0005_0000_1050_0004_1050_4478;
    PTR_LOOP_1050_49b6 = PTR_PTR_DAT_0005_0000_1050_0004_1050_4478;
    PTR_LOOP_1050_49b8 = 0x4470;
    PTR_LOOP_1050_49ba = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49bc = PTR_PTR_DAT_0005_0000_1050_0004_1050_4478;
    PTR_LOOP_1050_49be = 0x4470;
    PTR_LOOP_1050_49c0 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49c2 = PTR_PTR_DAT_0005_0000_1050_0004_1050_4478;
    PTR_LOOP_1050_49c4 = 0x4470;
    PTR_LOOP_1050_49c6 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49c8 = PTR_PTR_DAT_0005_0000_1050_0004_1050_4478;
    PTR_LOOP_1050_49ca = s_f_1050_4448;
    PTR_LOOP_1050_49cc = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49ce = u16_1050_4452;
    PTR_LOOP_1050_49d0 = s_f_1050_4448;
    PTR_LOOP_1050_49d2 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49d4 = u16_1050_4452;
    PTR_LOOP_1050_49d6 = s_f_1050_4448;
    PTR_LOOP_1050_49d8 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49da = u16_1050_4452;
    PTR_LOOP_1050_49dc = s_f_1050_4448;
    PTR_LOOP_1050_49de = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49e0 = u16_1050_4452;
    PTR_LOOP_1050_49e2 = 0x4482;
    PTR_LOOP_1050_49e4 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49e8 = 0x4482;
    PTR_LOOP_1050_49ea = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49e6 = PTR_LOOP_1050_4486;
    PTR_LOOP_1050_49ec = PTR_LOOP_1050_4486;
    PTR_LOOP_1050_49ee = 0x4470;
    PTR_LOOP_1050_49f0 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49f2 = PTR_PTR_DAT_0005_0000_1050_0004_1050_4478;
    PTR_LOOP_1050_49f4 = 0x4470;
    PTR_LOOP_1050_49f6 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49f8 = PTR_PTR_DAT_0005_0000_1050_0004_1050_4478;
    PTR_LOOP_1050_49fa = 0x4470;
    PTR_LOOP_1050_49fc = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_49fe = PTR_PTR_DAT_0005_0000_1050_0004_1050_4478;
    PTR_LOOP_1050_4a02 = 0x0;
    PTR_LOOP_1050_4a00 = 0x0;
    PTR_LOOP_1050_4a08 = 0x0;
    PTR_LOOP_1050_4a06 = 0x0;
    PTR_LOOP_1050_4a0e = 0x0;
    PTR_LOOP_1050_4a0c = 0x0;
    PTR_LOOP_1050_4a14 = 0x0;
    PTR_LOOP_1050_4a12 = 0x0;
    PTR_LOOP_1050_4a1a = 0x0;
    PTR_LOOP_1050_4a18 = 0x0;
    PTR_LOOP_1050_4a1e = 0x4470;
    PTR_LOOP_1050_4a20 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4a22 = PTR_PTR_DAT_0005_0000_1050_0004_1050_4478;
    PTR_LOOP_1050_4a24 = s_f_1050_4448;
    PTR_LOOP_1050_4a26 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4a28 = u16_1050_4452;
    PTR_LOOP_1050_4a2a = s_f_1050_4448;
    PTR_LOOP_1050_4a2c = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4a2e = u16_1050_4452;
    _PTR_LOOP_1050_4a30 = 0;
    _PTR_LOOP_1050_4a36 = 0;
    PTR_LOOP_1050_492c = 0xffff;
    PTR_LOOP_1050_4932 = 0xffff;
    PTR_LOOP_1050_4938 = 0xffff;
    PTR_LOOP_1050_494a = 0xffff;
    PTR_LOOP_1050_4950 = 0xffff;
    PTR_LOOP_1050_4a34 = 0xffff;
    PTR_LOOP_1050_4a3a = 0xffff;
    _PTR_LOOP_1050_4a3c = 0;
    _PTR_LOOP_1050_4a42 = 0;
    PTR_LOOP_1050_4956 = 0x0;
    PTR_LOOP_1050_495c = 0x0;
    PTR_LOOP_1050_4962 = 0x0;
    PTR_LOOP_1050_4968 = 0x0;
    PTR_LOOP_1050_496e = 0x0;
    PTR_LOOP_1050_4974 = 0x0;
    PTR_LOOP_1050_497a = 0x0;
    PTR_LOOP_1050_4980 = 0x0;
    PTR_LOOP_1050_4986 = 0x0;
    PTR_LOOP_1050_498c = 0x0;
    PTR_LOOP_1050_4992 = 0x0;
    PTR_LOOP_1050_4998 = 0x0;
    PTR_LOOP_1050_4a04 = 0x0;
    PTR_LOOP_1050_4a0a = 0x0;
    PTR_LOOP_1050_4a10 = 0x0;
    PTR_LOOP_1050_4a16 = 0x0;
    PTR_LOOP_1050_4a1c = 0x0;
    PTR_LOOP_1050_4a40 = 0x0;
    PTR_LOOP_1050_4a46 = 0x0;
    PTR_LOOP_1050_4a48 = s_f_1050_4448;
    PTR_LOOP_1050_4a4a = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4a4c = u16_1050_4452;
    PTR_LOOP_1050_4a4e = s_f_1050_4448;
    PTR_LOOP_1050_4a50 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4a52 = u16_1050_4452;
    _PTR_LOOP_1050_4a54 = 0;
    _PTR_LOOP_1050_4a5a = 0;
    _PTR_LOOP_1050_4a60 = 0;
    _PTR_LOOP_1050_4a66 = 0;
    _PTR_LOOP_1050_4a6c = 0;
    _PTR_LOOP_1050_4a72 = 0;
    _PTR_LOOP_1050_4a78 = 0;
    _PTR_LOOP_1050_4a7e = 0;
    _PTR_LOOP_1050_4a84 = 0;
    _PTR_LOOP_1050_4a8a = 0;
    _PTR_LOOP_1050_4a90 = 0;
    _PTR_LOOP_1050_4a96 = 0;
    _PTR_LOOP_1050_4a9c = 0;
    _PTR_LOOP_1050_4aa2 = 0;
    _PTR_LOOP_1050_4aa8 = 0;
    _PTR_LOOP_1050_4aae = 0;
    _PTR_LOOP_1050_4ab4 = 0;
    _PTR_LOOP_1050_4aba = 0;
    _PTR_LOOP_1050_4ac0 = 0;
    _PTR_LOOP_1050_4ac6 = 0;
    _PTR_LOOP_1050_4acc = 0;
    _PTR_LOOP_1050_4ad2 = 0;
    _PTR_LOOP_1050_4ad8 = 0;
    _PTR_LOOP_1050_4ade = 0;
    _PTR_LOOP_1050_4ae4 = 0;
    _PTR_LOOP_1050_4aea = 0;
    _PTR_LOOP_1050_4af0 = 0;
    PTR_LOOP_1050_4a58 = 0x0;
    PTR_LOOP_1050_4a5e = 0x0;
    PTR_LOOP_1050_4a64 = 0x0;
    PTR_LOOP_1050_4a6a = 0x0;
    PTR_LOOP_1050_4a70 = 0x0;
    PTR_LOOP_1050_4a76 = 0x0;
    PTR_LOOP_1050_4a7c = 0x0;
    PTR_LOOP_1050_4a82 = 0x0;
    PTR_LOOP_1050_4a88 = 0x0;
    PTR_LOOP_1050_4a8e = 0x0;
    PTR_LOOP_1050_4a94 = 0x0;
    PTR_LOOP_1050_4a9a = 0x0;
    PTR_LOOP_1050_4aa0 = 0x0;
    PTR_LOOP_1050_4aa6 = 0x0;
    PTR_LOOP_1050_4aac = 0x0;
    PTR_LOOP_1050_4ab2 = 0x0;
    PTR_LOOP_1050_4ab8 = 0x0;
    PTR_LOOP_1050_4abe = 0x0;
    PTR_LOOP_1050_4ac4 = 0x0;
    PTR_LOOP_1050_4aca = 0x0;
    PTR_LOOP_1050_4ad0 = 0x0;
    PTR_LOOP_1050_4ad6 = 0x0;
    PTR_LOOP_1050_4adc = 0x0;
    PTR_LOOP_1050_4ae2 = 0x0;
    PTR_LOOP_1050_4ae8 = 0x0;
    PTR_LOOP_1050_4aee = 0x0;
    PTR_LOOP_1050_4af4 = 0x0;
    i_var4 = 0x1b;
    pu_var5 = 0x4af6;
    while (i_var4 != 0) {
        i_var4 = i_var4 + -1;
        pu_var1 = pu_var5;
        pu_var5 = pu_var5 + 1;
        unsafe { *pu_var1 = 0 };
    }
    PTR_LOOP_1050_4b9c = PTR_LOOP_1050_4434;
    _PTR_LOOP_1050_4b9e = 0;
    _PTR_LOOP_1050_4ba4 = 0;
    _PTR_LOOP_1050_4baa = 0;
    PTR_LOOP_1050_4ba2 = 0xffff;
    PTR_LOOP_1050_4ba8 = 0xffff;
    PTR_LOOP_1050_4bae = 0xffff;
    _PTR_LOOP_1050_4bb0 = 0;
    _PTR_LOOP_1050_4bb6 = 0;
    PTR_LOOP_1050_4bbc = 0x448e;
    PTR_LOOP_1050_4bbe = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4bc2 = 0x448e;
    PTR_LOOP_1050_4bc4 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4bc0 = u16_1050_4494;
    PTR_LOOP_1050_4bc6 = u16_1050_4494;
    PTR_LOOP_1050_4bc8 = 0x448e;
    PTR_LOOP_1050_4bca = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4bcc = u16_1050_4494;
    PTR_LOOP_1050_4bce = 0x4482;
    PTR_LOOP_1050_4bd0 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4bd4 = 0x4482;
    PTR_LOOP_1050_4bd6 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4bd2 = PTR_LOOP_1050_4486;
    PTR_LOOP_1050_4bd8 = PTR_LOOP_1050_4486;
    PTR_LOOP_1050_4bda = 0x4482;
    PTR_LOOP_1050_4bdc = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4bde = PTR_LOOP_1050_4486;
    PTR_LOOP_1050_4be2 = 0x0;
    PTR_LOOP_1050_4be0 = 0x0;
    PTR_LOOP_1050_4bb4 = 0x0;
    PTR_LOOP_1050_4bba = 0x0;
    PTR_LOOP_1050_4be4 = 0x0;
    PTR_LOOP_1050_4be6 = 0x44ac;
    PTR_LOOP_1050_4be8 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4bec = 0x44ac;
    PTR_LOOP_1050_4bee = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4bea = u16_1050_44b2;
    PTR_LOOP_1050_4bf0 = u16_1050_44b2;
    PTR_LOOP_1050_4bf2 = 0x44ac;
    PTR_LOOP_1050_4bf4 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4bf6 = u16_1050_44b2;
    PTR_LOOP_1050_4bf8 = 0x446a;
    PTR_LOOP_1050_4bfa = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4bfe = 0x446a;
    PTR_LOOP_1050_4c00 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4bfc = PTR_LOOP_1050_446e;
    PTR_LOOP_1050_4c02 = PTR_LOOP_1050_446e;
    PTR_LOOP_1050_4c04 = 0x446a;
    PTR_LOOP_1050_4c06 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4c08 = PTR_LOOP_1050_446e;
    PTR_LOOP_1050_4c0a = 0x448e;
    PTR_LOOP_1050_4c0c = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4c0e = u16_1050_4494;
    PTR_LOOP_1050_4c10 = 0x448e;
    PTR_LOOP_1050_4c12 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4c14 = u16_1050_4494;
    PTR_LOOP_1050_4c16 = 0x44ac;
    PTR_LOOP_1050_4c18 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4c1a = u16_1050_44b2;
    PTR_LOOP_1050_4c22 = 0x448e;
    PTR_LOOP_1050_4c24 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4c26 = u16_1050_4494;
    _PTR_LOOP_1050_4c28 = 0;
    _PTR_LOOP_1050_4c2e = 0;
    _PTR_LOOP_1050_4c34 = 0;
    _PTR_LOOP_1050_4c3a = 0;
    _PTR_LOOP_1050_4c40 = 0;
    _PTR_LOOP_1050_4c46 = 0;
    _PTR_LOOP_1050_4c4c = 0;
    _PTR_LOOP_1050_4c52 = 0;
    PTR_LOOP_1050_4c1c = 0x44ac;
    PTR_LOOP_1050_4c1e = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4c58 = 0x44ac;
    PTR_LOOP_1050_4c5a = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4c20 = u16_1050_44b2;
    PTR_LOOP_1050_4c5c = u16_1050_44b2;
    PTR_LOOP_1050_4c5e = 0x44ac;
    PTR_LOOP_1050_4c60 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4c62 = u16_1050_44b2;
    _PTR_LOOP_1050_4c64 = 0;
    _PTR_LOOP_1050_4c6a = 0;
    _PTR_LOOP_1050_4c70 = 0;
    PTR_LOOP_1050_4c76 = 0x446a;
    PTR_LOOP_1050_4c78 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4c7c = 0x446a;
    PTR_LOOP_1050_4c7e = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4c7a = PTR_LOOP_1050_446e;
    PTR_LOOP_1050_4c80 = PTR_LOOP_1050_446e;
    _PTR_LOOP_1050_4c82 = 0;
    _PTR_LOOP_1050_4c88 = 0;
    PTR_LOOP_1050_4c2c = 0xffff;
    PTR_LOOP_1050_4c32 = 0xffff;
    PTR_LOOP_1050_4c38 = 0xffff;
    PTR_LOOP_1050_4c3e = 0xffff;
    PTR_LOOP_1050_4c44 = 0xffff;
    PTR_LOOP_1050_4c4a = 0xffff;
    PTR_LOOP_1050_4c68 = 0xffff;
    PTR_LOOP_1050_4c6e = 0xffff;
    PTR_LOOP_1050_4c74 = 0xffff;
    PTR_LOOP_1050_4c86 = 0xffff;
    PTR_LOOP_1050_4c8c = 0xffff;
    _PTR_LOOP_1050_4c8e = 0;
    _PTR_LOOP_1050_4c94 = 0;
    _PTR_LOOP_1050_4c9a = 0;
    _PTR_LOOP_1050_4ca0 = 0;
    _PTR_LOOP_1050_4ca6 = 0;
    _PTR_LOOP_1050_4cac = 0;
    _PTR_LOOP_1050_4cb2 = 0;
    _PTR_LOOP_1050_4cb8 = 0;
    _PTR_LOOP_1050_4cbe = 0;
    _PTR_LOOP_1050_4cc4 = 0;
    _PTR_LOOP_1050_4cca = 0;
    _PTR_LOOP_1050_4cd0 = 0;
    PTR_LOOP_1050_4cd6 = 0x4496;
    PTR_LOOP_1050_4cd8 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4cdc = 0x4496;
    PTR_LOOP_1050_4cde = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4cda = u16_1050_44a2;
    PTR_LOOP_1050_4ce0 = u16_1050_44a2;
    PTR_LOOP_1050_4ce2 = 0x4496;
    PTR_LOOP_1050_4ce4 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4ce6 = u16_1050_44a2;
    PTR_LOOP_1050_4ce8 = 0x4496;
    PTR_LOOP_1050_4cea = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4cec = u16_1050_44a2;
    PTR_LOOP_1050_4cee = 0x4496;
    PTR_LOOP_1050_4cf0 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4cf2 = u16_1050_44a2;
    PTR_LOOP_1050_4cf4 = 0x44a4;
    PTR_LOOP_1050_4cf6 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4cfa = 0x44a4;
    PTR_LOOP_1050_4cfc = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4cf8 = u16_1050_44aa;
    PTR_LOOP_1050_4cfe = u16_1050_44aa;
    PTR_LOOP_1050_4d00 = 0x44a4;
    PTR_LOOP_1050_4d02 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d04 = u16_1050_44aa;
    PTR_LOOP_1050_4d06 = 0x4496;
    PTR_LOOP_1050_4d08 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d0a = u16_1050_44a2;
    PTR_LOOP_1050_4d0c = 0x4496;
    PTR_LOOP_1050_4d0e = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d10 = u16_1050_44a2;
    PTR_LOOP_1050_4d12 = 0x4496;
    PTR_LOOP_1050_4d14 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d16 = u16_1050_44a2;
    PTR_LOOP_1050_4d18 = 0x4496;
    PTR_LOOP_1050_4d1a = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d1c = u16_1050_44a2;
    PTR_LOOP_1050_4d1e = 0x4482;
    PTR_LOOP_1050_4d20 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d24 = 0x4482;
    PTR_LOOP_1050_4d26 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d22 = PTR_LOOP_1050_4486;
    PTR_LOOP_1050_4d28 = PTR_LOOP_1050_4486;
    PTR_LOOP_1050_4d2a = 0x44a4;
    PTR_LOOP_1050_4d2c = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d2e = u16_1050_44aa;
    PTR_LOOP_1050_4d30 = 0x44a4;
    PTR_LOOP_1050_4d32 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d34 = u16_1050_44aa;
    PTR_LOOP_1050_4d36 = 0x44a4;
    PTR_LOOP_1050_4d38 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d3a = u16_1050_44aa;
    _PTR_LOOP_1050_4d3c = 0;
    _PTR_LOOP_1050_4d42 = 0;
    PTR_LOOP_1050_4c50 = 0x0;
    PTR_LOOP_1050_4c56 = 0x0;
    PTR_LOOP_1050_4c92 = 0x0;
    PTR_LOOP_1050_4c98 = 0x0;
    PTR_LOOP_1050_4c9e = 0x0;
    PTR_LOOP_1050_4ca4 = 0x0;
    PTR_LOOP_1050_4caa = 0x0;
    PTR_LOOP_1050_4cb0 = 0x0;
    PTR_LOOP_1050_4cb6 = 0x0;
    PTR_LOOP_1050_4cbc = 0x0;
    PTR_LOOP_1050_4cc2 = 0x0;
    PTR_LOOP_1050_4cc8 = 0x0;
    PTR_LOOP_1050_4cce = 0x0;
    PTR_LOOP_1050_4cd4 = 0x0;
    PTR_LOOP_1050_4d40 = 0x0;
    PTR_LOOP_1050_4d46 = 0x0;
    _PTR_LOOP_1050_4d48 = 0;
    _PTR_LOOP_1050_4d4e = 0;
    _PTR_LOOP_1050_4d54 = 0;
    PTR_LOOP_1050_4d5a = 0x44a4;
    PTR_LOOP_1050_4d5c = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d5e = u16_1050_44aa;
    PTR_LOOP_1050_4d60 = 0x4496;
    PTR_LOOP_1050_4d62 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d66 = 0x4496;
    PTR_LOOP_1050_4d68 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d64 = u16_1050_44a2;
    PTR_LOOP_1050_4d6a = u16_1050_44a2;
    _PTR_LOOP_1050_4d6c = 0;
    _PTR_LOOP_1050_4d72 = 0;
    PTR_LOOP_1050_4d70 = 0xffff;
    PTR_LOOP_1050_4d76 = 0xffff;
    _PTR_LOOP_1050_4d78 = 0;
    _PTR_LOOP_1050_4d7e = 0;
    PTR_LOOP_1050_4d84 = 0x4496;
    PTR_LOOP_1050_4d86 = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d88 = u16_1050_44a2;
    PTR_LOOP_1050_4d8a = 0x4496;
    PTR_LOOP_1050_4d8c = &g_alloc_addr_1050_1050;
    PTR_LOOP_1050_4d8e = u16_1050_44a2;
    _PTR_LOOP_1050_4d90 = 0;
    _PTR_LOOP_1050_4d96 = 0;
    _PTR_LOOP_1050_4d9c = 0;
    _PTR_LOOP_1050_4da2 = 0;
    _PTR_LOOP_1050_4da8 = 0;
    _PTR_LOOP_1050_4dae = 0;
    _PTR_LOOP_1050_4db4 = 0;
    _PTR_LOOP_1050_4dba = 0;
    _PTR_LOOP_1050_4dc0 = 0;
    _PTR_LOOP_1050_4dc6 = 0;
    _PTR_LOOP_1050_4dcc = 0;
    _PTR_LOOP_1050_4dd2 = 0;
    _PTR_LOOP_1050_4dd8 = 0;
    _PTR_LOOP_1050_4dde = 0;
    _PTR_LOOP_1050_4de4 = 0;
    _PTR_LOOP_1050_4dea = 0;
    _PTR_LOOP_1050_4df0 = 0;
    _PTR_LOOP_1050_4df6 = 0;
    _PTR_LOOP_1050_4dfc = 0;
    _PTR_LOOP_1050_4e02 = 0;
    _PTR_LOOP_1050_4e08 = 0;
    _PTR_LOOP_1050_4e0e = 0;
    _PTR_LOOP_1050_4e14 = 0;
    _PTR_LOOP_1050_4e1a = 0;
    _PTR_LOOP_1050_4e20 = 0;
    _PTR_LOOP_1050_4e26 = 0;
    _PTR_LOOP_1050_4e2c = 0;
    PTR_LOOP_1050_4d4c = 0x0;
    PTR_LOOP_1050_4d52 = 0x0;
    PTR_LOOP_1050_4d58 = 0x0;
    PTR_LOOP_1050_4d7c = 0x0;
    PTR_LOOP_1050_4d82 = 0x0;
    PTR_LOOP_1050_4d94 = 0x0;
    PTR_LOOP_1050_4d9a = 0x0;
    PTR_LOOP_1050_4da0 = 0x0;
    PTR_LOOP_1050_4da6 = 0x0;
    PTR_LOOP_1050_4dac = 0x0;
    PTR_LOOP_1050_4db2 = 0x0;
    PTR_LOOP_1050_4db8 = 0x0;
    PTR_LOOP_1050_4dbe = 0x0;
    PTR_LOOP_1050_4dc4 = 0x0;
    PTR_LOOP_1050_4dca = 0x0;
    PTR_LOOP_1050_4dd0 = 0x0;
    PTR_LOOP_1050_4dd6 = 0x0;
    PTR_LOOP_1050_4ddc = 0x0;
    PTR_LOOP_1050_4de2 = 0x0;
    PTR_LOOP_1050_4de8 = 0x0;
    PTR_LOOP_1050_4dee = 0x0;
    PTR_LOOP_1050_4df4 = 0x0;
    PTR_LOOP_1050_4dfa = 0x0;
    PTR_LOOP_1050_4e00 = 0x0;
    PTR_LOOP_1050_4e06 = 0x0;
    PTR_LOOP_1050_4e0c = 0x0;
    PTR_LOOP_1050_4e12 = 0x0;
    PTR_LOOP_1050_4e18 = 0x0;
    PTR_LOOP_1050_4e1e = 0x0;
    PTR_LOOP_1050_4e24 = 0x0;
    PTR_LOOP_1050_4e2a = 0x0;
    PTR_LOOP_1050_4e30 = 0x0;
    i_var4 = 0x1b;
    pu_var5 = 0x4e32;
    while (i_var4 != 0) {
        i_var4 = i_var4 + -1;
        pu_var1 = pu_var5;
        pu_var5 = pu_var5 + 1;
        unsafe { *pu_var1 = 0 };
    }
    return;
}

/*
Unable to decompile 'big_switch_statement_1020_c2f8'
// Cause: Exception while decompiling 1020:c2f8: The pipe is being closed

*/

// Unable to decompile 'bad_1028_e28a'
// WARNING: Instruction at (ram,0x10287af1) overlaps instruction at (ram,0x10287af0)
// WARNING: Instruction at (ram,0x10389f8e) overlaps instruction at (ram,0x10389f8d)
// WARNING: Instruction at (ram,0x1038aee3) overlaps instruction at (ram,0x1038aee2)
// WARNING: Instruction at (ram,0x1038cb02) overlaps instruction at (ram,0x1038cb00)
/*
Unable to decompile 'bad1_1038_de20'
// Cause: Exception while decompiling 1038:de20: The pipe is being closed
*/
// WARNING: Instruction at (ram,0x10407763) overlaps instruction at (ram,0x10407760)
// Unable to decompile 'pass1_1040_805a'
// WARNING: Instruction at (ram,0x1040d972) overlaps instruction at (ram,0x1040d96f)
// WARNING: Removing unreachable block (ram,0x1040d9cd)
// WARNING: Removing unreachable block (ram,0x1040d961)
// WARNING: Removing unreachable block (ram,0x1040d9c8)
// WARNING: Removing unreachable block (ram,0x1040d963)
// WARNING: Removing unreachable block (ram,0x1040d986)
// WARNING: Removing unreachable block (ram,0x1040d966)
// WARNING: Removing unreachable block (ram,0x1040d968)
// WARNING: Removing unreachable block (ram,0x1040d9d2)
// WARNING: Removing unreachable block (ram,0x1040d96d)
// WARNING: Removing unreachable block (ram,0x1040d9d4)
// WARNING: Removing unreachable block (ram,0x1040d97a)
// WARNING: Removing unreachable block (ram,0x1040d972)
// WARNING: Removing unreachable block (ram,0x1040d9c1)
// WARNING: Removing unreachable block (ram,0x1040d9bb)
// WARNING: Instruction at (ram,0x105024ee) overlaps instruction at (ram,0x105024ed)
// WARNING: Removing unreachable block (ram,0x105024ab)
// WARNING: Instruction at (ram,0x10505f2b) overlaps instruction at (ram,0x10505f2a)
// WARNING: Removing unreachable block (ram,0x10505a36)
// WARNING: Removing unreachable block (ram,0x1050578b)
// WARNING: Removing unreachable block (ram,0x1050571e)
// WARNING: Removing unreachable block (ram,0x105057b9)
// WARNING: Removing unreachable block (ram,0x1050582a)
// WARNING: Removing unreachable block (ram,0x10505a0b)
// WARNING: Removing unreachable block (ram,0x10505ce9)
// WARNING: Removing unreachable block (ram,0x10505ceb)
// WARNING: Removing unreachable block (ram,0x10505cf3)
// WARNING: Removing unreachable block (ram,0x10505d30)
// WARNING: Removing unreachable block (ram,0x10505d50)
// WARNING: Removing unreachable block (ram,0x10505d52)
// WARNING: Removing unreachable block (ram,0x10505d6b)
// WARNING: Removing unreachable block (ram,0x10505d54)
// WARNING: Removing unreachable block (ram,0x10505d6d)
// WARNING: Removing unreachable block (ram,0x10505d56)
// WARNING: Removing unreachable block (ram,0x10505d6f)
// WARNING: Removing unreachable block (ram,0x10505d58)
// WARNING: Removing unreachable block (ram,0x10505d5a)
// WARNING: Removing unreachable block (ram,0x10505d75)
// WARNING: Removing unreachable block (ram,0x10505d71)
// WARNING: Removing unreachable block (ram,0x10505d67)
// WARNING: Removing unreachable block (ram,0x10505d83)
// WARNING: Heritage AFTER dead removal. Example location: s0xfffe : 0x105056f7
// WARNING: Instruction at (ram,0x113800da) overlaps instruction at (ram,0x113800d9)
// WARNING: Instruction at (ram,0x11a004b9) overlaps instruction at (ram,0x11a004b7)
// WARNING: Removing unreachable block (ram,0x11a004a2)
// WARNING: Removing unreachable block (ram,0x11a00452)
// WARNING: Removing unreachable block (ram,0x11a00506)
// WARNING: Removing unreachable block (ram,0x11a00508)
// WARNING: Removing unreachable block (ram,0x11a0056f)
// WARNING: Removing unreachable block (ram,0x11a0050e)
// WARNING: Removing unreachable block (ram,0x11a00577)
// WARNING: Removing unreachable block (ram,0x11d80447)
// WARNING: Removing unreachable block (ram,0x11d80449)
// WARNING: Removing unreachable block (ram,0x11d804bb)
// WARNING: Removing unreachable block (ram,0x11d8043f)
// WARNING: Removing unreachable block (ram,0x11d80441)
// WARNING: Removing unreachable block (ram,0x11d804b0)

fn main() {}
