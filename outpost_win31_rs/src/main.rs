// #include <stdio.h>

use crate::app_context::AppContext;
use crate::program_lifecycle::entry;

mod block_1000;
mod block_1008;
mod block_1010;
mod block_1018;
mod block_1020;
mod block_1028;
mod block_1030;
mod block_1038;
mod block_1040;
mod program_lifecycle;
mod func_ptrs;
mod globals;
mod prog_types;
mod structs;
mod winbase;
mod utils;
mod mem_ops;
mod win_ui;
mod string_defs;
mod sys_ops;
mod app_context;
mod no_refs;
mod mem_container;
mod unk;
mod windef;
mod dos_interrupt;
mod file_ops;
mod mem_address;


pub unsafe fn main()
{

    // printf("Hello, World!\n");
    // return 0;
    println!("test");
    let mut ctx: AppContext = AppContext::default();
    let mut a = 0u16;
    let mut b = 0u16;
    let mut c = 0i16;
    let mut d: Vec<u8> = vec![];
    let mut e = 0u16;
    let mut f = 0u16;
    let mut result = entry(&mut ctx, a, b, c, d.as_mut_ptr(), e.as_mut_ptr(), f);


}
