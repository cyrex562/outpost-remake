//noinspection ALL
#[derive(Default,Debug,Clone,Copy)]
pub struct AppContext {
    pub AH_REG: u8,
    pub CS_REG: u16,
    pub SI_REG: u16,
    pub DI_REG: u16,
    pub ES_REG: u16,
    pub BP_REG: u16,
}
