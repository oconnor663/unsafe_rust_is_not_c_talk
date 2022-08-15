# Unsafe Rust is not C

- unsafe doesn't "turn off the borrow checker"
  - https://godbolt.org/z/8c5MMqoWf doesn't compile
  - and https://godbolt.org/z/cr5a7jq44 still doesn't compile
- specific superpowers, most importantly raw pointers
  - basic int use-after-scope: https://godbolt.org/z/GsfMc3rTq
  - same thing failing Miri: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=5cd025aad88bd35c05f0bf34f846bf81
- motivation: use-after-free and race conditions
- aliasing rules
  - comparison with the `restrict` keyword: https://godbolt.org/z/16q3fbKrs
- C strict aliasing examples
  - with structs: https://godbolt.org/z/Ghzs9T18d
