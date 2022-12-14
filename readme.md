# Unsafe Rust is not C

This talk was recorded and [**uploaded to YouTube**](https://youtu.be/DG-VLezRkYQ).

<a href="https://www.youtube.com/watch?v=SVi3-PrQ0pY"><img src="wonka.png" alt="Wonka" width="400"></a>

- Unsafe doesn't "turn off the borrow checker".
  - A dangling reference into Vec doesn't compile: https://godbolt.org/z/vss7Wb585
  - With unsafe it still doesn't compile: https://godbolt.org/z/T55Tx8KEn
  - We need a raw pointer to make it compile, run, and fail ASan: https://godbolt.org/z/dzo6Pan5q
- Rust is stricter than C.
  - Compiler errors like this might feel like a "false positive": https://godbolt.org/z/jjTGPE1fK
  - But breaking the aliasing rules for references is UB: https://godbolt.org/z/769oM8e6x
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
- Modern compilers can perform miracles, but they often rely on aliasing rules: https://godbolt.org/z/shTTcv69h
