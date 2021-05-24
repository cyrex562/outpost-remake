pub fn get_fn_ptr_at_address(address: u32) -> fn() {
    unimplemented!()
}

pub fn get_type_at_address<T>(address: u32) -> T {
    unimplemented!()
}

pub fn get_struct_from_address<T>(address: u32) -> T {
    unimplemented!()
}


pub fn read_u8_from_vec(buff: &Vec<u8>, offset: usize) -> u8 {
    buff[offset]
}

pub fn read_u16_from_vec(buff: &Vec<u8>, offset: usize) -> u16 {
    let mut conv_array: [u8;2] = [0;2];
    conv_array.clone_from_slice(&buff[offset..offset+2]);
    u16::from_le_bytes(conv_array)
}

pub fn read_u32_from_vec(buff: &Vec<u8>, offset: usize) -> u32 {
    let mut conv_array: [u8;4] = [0;4];
    conv_array.clone_from_slice(&buf[offset..offset+4]);
    u32::from_le_bytes(conv_array)
}

pub struct StructuredData {
    pub base: u16,
    pub offset: u16,
    pub full_address: u32,
    pub buffer: Vec<u8>,
}

impl StructuredData {
    pub fn new() -> StructuredData {
        StructuredData{
            base: 0,
            offset: 0,
            full_address: 0,
            buffer: Vec::new()
        }
    }

    pub fn get_string(offset: Option<usize>, len: Option<usize>) -> String {
        unimplemented!()
    }

    pub fn get_u16(offset: Option<usize>) -> u16 {
        unimplemented!()
    }

    pub fn
}

pub fn get_string_from_address(address: u32) -> String {
    unimplemented!()
}
