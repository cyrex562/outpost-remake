@ cdecl __isascii ucrtbase.__isascii
@ cdecl __iscsym ucrtbase.__iscsym
@ cdecl __iscsymf ucrtbase.__iscsymf
@ stub __iswcsym
@ stub __iswcsymf
@ stub __strncnt
@ stub __wcsncnt
@ cdecl _isalnum_l(long ptr) ucrtbase._isalnum_l
@ cdecl _isalpha_l(long ptr) ucrtbase._isalpha_l
@ cdecl _isblank_l(long ptr) ucrtbase._isblank_l
@ cdecl _iscntrl_l(long ptr) ucrtbase._iscntrl_l
@ cdecl _isctype(long long) ucrtbase._isctype
@ cdecl _isctype_l(long long ptr) ucrtbase._isctype_l
@ cdecl _isdigit_l(long ptr) ucrtbase._isdigit_l
@ cdecl _isgraph_l(long ptr) ucrtbase._isgraph_l
@ cdecl _isleadbyte_l(long ptr) ucrtbase._isleadbyte_l
@ cdecl _islower_l(long ptr) ucrtbase._islower_l
@ cdecl _isprint_l(long ptr) ucrtbase._isprint_l
@ stub _ispunct_l
@ cdecl _isspace_l(long ptr) ucrtbase._isspace_l
@ cdecl _isupper_l(long ptr) ucrtbase._isupper_l
@ stub _iswalnum_l
@ cdecl _iswalpha_l(long ptr) ucrtbase._iswalpha_l
@ cdecl _iswblank_l(long ptr) ucrtbase._iswblank_l
@ stub _iswcntrl_l
@ stub _iswcsym_l
@ stub _iswcsymf_l
@ cdecl _iswctype_l(long long ptr) ucrtbase._iswctype_l
@ cdecl _iswdigit_l(long ptr) ucrtbase._iswdigit_l
@ stub _iswgraph_l
@ stub _iswlower_l
@ stub _iswprint_l
@ cdecl _iswpunct_l(long ptr) ucrtbase._iswpunct_l
@ cdecl _iswspace_l(long ptr) ucrtbase._iswspace_l
@ stub _iswupper_l
@ stub _iswxdigit_l
@ cdecl _isxdigit_l(long ptr) ucrtbase._isxdigit_l
@ cdecl _memccpy(ptr ptr long long) ucrtbase._memccpy
@ cdecl _memicmp(str str long) ucrtbase._memicmp
@ cdecl _memicmp_l(str str long ptr) ucrtbase._memicmp_l
@ cdecl _strcoll_l(str str ptr) ucrtbase._strcoll_l
@ cdecl _strdup(str) ucrtbase._strdup
@ cdecl _stricmp(str str) ucrtbase._stricmp
@ cdecl _stricmp_l(str str ptr) ucrtbase._stricmp_l
@ cdecl _stricoll(str str) ucrtbase._stricoll
@ cdecl _stricoll_l(str str ptr) ucrtbase._stricoll_l
@ cdecl _strlwr(str) ucrtbase._strlwr
@ cdecl _strlwr_l(str ptr) ucrtbase._strlwr_l
@ cdecl _strlwr_s(ptr long) ucrtbase._strlwr_s
@ cdecl _strlwr_s_l(ptr long ptr) ucrtbase._strlwr_s_l
@ cdecl _strncoll(str str long) ucrtbase._strncoll
@ cdecl _strncoll_l(str str long ptr) ucrtbase._strncoll_l
@ cdecl _strnicmp(str str long) ucrtbase._strnicmp
@ cdecl _strnicmp_l(str str long ptr) ucrtbase._strnicmp_l
@ cdecl _strnicoll(str str long) ucrtbase._strnicoll
@ cdecl _strnicoll_l(str str long ptr) ucrtbase._strnicoll_l
@ cdecl _strnset(str long long) ucrtbase._strnset
@ cdecl _strnset_s(str long long long) ucrtbase._strnset_s
@ cdecl _strrev(str) ucrtbase._strrev
@ cdecl _strset(str long) ucrtbase._strset
@ stub _strset_s
@ cdecl _strupr(str) ucrtbase._strupr
@ cdecl _strupr_l(str ptr) ucrtbase._strupr_l
@ cdecl _strupr_s(str long) ucrtbase._strupr_s
@ cdecl _strupr_s_l(str long ptr) ucrtbase._strupr_s_l
@ cdecl _strxfrm_l(ptr str long ptr) ucrtbase._strxfrm_l
@ cdecl _tolower ucrtbase._tolower
@ cdecl _tolower_l(long ptr) ucrtbase._tolower_l
@ cdecl _toupper ucrtbase._toupper
@ cdecl _toupper_l(long ptr) ucrtbase._toupper_l
@ cdecl _towlower_l(long ptr) ucrtbase._towlower_l
@ cdecl _towupper_l(long ptr) ucrtbase._towupper_l
@ cdecl _wcscoll_l(wstr wstr ptr) ucrtbase._wcscoll_l
@ cdecl _wcsdup(wstr) ucrtbase._wcsdup
@ cdecl _wcsicmp(wstr wstr) ucrtbase._wcsicmp
@ cdecl _wcsicmp_l(wstr wstr ptr) ucrtbase._wcsicmp_l
@ cdecl _wcsicoll(wstr wstr) ucrtbase._wcsicoll
@ cdecl _wcsicoll_l(wstr wstr ptr) ucrtbase._wcsicoll_l
@ cdecl _wcslwr(wstr) ucrtbase._wcslwr
@ cdecl _wcslwr_l(wstr ptr) ucrtbase._wcslwr_l
@ cdecl _wcslwr_s(wstr long) ucrtbase._wcslwr_s
@ cdecl _wcslwr_s_l(wstr long ptr) ucrtbase._wcslwr_s_l
@ cdecl _wcsncoll(wstr wstr long) ucrtbase._wcsncoll
@ cdecl _wcsncoll_l(wstr wstr long ptr) ucrtbase._wcsncoll_l
@ cdecl _wcsnicmp(wstr wstr long) ucrtbase._wcsnicmp
@ cdecl _wcsnicmp_l(wstr wstr long ptr) ucrtbase._wcsnicmp_l
@ cdecl _wcsnicoll(wstr wstr long) ucrtbase._wcsnicoll
@ cdecl _wcsnicoll_l(wstr wstr long ptr) ucrtbase._wcsnicoll_l
@ cdecl _wcsnset(wstr long long) ucrtbase._wcsnset
@ stub _wcsnset_s
@ cdecl _wcsrev(wstr) ucrtbase._wcsrev
@ cdecl _wcsset(wstr long) ucrtbase._wcsset
@ cdecl _wcsset_s(wstr long long) ucrtbase._wcsset_s
@ cdecl _wcsupr(wstr) ucrtbase._wcsupr
@ cdecl _wcsupr_l(wstr ptr) ucrtbase._wcsupr_l
@ cdecl _wcsupr_s(wstr long) ucrtbase._wcsupr_s
@ cdecl _wcsupr_s_l(wstr long ptr) ucrtbase._wcsupr_s_l
@ cdecl _wcsxfrm_l(ptr wstr long ptr) ucrtbase._wcsxfrm_l
@ stub _wctype
@ cdecl is_wctype(long long) ucrtbase.is_wctype
@ cdecl isalnum ucrtbase.isalnum
@ cdecl isalpha ucrtbase.isalpha
@ cdecl isblank ucrtbase.isblank
@ cdecl iscntrl ucrtbase.iscntrl
@ cdecl isdigit ucrtbase.isdigit
@ cdecl isgraph ucrtbase.isgraph
@ cdecl isleadbyte ucrtbase.isleadbyte
@ cdecl islower ucrtbase.islower
@ cdecl isprint ucrtbase.isprint
@ cdecl ispunct ucrtbase.ispunct
@ cdecl isspace ucrtbase.isspace
@ cdecl isupper ucrtbase.isupper
@ cdecl iswalnum ucrtbase.iswalnum
@ cdecl iswalpha ucrtbase.iswalpha
@ cdecl iswascii ucrtbase.iswascii
@ cdecl iswblank ucrtbase.iswblank
@ cdecl iswcntrl ucrtbase.iswcntrl
@ cdecl iswctype(long long) ucrtbase.iswctype
@ cdecl iswdigit ucrtbase.iswdigit
@ cdecl iswgraph ucrtbase.iswgraph
@ cdecl iswlower ucrtbase.iswlower
@ cdecl iswprint ucrtbase.iswprint
@ cdecl iswpunct ucrtbase.iswpunct
@ cdecl iswspace ucrtbase.iswspace
@ cdecl iswupper ucrtbase.iswupper
@ cdecl iswxdigit ucrtbase.iswxdigit
@ cdecl isxdigit ucrtbase.isxdigit
@ cdecl mblen(ptr long) ucrtbase.mblen
@ cdecl mbrlen(ptr long ptr) ucrtbase.mbrlen
@ cdecl memcpy_s(ptr long ptr long) ucrtbase.memcpy_s
@ cdecl memmove_s(ptr long ptr long) ucrtbase.memmove_s
@ cdecl memset(ptr long long) ucrtbase.memset
@ cdecl strcat(str str) ucrtbase.strcat
@ cdecl strcat_s(str long str) ucrtbase.strcat_s
@ cdecl strcmp(str str) ucrtbase.strcmp
@ cdecl strcoll(str str) ucrtbase.strcoll
@ cdecl strcpy(ptr str) ucrtbase.strcpy
@ cdecl strcpy_s(ptr long str) ucrtbase.strcpy_s
@ cdecl strcspn(str str) ucrtbase.strcspn
@ cdecl strlen(str) ucrtbase.strlen
@ cdecl strncat(str str long) ucrtbase.strncat
@ cdecl strncat_s(str long str long) ucrtbase.strncat_s
@ cdecl strncmp(str str long) ucrtbase.strncmp
@ cdecl strncpy(ptr str long) ucrtbase.strncpy
@ cdecl strncpy_s(ptr long str long) ucrtbase.strncpy_s
@ cdecl strnlen(str long) ucrtbase.strnlen
@ cdecl strpbrk(str str) ucrtbase.strpbrk
@ cdecl strspn(str str) ucrtbase.strspn
@ cdecl strtok(str str) ucrtbase.strtok
@ cdecl strtok_s(ptr str ptr) ucrtbase.strtok_s
@ cdecl strxfrm(ptr str long) ucrtbase.strxfrm
@ cdecl tolower ucrtbase.tolower
@ cdecl toupper ucrtbase.toupper
@ stub towctrans
@ cdecl towlower ucrtbase.towlower
@ cdecl towupper ucrtbase.towupper
@ cdecl wcscat(wstr wstr) ucrtbase.wcscat
@ cdecl wcscat_s(wstr long wstr) ucrtbase.wcscat_s
@ cdecl wcscmp(wstr wstr) ucrtbase.wcscmp
@ cdecl wcscoll(wstr wstr) ucrtbase.wcscoll
@ cdecl wcscpy(ptr wstr) ucrtbase.wcscpy
@ cdecl wcscpy_s(ptr long wstr) ucrtbase.wcscpy_s
@ cdecl wcscspn(wstr wstr) ucrtbase.wcscspn
@ cdecl wcslen(wstr) ucrtbase.wcslen
@ cdecl wcsncat(wstr wstr long) ucrtbase.wcsncat
@ cdecl wcsncat_s(wstr long wstr long) ucrtbase.wcsncat_s
@ cdecl wcsncmp(wstr wstr long) ucrtbase.wcsncmp
@ cdecl wcsncpy(ptr wstr long) ucrtbase.wcsncpy
@ cdecl wcsncpy_s(ptr long wstr long) ucrtbase.wcsncpy_s
@ cdecl wcsnlen(wstr long) ucrtbase.wcsnlen
@ cdecl wcspbrk(wstr wstr) ucrtbase.wcspbrk
@ cdecl wcsspn(wstr wstr) ucrtbase.wcsspn
@ cdecl wcstok(wstr wstr) ucrtbase.wcstok
@ cdecl wcstok_s(ptr wstr ptr) ucrtbase.wcstok_s
@ cdecl wcsxfrm(ptr wstr long) ucrtbase.wcsxfrm
@ cdecl wctype(str) ucrtbase.wctype
@ cdecl wmemcpy_s(ptr long ptr long) ucrtbase.wmemcpy_s
@ cdecl wmemmove_s(ptr long ptr long) ucrtbase.wmemmove_s
