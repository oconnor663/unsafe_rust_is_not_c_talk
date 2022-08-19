# Unsafe Rust is not C

- Unsafe doesn't "turn off the borrow checker".
  - A dangling reference into Vec doesn't compile: https://godbolt.org/z/P3Tfqbez8
  - With unsafe it still doesn't compile: https://godbolt.org/z/xKq79dToP
  - We need a raw pointer to make it compile, run, and fail ASan: https://godbolt.org/z/qYd893Ke3
- Unsafe Rust is stricter than C.
  - Breaking the aliasing rules for references is UB: https://godbolt.org/z/769oM8e6x
  - We can detect this with Miri:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9650578b9097ad8e8766d15fda95133e
  - A side-by-side comparison with the `restrict` keyword in C: https://godbolt.org/z/rzv7Taof6
  - The very *existence* of an aliasing mutable reference is UB according to Miri:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=11c2ef20afd76759fcc9672a3a5523bb
- C is stricter than you might think.
  - https://blog.regehr.org/archives/1307
  - Accessing an object through the wrong pointer type is UB: https://godbolt.org/z/EMjhKzhq5
  - Reading and writing overlapping objects is UB: https://godbolt.org/z/8PfrrE1P9
  - ASan and UBSan aren't catching either of those examples today.
  - Raw pointers in Rust don't have those restrictions (at least not today):
    - https://godbolt.org/z/5Wsz6MTvG
    - https://godbolt.org/z/rd9Tvv3eh
- Modern compilers can perform miracles, but they often rely on aliasing rules: https://godbolt.org/z/jdeM4Wv9v
