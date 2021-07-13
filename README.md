<div align="center">
	<img width="400" src="rbx-dom-logo.png" />
</div>

<h1 align="center">rbx-dom</h1>
<div align="center">
	<a href="https://github.com/rojo-rbx/rbx-dom/actions">
		<img title="GitHub Actions" src="https://github.com/rojo-rbx/rbx-dom/workflows/CI/badge.svg" />
	</a>
</div>

rbx-dom is a collection of cross-platform libraries that enables any software to interact with Roblox instances.

Documentation about the project is hosted at [dom.rojo.space](https://dom.rojo.space).

## [rbx_dom_weak](rbx_dom_weak)
[![rbx_dom_weak on crates.io](https://img.shields.io/crates/v/rbx_dom_weak.svg)](https://crates.io/crates/rbx_dom_weak)
[![rbx_dom_weak docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_dom_weak)

Weakly-typed Roblox DOM implementation. Defines types for representing instances and properties on them.

## [rbx_types](rbx_types)
[![rbx_types on crates.io](https://img.shields.io/crates/v/rbx_types.svg)](https://crates.io/crates/rbx_types)
[![rbx_types docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_types)

Contains Roblox's value types like `Vector3` and `NumberSequence`. Used by crates like rbx_dom_weak and a future rbx_dom_strong crate to let them share types and conversions.

## [rbx_xml](rbx_xml)
[![rbx_xml on crates.io](https://img.shields.io/crates/v/rbx_xml.svg)](https://crates.io/crates/rbx_xml)
[![rbx_xml docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_xml)

Serializer and deserializer for for Roblox's XML model and place formats, `rbxmx` and `rbxlx`.

## [rbx_binary](rbx_binary)
[![rbx_binary on crates.io](https://img.shields.io/crates/v/rbx_binary.svg)](https://crates.io/crates/rbx_binary)
[![rbx_binary docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_binary)

Serializer and deserializer for for Roblox's binary model and place formats, `rbxm` and `rbxl`.

## [rbx_reflection](rbx_reflection)
[![rbx_reflection on crates.io](https://img.shields.io/crates/v/rbx_reflection.svg)](https://crates.io/crates/rbx_reflection)
[![rbx_reflection docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_reflection)

Roblox reflection types for working with Instances in external tooling.

## [rbx_reflection_database](rbx_reflection_database)
[![rbx_reflection_database on crates.io](https://img.shields.io/crates/v/rbx_reflection_database.svg)](https://crates.io/crates/rbx_reflection_database)
[![rbx_reflection_database docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/rbx_reflection_database)

Bundled reflection database using types from rbx_reflection. Intended for users migrating from rbx_reflection 4.x and users who need reflection information statically.

## [rbx_util](rbx_util)
Command line utility to convert and debug Roblox model files.

## [rbx_dom_lua](rbx_dom_lua)

Roblox Lua implementation of DOM APIs, allowing Instance reflection from inside Roblox. Uses a data format that's compatible with rbx_dom_weak to facilitate communication with applications outside Roblox about instances.

## Property Type Coverage

| Property Type      | Example Property                | rbx_types | rbx_dom_lua | rbx_xml | rbx_binary
|:-------------------|:--------------------------------|:--:|:--:|:--:|:--:|
| Axes               | `ArcHandles.Axes`               | ✔ | ✔ | ✔ | ✔ |
| BinaryString       | `Terrain.MaterialColors`        | ✔ | ➖ | ✔ | ✔ |
| Bool               | `Part.Anchored`                 | ✔ | ✔ | ✔ | ✔ |
| BrickColor         | `Part.BrickColor`               | ✔ | ✔ | ✔ | ✔ |
| CFrame             | `Camera.CFrame`                 | ✔ | ✔ | ✔ | ✔ |
| Color3             | `Lighting.Ambient`              | ✔ | ✔ | ✔ | ✔ |
| Color3uint8        | `Part.BrickColor`               | ✔ | ✔ | ✔ | ✔ |
| ColorSequence      | `Beam.Color`                    | ✔ | ✔ | ✔ | ✔ |
| Content            | `Decal.Texture`                 | ✔ | ✔ | ✔ | ✔ |
| Enum               | `Part.Shape`                    | ✔ | ✔ | ✔ | ✔ |
| Faces              | `Handles.Faces`                 | ✔ | ✔ | ✔ | ✔ |
| Float32            | `Players.RespawnTime`           | ✔ | ✔ | ✔ | ✔ |
| Float64            | `Sound.PlaybackLoudness`        | ✔ | ✔ | ✔ | ✔ |
| Int32              | `Frame.ZIndex`                  | ✔ | ✔ | ✔ | ✔ |
| Int64              | `Player.UserId`                 | ✔ | ✔ | ✔ | ✔ |
| NumberRange        | `ParticleEmitter.Lifetime`      | ✔ | ✔ | ✔ | ✔ |
| NumberSequence     | `Beam.Transparency`             | ✔ | ✔ | ✔ | ✔ |
| Optional           | `Model.WorldPivotData`          | ✔ | ❌ | ✔ | ✔ |
| PhysicalProperties | `Part.CustomPhysicalProperties` | ✔ | ✔ | ✔ | ✔ |
| ProtectedString    | `ModuleScript.Source`           | ✔ | ✔ | ✔ | ✔ |
| Ray                | `RayValue.Value`                | ✔ | ✔ | ✔ | ✔ |
| Rect               | `ImageButton.SliceCenter`       | ✔ | ✔ | ✔ | ✔ |
| Ref                | `Model.PrimaryPart`             | ✔ | ✔ | ✔ | ✔ |
| Region3            | N/A                             | ✔ | ✔ | ❌ | ❌ |
| Region3int16       | `Terrain.MaxExtents`            | ✔ | ✔ | ❌ | ❌ |
| SharedString       | N/A                             | ✔ | ✔ | ✔ | ✔ |
| String             | `Instance.Name`                 | ✔ | ✔ | ✔ | ✔ |
| UDim               | `UIListLayout.Padding`          | ✔ | ✔ | ✔ | ✔ |
| UDim2              | `Frame.Size`                    | ✔ | ✔ | ✔ | ✔ |
| Vector2            | `ImageLabel.ImageRectSize`      | ✔ | ✔ | ✔ | ✔ |
| Vector2int16       | N/A                             | ✔ | ✔ | ✔ | ❌ |
| Vector3            | `Part.Size`                     | ✔ | ✔ | ✔ | ✔ |
| Vector3int16       | `TerrainRegion.ExtentsMax`      | ✔ | ✔ | ✔ | ✔ |
| QDir               | `Studio.Auto-Save Path`         | ⛔ | ⛔ | ⛔ | ⛔ |
| QFont              | `Studio.Font`                   | ⛔ | ⛔ | ⛔ | ⛔ |

✔ Implemented | ❌ Unimplemented | ➖ Partially Implemented | ⛔ Never

## Outcome
This project has unveiled a handful of interesting bugs and quirks in Roblox!

- `GuiMain.DisplayOrder` is uninitialized, so its default value isn't stable
- `MaxPlayersInternal` and `PreferredPlayersInternal` on `Players` are scriptable and accessible by the command bar
- Instantiating a `NetworkClient` will turn your edit session into a game client and stop you from sending HTTP requests
- `ContentProvider.RequestQueueSize` is mistakenly marked as serializable
- Trying to invoke `game:GetService("Studio")` causes a unique error: `singleton Studio already exists`
- `Color3` properties not serialized as `Color3uint8` would have their colors mistakenly clamped in the XML place format. This was bad for properties on `Lighting`.
- `ColorSequence`'s XML serialization contains an extra value per keypoint that was intended to be used as an envelope value, but was never implemented.

## Minimum Rust Version
rbx-dom supports Rust 1.40.0 and newer. Updating the minimum supported Rust version will only be done when necessary, but may happen as part of minor version bumps.

## License
rbx-dom is available under the MIT license. See [LICENSE.txt](LICENSE.txt) for details.
