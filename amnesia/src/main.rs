fn main() {
    // when we want to handover control of memory to other app
    // so as rust don't need to invoke drop constructor automatically on that type
    loop {
        let buffer = (0..1000).collect::<Vec<u32>>();
        std::mem::forget(buffer);
        print!(".");
    }
}
