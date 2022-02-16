
void __stdcall16far struct_1040_b082(astruct_57 *param_1,ulong param_2)

{
  astruct_437 *iVar1;
  undefined2 uVar1;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,(ushort)param_2,(ushort)(param_2 >> 0x10));
  uVar1 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (astruct_437 *)param_1;
  iVar1->field_0x8e = 0x0;
  iVar1->field_0x90 = 0x0;
  *(undefined2 *)param_1 = 0xb772;
  iVar1->field_0x2 = (int)&PTR_LOOP_1050_1040;
  return;
}


void __stdcall16far pass1_1040_b040(astruct_57 *param_1,ulong param_2,ushort param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,*(ushort *)((int)param_2 + 0x12),param_3);
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 0x8e) = 0x0;
  *(ulong *)(iVar1 + 0x90) = param_2;
  *(undefined2 *)param_1 = 0xb772;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return;
}


void __stdcall16far pass1_1040_b0bc(astruct_57 *param_1,ulong param_2,ulong param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  get_sys_metrics_1040_7728(param_1,0x1,0x0,(ushort)param_3,(ushort)(param_3 >> 0x10));
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 0x8e) = 0x0;
  *(ulong *)(iVar1 + 0x90) = param_2;
  *(undefined2 *)param_1 = 0xb772;
  *(undefined2 *)(iVar1 + 0x2) = (int)&PTR_LOOP_1050_1040;
  return;
}


void __stdcall16far post_win_msg_1040_7f56(ulong param_1,char *param_2)

{
  unk_str_op_1000_3d3e((char *)(param_1 & 0xffff0000 | (ulong)((int)param_1 + 0x10)),param_2);
  PostMessage16(0x1000,0x0,0x0,0x850000);
  return;
}


BOOL16 __stdcall16far post_win_msg_1040_7f1c(ulong param_1,int param_2,HWND16 param_3)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 0x78) != 0x0) {
    return 0x0;
  }
  if (*(int *)(iVar1 + 0x60) != param_2) {
    *(int *)(iVar1 + 0x60) = param_2;
    PostMessage16(param_3,0x0,0x0,0x850000);
  }
  return 0x1;
}


