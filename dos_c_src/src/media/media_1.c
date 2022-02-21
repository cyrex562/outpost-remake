
void __stdcall16far mci_send_command_1008_5cb6(ulong param_1,int param_2,ushort param_3)

{
  int iVar1;
  uint uVar2;
  ushort unaff_SS;
  
  mciSendCommand16(param_3,0x0,0x0,0x8040000);
  uVar2 = (uint)(param_1 >> 0x10);
  iVar1 = (int)param_1;
  if ((*(int *)(iVar1 + 0xa) == 0x0) || (*(int *)(iVar1 + 0xa) != param_2)) {
    *(undefined2 *)(iVar1 + 0x12) = 0x0;
    iVar1 = 0x11;
  }
  else {
    *(undefined2 *)(iVar1 + 0x10) = 0x0;
    iVar1 = 0x10;
  }
  pass1_1010_1f62(unaff_SS,param_1 & 0xffff | (ulong)uVar2 << 0x10,iVar1);
  return;
}



void __cdecl16far mci_send_command_1008_53ae(ulong param_1,ushort param_2,ushort param_3,ushort param_4)

{
  DWORD DVar1;
  CHAR local_432 [0x400];
  ushort local_32;
  undefined2 uStack48;
  ushort local_2e;
  undefined2 uStack44;
  uint uStack34;
  uint uStack32;
  undefined4 local_1e;
  int iStack26;
  undefined2 uStack22;
  undefined2 uStack20;
  ulong uStack18;
  undefined4 uStack14;
  undefined2 uStack10;
  undefined2 uStack8;
  ushort uStack6;
  
  local_1e = 0x0;
  uStack22 = 0x28c;
  uStack20 = SUB42(&USHORT_1050_1050,0x0);
  uStack18 = param_1;
  uStack14 = 0x0;
  uStack10 = 0x0;
  uStack8 = 0x4000;
  uStack6 = param_2;
  DVar1 = mciSendCommand16(param_3,(UINT16)&local_1e,CONCAT22(0x200,param_4),0x8030003);
  uStack32 = (uint)(DVar1 >> 0x10);
  uStack34 = (uint)DVar1;
  if (iStack26 != 0x0) {
    if ((uStack32 | uStack34) != 0x0) {
      mciGetErrorString16(0x4001538,local_432,param_4);
    }
    pass1_1000_4906((astruct_20 *)CONCAT22(param_4,&local_2e),(WNDCLASS16 *)0x0,0xc);
    local_2e = param_2;
    uStack44 = 0x0;
    DVar1 = mciSendCommand16(0x1000,(UINT16)&local_2e,CONCAT22(0x2,param_4),0x8060000);
    uStack32 = (uint)(DVar1 >> 0x10);
    uStack34 = (uint)DVar1;
    if ((uStack32 | uStack34) != 0x0) {
      mciGetErrorString16(0x4001538,local_432,param_4);
    }
    local_32 = param_2;
    uStack48 = 0x0;
    DVar1 = mciSendCommand16((UINT16)s_tile2_bmp_1050_1538,(UINT16)&local_32,CONCAT22(0x1,param_4),0x8040000);
    uStack32 = (uint)(DVar1 >> 0x10);
    uStack34 = (uint)DVar1;
    if ((uStack32 | uStack34) != 0x0) {
      mciGetErrorString16(0x4001538,local_432,param_4);
    }
  }
  return;
}

