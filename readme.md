# Unsafe Rust is not C

- Unsafe doesn't "turn off the borrow checker".
  - A dangling reference into Vec doesn't compile:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9019e5ed43179d3243ba403e8e11c094
  - With unsafe it still doesn't compile:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=bee5393d566450a9a0df4c6437a55ce3
- Unsafe gives us specific superpowers, particularly raw pointers.
  - Now it compiles, runs, and fails Miri:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ceee9a7cc498f8a39548c0d5833e8657
  - Insert an extra dummy alloc to force obvious UB:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=e8f579f55b5ec161f1afc45c32c7aba6
- Unsafe Rust is stricter than C.
  - breaking the aliasing rules for references is UB:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=32b8750ce7e9ef752a1e372b2dbd4d9e
  - comparison with the `restrict` keyword in C: https://godbolt.org/z/rzv7Taof6
  - in fact the very *existence* of aliasing references is UB according to Miri:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=11c2ef20afd76759fcc9672a3a5523bb
- C is stricter than you might think.
  - https://blog.regehr.org/archives/1307
  - the standard says it's illegal to access a struct through the wrong pointer type
  - in practice, this UB is triggered by aliasing: https://godbolt.org/z/EMjhKzhq5
  - a similar bug with overlap: https://godbolt.org/z/8PfrrE1P9
  - Rust raw pointers do not have these limitations (at least not today):
    - https://godbolt.org/z/5Wsz6MTvG
    - https://godbolt.org/z/rd9Tvv3eh
- Aliasing analysis is important for big optimizations: https://godbolt.org/z/jdeM4Wv9v
