

void pass1_1040_869a(Struct18 *param_1)

{
    i16 iVar1;
    u16 uVar2;

    uVar2              = (param_1 >> 0x10);
    iVar1              = param_1;
    param_1->field_0x0 = 0x8ddc;
    (iVar1 + 0x2)      = &PTR_LOOP_1050_1040;
    fn_ptr_1000_17ce(*(Struct18 **)(iVar1 + 0x90), 0x1000);
    fn_ptr_1000_17ce(*(Struct18 **)(iVar1 + 0x94), 0x1000);
    ui_cleanup_op_1040_782c(param_1, 0x1000);
    return;
}