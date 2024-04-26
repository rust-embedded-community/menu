# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased](https://github.com/rust-embedded-community/menu/compare/v0.4.0...master)

### Changed
* [breaking] The `menu` `Context` is now borrowed during I/O input processing to support borrowed data
* [breaking] The `pub context` item on the runner was updated to `pub interface`

## [v0.4.0] - 2023-09-16

Note: Changes before this version were not tracked via the CHANGELOG

### Changed
* Changed the struct `Runner` to own the struct `Menu` instead of borrowing it

### Added

* Made struct `Menu` implement `Clone`
* Add the possibility to disable local echo (via `echo` feature, enabled by default)

[v0.4.0]: https://github.com/rust-embedded-community/menu/releases/tag/v0.4.0
