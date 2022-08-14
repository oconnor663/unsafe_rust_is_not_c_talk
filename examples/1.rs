fn main() {
    let myref: &i32;
    {
        let myint = 5;
        myref = &myint;
    }
    std::process::exit(*myref);
}
