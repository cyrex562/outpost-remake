use crate::{util::{CONCAT11, CONCAT22}, win_struct::{HFILE16, SEGPTR}};
use crate::defines::{Struct18, Struct_1008_496c, Struct_1008_49e8, U32Ptr};
use crate::file::file_1028::{read_file_1028_def2, write_to_file_1028_dce2};
use crate::file::file_1038::read_file_1038_7c02;
use crate::file::write;
use crate::fn_ptr::fn_ptr_1000::{call_fn_ptr_1000_0dc6, fn_ptr_1000_17ce};
use crate::global::AppContext;
use crate::mem_1000::{mem_op_1000_0a48, mem_op_1000_179c};
use crate::pass::pass_1008::{pass1_1008_3e76, pass1_1008_4b8e, pass1_1008_7006, pass1_1008_72a8};
use crate::pass::pass_1020::{pass1_1020_a43e, pass1_1020_a644, pass1_1020_ba3e, pass1_1020_bb16};
use crate::pass::pass_1028::pass1_1028_d7a0;
use crate::pass::pass_1030::pass1_1030_5c1a;
use crate::pass::pass_1038::pass1_1038_7b20;
use crate::string::string_1000::str_op_1000_3da4;
use crate::string::string_1008::str_op_1008_60e8;
use crate::struct_ops::struct_1008::{set_struct_1008_574a, struct_1008_dcdc, struct_op_1008_3f92, struct_op_1008_48fe, struct_op_1008_4c98};
use crate::struct_ops::struct_1020::struct_1020_c444;
use crate::struct_ops::struct_1030::struct_op_1030_1cd8;
use crate::ui::ui_1008::set_stuct_1008_b0bc;
use crate::util::{read_string_from_rsrc, ZEXT24, read_string_from_addr};
use crate::winapi::{_hwrite16, _lclose16, _lcreat16, _llseek16, _lopen16, WIN16_hread};

pub fn close_file_1008_496c(param_1: &mut Struct_1008_496c) {
    let pu_var1: u32;
    let u_var2: u16;
    let mut u_var3: &mut Struct18;
    let fn_ptr: u32;

    param_1.field_0x0 = ctx.PTR_LOOP_1050_4c4c;
    param_1.field_0x2 = 0x1008;
    pu_var1 = param_1.field_0x4 as u32;
    u_var2 = param_1.field_0x6;
    if (u_var2 | pu_var1) != 0x0 {
        fn_ptr = *pu_var1;
        (**fn_ptr)();
    }
    fn_ptr_1000_17ce(ctx, &mut param_1.field_0x8, 0x1000);
    if param_1.field_0x1a != 0x0 {
        u_var3 = &mut param_1.field_0x1a;
        call_fn_ptr_1000_0dc6(ctx, u_var3, 0x1000);
    }
    if param_1.field_0xc != -0x1 {
        _lclose16(0x1000);
    }
    param_1.field_0x0 = 0x389a;
    param_1.field_0x2 = 0x1008;
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub unsafe fn read_file_1008_49e8(
    ctx: &mut AppContext,
    param_1: &mut Struct_1008_49e8,
    param_2: u16,
    param_3: u16,
    extraout_dx: u16,
    unaff_di: u16,
    unaff_ss: u16) -> u16

{
    let hvar1: HFILE16;
    let i_var2: i16;
    let u_var3: u32;
    let u_var4: u32;
    let pu_var5: U32Ptr;
    let pu_var6: U32Ptr;
    // let extraout_dx: U32Ptr;
    // let i_var7: i16;
    // let unaff_di: i16;
    let h_file: u16;
    // let unaff_ss: u16;
    let l_var9: i32;
    let mut local_18: usize = 0;
    let mut u_stack22: u32 = 0;
    let u_stack10: u16;
    let pu_stack8: U32Ptr;
    let u_stack6: u32;

    // uVar8 = (param_1 >> 0x10);
    //  i_var7 = param_1;
    if (param_1.field_0x8) != 0x0 {
        if (param_1.field_0x1e) != 0x0 {
            return param_3;
        }
        h_file = param_2;
        if (param_1 + 0xc) == -0x1 {
            h_file = ctx.s_tile2_bmp_1050_1538 as HFILE16;
            hvar1 = _lopen16(&read_string_from_rsrc(param_2), 0x0);
            (param_1 + 0xc) = hvar1;
            if hvar1 == 0xffff {
                return param_3;
            }
        }
        u_stack6 = 0x0;
        l_var9 = WIN16_hread(h_file, 0xe, local_18);
        // param_3 = (l_var9 >> 0x10);
        if (l_var9 == 0xe) && (param_3 == 0x0) {
            u_stack6 = u_stack22;
            if local_18 == ctx.PTR_LOOP_1050_4d42 {
                _llseek16(ctx.s_tile2_bmp_1050_1538, 0x0, 0x0);
                l_var9 = mem_op_1000_0a48(ctx, 0x1, u_stack6, ctx._PTR_LOOP_1050_5f2c, 0x1000);
                // pu_var6 = (l_var9 >> 0x10);
                (param_1.field_0x1a) = l_var9;
                (param_1.field_0x1c) = pu_var6;
                if (pu_var6 | (param_1.field_0x1a)) == 0x0 {
                    return pu_var6;
                }
                l_var9 = WIN16_hread(0x1000, u_stack6,
                                     CONCAT22((param_1.field_0x1a),
                                              (u_stack6 >> 0x10)));
                // pu_var5 = (l_var9 >> 0x10);
                u_stack10 = l_var9;
                pu_stack8 = pu_var5;
                _lclose16(ctx.s_tile2_bmp_1050_1538 as HFILE16);
                (param_1.field_0xc) = 0xffff;
                (param_1.field_0x1e) = 0x1;
                (param_1.field_0xe) = (param_1.field_0x1a);
                u_var3 = param_1.field_0x1a;
                i_var2 = u_var3;
                u_var3 &= 0xffff0000;
                (param_1.field_0x12) = u_var3 | i_var2 + 0xe;
                u_var3 |= i_var2 + 0x436;
                (param_1 + 0x16) = u_var3;
                mem_op_1000_179c(ctx, 0x14, pu_var5, 0x1000);
                pu_var6 = (pu_var5 | u_var3);
                if pu_var6 == 0x0 {
                    (param_1.field_0x4) = 0x0;
                } else {
                    u_var4 = (param_1.field_0x12);
                    u_var4 = u_var4 & 0xffff0000 | (u_var4 + 0x28);
                    struct_op_1008_4c98((u_var3 & 0xffff | ZEXT24(pu_var5) << 0x10),
                                        u_var4, 0x100);
                    (param_1.field_0x4) = u_var4;
                    (param_1.field_0x6) = extraout_dx;
                    pu_var6 = extraout_dx;
                }
                if (i_var7 + 0x22) != 0x0 {
                    pass1_1008_4b8e(ctx, param_1, pu_var6, unaff_di, unaff_ss);
                    return pu_var6;
                }
                return pu_var6;
            }
        }
        _lclose16(ctx.s_tile2_bmp_1050_1538);
        (param_1.field_0xc) = 0xffff;
    }
    return param_3;
}


pub fn file_1008_4c26(param_1: u32, param_2: u8) -> u32

{
    close_file_1008_496c(param_1);
    if ((param_2 & 0x1) != 0x0) {
        fn_ptr_1000_17ce(ctx, param_1, 0x1000);
    }
    return param_1;
}


pub fn file_1008_6414(param_1: &mut Struct18, param_2: &mut Struct18, param_3: u16, param_4: &mut Struct18) {
    let ppcVar1: u32;
    let pu_var2: U32Ptr;
    let u_var3: u16;
    let extraout_dx: u16;
    let i_var4: i16;
    let u_var5: u16;
    let paStack42: &mut Struct76;
    let local_26: [u8; 24];

    // u_var5 = (param_1 >> 0x10);
    i_var4 = param_1;
    *param_1 = 0x0;
    (i_var4 + 0x4) = 0x0;
    pu_var2 = local_26;
    struct_op_1008_48fe(CONCAT22(param_3, pu_var2), 0x1, param_2,
                        param_4);
    mem_op_1000_179c(0x1e, param_4, 0x1000);
    paStack42 = CONCAT22(param_4, pu_var2);
    u_var3 = param_4 | pu_var2;
    if (u_var3 == 0x0) {
        *param_1 = 0x0;
    } else {
        pu_var2 = local_26;
        struct_op_1008_3f92(paStack42, CONCAT22(param_3, pu_var2));
        param_1 = pu_var2;
        (i_var4 + 0x2) = u_var3;
    }
    ppcVar1 = (*param_1 + 0x14);
    (**ppcVar1)();
    (i_var4 + 0x4) = pu_var2;
    (i_var4 + 0x6) = extraout_dx;
    close_file_1008_496c(local_26, param_3);
    return;
}


pub fn close_file_1008_6dd0(param_1: U32Ptr, param_2: HFILE16) {
    let u_var1: u16;

    // u_var1 = (param_1 >> 0x10);
    if ((param_1 + 0x4) != -0x1) {
        _lclose16(param_2);
        (param_1 + 0x4) = 0xffff;
    }
    fn_ptr_1000_17ce(ctx, *param_1, 0x1000);
    return;
}


pub fn file_fn_1008_6e02(
    ctx: &mut AppContext,
    param_1: u32,
    in_string: &String,
    param_3: u16,
    extraout_dx: u16,
    unaff_di: i16) -> bool {
    let var1: i16;
    let var2: bool;
    let u_var1: u16;
    let local_4: [u8; 2];

    ctx.PTR_LOOP_1050_0310 = 0x0;
    var1 = write_to_file_1008_70a6(param_1, in_string);
    if var1 != 0x0 {
        // u_var1 = (param_1 >> 0x10);
        pass1_1008_72a8(0, 0);
        var1 = pass1_1008_7006(param_1, u_var1, CONCAT22(param_3, local_4), extraout_dx,
                               unaff_di, param_3);
        if (var1 != 0x0) && (var1 = file_fn_1008_6eee(param_1, local_4, param_3), var1 != 0x0) {
            var2 = file_fn_1008_726c(param_1, u_var1, in_string);
            if (var2 == 0x0) {
                return 0x0;
            }
            return 0x1;
        }
        _lclose16(in_string);
    }
    return 0x0;
}


pub fn read_file_1008_6e78(
    param_1: u32,
    param_2: u16,
    in_string: &String,
    param_4: u16,
    extraout_dx: U32Ptr,
    unaff_DI: i16,
) -> bool

{
    let b_var1: bool;
    let i_var2: i16;
    let var3: U32Ptr;
    let local_4: [u8; 2];

    ctx.PTR_LOOP_1050_0310 = 0x0;
    b_var1 = read_file_1008_7146(param_1, param_2, in_string, param_4);
    if b_var1 != 0x0 {
        pass1_1008_72a8();
        i_var2 = pass1_1008_7056(param_1, param_2, CONCAT22(param_4, local_4), extraout_dx,
                                 unaff_DI, param_4);
        if (i_var2 != 0x0) {
            var3 = local_4;
            read_file_1008_6f7a(param_1, param_2, CONCAT22(param_4, var3), param_4);
            if (var3 != 0x0) {
                b_var1 = file_fn_1008_726c(param_1, param_2, in_string);
                if (b_var1 == 0x0) {
                    return 0x0;
                }
                return 0x1;
            }
        }
        _lclose16(in_string);
    }
    return 0x0;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn file_fn_1008_6eee(param_1: u16, param_2: u16, param_3: u32) {
    let b_var1: bool;
    let u_var2: u16;
    let in_DX: U32Ptr;
    let unaff_SS: u16;
    let u_var3: u32;
    let local_e: [u8; 4];
    let uStack10: u32;
    let puStack6: u32;

    puStack6 = *_PTR_LOOP_1050_5748;
    uStack10 = *puStack6;
    pass1_1020_a43e(unaff_SS, in_DX, CONCAT22(unaff_SS, local_e));
    b_var1 = pass1_1028_d7a0(uStack10, (uStack10 >> 0x10), param_3,
                             unaff_SS);
    if (b_var1 != 0x0) {
        b_var1 = pass1_1030_5c1a(ctx.PTR_LOOP_1050_5736, param_3, unaff_SS);
        if (b_var1 != 0x0) {
            u_var3 = write_to_file_1028_dce2(ctx.PTR_LOOP_1050_65e2, param_3, unaff_SS);
            if ((u_var3 >> 0x10) != 0x0) {
                u_var2 = pass1_1038_7b20(ctx.PTR_LOOP_1050_5a64, param_3, unaff_SS);
                if (u_var2 != 0x0) {
                    b_var1 = pass1_1020_a644(local_e, unaff_SS, param_3, unaff_SS);
                    if (b_var1 != 0x0) {
                        return;
                    }
                }
            }
        }
    }
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn read_file_1008_6f7a(param_1: u16, param_2: u16, param_3: u32,
                           param_4: u16)

{
    let var5: u16;
    let i_var3: i16;
    let b_var4: bool;
    let in_DX: U32Ptr;
    let u_var1: u16;
    let pu_var2: U32Ptr;
    let local_e: [u8; 4];
    let uStack10: u32;
    let puStack6: u32;

    puStack6 = *_PTR_LOOP_1050_5748;
    uStack10 = *puStack6;
    pu_var2 = pass1_1020_a43e(param_4, in_DX, CONCAT22(param_4, local_e));
    // u_var1 = (pu_var2 >> 0x10);
    var5 = read_file_1028_d7ba(uStack10, (uStack10 >> 0x10), param_3, param_4,
                               pu_var2);
    if (var5 != 0x0) {
        var5 = read_file_1030_5c52(ctx.PTR_LOOP_1050_5736, param_3, var5, param_4);
        if (var5 != 0x0) {
            read_file_1028_def2(ctx.PTR_LOOP_1050_65e2, param_3, param_4, var5);
            if (var5 != 0x0) {
                i_var3 = read_file_1038_7c02(ctx.PTR_LOOP_1050_5a64, param_3, var5, u_var1);
                if (i_var3 != 0x0) {
                    b_var4 = read_file_1020_a65e(CONCAT22(param_4, local_e), param_3, param_4,
                                                 local_e);
                    if (b_var4 != 0x0) {
                        return;
                    }
                }
            }
        }
    }
    return;
}


pub fn write_to_file_1008_70a6(param_1: U32Ptr, param_2: &String) -> u16

{
    let HVar1: HFILE16;
    let i_var2: i16;
    let u_var3: u16;
    let mut pCVar4: String;
    let unaff_SS: u16;
    let in_AF: u8;
    let lVar5: i32;

    // u_var3 = (param_1 >> 0x10);
    i_var2 = param_1;
    pCVar4 = param_2;
    if ((i_var2 + 0x4) != -0x1) {
        pCVar4 = ctx.s_tile2_bmp_1050_1538;
        _lclose16(param_2);
        (i_var2 + 0x4) = 0xffff;
    }
    HVar1 = _lcreat16(pCVar4, 0x0);
    (i_var2 + 0x4) = HVar1;
    if (HVar1 == 0xffff) {
        ctx.PTR_LOOP_1050_0310 = 0x6cf;
    } else {
        ctx.PTR_LOOP_1050_0312 = &DAT_1050_0004;
        sys_1000_3f9c(0x65a0, ctx.data_seg,
                      ctx._PTR_s_SC_03d_1050_0314_1050_031c,
                      (ctx.PTR_s_SC_03d_1050_0314_1050_031c >> 0x10), 0x4,
                      &stack0xfffe, u_var3, 0x1000, unaff_SS, in_AF);
        pCVar4 = str_op_1000_3da4(0x105065a0);
        lVar5 = _hwrite16(0x1000, pCVar4, CONCAT22(0x65a0, pCVar4 >> 0xf));
        if ((lVar5 == pCVar4) && (true)) {
            return 0x1;
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    }
    return 0x0;
}


pub fn read_file_1008_7146(
    param_1: i32,
    param_2: u16,
    param_3: &String,
    param_4: u16,
) -> bool

{
    let file_handle: HFILE16;
    let i_var2: i16;
    let mut path: &String;

    path = param_3;
    if (param_1 + 0x4) != -0x1 {
        path = ctx.s_tile2_bmp_1050_1538;
        _lclose16(param_3);
        (param_1 + 0x4) = 0xffff;
    }
    file_handle = _lopen16(path, 0x0);
    (param_1 + 0x4) = file_handle;
    if file_handle == 0xffff {
        ctx.PTR_LOOP_1050_0310 = 0x6cf;
    } else {
        i_var2 = read_file_1008_71a0(CONCAT22(param_2, param_1), param_4);
        if i_var2 != 0x0 {
            return true;
        }
    }
    return false;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn read_file_1008_71a0(param_1: u32, param_2: u16) -> u16

{
    let buffer: u16;
    let u_var1: u16;
    let in_af: u8;
    let l_var2: i32;
    let i_stack26: i16;
    let i_stack24: i16;
    let i_stack22: i16;
    let mut local_e: [u8;0x9] = [0;9];
    let u_stack5: u8;
    let u_stack4: u16;

    u_stack4 = 0x1;
    buffer = str_op_1000_3da4(read_string_from_addr(0x105065a0));
    i_stack22 = 0x0;
    l_var2 = WIN16_hread(0x1000, buffer, CONCAT22(local_e, buffer >> 0xf));
    u_var1 = l_var2;
    if ((buffer < l_var2) && ((true || (buffer < u_var1)))) {
        u_var1 = buffer;
    }
    i_stack24 = u_var1 - 0x2;
    if (i_stack24 < 0x0) {
        i_stack24 = 0x0;
    }
    i_stack26 = 0x2;
    while (i_stack24 != 0x0) {
        i_stack22 = i_stack22 * 0xa + local_e[i_stack26] + -0x30;
        i_stack26 += 0x1;
        i_stack24 = i_stack24 + -0x1;
    }
    if (i_stack22 == 0x1) {
        ctx.PTR_LOOP_1050_0312 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
    } else {
        if (i_stack22 == 0x4) {
            ctx.PTR_LOOP_1050_0312 = &DAT_1050_0004;
        } else {
            u_stack5 = 0x0;
            ctx.PTR_LOOP_1050_0312 = (&ctx.PTR_LOOP_1050_0000 + 0x1);
            u_stack4 = 0x0;
        }
    }
    sys_1000_3f9c(0x65a0, ctx.data_seg,
                  ctx._PTR_s_SC_03d_1050_0314_1050_031c,
                  (ctx.PTR_s_SC_03d_1050_0314_1050_031c >> 0x10),
                  ctx.PTR_LOOP_1050_0312, &stack0xfffe, (param_1 >> 0x10),
                  0x1000, param_2, in_af);
    return u_stack4;
}


pub fn file_fn_1008_726c(param_1: u32, param_2: u16, file_handle: HFILE16) -> bool

{
    let handle_1: HFILE16;

    if ((param_1 + 0x4) != -0x1) {
        handle_1 = _lclose16(file_handle);
        if (handle_1 == 0xffff) {
            ctx.PTR_LOOP_1050_0310 = 0x6d1;
            return 0x0;
        }
        (param_1 + 0x4) = 0xffff;
        ctx.PTR_LOOP_1050_0310 = 0x0;
    }
    return 0x1;
}


pub fn file_1008_7548(param_1: u32, param_2: &i32, param_3: HFILE16, param_4: u16) {
    let ppcVar1: u32;
    let u_var2: u16;
    let BVar3: bool;
    let u_var4: u16;
    let u_var5: u32;
    let u_var6: u16;
    let uVar7: u16;
    let local_1c: u32;
    let local_18: [u16; 0x5];
    let uStack14: u32;
    let uStack10: u32;
    let local_6: u32;

    local_6 = 0x0;
    uVar7 = param_1;
    // u_var2 = (param_1 >> 0x10);
    BVar3 = read_file_1008_7dee(uVar7, u_var2, &local_6, 0x0, param_4, 0x4, param_3);
    if (BVar3 == 0x0) {
        return;
    }
    if (local_6 != 0x0) {
        u_var5 = local_6;
        if (local_6 < 0xc8) {
            local_6._2_2_ = 0x0;
            u_var5 = 0xc8;
        }
        u_var4 = u_var5;
        uStack10 = u_var5 & 0xffff | ZEXT24(local_6._2_2_) << 0x10;
        if (*param_2 == 0x0) {
            param_3 = 0x1000;
            mem_op_1000_179c(0x1e, local_6._2_2_, 0x1000);
            u_var6 = local_6._2_2_ | u_var4;
            if (u_var6 == 0x0) {
                *param_2 = 0x0;
            } else {
                param_3 = 0x1020;
                struct_1020_c444(CONCAT22(local_6._2_2_, u_var4), 0x64, uStack10);
                param_2 = u_var4;
                (param_2 + 0x2) = u_var6;
            }
        }
        ppcVar1 = (*param_2 + 0x24);
        (**ppcVar1)(param_3, *param_2);
        // TODO: refactor for loop
        // for (uStack14 = 0x0; uStack14 < local_6; uStack14 += 0x1) {
        //   BVar3 = read_file_1008_7dee(uVar7,u_var2,&local_1c,0x0,param_4,0x4,param_3);
        //   if ((BVar3 == 0x0) ||
        //      (BVar3 = read_file_1008_7dee(uVar7,u_var2,local_18,0x0,param_4,0x2,param_3
        //                                  ), BVar3 == 0x0)) {
        //     ppcVar1 = (*param_2 + 0x1c);
        //     (**ppcVar1)(param_3,*param_2,(*param_2 >> 0x10));
        //     return;
        //   }
        //   ppcVar1 = (*param_2 + 0x28);
        //   (**ppcVar1)(param_3,*param_2,(*param_2 >> 0x10),local_18[0],
        //               local_1c,(local_1c >> 0x10));
        // }
        ppcVar1 = (*param_2 + 0x1c);
        (**ppcVar1)(param_3, *param_2, (*param_2 >> 0x10));
    }
    return;
}


pub fn file_1008_76e4(param_1: u32, param_2: &i32, param_3: u16, param_4: u16, param_5: u16) {
    let var1: u32;
    let var2: bool;
    let b_var3: bool;
    let extraout_dx: u16;
    let u_var4: u16;
    let local_18: [u8;0xe];
    let var10: u32;
    let var6: u32;

    var6 = 0x0;
    // u_var4 = (param_1 >> 0x10);
    var2 = read_file_1008_7dee(param_1, u_var4, &var6, 0x0, param_4, 0x4,
                                 param_3);
    if (var2 == 0x0) {
        return;
    }
    if (var6 != 0x0) {
        if (*param_2 == 0x0) {
            param_3 = 0x1000;
            mem_op_1000_179c(0x18, param_5, 0x1000);
            if ((param_5 | var2) == 0x0) {
                *param_2 = 0x0;
            } else {
                param_3 = 0x1030;
                struct_op_1030_1cd8(CONCAT22(param_5, var2), 0x5, var6);
                param_2 = var2;
                (param_2 + 0x2) = extraout_dx;
            }
        }
        var1 = (*param_2 + 0x14);
        (**var1)(param_3, *param_2, (*param_2 >> 0x10), var6);
        // TODO: refactor for loop
        // for (uStack10 = 0x0; uStack10 < local_6; uStack10 += 0x1) {
        //   BVar3 = read_file_1008_7dee(param_1,u_var4,local_18,0x0,param_4,0x4,
        //                               param_3);
        //   if (BVar3 == 0x0) {
        //     return;
        //   }
        //   ppcVar1 = (*param_2 + 0x18);
        //   (**ppcVar1)();
        // }
        var1 = (*param_2 + 0x1c);
        (**var1)();
    }
    return;
}


pub fn file_1008_77cc(param_1: u32, param_2: &i32, param_3: U32Ptr, param_4: HFILE16, param_5: u16) -> u16

{
    let u_var1: u16;
    let BVar2: bool;
    let u_var3: u16;
    let unaff_SI: u16;
    let unaff_DI: u16;
    let u_var4: u16;
    let u_var5: u16;
    let local_14: [u16; 0x2];
    let local_10: [u32; 0x2];
    let uStack6: u16;
    let local_4: u16;

    local_4 = 0x0;
    u_var4 = param_1;
    // u_var5 = (param_1 >> 0x10);
    u_var1 = read_file_1008_7dee(u_var4, u_var5, &local_4, 0x0, param_5, 0x2, param_4);
    if (u_var1 == 0x0) {
        return 0x0;
    }
    if (local_4 != 0x0) {
        if (*param_2 == 0x0) {
            param_4 = 0x1000;
            mem_op_1000_179c(0xa, param_3, 0x1000);
            u_var3 = param_3 | u_var1;
            if (u_var3 == 0x0) {
                *param_2 = 0x0;
            } else {
                param_4 = 0x1020;
                pass1_1020_ba3e(CONCAT22(param_3, u_var1), 0x5, 0x5, unaff_DI, unaff_SI);
                param_2 = u_var1;
                (param_2 + 0x2) = u_var3;
            }
        }
        // TODO: refactor for loop
        // for (uStack6 = 0x0; uStack6 < local_4; uStack6 += 0x1) {
        //   BVar2 = read_file_1008_7dee(u_var4,u_var5,local_14,0x0,param_5,0x2,param_4);
        //   if (BVar2 == 0x0) {
        //     return 0x0;
        //   }
        //   BVar2 = read_file_1008_7dee(u_var4,u_var5,local_10,0x0,param_5,0x4,param_4);
        //   if (BVar2 == 0x0) {
        //     return 0x0;
        //   }
        //   param_4 = 0x1020;
        //   pass1_1020_bb8a(*param_2,local_10[0],
        //                   CONCAT22(local_14[0],(local_10[0] >> 0x10)),unaff_DI,
        //                   param_5);
        // }
    }
    return 0x1;
}



pub fn write_to_file_1008_7954
(param_1: u32,param_2: U32Ptr,param_3: u16,param_4: HFILE16,
param_5: u16) -> u16

{
let ppcVar1: u32; let BVar2: bool;
let u_var3: u32; let extraout_dx: u16;
let u_var4: u16; let extraout_DX_00: u16;
let u_var5: u16; let local_20: u16;
let uStack30: u16; let local_18: u16;
let uStack22: u16; let uStack10: u32;
let uStack6: u32;

if (param_2 == 0x0) {
param_3 = 0x0; u_var4 = 0x0;
}
else {
ppcVar1 = ( * param_2 + 0x10); (* * ppcVar1)(param_4, param_2);
u_var4 = extraout_dx;
}
uStack6 = CONCAT22(u_var4, param_3);
// u_var5 = (param_1 >> 0x10);
local_18 = param_3; uStack22 = u_var4; BVar2 = write_to_file_1008_7e1c
(param_1, u_var5, & local_18, param_5,0x4, param_4); if (BVar2 != 0x0) {
uStack10 = 0x0; loop {
if (uStack6 <= uStack10) {
return u_var4;
}
ppcVar1 = ( * param_2 + 0x4); u_var3 = uStack6; ( * * ppcVar1)(); local_20 = u_var3; u_var4 = extraout_DX_00; uStack30 = extraout_DX_00; local_18 = local_20; uStack22 = extraout_DX_00; BVar2 = write_to_file_1008_7e1c
(param_1, u_var5, &local_20, param_5, 0x4,
param_4); if (BVar2 == 0x0) { break; }
uStack10 += 0x1;
}
}
ctx.PTR_LOOP_1050_0310 = 0x6d0; return u_var4;
}


pub fn write_to_file_1008_7a22(param_1: u32, param_2: i32, param_3: HFILE16, param_4: u16) {
    let b_var1: bool;
    let u_var2: u16;
    let u_var3: u16;
    let local_24: [u32; 0x2];
    let local_1c: [u16; 0x5];
    let local_12: u16;
    let local_10: u32;
    let uStack10: u16;
    let uStack6: u16;
    let uStack4: u16;

    if (param_2 == 0x0) {
        uStack4 = 0x0;
    } else {
        uStack4 = (param_2 + 0x4);
    }
    local_12 = uStack4;
    u_var2 = param_1;
    // u_var3 = (param_1 >> 0x10);
    b_var1 = write::write_to_file_1008_7e1c(u_var2, u_var3, &local_12, param_4, 0x2, param_3);
    if (b_var1 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
    } else {
        uStack6 = 0x0;
        loop {
            if (uStack4 <= uStack6) {
                return;
            }
            pass1_1020_bb16(param_2, CONCAT22(param_4, &local_10),
                            CONCAT22(param_4, &local_12), uStack6);
            uStack10 = local_12;
            local_1c[0] = local_12;
            b_var1 = write::write_to_file_1008_7e1c(u_var2, u_var3, local_1c, param_4, 0x2, 0x1020);
            if (b_var1 == 0x0) { break; }
            local_24[0] = local_10;
            b_var1 = write::write_to_file_1008_7e1c(u_var2, u_var3, local_24, param_4, 0x4, 0x1020);
            if (b_var1 == 0x0) {
                return;
            }
            uStack6 += 0x1;
        }
    }
    return;
}


pub fn write_to_file_1008_7b4c(
    param_1: u32,
    param_2: * mut u16,
    param_3: HFILE16,
    param_4: u16) -> u16

{
let b_var1: bool; let u_var2: u16;
let u_var3: u16; let local_12: [u16; 0x3]; let local_c: [u16;0x2]; let local_8: u16; let local_6: u16; let local_4: u16;

pass1_1008_3eb4(param_2, CONCAT22(param_4, & local_8),
CONCAT22(param_4, & local_6),
CONCAT22(param_4, & local_4)); local_12[0] = local_4; u_var2 = param_1;
// u_var3 = (param_1 >> 0x10);
b_var1 = write_to_file_1008_7e1c
(u_var2, u_var3, local_12, param_4,0x2, param_3); if (b_var1 != 0x0) {
local_c[0] = local_6; b_var1 = write_to_file_1008_7e1c
(u_var2, u_var3, local_c, param_4, 0x2, param_3); if (b_var1 != 0x0) {
local_c[0] = local_8; b_var1 = write_to_file_1008_7e1c
(u_var2, u_var3, local_c, param_4, 0x2, param_3); if (b_var1 != 0x0) {
return 0x1;
}
}
}
return 0x0;
}



pub fn read_file_1008_7bc8(param_1: u32, param_2: U32Ptr, param_3: HFILE16, param_4: u16) -> bool

{
    let b_var1: bool;
    let u_var2: u16;
    let u_var3: u16;
    let local_8: u16;
    let local_6: u32;

    u_var2 = param_1;
    // u_var3 = (param_1 >> 0x10);
    b_var1 = read_file_1008_7dee(u_var2, u_var3, &local_6 + 0x2, 0x0, param_4, 0x2, param_3);
    if (b_var1 != 0x0) {
        b_var1 = read_file_1008_7dee(u_var2, u_var3, &local_6, 0x0, param_4, 0x2, param_3);
        if (b_var1 != 0x0) {
            b_var1 = read_file_1008_7dee(u_var2, u_var3, &local_8, 0x0, param_4, 0x2, param_3);
            if (b_var1 != 0x0) {
                pass1_1008_3e76(param_2, local_8, local_6, (local_6 >> 0x10));
                return 0x1;
            }
        }
    }
    return 0x0;
}


pub fn read_file_1008_7c6e(param_1: u16, param_2: u16, param_3: &mut String, param_4: HFILE16) {
    let mut pcVar1: String;
    let local_c: [u8;0xa];

    loop {
        pcVar1 = param_3;
        WIN16_hread(param_4, 0x1, ZEXT24(local_c) << 0x10);
        if (local_c[0] == '\0') { break; }
        param_3 = (param_3 & 0xffff0000 | (param_3 + 0x1));
        *pcVar1 = local_c[0];
        param_4 = s_tile2_bmp_1050_1538;
    }
    *param_3 = '\0';
    return;
}


// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn write_to_file_1008_7cac(
    param_1: u32, 
    param_2: u16) -> bool

{
    let u_var1: u16;
    let BVar2: bool;
    let unaff_ES: u16;
    let in_AF: u8;
    let ulocal_c: [u8;0xa];

    sys_1000_3f9c(local_c, param_2, 0x340, ctx.data_seg,
                  ctx._PTR_s_dcbSC_1050_0336_1050_033c, &stack0xfffe, unaff_ES, 0x1000,
                  param_2, in_AF);
    u_var1 = str_op_1000_3da4(CONCAT22(param_2, local_c));
    BVar2 = write::write_to_file_1008_7e1c(param_1, (param_1 >> 0x10), local_c,
                                           param_2, u_var1, 0x1000);
    if (BVar2 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d0;
        return BVar2;
    }
    return 0x1;
}


// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

pub fn read_file_1008_7cfe(param_1: u32, param_3: u16,
                           param_4: u16, param_5: u16)

{
    let bVar1: bool;
    let u_var2: u16;
    let in_AF: u8;
    let lVar3: i32;
    let in_stack_0000fbd2: u16;
    let in_stack_0000fbd4: u16;
    let uStack1040: u32;
    let local_406: [u8;0x400];
    let uStack6: u32;

    uStack6 = 0x0;
    bVar1 = false;
    loop {
        _llseek16(param_4, uStack6 << 0x10, (uStack6 >> 0x10));
        param_4 = ctx.s_tile2_bmp_1050_1538;
        lVar3 = WIN16_hread(s_tile2_bmp_1050_1538, 0x400, ZEXT24(local_406) << 0x10);
// TODO: refactor for loop
        // for (uStack1040 = 0x0; uStack1040 < lVar3; uStack1040 += 0x1) {
        //   if (local_406[uStack1040] == *_PTR_s_dcbSC_1050_0336_1050_033c) {
        //     if (!bVar1) {
        //       bVar1 = true;
        //       uStack6 = CONCAT22((uStack6 >> 0x10) + uStack1040._2_2_ +
        //                          CARRY2(uStack6,uStack1040),
        //                          uStack6 + uStack1040);
        //       break;
        //     }
        //     bVar1 = false;
        //     u_var2 = pass1_1008_7e4a((ctx.PTR_s_dcbSC_1050_0336_1050_033c >> 0x10),
        //                             param_5,in_AF,
        //                             CONCAT22(param_5,local_406 + uStack1040),
        //                             in_stack_0000fbd2,in_stack_0000fbd4);
        //     if (u_var2 != 0x0) {
        //       lVar3 = uStack1040 + uStack6 + 0x7;
        //       _llseek16(s_tile2_bmp_1050_1538,lVar3 * 0x10000,
        //                 (lVar3 >> 0x10));
        //       return;
        //     }
        //   }
        // }
        if (!bVar1) {
            if (lVar3 < 0x400) {
                return;
            }
            uStack6._0_2_ = CONCAT11(uStack6._1_1_ + 0x4, uStack6);
            uStack6 = CONCAT22((uStack6 >> 0x10) + (0xfb < uStack6._1_1_),
                               uStack6);
        }
    }
}


pub fn read_file_1008_7dee(param_1: u16, param_2: u16, param_3: u16, param_4: u16,
                           param_5: u16, read_buf: SEGPTR, file_handle: HFILE16) -> bool

{
    let bytes_read: i32;

    bytes_read = WIN16_hread(file_handle, read_buf, param_4);
    if bytes_read == param_4 {
        return true;
    }
    return false;
}


pub fn file_1008_bb5e(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16,
                      param_6: u16)

{
    let ppcVar1: u32;
    let u_var2: u16;
    let i_var3: &mut Struct199;
    let BVar3: bool;
    let u_var5: u16;
    let u_var4: &mut Struct200;
    let puVar6: U32Ptr;
    let uVar7: u16;
    let extraout_dx: U32Ptr;
    let puVar8: U32Ptr;
    let uVar9: u16;
    let u_var10: u16;
    let extraout_DX_00: U32Ptr;
    let extraout_DX_01: u16;
    let u_var11: u16;
    let uVar12: u16;
    let uVar13: u16;
    let uVar14: u16;
    let paStack286: &mut Struct200;
    let puStack284: u32;
    let local_118: [u8; 100];
    let local_18: [u16; 0x2];
    let local_14: [u16; 0x2];
    let local_10: [Struct200; 0x4];
    let local_8: u32;

    if (ctx.PTR_LOOP_1050_0312 < 0x2) {
        return;
    }
    u_var11 = param_2;
    // uVar12 = (param_2 >> 0x10);
    read_file_1008_7cfe(u_var11, 0x16, param_5, param_6);
    if (param_3 == 0x0) {
        ctx.PTR_LOOP_1050_0310 = 0x6d4;
    } else {
        i_var3 = param_1;
        i_var3 = &i_var3.field_0x22;
        // u_var2 = (param_1 >> 0x10);
        BVar3 = read_file_1008_7dee(u_var11, uVar12, i_var3, 0x0, u_var2, 0x2, param_5);
        if ((BVar3 != 0x0) && (u_var5 = read_file_1008_7dee(u_var11, uVar12, local_10, 0x0, param_6, 0x2, param_5,
        ), u_var5 != 0x0)) {
            if (local_10[0] == 0x0) {
                return;
            }
            uVar14 = 0xc;
            mem_op_1000_179c(0xc, param_4, 0x1000);
            if ((param_4 | u_var5) == 0x0) {
                u_var5 = 0x0;
                puVar8 = 0x0;
            } else {
                set_struct_1008_574a(CONCAT22(param_4, u_var5));
                puVar8 = extraout_dx;
            }
            &i_var3.field_0xa = u_var5;
            (&i_var3.field_0xa + 0x2) = puVar8;
            paStack286 = 0x0;
            loop {
                if (local_10[0] <= paStack286) {
                    return;
                }
                uVar13 = 0x12;
                u_var4 = local_10[0];
                mem_op_1000_179c(0x12, puVar8, 0x1000);
                if ((puVar8 | u_var4) == 0x0) {
                    u_var4 = 0x0;
                    uVar9 = 0x0;
                } else {
                    set_stuct_1008_b0bc(CONCAT22(puVar8, u_var4));
                    uVar9 = extraout_DX_01;
                }
                puStack284 = CONCAT22(uVar9, u_var4);
                puVar6 = local_118;
                u_var10 = uVar9;
                read_file_1008_7c6e(u_var11, uVar12, CONCAT22(param_6, puVar6), 0x1000);
                if ((((puVar6 == 0x0) || (BVar3 = read_file_1008_7dee(u_var11, uVar12, local_14, 0x0, param_6, 0x2,
                                                                      0x1000), BVar3 == 0x0)) || (BVar3 = read_file_1008_7dee(u_var11, uVar12, &local_8, 0x0, param_6, 0x4,
                                                                                                                              0x1000), BVar3 == 0x0)) || (BVar3 = read_file_1008_7dee(u_var11, uVar12, local_18, 0x0, param_6, 0x2,
                                                                                                                                                                                      0x1000), BVar3 == 0x0)) { break; }
                uVar7 = str_op_1008_60e8(CONCAT22(param_6, local_118), u_var10);
                u_var4.field_0x4 = uVar7;
                u_var4.field_0x6 = u_var10;
                u_var4.field_0x8 = local_14[0];
                u_var4.field_0xa = local_8;
                u_var4.field_0xe = local_18[0];
                ppcVar1 = (*i_var3.field_0xa + 0x8);
                (**ppcVar1)();
                paStack286 = &paStack286.field_0x1;
                puVar8 = extraout_DX_00;
            }
            if (puStack284 != 0x0) {
                ppcVar1 = *puStack284;
                (**ppcVar1)(0x1000, u_var4, uVar9, 0x1, uVar13, uVar14, puStack284);
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}


pub fn file_1008_e70e(param_1: u32, param_2: u32, param_3: i16, param_4: U32Ptr, param_5: u16,
                      param_6: u16)

{
    let u_var1: u32;
    let ppcVar2: u32;
    let BVar3: bool;
    let u_var4: u16;
    let extraout_dx: U32Ptr;
    let u_var5: u16;
    let u_var6: u16;
    let uVar7: u16;
    let uVar8: u16;
    let uVar9: u16;
    let local_12: [u16; 0x2];
    let puStack14: u32;
    let uStack10: u16;
    let local_4: u16;

    if (ctx.PTR_LOOP_1050_0312 < 0x2) {
        return;
    }
    uVar7 = param_2;
    // uVar8 = (param_2 >> 0x10);
    read_file_1008_7cfe(uVar7, 0x14, param_5, param_6);
    if (param_3 != 0x0) {
        BVar3 = read_file_1008_7dee(uVar7, uVar8, &local_4, 0x0, param_6, 0x2, param_5);
        if (BVar3 != 0x0) {
            if (local_4 == 0x0) {
                return;
            }
            uStack10 = 0x0;
            loop {
                if (local_4 <= uStack10) {
                    return;
                }
                uVar9 = 0x14;
                u_var4 = local_4;
                mem_op_1000_179c(0x14, param_4, 0x1000);
                u_var5 = param_4 | u_var4;
                if (u_var5 == 0x0) {
                    u_var4 = 0x0;
                    u_var5 = 0x0;
                } else {
                    struct_1008_dcdc(CONCAT22(param_4, u_var4));
                }
                puStack14 = CONCAT22(u_var5, u_var4);
                BVar3 = read_file_1008_7dee(uVar7, uVar8, u_var4 + 0x4, 0x0, u_var5, 0x4, 0x1000);
                if ((((BVar3 == 0x0) || (BVar3 = read_file_1008_7dee(uVar7, uVar8, puStack14 + 0x8, 0x0,
                                                                     (puStack14 >> 0x10), 0x4, 0x1000),
                                         BVar3 == 0x0)) || (BVar3 = read_file_1008_7dee(uVar7, uVar8, local_12, 0x0, param_6, 0x2,
                                                                                        0x1000), BVar3 == 0x0)) || ((BVar3 = read_file_1008_7dee(uVar7, uVar8, puStack14 + 0xe, 0x0,
                                                                                                                                                 (puStack14 >> 0x10), 0x4, 0x1000),
                                                                                                                     BVar3 == 0x0 || (BVar3 = read_file_1008_7dee(uVar7, uVar8, puStack14 + 0x12, 0x0,
                                                                                                                                                                  (puStack14 >> 0x10), 0x2, 0x1000),
                                                                                                                                      BVar3 == 0x0)))) { break; }
                // uVar9 = (puStack14 >> 0x10);
                (puStack14 + 0xc) = local_12[0];
                // u_var6 = (param_1 >> 0x10);
                u_var1 = (param_1 + 0xa);
                ppcVar2 = ((param_1 + 0xa) + 0x4);
                (**ppcVar2)(0x0, u_var1, (u_var1 >> 0x10), puStack14, uVar9);
                uStack10 += 0x1;
                param_4 = extraout_dx;
            }
            if (puStack14 != 0x0) {
                ppcVar2 = *puStack14;
                (**ppcVar2)(0x1000, puStack14, (puStack14 >> 0x10), 0x1, uVar9,
                            puStack14);
            }
        }
        ctx.PTR_LOOP_1050_0310 = 0x6d2;
    }
    return;
}

