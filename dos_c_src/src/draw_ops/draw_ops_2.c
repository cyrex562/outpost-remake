
void __stdcall16far pass1_1038_9a48(astruct_18 *param_1)

{
    param_1->field_0x0                  = 0x9af6;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    unk_draw_op_1040_b0f8(param_1);
    return;
}


void __stdcall16far pass1_1038_7d5c(astruct_18 *param_1)

{
    undefined2 uVar1;

    uVar1                               = (undefined2)((ulong)param_1 >> 0x10);
    param_1->field_0x0                  = 0x8876;
    *(undefined2 *)((int)param_1 + 0x2) = (int)&PTR_LOOP_1050_1038;
    pass1_1038_b6e0(_PTR_LOOP_1050_5b7c, *(int *)((int)param_1 + 0x6));
    unk_draw_op_1040_b0f8(param_1);
    return;
}
