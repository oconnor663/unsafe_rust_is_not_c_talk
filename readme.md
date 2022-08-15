# Unsafe Rust is not C

- Unsafe doesn't "turn off the borrow checker".
  - A dangling reference into Vec doesn't compile:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9019e5ed43179d3243ba403e8e11c094
  - With unsafe it still doesn't compile:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b8963fed33351ce7b9962491e996b311
- Unsafe gives us specific superpowers, particularly raw pointers.
  - Now it compiles, runs, and fails Miri:
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d4a6b1c46cc9c123dc874c0ab20f9df8
- Motivation: use-after-free and race conditions
- aliasing rules
  - comparison with the `restrict` keyword: https://godbolt.org/z/deaKqfsbT
- C strict aliasing examples
  - with structs: https://godbolt.org/z/nEajEGqKG
