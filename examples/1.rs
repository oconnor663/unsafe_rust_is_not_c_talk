pub fn foo1() -> i32 {
    let myref: &i32;
    {
        let myint = 5;
        myref = &myint;
    }
    *myref
}

fn main() {
    dbg!(foo1());
}
