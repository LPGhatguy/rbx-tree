# rbx_types Changelog

## Unreleased Changes
* Added an `Attributes` struct to facilitate reading and writing of attribute values.

## 1.1.0 (2021-07-02)
* Critical fix: changed serde serialization of fields from PascalCase to camelCase. ([#191][#191])

[#191]: https://github.com/rojo-rbx/rbx-dom/pull/191

## 1.0.0 (2021-06-26) (yanked)
* Removed `legacy-compact` feature, which added conversions from rbx\_dom\_weak 1.x types.

## 0.4.0 (2021-06-26)
* Changed `Variant` serialization to use Serde's default enum representation.
  * Before: `{ "type": "Vector2", "value": [1.0, 2.0] }`
  * After: `{ "Vector2": [1.0, 2.0] }`
* Implemented `PartialOrd` and `Ord` for `SharedStringHash`.

## 0.3.1 (2021-05-14)
* Added `Variant::OptionalCoordinateFrame`.

## 0.3.0 (2021-02-16)
* Renamed `EnumValue` to `Enum`.
* Added `Display` and `FromStr` implementations for `Ref`.

## 0.2.0 (2020-04-27)
* `Ref` can now represent null explicitly via `Ref::none` and `Ref::is_none`.
* Added `Region3` and `Region3int16` to `Variant` and `VariantType`.
* Added `legacy-compat` feature to enable conversions with rbx_dom_weak 1.x.

## 0.1.0 (2020-02-05)
* Initial release of types crate, should have rough feature parity with rbx_dom_weak.
* API will move a lot before becoming stable.
