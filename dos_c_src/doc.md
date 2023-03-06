# Notes

## Program API Calls

| API Call | Category | Description |
| -------- | -------- | ----------- |
| GetSaveFileName16 | | |
| InitTask
| Swi
| InitApp
| FatalAppExit
| LoadString
| MakeProcInstance16
| FindResource16
| _lcreat16
| _llseek16
| _lopen16
| DOS3Call
| GetPrivateProfileString
| WinExec16
| Win16_Hread
| _hwrite16
| SetBkColor16
| SetMapMode16
| SetTextColor16
| LineTo16
| MoveTo16
| Ellipse16
| Rectangle16
| TextOut16
| Polygon16
| Selectobject16
| CreateDC16
| CreatePen16
| CreateSolidBrush16
| GetCurrentPosition16
| GetStockObject16
| GetTextExtent16
| CreatePalette16
| StretchDIBits16
| SetDIBitsToDevice16
| MoveToEx16
| PostQuitMessage16
| MessageBox16
| GetCursorPos16
| SetFocus16
| GetProp16
| SetProp16
| ClientToScreen16
| ScreenToClient16
| GetWindowRect16
| GetWindowText16
| SetWindowText16
| BeginPaint16
| CreateWindow16
| ShowWindow16
| BringWindowToTop16
| EnumChildWindows16
| MoveWindow16
| RegisterClass16
| GetDC16
| GetWindow16
| SetCursor16
| ShowCursor16
| PtInRect16
| FillRect16
| FrameRect16
| DrawIcon16
| DrawText16
| CreateDialog16
| SetDlgItemText16
| GetDlgItemText16
| GetDlgItem16
| SetDlgItemInt16
| GetDlgItemInt16
| CheckRadioButton16
| CheckDlgButton16
| IsDlgButtonChecked16
| SendDlgItemMessage16
| MapDialogRect16
| DefWindowProc16
| GetMessage16
| PostMessage16
| SendMessage16
| DispatchMessage16
| CallWindowProc16
| UpdateWindow16
| InvalidateRect16
| ValidateRect16
| GetWindowWord16
| SetWindowWord16
| GetWindowLong16
| SetWindowLong16
| LoadMenu16
| CheckMenuItem16
| EnableMenuItem16
| LoadCursor16
| LoadIcon16
| LoadAccelerators16
| TranslateAccelerators16
| GetSysColors16
| SetSysColors16
| GetNextDlgTabItem16
| SetWindowPos16
| GetMenuState16
| GetDlgCtrlId16
| SelectPalette16
| RealizePalette16
| GetWindowPlacement16
| GetClassInfo16
| InsertMenu16
| ModifyMenu16
| CreateWindowEx16
| MciSendCommand16
| GetOpenFileName16

## Address Tables

- 1008:051e -> pass1_1008_04f8
- 1008:052a -> pass1_1008_04d2
- 1008:380a -> pass1_1008_37aa
- 1008:389a -> pass1_1008_377e
- 1008:3aa8 -> pass1_1008_3a14
- 1008:3ab0 -> pass1_1008_3a40
- 1008:3b46 -> pass1_1008_3afe
- 1008:3cfc -> pass1_1008_3cd6
- 1008:3e38 -> pass1_1008_3e38
- 1008:4f1c -> pass1_1008_4ef6
- 1008:5632 -> def_win_proc_1008_5632
- 1008:5bc0 -> pass1_1008_5b6e
- 1008:5bc4 -> pass1_1008_5b9a
- 1008:5fc8 -> pass1_1008_5fa2
- 1008:6378 -> pass1_1008_6330
- 1008:685a -> pass1_1008_6834
- 1008:6bfc -> pass1_1008_6bb4
- 1008:6c8c -> pass1_1008_6b5a
- 1008:8042 -> pass1_1008_7ffa
- 1008:84f2 -> win_sys_op_1008_84f2
- 1008:87c8 -> pass1_1008_87a2
- 1008:8e9a -> pass1_1008_8e74
- 1008:9170 -> pass1_1008_914a
- 1008:9412 -> pass1_1008_93c0
- 1008:9416 -> pass1_1008_93ec
- 1008:9d2e -> pass1_1008_9d02
- 1008:9fb2 -> pass1_1008_9fb2
- 1008:9fca -> pass1_1008_9f80
- 1008:a230 -> INVALID
- 1008:ad8a -> pass1_1008_ad38
- 1008:ad92 -> pass1_1008_ad92
- 1008:af7c -> pass1_1008_af56
- 1008:bdc0 -> pass1_1008_bd74
- 1008:bdc4 -> pass1_1008_bd4e
- 1008:bdc8 -> pass1_1008_bd28
- 1008:bdcc -> pass1_1008_bd02
- 1008:bddc -> pass1_1008_bd9a
- 1008:ca4a -> pass1_1008_ca24
- 1008:d71a -> pass1_1008_d6f4
- 1008:d780 -> pass1_1008_d75a
- 1008:d98e -> pass1_1008_d968
- 1008:d9fa -> pass1_1008_d9d4
- 1008:dc80 -> pass1_1008_dc80
- 1008:dd4a -> pass1_1008_dd1e
- 1008:eaac -> pass1_1008_ea86
- 1008:eb1a -> pass1_1008_eaf4
- 1008:ec00 -> pass1_1008_ebda
- 1008:ec62 -> pass1_1008_ec3c
- 1008:ef9c -> pass1_1008_ef76
- 1008:efc4 -> pass1_1008_ef50

### Segment 1010

- 1010:02c8 -> pass1_1010_02a2
- 1010:0ea8 -> pass1_1010_0e6c
- 1010:191a -> pass1_1010_18f4
- 1010:1b2a -> pass1_1010_1b04
- 1010:1d04 -> pass1_1010_1cde
- 1010:2010 -> pass1_1010_1fbe
- 1010:2014 -> pass1_1010_1fea
- 1010:2be4 -> pass1_1010_2bbe
- 1010:2cc2 -> pass1_1010_2c9c
- 1010:36da -> pass1_1010_36b4
- 1010:37c4 -> pass1_1010_379e
- 1010:3b3e -> pass1_1010_3af2
- 1010:3b5e -> pass1_1010_3b18
- 1010:3d6a -> FUN_1010_3d44
- 1010:3d7a -> pass1_1010_3d38
- 1010:3e2c -> pass1_1010_3e06
- 1010:4a46 -> pass1_1010_4a20
- 1010:4a82 -> pass1_1010_4994
- 1010:502a -> pass1_1010_5004
- 1010:509a -> INVALD
- 1010:53f4 -> pass1_1010_53ce
- 1010:6312 -> pass1_1010_62ec
- 1010:6322 -> pass1_1010_62a4
- 1010:6aac -> pass1_1010_6a86
- 1010:7e28 -> pass1_1010_7dfe
- 1010:7e38 -> pass1_1010_7dc6
- 1010:8ee2 -> pass1_1010_8ebc
- 1010:9254 -> pass1_1010_922e
- 1010:9566 -> pass1_1010_9540
- 1010:958e -> pass1_1010_951a
- 1010:9e8c -> INVALID
- 1010:a1c4 -> pass1_1010_a198
- 1010:a1c8 -> pass1_1010_a172
- 1010:e9cc -> FUN_1010_e9a6

### 1018

- 1018:0558 -> FUN_1018_0532
- 1018:0568 -> pass1_1018_0526
- 1018:1874 -> FUN_1018_184e
- 1018:18b0 -> pass1_1018_1842
- 1018:1fb0 -> pass1_1018_1f8a
- 1018:1fec -> pass1_1018_1f6a
- 1018:21e8 -> pass1_1018_21c2
- 1018:2ada -> FUN_1018_2ab4
- 1018:2af2 -> pass1_1018_2aa8
- 1018:32d8 -> FUN_1018_32b2
- 1018:3314 -> pass1_1018_32a6
- 1018:470c -> pass1_1018_46e6
- 1018:4a8a -> pass1_1018_4a64
- 1018:4a8e -> pass1_1018_49f2
- 1018:4a92 -> pass1_1018_4980
- 1018:4a96 -> pass1_1018_4a18
- 1018:4a9a -> pass1_1018_49a6
- 1018:4a9e -> pass1_1018_4a3e
- 1018:4aa2 -> pass1_1018_49cc
- 1018:4aa6 -> pass1_1018_495a
- 1018:4b06 -> pass1_1018_4ae0
- 1018:4c9e -> pass1_1018_4c78
- 1018:5058 -> pass1_1018_5032
- 1018:56ce -> pass1_1018_567c
- 1018:56d2 -> pass1_1018_56a8
- 1018:5830 -> pass1_1018_580a
- 1018:5a62 -> FUN_1018_5a3c
- 1018:5afe -> pass1_1018_5a2e
- 1018:5e1a -> pass1_1018_5df4
- 1018:6128 -> pass1_1018_6102
- 1018:66c0 -> pass1_1018_669a
- 1018:6880 -> FUN_1018_685a
- 1018:691c -> pass1_1018_684c
- 1018:6a02 -> pass1_1018_69dc
- 1018:6c66 -> pass1_1018_6c1e
- 1018:93de -> pass1_1018_8106
- 1018:947a -> pass1_1018_934e
- 1018:9516 -> pass1_1018_88e6
- 1018:95b2 -> pass1_1018_8ece
- 1018:964e -> pass1_1018_7f9e
- 1018:96ea -> pass1_1018_877e
- 1018:9786 -> pass1_1018_8d66
- 1018:9822 -> pass1_1018_7e36
- 1018:98be -> pass1_1018_8586
- 1018:995a -> pass1_1018_841e
- 1018:99f6 -> pass1_1018_922e
- 1018:9a92 -> pass1_1018_8346
- 1018:9b2e -> pass1_1018_8b26
- 1018:9bca -> pass1_1018_910e
- 1018:9c66 -> pass1_1018_81de
- 1018:9d02 -> pass1_1018_89be
- 1018:9e3a -> pass1_1018_8076
- 1018:9ed6 -> pass1_1018_92be
- 1018:9f72 -> pass1_1018_8856
- 1018:a00e -> pass1_1018_8e3e
- 1018:a0aa -> pass1_1018_7f0e
- 1018:a146 -> pass1_1018_86ee
- 1018:a1e2 -> pass1_1018_ecd6
- 1018:a27e -> pass1_1018_7da6
- 1018:a31a -> pass1_1018_a4f6
- 1018:a3b6 -> pass1_1018_8466
- 1018:a452 -> pass1_1018_8466
- 1018:a4ee -> pass1_1018_a2b6
- 1018:a58a -> pass1_1018_8a96
- 1018:a626 -> pass1_1018_907e
- 1018:a6c2 -> pass1_1018_814e
- 1018:a75e -> pass1_1018_9396
- 1018:a7fa -> pass1_1018_892e
- 1018:a896 -> pass1_1018_8f16
- 1018:a932 -> pass1_1018_7fe6
- 1018:a9ce -> pass1_1018_87c6
- 1018:aa6a -> pass1_1018_8dae
- 1018:ab06 -> pass1_1018_7e7e
- 1018:aba2 -> pass1_1018_85ce
- 1018:ac3e -> pass1_1018_865e
- 1018:acda -> pass1_1018_8c46
- 1018:ad76 -> pass1_1018_838e
- 1018:ae12 -> pass1_1018_8b6e
- 1018:aeae -> pass1_1018_9156
- 1018:af4a -> pass1_1018_8226
- 1018:afe6 -> pass1_1018_8a06
- 1018:b082 -> pass1_1018_8fee
- 1018:b11e -> pass1_1018_80be
- 1018:b1ba -> pass1_1018_9306
- 1018:b256 -> pass1_1018_889e
- 1018:b2f2 -> pass1_1018_8e86
- 1018:b38e -> pass1_1018_7f56
- 1018:b42a -> pass1_1018_8736
- 1018:b4c6 -> pass1_1018_8d1e
- 1018:b562 -> pass1_1018_7dee
- 1018:b5fe -> pass1_1018_853e
- 1018:b69a -> pass1_1018_83d6
- 1018:b736 -> pass1_1018_91e6
- 1018:b7d2 -> pass1_1018_82fe
- 1018:b86e -> pass1_1018_8ade
- 1018:b90a -> pass1_1018_90c6
- 1018:b9a6 -> pass1_1018_8196
- 1018:ba42 -> pass1_1018_9276
- 1018:bade -> pass1_1018_8976
- 1018:bb7a -> pass1_1018_8f5e
- 1018:bc16 -> pass1_1018_802e
- 1018:bcb2 -> pass1_1018_880e
- 1018:bd4e -> pass1_1018_8df6
- 1018:bdea -> pass1_1018_7ec6
- 1018:be86 -> pass1_1018_8616
- 1018:bf22 -> pass1_1018_86a6
- 1018:bfbe -> pass1_1018_8c8e
- 1018:c05a -> pass1_1018_84ae
- 1018:c0f6 -> pass1_1018_8bb6
- 1018:c192 -> pass1_1018_919e
- 1018:c22e -> pass1_1018_826e
- 1018:c2ca -> pass1_1018_8a4e
- 1018:c366 -> pass1_1018_9036
- 1018:c8bc -> pass1_1018_c896
- 1018:d3d2 -> pass1_1018_d386
- 1018:d46e -> pass1_1018_d2c8
- 1018:d50a -> pass1_1018_d20a
- 1018:d5a6 -> pass1_1018_d33a
- 1018:d642 -> pass1_1018_d27c
- 1018:d6de -> pass1_1018_d1be
- 1018:d77a -> pass1_1018_d3ac
- 1018:d816 -> pass1_1018_d2ee
- 1018:d8b2 -> pass1_1018_d230
- 1018:d94e -> pass1_1018_d360
- 1018:d9ea -> pass1_1018_d2a2
- 1018:da86 -> pass1_1018_d1e4
- 1018:db22 -> pass1_1018_d314
- 1018:dbbe -> pass1_1018_d256
- 1018:dc5a -> pass1_1018_d198
- 1018:df3c -> pass1_1018_df10
- 1018:e228 -> pass1_1018_e1ee
- 1018:e44e -> pass1_1018_e428
- 1018:e4ea -> pass1_1018_e41a
- 1018:e5d0 -> pass1_1018_e5aa
- 1018:e790 -> pass1_1018_e76a
- 1018:e82c -> pass1_1018_e75c
- 1018:e912 -> pass1_1018_e8ec
- 1018:ebd0 -> pass1_1018_ebaa
- 1018:ec6c -> pass1_1018_eb9c

### 1020

- 1020:01cc -> pass1_1020_01a6
- 1020:045a -> pass1_1020_0434
- 1020:075a -> pass1_1020_0734
- 1020:081a -> pass1_1020_07f4
- 1020:0b0e -> pass1_1020_0ae8
- 1020:0dbc -> pass1_1020_0d82
- 1020:1384 -> pass1_1020_135e
- 1020:1730 -> pass1_1020_170a
- 1020:1e7a -> pass1_1020_1e54
- 1020:2518 -> pass1_1020_24f2
- 1020:270c -> FUN_1020_26e6
- 1020:27a8 -> pass1_1020_26d8
- 1020:288e -> pass1_1020_2868
- 1020:2e4a -> pass1_1020_2e24
- 1020:363c -> pass1_1020_3616
- 1020:3d08 -> pass1_1020_3d08
- 1020:3d9c -> INVALID
- 1020:408a -> pass1_1020_4064
- 1020:623c -> FUN_1020_6216
- 1020:62d8 -> pass1_1020_6208
- 1020:67c2 -> pass1_1020_679c
- 1020:70e6 -> pass1_1020_70c0
- 1020:754c -> pass1_1020_7526
- 1020:7780 -> pass1_1020_775a
- 1020:781c -> pass1_1020_774c
- 1020:7902 -> pass1_1020_78dc
- 1020:7b86 -> pass1_1020_7b60
- 1020:7f72 -> pass1_1020_7f38
- 1020:82b6 -> INVALID
- 1020:8358 -> pass1_1020_8288
- 1020:8462 -> pass1_1020_843c
- 1020:87aa -> pass1_1020_8784
- 1020:8a84 -> pass1_1020_8a5e
- 1020:8e92 -> pass1_1020_8e6c
- 1020:9204 -> pass1_1020_91de
- 1020:96c8 -> pass1_1020_96a2
- 1020:ba36 -> pass1_1020_a644
- 1020:c834 -> pass1_1020_c80e
- 1020:c9e6 -> pass1_1020_c9ba
- 1020:cc7c -> pass1_1020_cc56
- 1020:cd7e -> pass1_1020_cd58
- 1020:d004 -> pass1_1020_cfde
- 1020:d314 -> pass1_1020_d2ee
- 1020:d53e -> pass1_1020_d518
- 1020:d7fe -> pass1_1020_d7d8
- 1020:d8ec -> pass1_1020_d8c6
- 1020:e792 -> pass1_1020_e76c
- 1020:e88e -> pass1_1020_e868
- 1020:eef6 -> pass1_1020_eed0

### 1028

- 1028:0ada -> pass1_1028_0ab4

### 1030

- 1030:10b0 -> pass1_1030_10b0
- 1030:1120 -> pass1_1030_1120
- 1030:11a6 -> pass1_1030_117a
- 1030:1624 -> pass1_1030_15fe
- 1030:17ba -> pass1_1030_1794
- 1030:1a16 -> pass1_1030_19f0
- 1030:1cbc -> pass1_1030_1c96
- 1030:2044 -> pass1_1030_201e
- 1030:293c -> pass1_1030_2916
- 1030:3130 -> pass1_1030_310a
- 1030:3af2 -> pass1_1030_3ac6
- 1030:55ee -> pass1_1030_55c2
- 1030:55fe -> pass1_1030_5596
- 1030:5bd0 -> pass1_1030_5baa
- 1030:613e -> pass1_1030_6118
- 1030:8114 -> pass1_1030_80ee
- 1030:8e38 -> pass1_1030_8e12
- 1030:9788 -> INVALID
- 1030:9ec8 -> pass1_1030_9e9c
- 1030:b932 -> pass1_1030_b90c
- 1030:bc0c -> pass1_1030_bbe6
- 1030:bc96 -> pass1_1030_bc70
- 1030:c006 -> pass1_1030_bfe0
- 1030:c68e -> pass1_1030_c668
- 1030:c940 -> pass1_1030_c91a
- 1030:d88e -> pass1_1030_d868
- 1030:dc2e -> pass1_1030_dc08
- 1030:e036 -> pass1_1030_e010
- 1030:e2ae -> pass1_1030_e282
- 1030:e4ea -> pass1_1030_e4be
- 1030:e62e -> pass1_1030_e602
- 1030:e78a -> pass1_1030_e75e
- 1030:e890 -> pass1_1030_e864
- 1030:eb40 -> pass1_1030_eb14
- 1030:ecb2 -> pass1_1030_ec86

### 1038

NONE

### 1040

- 1040:d8c4 -> pass1_1040_d89e
- 1040:d07c -> pass1_1040_d056

### 1048

NONE

### 1050

NONE
