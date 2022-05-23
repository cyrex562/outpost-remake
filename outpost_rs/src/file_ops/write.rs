use crate::global::AppContext;
use crate::win_struct::HFILE16;
use crate::winapi::_hwrite16;

pub fn write_to_file_1008_7e1c(
    param_1: u16,
    param_2: u16,
    count: usize,
    param_4: u16,
    buf_to_write: &mut String,
    file_handle: HFILE16,
) -> bool {
    let mut bytes_written: usize;

    bytes_written = _hwrite16(file_handle, buf_to_write, count);
    if bytes_written == count {
        return true;
    }
    return false;
}

pub fn write_to_file_1030_32e4(ctx: &mut AppContext, param_1: u32, param_2: u32, param_3: u16) {
    let u_var1: u16;
    let i_var2: i16;
    let bvar3: bool;
    let u_var4: u16;
    let u_var5: u16;
    let local_16: [u32; 0x2];
    let local_c: u16;
    let local_a: [u32; 0x2];

    i_var2 = param_1;
    // u_var1 = (param_1 >> 0x10);
    u_var4 = param_2;
    // u_var5 = (param_2 >> 0x10);
    bvar3 = write_to_file_1008_7e1c(u_var4, u_var5, i_var2 + 0x4, u_var1, 0x16c, 0x1008);
    if (bvar3 != 0x0) {
        bvar3 = write_to_file_1008_7e1c(
            u_var4,
            u_var5,
            i_var2 + 0x174,
            u_var1,
            &ctx.DAT_0000_000c,
            0x1008,
        );
        if (bvar3 != 0x0) {
            bvar3 = write_to_file_1008_7e1c(
                u_var4,
                u_var5,
                i_var2 + 0x180,
                u_var1,
                &ctx.DAT_0000_000c,
                0x1008,
            );
            if (bvar3 != 0x0) {
                bvar3 =
                    write_to_file_1008_7e1c(u_var4, u_var5, i_var2 + 0x18c, u_var1, 0x18, 0x1008);
                if (bvar3 != 0x0) {
                    local_c = (i_var2 + 0x1a8);
                    bvar3 = write_to_file_1008_7e1c(u_var4, u_var5, &local_c, param_3, 0x2, 0x1008);
                    if (bvar3 != 0x0) {
                        local_16[0] = (i_var2 + 0x1aa);
                        bvar3 =
                            write_to_file_1008_7e1c(u_var4, u_var5, local_16, param_3, 0x4, 0x1008);
                        if (bvar3 != 0x0) {
                            local_a[0] = (i_var2 + 0x170);
                            bvar3 = write_to_file_1008_7e1c(
                                u_var4, u_var5, local_a, param_3, 0x4, 0x1008,
                            );
                            if (bvar3 != 0x0) {
                                local_c = (i_var2 + 0x1ae);
                                bvar3 = write_to_file_1008_7e1c(
                                    u_var4, u_var5, &local_c, param_3, 0x2, 0x1008,
                                );
                                if (bvar3 != 0x0) {
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ctx.PTR_LOOP_1050_0310 = 0x6d0;
    return;
}
