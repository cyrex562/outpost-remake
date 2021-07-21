use crate::defines::U32Ptr;
use crate::global::AppContext;
use crate::util::{CONCAT11, CONCAT22, read_string_from_addr};
use crate::win_struct::WNDCLASS16;

pub fn poss_str_op_1000_28dc(
    ctx: &mut AppContext,
    param_1: i16) -> String

{
    let string_1: &String;
    let string_3: &mut String;
    let i_var2: i16;
    let mut string_2: &mut String;

    string_2 = read_string_from_addr(ctx.PTR_LOOP_1050_63fe);
    loop {
        string_1 = &string_2;
        string_2 = &mut string_2[2..].to_string();
        i_var2 = string_1[0];
        string_3 = string_2;
        // string_3 = (i_var2 + 0x1);
        if (i_var2 == param_1) || (string_3 == 0x0) {
            return string_3.clone();
        }
        i_var2 = -0x1;
        loop {
            if i_var2 == 0x0 { break; }
            i_var2 += -0x1;
            string_1 = string_2;
            string_2 = (string_2 + 0x1);
            if *string_1 == 0 {
                break;
            }
        }
    }
}


pub fn string_1000_3d3e(
    string_1: &mut String,
    string_2: &mut String
) {
    let ptr_1: U32Ptr;
    let ptr_2: U32Ptr;
    let var_6: u16;
    let var_7: u16;
    let mut string_3: &mut String;
    let mut string_4: &mut String;
    let mut string_5: &mut String;
    let var_8: u16;
    let mut string_6: &mut String;
    let bool_1: bool;
    let mut string_7: &mut String;
    let mut string_8: &mut String;
    let mut string_9: &mut String;

    // l_string_1 = (in_string_2 >> 0x10);
    string_3 = string_2;
    bool_1 = true;
    var_6 = 0xffff;
    string_4 = string_3;
    loop {
        if var_6 == 0x0 { break; }
        var_6 -= 0x1;
        string_9 = string_4;
        string_4 = string_4 + 0x1;
        bool_1 = *string_9 == '\0';
        if bool_1 == true {
            break;
        }
    }
    var_6 = !var_6;
    // uVar8 = (param_1 >> 0x10);
    string_5 = string_1;
    if bool_1 {
        if (string_1 & 0x1) != 0x0 {
            string_5 = string_5 + 0x1;
            string_3 = string_3 + 0x1;
            string_1[0] = string_2[0];
            var_6 -= 0x1;
        }
    } else {
        string_5 = string_5 + 0x2;
        string_3 = string_3 + 0x2;
        *string_1 = string_2.clone();
        var_6 -= 0x1;
    }
    // TODO: refactor
    // for (uVar7 = uVar6 >> 0x1; uVar7 != 0x0; uVar7 -= 0x1) {
    //   puVar5 = puVar7;
    //   puVar7 = (puVar7 + 0x2);
    //   puVar4 = l_string_2;
    //   l_string_2 = (l_string_2 + 0x2);
    //   *puVar5 = *puVar4;
    // }
    // for (uVar6 = ((uVar6 & 0x1) != 0x0); uVar6 != 0x0; uVar6 -= 0x1) {
    //   puVar5 = puVar7;
    //   puVar7 = (puVar7 + 0x1);
    //   puVar4 = l_string_2;
    //   l_string_2 = (l_string_2 + 0x1);
    //   *puVar5 = *puVar4;
    // }
    return;
}


pub fn str_op_1000_3da4(string_1: &mut String) -> u16

{
    let mut string_2: &mut String;
    let var_2: u16;
    let mut string_3: &mut String;
    let bool_1: bool;

    string_3 = string_1;
    bool_1 = true;
    var_2 = 0xffff;
    loop {
        if var_2 == 0x0 { break; }
        var_2 -= 0x1;
        string_2 = string_3;
        string_3 = string_3 + 0x1;
        bool_1 = *string_2 == '\0';
        if (!bool_1) == false { break; }
    }
    var_2 = !var_2;
    if bool_1 {
        var_2 -= 0x1;
    }
    return var_2;
}


pub fn str_op_1000_3dbe(
    param_1: &mut String,
    param_2: &mut String,
    param_3: &mut u16,
) -> &mut String

{
    let mut string_1: &mut String;
    let var2: u8;
    let mut string_2: &mut String;
    let mut string_3: &mut String;
    let var3: u16;

    // uVar5 = (param_1 >> 0x10);
    string_3 = param_1;
    string_2 = param_2;
    if param_3 != 0x0 {
        loop {
            string_1 = string_2;
            string_2 = string_2 + 0x1;
            var2 = string_1[0];
            if var2 == '\0' as u8 { break; }
            string_1 = string_3;
            string_3 = string_3 + 0x1;
            string_1[0] = var2;
            *param_3 -= 0x1;
            if (param_3 != 0x0) == false { break; }
        }
        // for (; param_3 != 0x0; param_3 -= 0x1) {
        //   pcVar1 = pcVar4;
        //   pcVar4 = pcVar4 + 0x1;
        //   *pcVar1 = '\0';
        // }
    }
    return param_1;
}


pub fn str_1000_4d58(
    string_1: &mut String,
    string_2: Option<&mut String>,
    param_3: u32,
    param_4: &mut String,
    param_5: &mut String,
)

{
    let mut u_var1: u16;
    let string_4: &mut String;
    let string_5: &mut String;
    let u_var4: u16;
    let u_var5: u16 = 0;
    let mut string_3: &mut String;
    let mut string_6: &mut String;
    let u_stack10: u16;
    let u_stack8: u16;
    let u_stack6: u16;

    u_stack10 = 0x0;
    // string_6 = 0x0;
    string_4 = string_1;
    if (string_1[0] == '\0') || *(string_4[1..].to_string()[0] != ':') {
        if string_2 != 0x0 {
            string_2[0] = '\0';
        }
    } else {
        if string_2 != 0x0 {
            string_2[0] = string_1[0];
            (string_2[1..].to_string())[0] = (string_4[1..].to_string())[0];
            (string_2[2..].to_string())[0] = 0x0;
        }
        // string_1 = (string_1 & 0xffff0000 | (string_4 + 0x2));
        *string_1 = *string_4[2..];
    }
    u_stack6 = 0x0;
    u_stack8 = 0x0;
    string_3 = string_1;
    loop {
        // uVar5 = (pcStack18 >> 0x10);
        string_5 = string_3;
        if *string_3 == '\0' { break; }
        if (*string_3 == '/') || (*string_3 == '\\') {
            u_stack8 = string_5 + 0x1;
            u_stack6 = u_var5;
        } else {
            if string_3[0] == '.' as u8 {
                string_6 = string_5;
                u_stack10 = u_var5;
            }
        }
        string_3 = (string_3 & 0xffff0000 | (string_5 + 0x1));
    }
    if ((u_stack6 | u_stack8) == 0x0) {
        if (param_3 != 0x0) {
            *param_3 = 0x0;
        }
    } else {
        if (param_3 != 0x0) {
            u_var1 = u_stack8 - string_1;
            if (0xff < u_var1) {
                u_var1 = 0xff;
            }
            str_op_1000_3dbe((param_3 & 0xffff | param_3._2_2_ << 0x10),
                             string_1, &mut u_var1);
            *(param_3 + u_var1) = 0x0;
        }
        *string_1 = read_string_from_addr(CONCAT22(u_stack6, u_stack8)).clone();
    }
    // if ((u_stack10 | string_6) != 0x0) && (string_1 <= string_6) {
    //     if param_4 != 0x0 {
    //         // u_var1 = string_6 - string_1;
    //         if 0xff < u_var1 {
    //             u_var1 = 0xff;
    //         }
    //         str_op_1000_3dbe((param_4 & 0xffff | param_4._2_2_ << 0x10),
    //                          (string_1 & 0xffff | string_1._2_2_ << 0x10), u_var1);
    //         // *(param_4 + u_var1) = 0x0;
    //     }
    //     if param_5 == 0x0 {
    //         return;
    //     }
    //     // u_var1 = string_5 - string_6;
    //     if 0xff < u_var1 {
    //         u_var1 = 0xff;
    //     }
    //     // str_op_1000_3dbe((param_5 & 0xffff | param_5._2_2_ << 0x10),
    //     //                  CONCAT22(u_stack10, string_6), u_var1);
    //     // *(param_5 + u_var1) = 0x0;
    //     return;
    // }
    if param_4 != 0x0 {
        u_var1 = string_5 - string_1;
        if 0xff < u_var1 {
            u_var1 = 0xff;
        }
        str_op_1000_3dbe((param_4 & 0xffff | param_4._2_2_ << 0x10),
                         (string_1 & 0xffff | string_1._2_2_ << 0x10), &mut u_var1);
        *(param_4 + u_var1) = 0x0;
    }
    if param_5 != 0x0 {
        param_5.style = 0x0;
    }
    return;
}


pub fn string_1000_1fd2(param_1: i16) -> String

{
    if param_1 == 0x2 {
        return "Out of memory.  Please free some memory, then choose retry.".to_string();
    }
    return read_string_from_addr(CONCAT22(0x1000, (param_1 * 0x17 + 0x1c7a) as u16)).to_string();
}


pub fn string_1000_475e(
    ctx: &mut AppContext,
    param_1: &String,
    param_2: &String,
) -> u8

{
    let mut string_2: &String;
    let c_var2: u8;
    let c_var3: u8;
    let b_var4: u8;
    let b_var3: u8;
    let string_3: String;
    let mut string_1: &String;
    let mut string_4: &String;

    string_4 = param_2;
    string_1 = param_1;
    string_3 = (ctx.s_You_may_not_run_a_turn__The_game_1050_00df[0x20..].to_string());
    loop {
        loop {
            c_var3 = string_3[0];
            if c_var3 == '\0' as u8 {
                // goto LAB_1000_479d;
            }
            string_2 = string_4;
            string_4 = string_4 + 0x1;
            c_var3 = string_2[0];
            c_var2 = string_1[0];
            string_3 = read_string_from_addr(CONCAT11(c_var2, c_var3) as u32).clone();
            string_1 = &string_1[1..].to_string();
            if c_var2 != c_var3 { break; }
        }
        b_var4 = c_var3 + 0xbf + (-((c_var3 + 0xbf) < 0x1a) & 0x20) + 0x41;
        b_var3 = c_var2 + 0xbf;
        string_3[0] = b_var3 + (-(b_var3 < 0x1a) & 0x20) + 0x41;
        string_3[0] = CONCAT11(b_var4, string_3[0]);
        if (string_3 == b_var4) == false {
            break;
        }
    }
    c_var3 = (string_3 < b_var4) * -0x2 + '\x01';
//LAB_1000_479d:
    return c_var3;
}


pub fn string_1000_3cea(
    string_1: &mut String,
    string_2: &mut String
) -> &mut String

{
    let string_4: &mut String;
    let mut pc_var2: &String;
    let p_uvar3: U32Ptr;
    let i_var4: i16;
    let u_var5: u16;
    let u_var6: u16;
    let string_3: &mut String;
    let string_6: &mut String;
    let string_5: &mut String;
    let string_7: &mut String;
    let u_var11: u16;
    let u_var12: u16;
    let b_var13: bool;

    // u_var11 = (param_1 >> 0x10);
    b_var13 = true;
    i_var4 = -0x1;
    string_3 = string_1;
    loop {
        if i_var4 == 0x0 { break; }
        i_var4 += -0x1;
        string_4 = string_3;
        string_3 = (&mut string_3[1..].to_string());
        b_var13 = string_4[0] == '\0';
        if b_var13 { break; }
    }
    string_7 = (string_3 + -0x1);
    // u_var12 = (param_2 >> 0x10);
    string_6 = string_2;
    u_var5 = 0xffff;
    loop {
        if u_var5 == 0x0 { break; }
        u_var5 -= 0x1;
        pc_var2 = string_6;
        string_6 = string_6 + 0x1;
        b_var13 = *pc_var2 == '\0';
        if b_var13 { break; }
    }
    u_var5 = !u_var5;
    if !b_var13 {
        string_6 = string_6 + -u_var5;
        u_var5 += 0x1;
    }
    string_5 = (string_6 + -u_var5);
    if u_var5 == 0x0 {
        string_4 = string_5;
        string_5 = string_5 + 0x1;
        string_7[0] = string_4[0];
        u_var5 = 0xfffe;
        string_7 = (string_3 + 0x1);
    } else {
        if (string_5 & 0x1) != 0x0 {
            string_4 = string_5;
            string_5 = (string_5 + 0x1);
            string_7[0] = string_4[0];
            u_var5 -= 0x1;
            string_7 = string_3;
        }
    }
    // TODO: refactor for loop
    // for (u_var6 = u_var5 >> 0x1; u_var6 != 0x0; u_var6 -= 0x1) {
    //   p_uvar3 = p_uvar10;
    //   p_uvar10 = p_uvar10 + 0x1;
    //   p_uvar1 = p_uvar9;
    //   p_uvar9 = p_uvar9 + 0x1;
    //   *p_uvar3 = *p_uvar1;
    // }
    // TODO: refactor for loop
    // for (u_var5 = ((u_var5 & 0x1) != 0x0); u_var5 != 0x0; u_var5 -= 0x1) {
    //   p_uvar3 = p_uvar10;
    //   p_uvar10 = (p_uvar10 + 0x1);
    //   p_uvar1 = p_uvar9;
    //   p_uvar9 = (p_uvar9 + 0x1);
    //   *p_uvar3 = *p_uvar1;
    // }
    return string_1;
}
