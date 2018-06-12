# sink-rs

This crate aims to provide an abstraction for a thing which can be sent values
and, immediately, return a Result indicating success / failure of receipt.

As a base primitive this should enable a message oriented variant of the
inbound params to the familiar imperitive Result 'and_then' composition pattern.

## Some Helpful References

[Implementing Function Composition](https://users.rust-lang.org/t/implementing-function-composition/8255/2)

[Railway Oriented Programming](https://fsharpforfunandprofit.com/rop/)
