138 stdcall @(long str str str str long str long long str str long) USERENV_138

@ stdcall CreateEnvironmentBlock(ptr ptr long)
@ stdcall DestroyEnvironmentBlock(ptr)
@ stdcall EnterCriticalPolicySection
@ stdcall ExpandEnvironmentStringsForUserA(ptr str ptr long)
@ stdcall ExpandEnvironmentStringsForUserW(ptr wstr ptr long)
@ stdcall GetAllUsersProfileDirectoryA(ptr ptr)
@ stdcall GetAllUsersProfileDirectoryW(ptr ptr)
@ stdcall GetAppliedGPOListW(long wstr ptr ptr ptr)
@ stdcall GetDefaultUserProfileDirectoryA(ptr ptr)
@ stdcall GetDefaultUserProfileDirectoryW(ptr ptr)
@ stdcall GetProfilesDirectoryA(ptr ptr)
@ stdcall GetProfilesDirectoryW(ptr ptr)
@ stdcall GetProfileType(ptr)
@ stdcall GetUserProfileDirectoryA(ptr ptr ptr)
@ stdcall GetUserProfileDirectoryW(ptr ptr ptr)
@ stdcall LeaveCriticalPolicySection(ptr)
@ stdcall LoadUserProfileA(ptr ptr)
@ stdcall LoadUserProfileW(ptr ptr)
@ stdcall RegisterGPNotification(long long)
@ stdcall UnloadUserProfile(ptr ptr)
@ stdcall UnregisterGPNotification
