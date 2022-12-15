use crate::app_context::AppContext;


// typedef void(*code3)(void*);

// typedef u8(*code4)();

// typedef i16(*code5)();

// typedef bool(*code6);

// typedef u16(*code7);


type code = fn(u16);
// typedef u32(*code8)();
type code8 = fn() -> u32;

pub enum InterruptResult {
    CODE = code,
    CODE8 = code8,

}

pub fn swi(ctx: &mut AppContext, int_code: u16) -> InterruptResult {
    todo!()
}
