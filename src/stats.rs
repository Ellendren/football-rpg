use std::env::VarError;

pub mod player;
pub mod team;

#[derive(Debug)]
pub struct Error{
    err_msg: String
}

pub fn target() -> Result<String, Error> {
    let var_name = "TARGET";
    match std::env::var(var_name) {
        Ok(target) => Ok(target),
        Err(e) => Err(Error{ err_msg: format!("Can't find '{}' env var: ({:?})",  var_name, e.to_string())})
    }
}

struct util;

impl util {
    //u16_to_u8array(var: u16)
    //description: makes a u8 array for a u16 value
    //params:
    //    -var: the u16 value t0 convert
    //returns: a u8 array of var
    pub unsafe fn u16_to_u8array(var: u16) -> [u8; 2] {
        let u16_vec : Vec<u16> = vec![var];
        let u8_ref = &u16_vec.align_to::<u8>().1[..2];
        let u8array: [u8; 2] = [u8_ref[0], u8_ref[1]];
        u8array
    }

    //u32_to_u8array(var: u32)
    //description: makes a u8 array for a u32 value
    //params:
    //    -var: the u32 value to convert
    //returns: a u8 array of var
    pub unsafe fn u32_to_u8array(var: u32) -> [u8; 4] {
        let u16_vec : Vec<u32> = vec![var];
        let u8_ref = &u16_vec.align_to::<u8>().1[..4];
        let u8array: [u8; 4] = [u8_ref[0], u8_ref[1], u8_ref[2], u8_ref[3]];
        u8array
    }

    //i64_to_u8array(var: i64)
    //description: makes a u8 array for a i64 value
    //params:
    //    -var: the i64 value to convert
    //returns: a u8 array of var
    pub unsafe fn i64_to_u8array(var: i64) -> [u8; 8] {
        let u16_vec : Vec<i64> = vec![var];
        let u8_ref = &u16_vec.align_to::<u8>().1[..8];
        let u8array: [u8; 8] = [u8_ref[0], u8_ref[1], u8_ref[2], u8_ref[3], u8_ref[4], u8_ref[5], u8_ref[6], u8_ref[7]];
        u8array
    }

    //u8array_to_u16(byte_stream: &[u8])
    //descripts: turns a byte stream intp a u16
    //params:
    // - byte_stream: a 2 byte array represting a u16 var
    //returns: the original u16 value 
    pub unsafe fn u8array_to_u16(byte_stream: &[u8; 2]) -> u16{
        let num = byte_stream.align_to::<u16>().1;
        num[0]
    }

    //u8array_to_u32(byte_stream: &[u8])
    //descripts: turns a byte stream intp a u32
    //params:
    // - byte_stream: a 4 byte array represting a u32 var
    //returns: the original u16 value 
    pub unsafe fn u8array_to_u32(byte_stream: &[u8; 4]) -> u32{
        let num: &[u32] = byte_stream.align_to::<u32>().1;
        num[0]
    }

    //u8array_to_i64(byte_stream: &[u8])
    //descripts: turns a byte stream intp a i64
    //params:
    // - byte_stream: a 4 byte array represting a u32 var
    //returns: the original u16 value 
    pub unsafe fn u8array_to_i64(byte_stream: &[u8; 8]) -> i64{
        let num: &[i64] = byte_stream.align_to::<i64>().1;
        num[0]
    }
}