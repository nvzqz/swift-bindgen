# `swift-bindgen`

Bridging the gap between Swift and Rust, brought to you by
[Nikolai Vazquez](https://twitter.com/nikolaivazquez)!

This project is very much a work-in-progress. If you would like to contribute,
please get involved by submitting
[issues](https://github.com/rustswift/swift-bindgen/issues),
[pull requests](https://github.com/rustswift/swift-bindgen/pulls),
or [getting in contact](mailto:hello@nikolaivazquez.com?subject=swift-bindgen)
to brainstorm.

If this project is useful to you, please support it by
[sponsoring on GitHub](https://github.com/sponsors/nvzqz) or
[donating directly](https://www.paypal.me/nvzqz)!

## Project Structure

This project is composed of the following crates:

- `swift-bindgen`: Generates bindings for two-way bridging of Rust/Swift types.

- `swift`: High-level idiomatic bindings to the Swift standard library.

- `swift-rt`: High-level idiomatic bindings to the Swift runtime.

- `swift-sys`: Low-level bindings to the Swift runtime.

## Acknowledgements

- Jordan Rose ([@UINT_MIN](https://twitter.com/UINT_MIN)) for insight into how
  the Swift runtime works internally.

- Joe Groff ([@jckarter](https://twitter.com/jckarter)) for being my Swift
  emotional support dog while I ask for feedback on my assumptions of Swift
  internals and ABI.

- Contributors to Swift—past, present, and future—for creating a fascinating
  language that makes apps easier to develop and faster to run.

## License

This project is licensed under the [Apache License (Version 2.0)](https://github.com/rustswift/swift-bindgen/blob/main/LICENSE.txt).
