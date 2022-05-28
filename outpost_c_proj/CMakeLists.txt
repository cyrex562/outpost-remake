cmake_minimum_required(VERSION 3.21)
project(outpost_c_proj C)

set(CMAKE_C_STANDARD 23)

add_executable(outpost_c_proj main.c file2.c file2.h program.h types.h structs.h block_1000.c block_1000.h block_1008.c block_1008.h block_1010.c block_1010.h block_1018.c block_1018.h block_1020.c block_1020.h block_1028.c block_1028.h block_1030.c 1038.c 1038.h block_1040.c block_1040.h globals.h sys_api.c sys_api.h utils.c utils.h structs_2.h entry.c entry.h)
