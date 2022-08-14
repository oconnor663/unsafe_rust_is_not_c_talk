pub fn foo2() -> i32 {
    unsafe {
        let myref: &i32;
        {
            let myint = 5;
            myref = &myint;
        }
        *myref
    }
}

fn main() {
    dbg!(foo2());
}
