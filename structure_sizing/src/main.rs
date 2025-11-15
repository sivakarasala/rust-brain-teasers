use std::mem::size_of;

struct VeryImportantMessage {
    _message_type: u8,
    _destination: u16,
}

#[repr(C, packed)]
struct ReallyThreeBytes {
    a: u8,
    b: u16,
}

fn main() {
    println!("VeryImportantMessage occupies {} bytes.", size_of::<VeryImportantMessage>());
    println!("ReallyThreeBytes occupies {} bytes.", size_of::<ReallyThreeBytes>());
}
