fn main() {
    let myref: *const i32;
    {
        let myint = 5;
        myref = &myint;
    }
    unsafe {
        std::process::exit(*myref);
    }
}
