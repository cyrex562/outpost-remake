# Notes

## Program API Calls

| API Call | Category | Description |
| -------- | -------- | ----------- |
| GetSaveFileName16 | | |
| InitTask | | |
| Swi | | |
| InitApp | | |
| FatalAppExit | | |
| LoadString | | |
| MakeProcInstance16 | | |
| FindResource16 | | |
| _lcreat16 | | |
| _llseek16 | | |
| _lopen16 | | |
| DOS3Call | | |
| GetPrivateProfileString | | |
| WinExec16 | | |
| Win16_Hread | | |
| _hwrite16 | | |
| SetBkColor16 | | |
| SetMapMode16 | | |
| SetTextColor16 | | |
| LineTo16 | | |
| MoveTo16 | | |
| Ellipse16 | | |
| Rectangle16 | | |
| TextOut16 | | |
| Polygon16 | | |
| Selectobject16 | | |
| CreateDC16 | | |
| CreatePen16 | | |
| CreateSolidBrush16 | | |
| GetCurrentPosition16 | | |
| GetStockObject16 | | |
| GetTextExtent16 | | |
| CreatePalette16 | | |
| StretchDIBits16 | | |
| SetDIBitsToDevice16 | | |
| MoveToEx16 | | |
| PostQuitMessage16 | | |
| MessageBox16 | | |
| GetCursorPos16 | | |
| SetFocus16 | | |
| GetProp16 | | |
| SetProp16 | | |
| ClientToScreen16 | | |
| ScreenToClient16 | | |
| GetWindowRect16 | | |
| GetWindowText16 | | |
| SetWindowText16 | | |
| BeginPaint16 | | |
| CreateWindow16 | | |
| ShowWindow16 | | |
| BringWindowToTop16 | | |
| EnumChildWindows16 | | |
| MoveWindow16 | | |
| RegisterClass16 | | |
| GetDC16 | | |
| GetWindow16 | | |
| SetCursor16 | | |
| ShowCursor16 | | |
| PtInRect16 | | |
| FillRect16 | | |
| FrameRect16 | | |
| DrawIcon16 | | |
| DrawText16 | | |
| CreateDialog16 | | |
| SetDlgItemText16 | | |
| GetDlgItemText16 | | |
| GetDlgItem16 | | |
| SetDlgItemInt16 | | |
| GetDlgItemInt16 | | |
| CheckRadioButton16 | | |
| CheckDlgButton16 | | |
| IsDlgButtonChecked16 | | |
| SendDlgItemMessage16 | | |
| MapDialogRect16 | | |
| DefWindowProc16 | | |
| GetMessage16 | | |
| PostMessage16 | | |
| SendMessage16 | | |
| DispatchMessage16 | | |
| CallWindowProc16 | | |
| UpdateWindow16 | | |
| InvalidateRect16 | | |
| ValidateRect16 | | |
| GetWindowWord16 | | |
| SetWindowWord16 | | |
| GetWindowLong16 | | |
| SetWindowLong16 | | |
| LoadMenu16 | | |
| CheckMenuItem16 | | |
| EnableMenuItem16 | | |
| LoadCursor16 | | |
| LoadIcon16 | | |
| LoadAccelerators16 | | |
| TranslateAccelerators16 | | |
| GetSysColors16 | | |
| SetSysColors16 | | |
| GetNextDlgTabItem16 | | |
| SetWindowPos16 | | |
| GetMenuState16 | | |
| GetDlgCtrlId16 | | |
| SelectPalette16 | | |
| RealizePalette16 | | |
| GetWindowPlacement16 | | |
| GetClassInfo16 | | |
| InsertMenu16 | | |
| ModifyMenu16 | | |
| CreateWindowEx16 | | |
| MciSendCommand16 | | |
| GetOpenFileName16 | | |

## Address Tables

- 1008:051e

    1008:051e  f8  04  08  10       addr         pass1_1008_04f8
    1008:0522  9c  04  08  10       addr         pass1_1008_049c
    1008:0526  6e  01  08  10       addr         mixed_win_sys_op_1008_016e
    1008:052a  d2  04  08  10       addr         pass1_1008_04d2
    1008:052e  94  94  08  10       addr         LAB_1008_9494
    1008:0532  90  94  08  10       addr         LAB_1008_9490

- 1008:380a
- 1008:389a

    1008:380a  aa  37  08  10       addr         pass1_1008_37aa
    1008:380e  14  37  08  10       addr         pass1_1008_3714
    1008:3812  60  97  08  10       addr         create_window_ex_1008_9760
    1008:3816  ae  96  08  10       addr         show_win_1008_96ae
    1008:381a  40  96  08  10       addr         send_msg_1008_9640
    1008:381e  64  96  08  10       addr         set_win_text_1008_9664
    1008:3822  2c  37  08  10       addr         pass1_1008_372c
    1008:3826  f2  97  08  10       addr         unk_win_op_1008_97f2
    1008:382a  3c  37  08  10       addr         pass1_1008_373c
    1008:382e  40  37  08  10       addr         pass1_1008_3740
    1008:3832  44  37  08  10       addr         pass1_1008_3744
    1008:3836  48  37  08  10       addr         pass1_1008_3748
    1008:383a  4c  37  08  10       addr         pass1_1008_374c
    1008:383e  c8  97  08  10       addr         begin_end_paint_1008_97c8
    1008:3842  98  96  08  10       addr         destroy_win_1008_9698
    1008:3846  50  37  08  10       addr         pass1_1008_3750
    1008:384a  54  37  08  10       addr         pass1_1008_3754
    1008:384e  60  9c  08  10       addr         pass1_1008_9c60
    1008:3852  58  37  08  10       addr         pass1_1008_3758
    1008:3856  5e  37  08  10       addr         pass1_1008_375e
    1008:385a  4e  9c  08  10       addr         pass1_1008_9c4e
    1008:385e  62  37  08  10       addr         pass1_1008_3762
    1008:3862  4a  9c  08  10       addr         pass1_1008_9c4a
    1008:3866  66  37  08  10       addr         pass1_1008_3766
    1008:386a  6a  37  08  10       addr         FUN_1008_376a
    1008:386e  6e  37  08  10       addr         FUN_1008_376e
    1008:3872  72  37  08  10       addr         FUN_1008_3772
    1008:3876  76  37  08  10       addr         FUN_1008_3776
    1008:387a  7a  37  08  10       addr         pass1_1008_377a
    1008:387e  52  9c  08  10       addr         pass1_1008_9c52
    1008:3882  56  9c  08  10       addr         get_stock_obj_1008_9c56
    1008:3886  16  9c  08  10       addr         pass1_1008_9c16
    1008:388a  30  9c  08  10       addr         pass1_1008_9c30
    1008:388e  86  9c  08  10       addr         pass1_1008_9c86
    1008:3892  c4  9c  08  10       addr         pass1_1008_9cc4
    1008:3896  e0  9c  08  10       addr         pass1_1008_9ce0
    1008:389a  7e  37  08  10       addr         pass1_1008_377e
    1008:389e  e4  37  08  10       addr         pass1_1008_37e4
    1008:38a2  ea  68  08  10       addr         pass1_1008_68ea
    1008:38a6  f8  0a  08  10       addr         window_op_1008_0af8
    1008:38aa  c6  68  08  10       addr         pass1_1008_68c6
    1008:38ae  40  96  08  10       addr         send_msg_1008_9640
    1008:38b2  64  96  08  10       addr         set_win_text_1008_9664
    1008:38b6  2c  37  08  10       addr         pass1_1008_372c
    1008:38ba  f2  97  08  10       addr         unk_win_op_1008_97f2
    1008:38be  3c  37  08  10       addr         pass1_1008_373c
    1008:38c2  40  37  08  10       addr         pass1_1008_3740
    1008:38c6  c0  06  08  10       addr         win_ui_cursor_op_1008_06c0
    1008:38ca  32  09  08  10       addr         pass1_1008_0932
    1008:38ce  84  09  08  10       addr         pass1_1008_0984
    1008:38d2  30  12  08  10       addr         draw_op_1008_1230
    1008:38d6  98  96  08  10       addr         destroy_win_1008_9698
    1008:38da  92  0a  08  10       addr         pass1_1008_0a92
    1008:38de  60  0c  08  10       addr         mixed_win_op_1008_0c60
    1008:38e2  60  9c  08  10       addr         pass1_1008_9c60
    1008:38e6  3c  0a  08  10       addr         unk_win_msg_op_1008_0a3c
    1008:38ea  46  12  08  10       addr         pass1_1008_1246
    1008:38ee  4e  9c  08  10       addr         pass1_1008_9c4e
    1008:38f2  62  37  08  10       addr         pass1_1008_3762
    1008:38f6  4a  9c  08  10       addr         pass1_1008_9c4a
    1008:38fa  66  37  08  10       addr         pass1_1008_3766
    1008:38fe  ba  09  08  10       addr         menu_ui_op_1008_09ba
    1008:3902  4a  6a  08  10       addr         pass1_1008_6a4a
    1008:3906  2e  6b  08  10       addr         pass1_1008_6b2e
    1008:390a  02  6b  08  10       addr         pass1_1008_6b02
    1008:390e  7a  37  08  10       addr         pass1_1008_377a
    1008:3912  52  9c  08  10       addr         pass1_1008_9c52
    1008:3916  56  9c  08  10       addr         get_stock_obj_1008_9c56
    1008:391a  16  9c  08  10       addr         pass1_1008_9c16
    1008:391e  30  9c  08  10       addr         pass1_1008_9c30
    1008:3922  86  9c  08  10       addr         pass1_1008_9c86
    1008:3926  72  12  08  10       addr         pass1_1008_1272
    1008:392a  aa  12  08  10       addr         pass1_1008_12aa

- 1008:3aa8
- 1008:3ab0

    1008:3aa0  7a  3a  08  10       addr *       pass1_1008_3a7a
    1008:3aa4  10  3a  08  10       addr *       pass1_1008_3a10
    1008:3aa8  14  3a  08  10       addr *       pass1_1008_3a14
    1008:3aac  10  3a  08  10       addr *       pass1_1008_3a10
    1008:3ab0  40  3a  08  10       addr *       pass1_1008_3a40
    1008:3ab4  10  3a  08  10       addr *       pass1_1008_3a10

- 1008:3b46

    1008:3b46  fe  3a  08  10       addr         pass1_1008_3afe
    1008:3b4a  ea  68  08  10       addr         pass1_1008_68ea
    1008:3b4e  60  97  08  10       addr         create_window_ex_1008_9760
    1008:3b52  c6  68  08  10       addr         pass1_1008_68c6
    1008:3b56  40  96  08  10       addr         send_msg_1008_9640
    1008:3b5a  64  96  08  10       addr         set_win_text_1008_9664
    1008:3b5e  2c  37  08  10       addr         pass1_1008_372c
    1008:3b62  f2  97  08  10       addr         unk_win_op_1008_97f2
    1008:3b66  3c  37  08  10       addr         pass1_1008_373c
    1008:3b6a  40  37  08  10       addr         pass1_1008_3740
    1008:3b6e  44  37  08  10       addr         pass1_1008_3744
    1008:3b72  48  37  08  10       addr         pass1_1008_3748
    1008:3b76  4c  37  08  10       addr         pass1_1008_374c
    1008:3b7a  c8  97  08  10       addr         begin_end_paint_1008_97c8
    1008:3b7e  98  96  08  10       addr         destroy_win_1008_9698
    1008:3b82  f4  3a  08  10       addr         post_quit_msg_1008_3af4
    1008:3b86  54  37  08  10       addr         pass1_1008_3754
    1008:3b8a  60  9c  08  10       addr         pass1_1008_9c60
    1008:3b8e  58  37  08  10       addr         pass1_1008_3758
    1008:3b92  5e  37  08  10       addr         pass1_1008_375e
    1008:3b96  4e  9c  08  10       addr         pass1_1008_9c4e
    1008:3b9a  62  37  08  10       addr         pass1_1008_3762
    1008:3b9e  4a  9c  08  10       addr         pass1_1008_9c4a
    1008:3ba2  66  37  08  10       addr         pass1_1008_3766
    1008:3ba6  6a  37  08  10       addr         FUN_1008_376a
    1008:3baa  4a  6a  08  10       addr         pass1_1008_6a4a
    1008:3bae  2e  6b  08  10       addr         pass1_1008_6b2e
    1008:3bb2  02  6b  08  10       addr         pass1_1008_6b02
    1008:3bb6  7a  37  08  10       addr         pass1_1008_377a
    1008:3bba  52  9c  08  10       addr         pass1_1008_9c52
    1008:3bbe  56  9c  08  10       addr         get_stock_obj_1008_9c56
    1008:3bc2  16  9c  08  10       addr         pass1_1008_9c16
    1008:3bc6  30  9c  08  10       addr         pass1_1008_9c30
    1008:3bca  86  9c  08  10       addr         pass1_1008_9c86
    1008:3bce  c4  9c  08  10       addr         pass1_1008_9cc4
    1008:3bd2  e0  9c  08  10       addr         pass1_1008_9ce0

- 1008:3cfc

    1008:3cfc  d6  3c  08  10       addr    pass1_1008_3cd6
    1008:3d00  52  92  40  10       addr    pass1_1040_9252
    1008:3d04  dc  92  40  10       addr    create_window_1040_92dc
    1008:3d08  22  94  40  10       addr    pass1_1040_9422
    1008:3d0c  34  3c  08  10       addr    win_ui_op_1008_3c34
    1008:3d10  fc  94  40  10       addr    draw_text_1040_94fc
    1008:3d14  d2  3c  08  10       addr    FUN_1008_3cd2
    1008:3d18  e6  93  40  10       addr    pass1_1040_93e6
    1008:3d1c  04  94  40  10       addr    send_msg_1040_9404

- 1008:3e38 -> pass1_1008_3e38; function
- 1008:4f1c -> pass1_1008_4ef6g

    1008:4f1c  f6  4e  08  10       addr         pass1_1008_4ef6

- 1008:5632 -> def_win_proc_1008_5632; function
- 1008:5bc0
- 1008:5bc4

    1008:5bc0  6e  5b  08  10       addr         pass1_1008_5b6e
    1008:5bc4  9a  5b  08  10       addr         pass1_1008_5b9a
    1008:5bc8  a6  58  08  10       addr         pass1_1008_58a6
    1008:5bcc  3c  59  08  10       addr         pass1_1008_593c
    1008:5bd0  f4  59  08  10       addr         pass1_1008_59f4
    1008:5bd4  b8  5a  08  10       addr         pass1_1008_5ab8
    1008:5bd8  30  58  08  10       addr         pass1_1008_5830

- 1008:5fc8

    1008:5fc8  a2  5f  08  10       addr         pass1_1008_5fa2
    1008:5fcc  f2  1d  10  10       addr         pass1_1010_1df2
    1008:5fd0  ce  1d  10  10       addr         pass1_1010_1dce
    1008:5fd4  d4  1d  10  10       addr         pass1_1010_1dd4

- 1008:6378

    1008:6378  30  63  08  10       addr         pass1_1008_6330
    1008:637c  ea  68  08  10       addr         pass1_1008_68ea
    1008:6380  60  97  08  10       addr         create_window_ex_1008_9760
    1008:6384  c6  68  08  10       addr         pass1_1008_68c6
    1008:6388  40  96  08  10       addr         send_msg_1008_9640
    1008:638c  64  96  08  10       addr         set_win_text_1008_9664
    1008:6390  2c  37  08  10       addr         pass1_1008_372c
    1008:6394  f2  97  08  10       addr         unk_win_op_1008_97f2
    1008:6398  3c  37  08  10       addr         pass1_1008_373c
    1008:639c  40  37  08  10       addr         pass1_1008_3740
    1008:63a0  44  37  08  10       addr         pass1_1008_3744
    1008:63a4  48  37  08  10       addr         pass1_1008_3748
    1008:63a8  4c  37  08  10       addr         pass1_1008_374c
    1008:63ac  c0  62  08  10       addr         fill_rect_1008_62c0
    1008:63b0  98  96  08  10       addr         destroy_win_1008_9698
    1008:63b4  50  37  08  10       addr         pass1_1008_3750
    1008:63b8  54  37  08  10       addr         pass1_1008_3754
    1008:63bc  60  9c  08  10       addr         pass1_1008_9c60
    1008:63c0  58  37  08  10       addr         pass1_1008_3758
    1008:63c4  24  63  08  10       addr         FUN_1008_6324
    1008:63c8  4e  9c  08  10       addr         pass1_1008_9c4e
    1008:63cc  62  37  08  10       addr         pass1_1008_3762
    1008:63d0  4a  9c  08  10       addr         pass1_1008_9c4a
    1008:63d4  66  37  08  10       addr         pass1_1008_3766
    1008:63d8  6a  37  08  10       addr         FUN_1008_376a
    1008:63dc  4a  6a  08  10       addr         pass1_1008_6a4a
    1008:63e0  2e  6b  08  10       addr         pass1_1008_6b2e
    1008:63e4  02  6b  08  10       addr         pass1_1008_6b02
    1008:63e8  7a  37  08  10       addr         pass1_1008_377a
    1008:63ec  52  9c  08  10       addr         pass1_1008_9c52
    1008:63f0  56  9c  08  10       addr         get_stock_obj_1008_9c56
    1008:63f4  16  9c  08  10       addr         pass1_1008_9c16
    1008:63f8  30  9c  08  10       addr         pass1_1008_9c30
    1008:63fc  86  9c  08  10       addr         pass1_1008_9c86
    1008:6400  c4  9c  08  10       addr         pass1_1008_9cc4
    1008:6404  e0  9c  08  10       addr         pass1_1008_9ce0
    1008:6408  8e  62  08  10       addr         destroy_win_1008_628e
    1008:640c  28  63  08  10       addr         FUN_1008_6328
    1008:6410  2c  63  08  10       addr         FUN_1008_632c

- 1008:685a

    1008:685a  34  68  08  10       addr         pass1_1008_6834
    1008:685e  1a  68  08  10       addr         FUN_1008_681a
    1008:6862  22  68  08  10       addr         FUN_1008_6822
    1008:6866  e2  56  08  10       addr         cleanup_palette_1008_56e2
    1008:686a  1e  68  08  10       addr         FUN_1008_681e
    1008:686e  28  68  08  10       addr         FUN_1008_6828
    1008:6872  2e  68  08  10       addr         FUN_1008_682e
    1008:6876  14  68  08  10       addr         FUN_1008_6814

- 1008:6bfc
- 1008:6c8c

    1008:6bfc  b4  6b  08  10       addr         pass1_1008_6bb4
    1008:6c00  ea  68  08  10       addr         pass1_1008_68ea
    1008:6c04  60  97  08  10       addr         create_window_ex_1008_9760
    1008:6c08  c6  68  08  10       addr         pass1_1008_68c6
    1008:6c0c  40  96  08  10       addr         send_msg_1008_9640
    1008:6c10  64  96  08  10       addr         set_win_text_1008_9664
    1008:6c14  2c  37  08  10       addr         pass1_1008_372c
    1008:6c18  f2  97  08  10       addr         unk_win_op_1008_97f2
    1008:6c1c  3c  37  08  10       addr         pass1_1008_373c
    1008:6c20  40  37  08  10       addr         pass1_1008_3740
    1008:6c24  44  37  08  10       addr         pass1_1008_3744
    1008:6c28  48  37  08  10       addr         pass1_1008_3748
    1008:6c2c  4c  37  08  10       addr         pass1_1008_374c
    1008:6c30  c8  97  08  10       addr         begin_end_paint_1008_97c8
    1008:6c34  98  96  08  10       addr         destroy_win_1008_9698
    1008:6c38  50  37  08  10       addr         pass1_1008_3750
    1008:6c3c  54  37  08  10       addr         pass1_1008_3754
    1008:6c40  60  9c  08  10       addr         pass1_1008_9c60
    1008:6c44  58  37  08  10       addr         pass1_1008_3758
    1008:6c48  5e  37  08  10       addr         pass1_1008_375e
    1008:6c4c  4e  9c  08  10       addr         pass1_1008_9c4e
    1008:6c50  62  37  08  10       addr         pass1_1008_3762
    1008:6c54  4a  9c  08  10       addr         pass1_1008_9c4a
    1008:6c58  66  37  08  10       addr         pass1_1008_3766
    1008:6c5c  6a  37  08  10       addr         FUN_1008_376a
    1008:6c60  4a  6a  08  10       addr         pass1_1008_6a4a
    1008:6c64  2e  6b  08  10       addr         pass1_1008_6b2e
    1008:6c68  02  6b  08  10       addr         pass1_1008_6b02
    1008:6c6c  7a  37  08  10       addr         pass1_1008_377a
    1008:6c70  52  9c  08  10       addr         pass1_1008_9c52
    1008:6c74  56  9c  08  10       addr         get_stock_obj_1008_9c56
    1008:6c78  16  9c  08  10       addr         pass1_1008_9c16
    1008:6c7c  30  9c  08  10       addr         pass1_1008_9c30
    1008:6c80  86  9c  08  10       addr         pass1_1008_9c86
    1008:6c84  c4  9c  08  10       addr         pass1_1008_9cc4
    1008:6c88  e0  9c  08  10       addr         pass1_1008_9ce0
    1008:6c8c  5a  6b  08  10       addr         pass1_1008_6b5a

- 1008:8042

    1008:8042  fa  7f  08  10       addr         pass1_1008_7ffa
    1008:8046  ea  68  08  10       addr         pass1_1008_68ea
    1008:804a  60  97  08  10       addr         create_window_ex_1008_9760
    1008:804e  c6  68  08  10       addr         pass1_1008_68c6
    1008:8052  40  96  08  10       addr         send_msg_1008_9640
    1008:8056  64  96  08  10       addr         set_win_text_1008_9664
    1008:805a  2c  37  08  10       addr         pass1_1008_372c
    1008:805e  f2  97  08  10       addr         unk_win_op_1008_97f2
    1008:8062  3c  37  08  10       addr         pass1_1008_373c
    1008:8066  40  37  08  10       addr         pass1_1008_3740
    1008:806a  44  37  08  10       addr         pass1_1008_3744
    1008:806e  48  37  08  10       addr         pass1_1008_3748
    1008:8072  4c  37  08  10       addr         pass1_1008_374c
    1008:8076  c8  97  08  10       addr         begin_end_paint_1008_97c8
    1008:807a  98  96  08  10       addr         destroy_win_1008_9698
    1008:807e  50  37  08  10       addr         pass1_1008_3750
    1008:8082  54  37  08  10       addr         pass1_1008_3754
    1008:8086  60  9c  08  10       addr         pass1_1008_9c60
    1008:808a  58  37  08  10       addr         pass1_1008_3758
    1008:808e  5e  37  08  10       addr         pass1_1008_375e
    1008:8092  4e  9c  08  10       addr         pass1_1008_9c4e
    1008:8096  62  37  08  10       addr         pass1_1008_3762
    1008:809a  4a  9c  08  10       addr         pass1_1008_9c4a
    1008:809e  66  37  08  10       addr         pass1_1008_3766
    1008:80a2  6a  37  08  10       addr         FUN_1008_376a
    1008:80a6  4a  6a  08  10       addr         pass1_1008_6a4a
    1008:80aa  2e  6b  08  10       addr         pass1_1008_6b2e
    1008:80ae  02  6b  08  10       addr         pass1_1008_6b02
    1008:80b2  7a  37  08  10       addr         pass1_1008_377a
    1008:80b6  52  9c  08  10       addr         pass1_1008_9c52
    1008:80ba  56  9c  08  10       addr         get_stock_obj_1008_9c56
    1008:80be  16  9c  08  10       addr         pass1_1008_9c16
    1008:80c2  30  9c  08  10       addr         pass1_1008_9c30
    1008:80c6  86  9c  08  10       addr         pass1_1008_9c86
    1008:80ca  c4  9c  08  10       addr         pass1_1008_9cc4
    1008:80ce  e0  9c  08  10       addr         pass1_1008_9ce0

- 1008:84f2 -> win_sys_op_1008_84f2; function
- 1008:87c8

    1008:87c8  a2  87  08  10       addr         pass1_1008_87a2

- 1008:8e9a

    1008:8e9a  74  8e  08  10       addr         pass1_1008_8e74

- 1008:9170

    1008:9170  4a  91  08  10       addr         pass1_1008_914a

- 1008:9412
- 1008:9416

    1008:9412  c0  93  08  10       addr         pass1_1008_93c0
    1008:9416  ec  93  08  10       addr         pass1_1008_93ec

- 1008:9d2e

    1008:9d2e  02  9d  08  10       addr         pass1_1008_9d02
    1008:9d32  14  37  08  10       addr         pass1_1008_3714

- 1008:9fb2
- 1008:9fca

    1008:9fb2  8c  9f             addr         FUN_1008_9f8c
    1008:9fb4  08  10             addr         switchD_1008:1091::caseD_a7
    1008:9fb6  f2  1d             addr         FUN_1008_1df2
    1008:9fb8  10  10             addr         switchD_1008:1091::caseD_ac
    1008:9fba  ce  1d             addr         LAB_1008_1dcb+3
    1008:9fbc  10  10             addr         switchD_1008:1091::caseD_ac
    1008:9fbe  d4  1d             addr         LAB_1008_1dd2+2
    1008:9fc0  10  10             addr         switchD_1008:1091::caseD_ac
    1008:9fc2  04  4e             addr         LAB_1008_4e04
    1008:9fc4  18  10             addr         switchD_1008:1091::caseD_ae
    1008:9fc6  18  4f             addr         LAB_1008_4f18
    1008:9fc8  18  10             addr         switchD_1008:1091::caseD_ae
    1008:9fca  80  9f             addr         pass1_1008_9f80
    1008:9fcc  08  10             addr         switchD_1008:1091::caseD_a7
    1008:9fce  18  9f             addr         pass1_1008_9f18
    1008:9fd0  08  10             addr         switchD_1008:1091::caseD_a7

- 1008:a230 -> INVALID
- 1008:ad8a
- 1008:ad92

    1008:ad8a  38  ad  08  10       addr         pass1_1008_ad38
    1008:ad8e  0c  ad  08  10       addr         pass1_1008_ad0c
    1008:ad92  64  ad  08  10       addr         pass1_1008_ad64
    1008:ad96  f2  1d  10  10       addr         pass1_1010_1df2
    1008:ad9a  ce  1d  10  10       addr         pass1_1010_1dce
    1008:ad9e  d4  1d  10  10       addr         pass1_1010_1dd4

- 1008:af7c

    1008:af7c  56  af  08  10       addr         pass1_1008_af56
    1008:af80  f2  1d  10  10       addr         pass1_1010_1df2
    1008:af84  ce  1d  10  10       addr         pass1_1010_1dce
    1008:af88  d4  1d  10  10       addr         pass1_1010_1dd4
    1008:af8c  04  4e  18  10       addr         create_dc_1018_4e04
    1008:af90  18  4f  18  10       addr         unk_win_ui_op_1018_4f18

- 1008:bdc0
- 1008:bdc4
- 1008:bdc8
- 1008:bdcc
- 1008:bddc

    1008:bdc0  74  bd  08  10       addr         pass1_1008_bd74
    1008:bdc4  4e  bd  08  10       addr         pass1_1008_bd4e
    1008:bdc8  28  bd  08  10       addr         pass1_1008_bd28
    1008:bdcc  02  bd  08  10       addr         pass1_1008_bd02
    1008:bdd0  f2  1d  10  10       addr         pass1_1010_1df2
    1008:bdd4  38  ba  08  10       addr         pass1_1008_ba38
    1008:bdd8  5e  bb  08  10       addr         file_1008_bb5e
    1008:bddc  9a  bd  08  10       addr         pass1_1008_bd9a

- 1008:ca4a

    1008:ca4a  24  ca  08  10       addr         pass1_1008_ca24
    1008:ca4e  f2  1d  10  10       addr         pass1_1010_1df2
    1008:ca52  8e  c9  08  10       addr         pass1_1008_c98e
    1008:ca56  d4  c9  08  10       addr         pass1_1008_c9d4

- 1008:d71a

    1008:d71a  f4  d6  08  10       addr         pass1_1008_d6f4
    1008:d71e  f2  1d  10  10       addr         pass1_1010_1df2
    1008:d722  ce  1d  10  10       addr         pass1_1010_1dce
    1008:d726  d4  1d  10  10       addr         pass1_1010_1dd4
    1008:d72a  c6  ca  08  10       addr         pass1_1008_cac6

- 1008:d780

    1008:d780  5a  d7  08  10       addr         pass1_1008_d75a
    1008:d784  f2  1d  10  10       addr         pass1_1010_1df2
    1008:d788  ce  1d  10  10       addr         pass1_1010_1dce
    1008:d78c  d4  1d  10  10       addr         pass1_1010_1dd4

- 1008:d98e

    1008:d98e  68  d9  08  10       addr         pass1_1008_d968
    1008:d992  f2  1d  10  10       addr         pass1_1010_1df2
    1008:d996  ce  1d  10  10       addr         pass1_1010_1dce
    1008:d99a  d4  1d  10  10       addr         pass1_1010_1dd4

- 1008:d9fa

    1008:d9fa  d4  d9  08  10       addr         pass1_1008_d9d4
    1008:d9fe  f2  1d  10  10       addr         pass1_1010_1df2
    1008:da02  ce  1d  10  10       addr         pass1_1010_1dce
    1008:da06  d4  1d  10  10       addr         pass1_1010_1dd4
    1008:da0a  04  4e  18  10       addr         create_dc_1018_4e04
    1008:da0e  18  4f  18  10       addr         unk_win_ui_op_1018_4f18

- 1008:dc80 -> pass1_1008_dc80; function

- 1008:dd4a

    1008:dd4a  1e  dd  08  10       addr         pass1_1008_dd1e

- 1008:eaac

    1008:eaac  86  ea  08  10       addr         pass1_1008_ea86
    1008:eab0  f2  1d  10  10       addr         pass1_1010_1df2
    1008:eab4  da  e5  08  10       addr         pass1_1008_e5da
    1008:eab8  0e  e7  08  10       addr         file_1008_e70e

- 1008:eb1a

    1008:eb1a  f4  ea  08  10       addr         pass1_1008_eaf4
    1008:eb1e  f2  1d  10  10       addr         pass1_1010_1df2
    1008:eb22  ce  1d  10  10       addr         pass1_1010_1dce
    1008:eb26  d4  1d  10  10       addr         pass1_1010_1dd4

- 1008:ec00

    1008:ec00  da  eb  08  10       addr         pass1_1008_ebda
    1008:ec04  f2  1d  10  10       addr         pass1_1010_1df2
    1008:ec08  ce  1d  10  10       addr         pass1_1010_1dce
    1008:ec0c  d4  1d  10  10       addr         pass1_1010_1dd4

- 1008:ec62

    1008:ec62  3c  ec  08  10       addr         pass1_1008_ec3c
    1008:ec66  f2  1d  10  10       addr         pass1_1010_1df2
    1008:ec6a  ce  1d  10  10       addr         pass1_1010_1dce
    1008:ec6e  d4  1d  10  10       addr         pass1_1010_1dd4

- 1008:ef9c
- 1008:efc4

    1008:ef9c  76  ef             ULONG_PTR *  pass1_1008_ef76
    1008:ef9e  08  10             ULONG_PTR *  switchD_1008:1091::caseD_a7
    1008:efa0  f2  1d             ULONG_PTR *  FUN_1008_1df2
    1008:efa2  10  10             ULONG_PTR *  switchD_1008:1091::caseD_ac
    1008:efa4  ce  1d             ULONG_PTR *  LAB_1008_1dcb+3
    1008:efa6  10  10             ULONG_PTR *  switchD_1008:1091::caseD_ac
    1008:efa8  d4  1d             ULONG_PTR *  LAB_1008_1dd2+2
    1008:efaa  10  10             ULONG_PTR *  switchD_1008:1091::caseD_ac
    1008:efac  14  ee             ULONG_PTR *  pass1_1008_ee14
    1008:efae  08  10             ULONG_PTR *  switchD_1008:1091::caseD_a7
    1008:efb0  72  ee             ULONG_PTR *  pass1_1008_ee72
    1008:efb2  08  10             ULONG_PTR *  switchD_1008:1091::caseD_a7
    1008:efb4  5a  30             ULONG_PTR *  LAB_1008_305a
    1008:efb6  10  10             ULONG_PTR *  switchD_1008:1091::caseD_ac
    1008:efb8  80  36             ULONG_PTR *  LAB_1008_367e+2
    1008:efba  10  10             ULONG_PTR *  switchD_1008:1091::caseD_ac
    1008:efbc  a6  ee             ULONG_PTR *  pass1_1008_eea6
    1008:efbe  08  10             ULONG_PTR *  switchD_1008:1091::caseD_a7
    1008:efc0  ac  ee             ULONG_PTR *  pass1_1008_eeac
    1008:efc2  08  10             ULONG_PTR *  switchD_1008:1091::caseD_a7
    1008:efc4  50  ef             ULONG_PTR *  pass1_1008_ef50
    1008:efc6  08  10             ULONG_PTR *  switchD_1008:1091::caseD_a7
    1008:efc8  38  ef             ULONG_PTR *  pass1_1008_ef38
    1008:efca  08  10             ULONG_PTR *  switchD_1008:1091::caseD_a7
    1008:efcc  1e  ed             ULONG_PTR *  mem_1008_ed1e
    1008:efce  08  10             ULONG_PTR *  switchD_1008:1091::caseD_a7
    1008:efd0  62  ed             ULONG_PTR *  pass1_1008_ed62
    1008:efd2  08  10             ULONG_PTR *  switchD_1008:1091::caseD_a7
    1008:efd4  8a  ed             ULONG_PTR *  pass1_1008_ed8a
    1008:efd6  08  10             ULONG_PTR *  switchD_1008:1091::caseD_a7
    1008:efd8  56  ee             ULONG_PTR *  load_string_1008_ee56
    1008:efda  08  10             ULONG_PTR *  switchD_1008:1091::caseD_a7
    1008:efdc  4a  ef             ULONG_PTR *  pass1_1008_ef4a
    1008:efde  08  10             ULONG_PTR *  switchD_1008:1091::caseD_a7



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
