# sink-rs

This crate aims to provide an abstraction for a thing which can be sent values
and, immediately, return a Result indicating success / failure of receipt.

As a base primitive this should enable a message oriented variant of the
inbound params to the familiar imperitive Result 'and_then' composition pattern.

## Background

A primary goal of the interface for ISink is to hide the implementation details
of any sink.

Some decisions were made in order to facilitate that:

* Send takes Sink reference

We don't want to consume the sink so that it can be reused to send many times.

We also don't want to require the sender maintain a mutable reference to the sink.

* Delegated Mutation

StatefulSink requires that the creator manages mutation by taking ownership of the state type in it's struct.

This means that mutation and thread-safety level concerns are pushed out to the implementors.

* Send takes ownership of the sent value

Similar to Delegated Mutation, this forces the sender to be responsible for sharing, thread-safety, etc.

This also means that downstream recipients are explicitly decoupled from the originating source.

## Some Helpful References

[Implementing Function Composition](https://users.rust-lang.org/t/implementing-function-composition/8255/2)

[Railway Oriented Programming](https://fsharpforfunandprofit.com/rop/)

## CI/CD

[Building a Rust Project on CircleCI](https://abronan.com/building-a-rust-project-on-circleci/)

> To Get RustFmt Options:

````bash
rustup run nightly -- rustfmt --help
````