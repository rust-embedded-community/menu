# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased](https://github.com/rust-embedded-community/menu/compare/v0.6.1...master)

## [v0.6.1] - 2024-11-29

### Changed

* For `Runner::input_byte` the buffer `B` does not need to be `Sized`

### Added

* `impl core::error::Error for Error` on rust >= 1.81

## [v0.6.0] - 2024-08-30

### Changed

* We now run clippy in CI
* Add optional support for [`noline`](https://crates.io/crates/noline) as a line-editor with history 
* The interface we use for reading and writing bytes must now implement the [`embedded-io`](https://crates.io/crates/embedded-io) traits

## [v0.5.1] - 2024-08-22

### Fixed

* Fix Menu entry call order

## [v0.5.0] - 2024-04-26

### Changed

* [breaking] The `menu` `Context` is now borrowed during I/O input processing to support borrowed data
* [breaking] The `pub context` item on the runner was updated to `pub interface`

## [v0.4.0] - 2023-09-16

### Changed

* Changed the struct `Runner` to own the struct `Menu` instead of borrowing it

### Added

* Made `struct Menu` implement `Clone`
* Add the possibility to disable local echo (via `echo` feature, enabled by default)

## [v0.3.2] - 2019-08-22

### Changed

* Tidied up help text

## [v0.3.1] - 2019-08-11


## [v0.3.0] - 2019-08-11

### Changed

* Parameter / Argument support
* Re-worked help text system
* Example uses `pancurses`
* Remove use of fixed width (assumes a Western set with one byte per glyph)

## [v0.2.1] - 2018-10-04

### Changed

* Fixed broken example

## [v0.2.0] - 2018-10-04

* Add context to menu callback
* Fix width of help text

## [v0.1.1] - 2018-05-19

* First release

[v0.6.1]: https://github.com/rust-embedded-community/menu/releases/tag/v0.6.1
[v0.6.0]: https://github.com/rust-embedded-community/menu/releases/tag/v0.6.0
[v0.5.1]: https://github.com/rust-embedded-community/menu/releases/tag/v0.5.1
[v0.5.0]: https://github.com/rust-embedded-community/menu/releases/tag/v0.5.0
[v0.4.0]: https://github.com/rust-embedded-community/menu/releases/tag/v0.4.0
[v0.3.2]: https://github.com/rust-embedded-community/menu/releases/tag/v0.3.2
[v0.3.1]: https://github.com/rust-embedded-community/menu/releases/tag/v0.3.1
[v0.3.0]: https://github.com/rust-embedded-community/menu/releases/tag/v0.3.0
[v0.2.1]: https://github.com/rust-embedded-community/menu/releases/tag/v0.2.1
[v0.2.0]: https://github.com/rust-embedded-community/menu/releases/tag/v0.2.0
[v0.1.1]: https://github.com/rust-embedded-community/menu/releases/tag/v0.1.1
