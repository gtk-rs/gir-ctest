use libc::*;

pub type T2Foo = u32;
pub type T2Bar = u32;

pub type T2TypedefFoo = T2Foo;
pub type T2TypedefInt = c_int;

macro_rules! i {
    ($i:item) => {
        $i
    };
}

#[repr(C)]
#[derive(Debug)]
pub struct T2Baz {
    pub a: i64,
    pub b: u32,
}

#[repr(C)]
pub union T2Union {
    pub a: u32,
    pub b: i64,
}

pub const T2C: i32 = 5;

i! {
    pub const T2S: &'static str = "b";
}

pub const T2P: *const c_char = b"paddingZ\0" as *const u8 as *const c_char;

extern "C" {
    pub fn T2a();
}
