# Unsafe Rust is not C

[![wonka](wonka.png)](https://www.youtube.com/watch?v=SVi3-PrQ0pY)

- Unsafe doesn't "turn off the borrow checker".
  - A dangling reference into Vec doesn't compile: https://godbolt.org/z/jff8KMTM8
  - With unsafe it still doesn't compile: https://godbolt.org/z/r4PzhbcdP
  - We need a raw pointer to make it compile, run, and fail ASan: https://godbolt.org/z/5e4x77vbn
- Rust is stricter than C.
  - Breaking the aliasing rules for references is UB: https://godbolt.org/z/769oM8e6x
  - We can detect this with Miri:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f207dbdafb638cce4f96724a4512569b
  - A side-by-side comparison with the `restrict` keyword in C: https://godbolt.org/z/rzv7Taof6
  - The very *existence* of an aliasing mutable reference is UB according to Miri:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=11c2ef20afd76759fcc9672a3a5523bb
- C is stricter than you might think.
  - https://blog.regehr.org/archives/1307
  - Accessing an object through the wrong pointer type is UB: https://godbolt.org/z/7qWdfG5MG
  - Reading and writing overlapping objects is UB: https://godbolt.org/z/f73Ejjnbj
  - ASan and UBSan don't catch either of those examples today.
  - Raw pointers in Rust don't have those restrictions (at least not today):
    - https://godbolt.org/z/oh8q663sn
    - https://godbolt.org/z/bcheT4KoM
- Modern compilers can perform miracles, but they often rely on aliasing rules: https://godbolt.org/z/f8Mz33Wbe
