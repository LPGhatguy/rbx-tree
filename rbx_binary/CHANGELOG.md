# rbx_binary Changelog

## Unreleased

## 0.6.0-alpha.5 (2021-05-14)
* Added `OptionalCoordinateFrame` support. ([#176][pr-176])
* Added support for writing CFrame basic rotation IDs. ([#171][pr-171])

[pr-171]: https://github.com/rojo-rbx/rbx-dom/pull/171
[pr-176]: https://github.com/rojo-rbx/rbx-dom/pull/176

## 0.6.0-alpha.4 (2021-04-09)
* Unknown property types will no longer fail deserialization ([#168][pr-168])
    * This makes rbx_binary more resilient to new types being added by Roblox.
* `PROP` chunks that end after their prop name are now silently ignored. ([#168][pr-168])
    * This matches Roblox's behavior and fixes our handling of `OptionalCoordinateFrame`, added in the most recent Roblox Studio release.

[pr-168]: https://github.com/rojo-rbx/rbx-dom/pull/168

## 0.6.0-alpha.3 (2021-03-08)
* Added support for encoding `Int32` values as `BrickColor`.
  * This helps improve compatibility when decoding XML models and reencoding them as binary.

## 0.6.0-alpha.2 (2021-03-04)
* Added support for widening integer types when serializing.
    * `Int32` will widen to `Int64`, `Float32` will widen to `Float64`.

## 0.6.0-alpha.1 (2021-02-16)
This release is a major, breaking change that upgrades rbx\_xml's underlying DOM implementation from rbx\_dom\_weak 1.0 to 2.0. This release also realigned rbx\_binary's API to match rbx_xml.

* Breaking: ported crate to rbx\_dom_weak 2.0
* Breaking: updated top-level API to match rbx_xml.
* Added support for all remaining types.

## 0.5.0 (2019-12-18)
0.5.0 is intended to be mostly API-compatible with previous rbx_binary releases because it makes a lot of foundational changes. 0.6.0 will break the rbx_binary API significantly.

* Rewrote crate from ground-up using understanding gained from rbx_xml
* Added support for using reflection information, improving content compatibility
* Improved performance by up to 30% for some files
* Improved instrumentation using the `log` crate
* Improved the crate's error types ([#48](https://github.com/rojo-rbx/rbx-dom/issues/48))
* Fixed panics in many cases, instead returning an error ([#26](https://github.com/rojo-rbx/rbx-dom/issues/26))
* Fixed handling unknown `BinaryString` values ([#49](https://github.com/rojo-rbx/rbx-dom/issues/49))

## 0.4.1 (2019-05-29)
* Fixed bad interaction with rbx_dom_weak 1.6.0 causing instances to go missing.

## 0.4.0 (2019-03-01)
* Updated to `rbx_dom_weak` 1.0

## 0.3.0 (2019-02-14)
* Updated `rbx_tree` dependency to `rbx_dom_weak` 0.3.0

## 0.2.0 (2019-01-25)
* Updated `rbx_tree` dependency to 0.2.0

## 0.1.0
* Initial release
* Supports `String` and `Bool` types
