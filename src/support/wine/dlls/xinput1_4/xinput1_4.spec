1 stdcall -private DllMain(long long ptr)
2 stdcall XInputGetState(long ptr)
3 stdcall XInputSetState(long ptr)
4 stdcall XInputGetCapabilities(long long ptr)
5 stdcall XInputEnable
7 stdcall XInputGetBatteryInformation(long long ptr)
8 stdcall XInputGetKeystroke(long long ptr)
10 stub XInputGetAudioDeviceIds(long ptr ptr ptr ptr)
100 stdcall XInputGetStateEx(long ptr)
