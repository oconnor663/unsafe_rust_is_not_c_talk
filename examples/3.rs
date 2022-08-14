pub fn foo3() -> i32 {
    let myref: *const i32;
    {
        let myint = 5;
        myref = &myint;
    }
    unsafe { *myref }
}

fn main() {
    dbg!(foo3());
}
