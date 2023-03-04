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

- 1008:051e
- 1008:052a
- 1008:380a
- 1008:389a
- 1008:3aa8
- 1008:3ab0
- 1008:3b46
- 1008:3cfc
- 1008:3e38
- 1008:4f1c
- 1008:5632
- 1008:5bc0
- 1008:5bc4
- 1008:5fc8
- 1008:6378
- 1008:685a
- 1008:6bfc
- 1008:6c8c
- 1008:8042
- 1008:84f2
- 1008:87c8
- 1008:8e9a
- 1008:9170
- 1008:9412
- 1008:9416
- 1008:9d2e
- 1008:9fb2
- 1008:9fca
- 1008:a230
- 1008:ad8a
- 1008:ad92
- 1008:af7c
- 1008:bdc0
- 1008:bdc4
- 1008:bdc8
- 1008:bdcc
- 1008:bddc
- 1008:ca4a
- 1008:d71a
- 1008:d780
- 1008:d98e
- 1008:d9fa
- 1008:dc80
- 1008:dd4a
- 1008:eaac
- 1008:eb1a
- 1008:ec00
- 1008:ec62
- 1008:ef9c
- 1008:efc4

### Segment 1010

- 1010:a1c4
- 1010:0ea8
- 1010:6322
- 1010:53f4
- 1010:37c4
- 1010:3b3e
- 1010:3b5e
- 1010:4a46
- 1010:4a82
- 1010:36da
- 1010:e9cc
- 1010:e9dc
- 1010:9e8c
- 1010:a1c8
- 1010:6aac
- 1010:3d6a
- 1010:3d7a
- 1010:2cc2
- 1010:2014
- 1010:191a
- 1010:8ee2
- 1010:9254
- 1010:9566
- 1010:7e28
- 1010:7e38
- 1010:6312
- 1010:502a
- 1010:509a
- 1010:3e2c
- 1010:2be4
- 1010:1b2a
- 1010:1d04
- 1010:2010
- 1010:e9cc
- 1010:e9dc
- 1010:958e
- 1010:02c8

### 1018

- 1018:e790
- 1018:e82c
- 1018:e912
- 1018:ebd0
- 1018:ec6c
- 1018:e44e
- 1018:e4ea
- 1018:e5d0
- 1018:5e1a
- 1018:6880
- 1018:691c
- 1018:6a02
- 1018:5a62
- 1018:5afe
- 1018:56ce


### 1020

### 1028

### 1030

### 1038

### 1040

### 1048

### 1050

1008:389a
       1008:389a 7e  37  08  10    addr       pass1_1008_377e
       1008:389e e4  37  08  10    addr       pass1_1008_37e4
       1008:38a2 ea  68  08  10    addr       pass1_1008_68ea
       1008:38a6 f8  0a  08  10    addr       window_op_1008_0af8
       1008:38aa c6  68  08  10    addr       pass1_1008_68c6
       1008:38ae 40  96  08  10    addr       send_msg_1008_9640
       1008:38b2 64  96  08  10    addr       set_win_text_1008_9664
       1008:38b6 2c  37  08  10    addr       pass1_1008_372c
       1008:38ba f2  97  08  10    addr       unk_win_op_1008_97f2
       1008:38be 3c  37  08  10    addr       pass1_1008_373c
       1008:38c2 40  37  08  10    addr       pass1_1008_3740
       1008:38c6 c0  06  08  10    addr       win_ui_cursor_op_1008_06c0
       1008:38ca 32  09  08  10    addr       pass1_1008_0932
       1008:38ce 84  09  08  10    addr       pass1_1008_0984
       1008:38d2 30  12  08  10    addr       draw_op_1008_1230
       1008:38d6 98  96  08  10    addr       destroy_win_1008_9698
       1008:38da 92  0a  08  10    addr       pass1_1008_0a92
       1008:38de 60  0c  08  10    addr       mixed_win_op_1008_0c60
       1008:38e2 60  9c  08  10    addr       pass1_1008_9c60
       1008:38e6 3c  0a  08  10    addr       unk_win_msg_op_1008_0a3c
       1008:38ea 46  12  08  10    addr       pass1_1008_1246
       1008:38ee 4e  9c  08  10    addr       pass1_1008_9c4e
       1008:38f2 62  37  08  10    addr       pass1_1008_3762
       1008:38f6 4a  9c  08  10    addr       pass1_1008_9c4a
       1008:38fa 66  37  08  10    addr       pass1_1008_3766
       1008:38fe ba  09  08  10    addr       menu_ui_op_1008_09ba
       1008:3902 4a  6a  08  10    addr       pass1_1008_6a4a
       1008:3906 2e  6b  08  10    addr       pass1_1008_6b2e
       1008:390a 02  6b  08  10    addr       pass1_1008_6b02
       1008:390e 7a  37  08  10    addr       pass1_1008_377a
       1008:3912 52  9c  08  10    addr       pass1_1008_9c52
       1008:3916 56  9c  08  10    addr       get_stock_obj_1008_9c56
       1008:391a 16  9c  08  10    addr       pass1_1008_9c16
       1008:391e 30  9c  08  10    addr       pass1_1008_9c30
       1008:3922 86  9c  08  10    addr       pass1_1008_9c86
       1008:3926 72  12  08  10    addr       pass1_1008_1272
       1008:392a aa  12  08  10    addr       pass1_1008_12aa
