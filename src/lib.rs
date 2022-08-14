pub fn foo1() -> i32 {
    let myref: &i32;
    {
        let myint = 5;
        myref = &myint;
    }
    *myref
}

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

pub fn foo3() -> i32 {
    let myref: *const i32;
    {
        let myint = 5;
        myref = &myint;
    }
    unsafe { *myref }
}
