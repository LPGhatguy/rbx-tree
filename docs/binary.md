# Roblox Binary Model Format, Version 0
This is unofficial documentation for Roblox's binary model format. The binary model format is used for places (`.rbxl` files), models (`.rbxm` files), and many objects uploaded to Roblox's asset storage.

The binary model format intended to supersede Roblox's older [XML model format](/xml).

This document is based on:
- [*ROBLOX File Format* by Gregory Comer](http://www.classy-studios.com/Downloads/RobloxFileSpec.pdf)
- [LibRbxl by Gregory Comer](https://github.com/GregoryComer/LibRbxl)
- [rbxfile by Anaminus](https://github.com/RobloxAPI/rbxfile)
- [Roblox-File-Format by CloneTrooper1019](https://github.com/CloneTrooper1019/Roblox-File-Format)
- Observing `rbxm` and `rbxl` output from Roblox Studio

## Contents
- [Document Conventions](#document-conventions)
- [File Structure](#file-structure)
- [File Header](#file-header)
- [Chunks](#chunks)
	- [`META` Chunk](#meta-chunk)
	- [`SSTR` Chunk](#sstr-chunk)
	- [`INST` Chunk](#inst-chunk)
	- [`PROP` Chunk](#prop-chunk)
		- [Values](#values)
	- [`PRNT` Chunk](#prnt-chunk)
	- [`END` Chunk](#end-chunk)
- [Data Types](#data-types)
	- [String](#string)
	- [Bool](#bool)
	- [Int32](#int32)
	- [Float32](#float32)
	- [Float64](#float64)
	- [UDim](#udim)
	- [UDim2](#udim2)
	- [Ray](#ray)
	- [Faces](#faces)
	- [Axes](#axes)
	- [BrickColor](#brickcolor)
	- [Color3](#color3)
	- [Vector2](#vector2)
	- [Vector3](#vector3)
	- [CFrame](#cframe)
	- [Enum](#enum)
	- [Referent](#referent)
	- [Vector3int16](#vector3int16)
	- [NumberSequence](#numbersequence)
	- [ColorSequence](#colorsequence)
	- [NumberRange](#numberrange)
	- [Rect](#rect)
	- [PhysicalProperties](#physicalproperties)
	- [Color3uint8](#color3uint8)
	- [Int64](#int64)
	- [SharedString](#sharedstring)
	- [Optional](#optional)
- [Data Storage Notes](#data-storage-notes)
	- [Integer Transformations](#integer-transformations)
	- [Byte Interleaving](#byte-interleaving)
	- [Roblox Float Format](#roblox-float-format)

## Document Conventions
This document assumes a basic understanding of Rust's conventions for numeric types. For example:

- `u16` is an unsigned 16-bit integer
- `i32` is a signed 32-bit integer

Integers are assumed to be little endian and 2's complement unless otherwise specified. The presence of big endian integers and integers with interesting transformations are explicitly noted.

The data contained in a chunk may be compressed. The term "chunk data" refers to the decompressed contents.

## File Structure
Binary model files consist of a short header, followed by a series of chunks. Each chunk has the same framing, enabling consumers to partialy decode a file.

1. File Header
2. Chunks
	1. Zero or one `META` chunks
	2. Zero or one `SSTR` chunks
	3. Zero or more `INST` chunk
	4. Zero or more `PROP` chunks
	5. One `PRNT` chunk
	6. One `END` chunk

## File Header
Every file starts with a 32 byte header.

| Field Name     | Format  | Value                                                                     |
|:---------------|:--------|:--------------------------------------------------------------------------|
| Magic Number   | 8 bytes | Always `<roblox!`                                                         |
| Signature      | 6 bytes | Always `89 ff 0a 1a 0a`                                                   |
| Version        | `u16`   | Always `0`                                                                |
| Class Count    | `i32`   | Number of distinct classes in the file (i.e. the number of `INST` chunks) |
| Instance Count | `i32`   | Number of instances in the file                                           |
| Reserved       | 8 bytes | Always `0`                                                                |

## Chunks
Every chunk starts with a 16 byte header followed by the chunk's data.

| Field Name          | Format  | Value                                             |
|:--------------------|:--------|:--------------------------------------------------|
| Chunk Name          | 4 bytes | The chunk's name, like `META` or `INST`           |
| Compressed Length   | `u32`   | Length of the chunk in bytes, if it is compressed |
| Uncompressed Length | `u32`   | Length of the chunk's data after decompression    |
| Reserved            | 4 bytes | Always `0`                                        |

If **Chunk Name** is less than four bytes, the remainder is filled with zeros.

If **Compressed Length** is zero, **Chunk Data** contains **Uncompressed Length** bytes of data for the chunk.

If **Compressed Length** is nonzero, **Chunk Data** contains an LZ4 compressed block. It is **Compressed Length** bytes long and will expand to **Uncompressed Length** bytes when decompressed.

When the **chunk data** is compressed, it is done so using the [LZ4](https://github.com/lz4/lz4) compression algorithm.

### `META` Chunk
The `META` chunk has this layout:

| Field Name                 | Format         | Value                                       |
|:---------------------------|:---------------|:--------------------------------------------|
| Number of Metadata Entries | `u32`          | The number of metadata entries in the chunk |
| Metadata Entries           | Array(Entries) | The actual metadata entries                 |

Each metadata entry has the following format:

| Field Name     | Format              | Value                                    |
|:---------------|:--------------------|:-----------------------------------------|
| Metadata Key   | [`String`](#string) | The metadata key, which should be unique |
| Metadata Value | [`String`](#string) | The value for this metadata key          |

The Metadata chunk (`META`) is a map of strings to strings. It represents metadata about the model, such as whether it was authored with `ExplicitAutoJoints` enabled.

There should be zero or one `META` chunks.

Observed metadata entries and their values:

- `ExplicitAutoJoints`: `true` or `false`

### `SSTR` Chunk
The `SSTR` chunk has this layout:

| Field Name          | Format                | Value                                        |
|:--------------------|:----------------------|:---------------------------------------------|
| Version             | `u32`                 | The version of the `SSTR` chunk (always `0`) |
| Shared String Count | `u32`                 | The number of SharedStrings in the chunk     |
| Strings             | Array(Shared Strings) | The actual shared string entries             |

A shared string entry looks like this:

| Field Name    | Format              | Value                                                                   |
|:--------------|:--------------------|:------------------------------------------------------------------------|
| MD5 Hash      | 16 bytes            | An [MD5](https://en.wikipedia.org/wiki/MD5) hash of the `Shared String` |
| Shared String | [`String`](#string) | The string that's used by a later `PROP` chunk                          |

The Shared String chunk (`SSTR`) is an array of strings. It's used to reduce the overall size of a file by allowing large strings to be reused in [`PROP`](#prop-chunk) chunks. The `MD5 Hash` isn't used by Roblox Studio when loading the file.

There should be zero or one `SSTR` chunks.

### `INST` Chunk
The `INST` chunk has this layout:

| Field Name      | Format                         | Value                                                                  |
|:----------------|:-------------------------------|:-----------------------------------------------------------------------|
| Class ID        | `u32`                          | An arbitrarily-chosen ID referring to a Roblox class                   |
| Class Name      | [`String`](#string)            | The class name, like `Folder` or `Part`                                |
| Object Format   | `u8`                           | `1` if the class is a service, otherwise `0`                           |
| Instance Count  | `u32`                          | The number of instances belonging to the class                         |
| Referents       | Array([`Referent`](#referent)) | The referents of instances belonging to the class                      |
| Service Markers | Array(`u8`)                    | `1` for each instance if the class is a service, otherwise not present |

There should be one `INST` chunk for each type of instance defined.

There are two forms of the `INST` chunk determined by the **Object Format** field:

- `0`: regular
- `1`: service

If the **Object Format** is **regular**, the service markers section will not be present.

If the **Object Format** is **service**, the service markers section contains `1` repeated for the number of instances of that type in the file. If this field is not set, Roblox may create duplicate copies of services, like in [rojo-rbx/rbx-dom#11](https://github.com/rojo-rbx/rbx-dom/issues/11).

**Class ID** must be unique and ideally sorted monotonically among all `INST` chunks. It's used later in the file to refer to this type.

**Class Name** should match the `ClassName` specified on an instance in Roblox.

The length of **Referents** must equal **Instance Count**.

### `PROP` Chunk
The `PROP` chunk has this layout:

| Field Name    | Format              | Value                                         |
|:--------------|:--------------------|:----------------------------------------------|
| Class ID      | `u32`               | The class ID assigned in the `INST` chunk     |
| Property Name | [`String`](#string) | The name of the property, like `CFrame`       |
| Values        | [`Values`](#values) | The type of the property and a list of values | 
| Type ID       | `u8`                | The [Data Type](#data-types) of the property             |
| Values        | Array(Value)        | A list of values whose type is determined by **Type ID** |

The property chunk (`PROP`) defines a single property for a single instance type.

There should be one `PROP` chunk per property per instance type.

Because of the shape of this chunk, every instance of a given class must have the same properties specified with the same times. Put another way, if an instance in the file defines a property, all other instances of the same class must also specify that property!

**Class ID** defines the class that this property applies to as defined in a preceding `INST` chunk.

**Property Name** defines the serializable name of the property. Note that this is not necessarily the same as the name reflected to Lua, which is sometimes referred to as the _canonical name_.

**Values** is composed of a struct of the following shape:

#### Values
| Field Name | Format       | Value                                                    |
|:-----------|:-------------|:---------------------------------------------------------|
| Type ID    | `u8`         | The [Data Type](#data-types)                             |
| Values     | Array(Value) | A list of values whose type is determined by **Type ID** |

This struct is later used by the [`Optional`](#optional) type, so it's suggested that implementors decouple their **Values** (de)serializers from their chunk (de)serializers.

### `PRNT` Chunk
The `PRNT` chunk has this layout:

| Field Name       | Format                         | Value                                       |
|:-----------------|:-------------------------------|:--------------------------------------------|
| Version          | `u8`                           | Always `0`                                  |
| Instance Count   | `u32`                          | Number of instances described in this chunk |
| Child Referents  | Array([`Referent`](#referent)) | Referents of child instances                |
| Parent Referents | Array([`Referent`](#referent)) | Referents of parent instances               |

The parent chunk (`PRNT`) defines the hierarchy relationship between every instance in the file.

There should be exactly one `PRNT` chunk.

**Version** field should currently always be zero.

**Instance Count** should be equal to the number of instances in the file header chunk, since each object should have a parent.

**Child Referents** and **Parent Referents** should both have length equal to **Instance Count**. The parent of the ID at position *N* in **Child Referents** is a child of the ID at position *N* in **Parent Referents**.

A null parent referent (`-1`) indicates that the object is a root instance. In a place, that means the object is a child of `DataModel`. In a model, that means the object should be placed directly under the object the model is being inserted into.

### `END` Chunk
The `END` chunk has this layout:

| Field Name  | Format  | Value              |
|:------------|:--------|:-------------------|
| Magic Value | 9 bytes | Always `</roblox>` |

The ending chunk (`END`) signifies the end of the file.

The `END` chunk must not be compressed. It is used as a rough form of file validation when uploading places to the Roblox website.

## Data Types

### String
**Type ID `0x01`**

The `String` type is stored as a length-prefixed sequence of bytes. The length is stored as an untransformed 32-bit integer. `String` values are UTF-8 encoded.

| Field Name | Format       | Value                                    |
|:-----------|:-------------|:-----------------------------------------|
| Length     | `u32`        | The length of the string                 |
| Data       | Array(Bytes) | The actual bytes that make up the string |

When an array of `String` values is present, they are stored in sequence without any modification.

### Bool
**Type ID `0x02`**

The `Bool` type is stored as a single byte. If the byte is `0x00`, the bool is `false`. If it is `0x01`, it is `true`.

When an array of `Bool` values is present, they are stored in sequence.

### Int32
**Type ID `0x03`**

The `Int32` type is stored as a big-endian [transformed 32-bit integer](#integer-transformations).

When an array of `Int32` values is present, the bytes of the integers are subject to [byte interleaving](#byte-interleaving).

### Float32
**Type ID `0x04`**

The `Float32` type is stored using the [Roblox float format](#roblox-float-format) and is little-endian. This datatype is also called `float` or `single`.

When an array of `Float32` values is present, the bytes of the floats are subject to [byte interleaving](#byte-interleaving).

### Float64
**Type ID `0x05`**

The `Float64` type is stored using the [IEEE-754 format](https://en.wikipedia.org/wiki/Double-precision_floating-point_format) and is little-endian. This datatype is also called `double`.

When an array of `Float64` values is present, they are in sequence with no transformations.

### UDim
**Type ID `0x06`**

The `UDim` type is stored as a struct composed of a [`Float32`](#float32) and an [`Int32`](#int32):

| Field Name | Format                | Value                                |
|:-----------|:----------------------|:-------------------------------------|
| Scale      | [`Float32`](#float32) | The `Scale` component of the `UDim`  |
| Offset     | [`Int32`](#int32)     | The `Offset` component of the `UDim` |

When an array of `UDim` values is present, the bytes of each individual components are stored as arrays, meaning their bytes are subject to [byte interleaving](#byte-interleaving).

Two `UDim` values `{1, 2}` and `{3, 4}` look like this: `7f 80 00 80 00 00 00 00 00 00 00 00 00 00 04 08`.

The first 8 bytes (`7f 80 00 80 00 00 00 00`) represent the Scale values of the `UDim` values. The latter 8 bytes (`00 00 00 00 00 00 04 08`) represent the Offset values. From there, the values are paired off, so that the first value in each array make up the components of the first `UDim`, and so on.

### UDim2
**Type ID `0x07`**

The `UDim2` type is a struct composed of two `UDim` values, one for each axis:

| Field Name | Format          | Value                            |
|:-----------|:----------------|:---------------------------------|
| X          | [`UDim`](#udim) | The `X` component of the `UDim2` |
| Y          | [`UDim`](#udim) | The `Y` component of the `UDim2` |

`UDim2` is stored as four arrays of component values in the order `X.Scale`, `Y.Scale`, `X.Offset`, `Y.Offset`. Each array is separately [byte interleaved](#byte-interleaving).

An encoded `UDim2` with value `{0.75, -30, -1.5, 60}` looks like this: `7e 80 00 00 7f 80 00 01 00 00 00 3b 00 00 00 78`.

### Ray
**Type ID `0x08`**

The `Ray` type is a struct composed of six little-endian `f32` values, making up the components of the `Origin` and then the `Direction` of the `Ray`:

| Field Name  | Format | Value                                             |
|:------------|:-------|:--------------------------------------------------|
| Origin X    | `f32`  | The `X` component of the `Origin` of the `Ray`    |
| Origin Y    | `f32`  | The `Y` component of the `Origin` of the `Ray`    |
| Origin Z    | `f32`  | The `Z` component of the `Origin` of the `Ray`    |
| Direction X | `f32`  | The `X` component of the `Direction` of the `Ray` |
| Direction Y | `f32`  | The `Y` component of the `Direction` of the `Ray` |
| Direction Z | `f32`  | The `Z` component of the `Direction` of the `Ray` |

The components are stored in order without any additional transformations. When an array of `Ray` values is present, they're stored in order but otherwise without transformation.

### Faces
**Type ID `0x09`**

The `Faces` type is a single byte used as a bit field. The low 6 bits represent the `Front`, `Bottom`, `Left`, `Back`, `Top`, and `Right` faces, in that order. The remaining two bits have no meaning. `Faces` is stored as an array of bytes with no transformations or interleaving.

Three encoded `Faces` with values `Front`, `Back, Top` and `Bottom, Left, Right` looks like this: `01 18 26`.

### Axes
**Type ID `0x0a`**

The `Axes` type is a single byte used as a bit field. The low three bits represent the `X`, `Y`, and `Z` axes, in that order. The remaining five bits have no meaning. `Axes` is stored as an array of bytes with no transformations or interleaving.

Three encoded `Axes` with values `X`, `X Y`, and `X Z` look like this: `01 03 05`.

### BrickColor
**Type ID `0x0b`**

The `BrickColor` type is a single untransformed big-endian `u32` that represents the `Number` of a `BrickColor`. When an array of `BrickColor` values is present, the Numbers are [byte interleaved](#byte-interleaving) but otherwise are unchanged.

As an example, three encoded `BrickColor` values `Really red (1004)`, `Bright green (37)`, and `Really blue (1010)` look like this: `00 00 00 00 00 00 03 00 03 EC 25 F2`.

### Color3
**Type ID `0x0c`**

The `Color3` type is a struct composed of three `Float32` values:

| Field Name | Format                | Value                             |
|:-----------|:----------------------|:----------------------------------|
| R          | [`Float32`](#float32) | The `R` component of the `Color3` |
| G          | [`Float32`](#float32) | The `G` component of the `Color3` |
| B          | [`Float32`](#float32) | The `B` component of the `Color3` |

`Color3` is stored as three arrays of components in the order `R`, `G`, `B`. Each array is separately [byte interleaved](#byte-interleaving).

An encoded `Color3` with RGB value `255, 180, 20` looks like this: `7f 00 00 00 7e 69 69 6a 7b 41 41 42`.

### Vector2
**Type ID `0x0d`**

The `Vector2` type is a struct composed of two `Float32` values:

| Field Name | Format                | Value                              |
|:-----------|:----------------------|:-----------------------------------|
| X          | [`Float32`](#float32) | The `X` component of the `Vector2` |
| Y          | [`Float32`](#float32) | The `Y` component of the `Vector2` |

`Vector2` is stored as two arrays of components in the order `X`, `Y`. Each array is separately [byte interleaved](#byte-interleaving).

Two encoded `Vector2` values `-100.80, 200.55`, `200.55, -100.80` look like this: `85 86 93 91 33 19 35 9a 86 85 91 93 19 33 9a 35`

### Vector3
**Type ID `0x0e`**

The `Vector3` type is a struct composed of three `Float32` values:

| Field Name | Format                | Value                              |
|:-----------|:----------------------|:-----------------------------------|
| X          | [`Float32`](#float32) | The `X` component of the `Vector3` |
| Y          | [`Float32`](#float32) | The `Y` component of the `Vector3` |
| Z          | [`Float32`](#float32) | The `Z` component of the `Vector3` |

`Vector3` is stored as three arrays of components in the order `X`, `Y`, `Z`. Each array is separately [byte interleaved](#byte-interleaving).

Two encoded `Vector3` values `1, 2, 3` and `-1, -2, -3` look like this: `7F 7F 00 00 00 00 00 01 80 80 00 00 00 00 00 01 80 80 80 80 00 00 00 01`.

### CFrame
**Type ID `0x10`**

The `CFrame` type is more complicated than other types. To save space, there are 24 special cases where only the `CFrame`'s position is saved. The special case's ID is written as a single byte.

If the byte is `00`, a `CFrame` looks like this:

| Field Name  | Format                  | Value                                                                                                        |
|:------------|:------------------------|:-------------------------------------------------------------------------------------------------------------|
| ID          | `u8`                    | Always `00` in this case.                                                                                    |
| Orientation | Array of 9 `f32` values | The rotation matrix of the `CFrame`. It represents the RightVector, UpVector, and LookVector, in that order. |
| Position    | [`Vector3`](#vector3)   | The position of the `CFrame`.                                                                                |

In this case, the `Orientation` field is stored as nine untransformed [IEEE-754 standard](https://en.wikipedia.org/wiki/Single-precision_floating-point_format) 32-bit floats.

If the `ID` is **not** `00`, it will be a value from the following table. In this case, the `Orientation` field isn't present and is instead equivalent to the angles paired with the `ID` in the table. Rotations in this table are in degrees and are applied in the order `Y -> X -> Z`.

| ID   | Rotation       | ID   | Rotation       |
|:-----|:---------------|:-----|:---------------|
| `02` | (0, 0, 0)      | `14` | (0, 180, 0)    |
| `03` | (90, 0, 0)     | `15` | (-90, -180, 0) |
| `05` | (0, 180, 180)  | `17` | (0, 0, 180)    |
| `06` | (-90, 0, 0)    | `18` | (90, 180, 0)   |
| `07` | (0, 180, 90)   | `19` | (0, 0, -90)    |
| `09` | (0, 90, 90)    | `1b` | (0, -90, -90)  |
| `0a` | (0, 0, 90)     | `1c` | (0, -180, -90) |
| `0c` | (0, -90, 90)   | `1e` | (0, 90, -90)   |
| `0d` | (-90, -90, 0)  | `1f` | (90, 90, 0)    |
| `0e` | (0, -90, 0)    | `20` | (0, 90, 0)     |
| `10` | (90, -90, 0)   | `22` | (-90, 90, 0)   |
| `11` | (0, 90, 180)   | `23` | (0, -90, 180)  |

When an array of `CFrame` values is present, for each value the `ID` is stored followed by the `Rotation` field if it's present. Then, an array of [`Vector3`](#vector3) values that represent the `Position` field of each `CFrame`.

Two `CFrame` values with the components `CFrame.new(1, 2, 3)` and `CFrame.new(4, 5, 6)*CFrame.Angles(7, 8, 9)` are stored as `02 00 4B C0 07 3E 08 9C 75 3D 95 46 7D 3F 1D 25 90 BE 58 6C 74 BF 84 C5 C3 3D 1E 4A 73 3F 6F 19 95 BE 9F A6 E0 BD 7F 81 00 00 00 00 00 00 80 7F 00 22 00 D4 00 B2 80 81 80 80 00 00 00 00`.

The first part (the `ID` and `Rotation` array) is: `02 00 4B C0 07 3E 08 9C 75 3D 95 46 7D 3F 1D 25 90 BE 58 6C 74 BF 84 C5 C3 3D 1E 4A 73 3F 6F 19 95 BE 9F A6 E0 BD`, which is an split into `02` and `00 4B C0 07 3E 08 9C 75 3D 95 46 7D 3F 1D 25 90 BE 58 6C 74 BF 84 C5 C3 3D 1E 4A 73 3F 6F 19 95 BE 9F A6 E0 BD`.

The second part (the `Position` array) is: `7F 81 00 00 00 00 00 00 80 7F 00 22 00 D4 00 B2 80 81 80 80 00 00 00 00`.

### Enum
**Type ID `0x12`**

The `Enum` type is an unsigned 32-bit integer. It is stored as big endian and is subject to [byte interleaving](#byte-interleaving).

### Referent
**Type ID `0x13`**
The `Referent` type represents a specific Instance in the file and is stored as an [`Int32`](#int32). After untransforming a `Referent`, a value of `-1` represents the so-called 'null referent'. In a `PROP` chunk, a null referent represents a property with no set value: for example, the default value of `ObjectValue.Value`.

An array of `Referent` values is stored as an array of `Int32` values, and as a result they are subject to [byte interleaving](#byte-interleaving). When reading an array of `Referent` values, they must be read accumulatively. That is to say that the 'actual' value of the `Referent` is the value of the read value plus the preceding one.

Without accumulation, `Referent` values read from a file may look like this. This is **incorrect**:

| Referent 1 | Referent 2 | Referent 3 | Referent 4 | Referent 5 | Referent 6 |
|:----------:|:----------:|:----------:|:----------:|:----------:|:----------:|
| 1619       | 1          | 4          | 2          | 3          | 5          |

The **correct** interpretation of this data, with accumulation, is:

| Referent 1 | Referent 2 | Referent 3 | Referent 4 | Referent 5 | Referent 6 |
|:----------:|:----------:|:----------:|:----------:|:----------:|:----------:|
| 1619       | 1620       | 1624       | 1626       | 1629       | 1634       |

### Vector3int16
**Type ID `0x14`**

The `Vector3int16` type is stored as three little-endian `i16` values:

| Field Name | Format | Value                                   |
|:-----------|:-------|:----------------------------------------|
| X          | `i16`  | The `X` component of the `Vector3int16` |
| Y          | `i16`  | The `Y` component of the `Vector3int16` |
| Z          | `i16`  | The `Z` component of the `Vector3int16` |

Multiple `Vector3int16` values are stored in sequence without any transformations or interleaving. Two `Vector3int16` values `1, 2, 3` and `-1, -2, -3` are stored like this: `00 01 00 02 00 03 FF FF FE FF FD FF`.

### NumberSequence
**Type ID `0x15`**

The `NumberSequence` type is stored as a `u32` indicating how many `NumberSequenceKeypoint` values are in the sequence followed by an array of `NumberSequenceKeypoint` values:

| Field Name     | Format                          | Value                                   |
|:---------------|:--------------------------------|:----------------------------------------|
| Keypoint count | `u32`                           | The number of keypoints in the sequence |
| Keypoints      | Array(`NumberSequenceKeypoint`) | The data for the keypoints              |

`NumberSequenceKeypoint` is a struct composed of the following fields:

| Field Name | Format | Value                         |
|:-----------|:-------|:------------------------------|
| Time       | `f32`  | The time for the keypoint     |
| Value      | `f32`  | The value of the keypoint     |
| Envelope   | `f32`  | The envelope for the keypoint |

When multiple `NumberSequence` values are present, they are stored in sequence with no transformation or interleaving. Two `NumberSequence` values

```lua
NumberSequence.new(
	NumberSequenceKeypoint.new(0, 0),
	NumberSequenceKeypoint.new(0.5, 1),
	NumberSequenceKeypoint.new(1, 1, 0.5)
)
```

```lua
NumberSequence.new(
	NumberSequenceKeypoint.new(0, 1),
	NumberSequenceKeypoint.new(0.5, 0.5, 0.5),
	NumberSequenceKeypoint.new(1, 0.5)
)
```

look like this: `03 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 3f 00 00 80 3f 00 00 00 00 00 00 80 3f 00 00 80 3f 00 00 00 3f 03 00 00 00 00 00 00 00 00 00 80 3f 00 00 00 00 00 00 00 3f 00 00 00 3f 00 00 00 3f 00 00 80 3f 00 00 00 3f 00 00 00 00`

### ColorSequence
**Type ID `0x16`**

The `ColorSequence` type is stored as a `u32` indicating how many `ColorSequenceKeypoint` values are in the `ColorSequence` followed by an array of `ColorSequenceKeypoint` values:

| Field Name     | Format                         | Value                                   |
|:---------------|:-------------------------------|:----------------------------------------|
| Keypoint count | `u32`                          | The number of keypoints in the sequence |
| Keypoints      | Array(`ColorSequenceKeypoint`) | The data for each keypoint              |

`ColorSequenceKeypoint` is a struct composed of the following fields:

| Field Name        | Format              | Value                                          |
|:------------------|:--------------------|:-----------------------------------------------|
| Time              | `f32`               | The time value of the `ColorSequenceKeypoint`  |
| Color             | [`Color3`](#Color3) | The color value of the `ColorSequenceKeypoint` |
| Envelope (unused) | `f32`               | n/a; serialized, but not used                  |

Note that `Color` is **not** subject to interleaving like a normal `Color3`. When multiple `ColorSequence` values are present, they are stored in sequence with no transformation or interleaving. Two `ColorSequence` values

```lua
ColorSequence.new(
	ColorSequenceKeypoint.new(0, Color3.FromRGB(255, 255, 255)),
	ColorSequenceKeypoint.new(0.5, Color3.FromRGB(0, 0, 0)),
	ColorSequenceKeypoint.new(1, Color3.FromRGB(255, 255, 255))
)
```

```lua
ColorSequence.new(
	ColorSequenceKeypoint.new(0, Color3.FromRGB(255, 0, 0)),
	ColorSequenceKeypoint.new(0.5, Color3.FromRGB(0, 255, 0))
	ColorSequenceKeypoint.new(1, Color3.FromRGB(0, 0, 255))
)
```

look like this: `03 00 00 00 00 00 00 00 00 00 80 3f 00 00 80 3f 00 00 80 3f 00 00 00 00 00 00 00 3f 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 80 3f 00 00 80 3f 00 00 80 3f 00 00 80 3f 00 00 00 00 03 00 00 00 00 00 00 00 00 00 80 3f 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 3f 00 00 00 00 00 00 80 3f 00 00 00 00 00 00 00 00 00 00 80 3f 00 00 00 00 00 00 00 00 00 00 80 3f 00 00 00 00`.

### NumberRange
**Type ID `0x17`**

The `NumberRange` type is stored as two little-endian floats:

| Field Name | Format | Value                          |
|:-----------|:-------|:-------------------------------|
| Min        | `f32`  | The minimum value of the range |
| Max        | `f32`  | The maximum value of the range |

Multiple `NumberRange` values are stored in sequence with no transformation or interleaving. Two `NumberRange` values `NumberRange.new(0, 0.5)` and `NumberRange.new(0.5, 1)` look like this: `00 00 00 00 00 00 00 3f 00 00 00 3f 00 00 80 3f`.

### Rect
**Type ID `0x18`**

The `Rect` type is a struct composed of two `Vector2` values:

| Field Name | Format                | Value                           |
|:-----------|:----------------------|:--------------------------------|
| Min        | [`Vector2`](#vector2) | The minimum value of the `Rect` |
| Max        | [`Vector2`](#vector2) | The maximum value of the `Rect` |

`Rect` is stored as four arrays of [`Float32`](#float32)s in the order `Min.X`, `Min.Y`, `Max.X`, `Max.Y`. Each array is subject to [byte interleaving](#byte-interleaving).

Two encoded `Rect` values with values `Rect.new(-1, -10, 8, 9)` and `Rect.new(0, 1, 5, 6)` look like this: `7f 00 00 00 00 00 01 00 82 7f 40 00 00 00 01 00 82 81 00 40 00 00 00 00 82 81 20 80 00 00 00 00`.

### PhysicalProperties
**Type ID `0x19`**

The `PhysicalProperties` type contains a flag which may be followed by a `CustomPhysicalProperties` value. `CustomPhysicalProperties` is a struct composed of five `f32` values:

| Field Name       | Format | Value                                                        |
|:-----------------|:-------|:-------------------------------------------------------------|
| Density          | `f32`  | The density set for the custom physical properties           |
| Friction         | `f32`  | The friction set for the custom physical properties          |
| Elasticity       | `f32`  | The elasticity set for the custom physical properties        |
| FrictionWeight   | `f32`  | The friction weight set for the custom physical properties   |
| ElasticityWeight | `f32`  | The elasticity weight set for the custom physical properties |

If there is no `CustomPhysicalProperties` value, a `PhysicalProperties` is stored as a single byte of value `0`. Otherwise, it is stored as a byte of value `1` immediately followed by a `CustomPhysicalProperties` stored as little-endian floats (in the same order as the above table). When there are multiple `PhysicalProperties` present, they are stored in sequence with no transformations or interleaving.

A default `PhysicalProperties` (i.e. no custom properties set) followed by a `PhysicalProperties` of value `PhysicalProperties.new(0.7, 0.3, 0.5, 1, 1)` looks like this: `00 01 33 33 33 3f 9a 99 99 3e 00 00 00 3f 00 00 80 3f 00 00 80 3f`.

### Color3uint8
**Type ID `0x1a`**

The `Color3uint8` type is a struct made up of three bytes, one for each component:

| Field Name | Format | Value                                  |
|:-----------|:-------|:---------------------------------------|
| R          | `u8`   | The `R` component of the `Color3uint8` |
| G          | `u8`   | The `G` component of the `Color3uint8` |
| B          | `u8`   | The `B` component of the `Color3uint8` |

`Color3uint8` is stored as three consecutive arrays of components in the order `R`, `G`, `B`. It is not subject to any transformation or byte interleaving.

Two `Color3uint8` values with the values `0, 255, 255` and `63, 0, 127`, respectively, look like this: `00 3f ff 00 ff 7f`.

### Int64
**Type ID `0x1b`**

The `Int64` type is stored as a big-endian [transformed 64-bit integer](#integer-transformations).

When an array of `Int64` values is present, the bytes of the integers are subject to [byte interleaving](#byte-interleaving).

### SharedString
**Type ID `0x1c`**

`SharedString` values are stored as an [Interleaved Array](#byte-interleaving) of `u32` values that represent indices in the [`SSTR`](#sstr-chunk) string array.

### Optional
**Type ID `0x1e`**

The `Optional` type is stored as a struct of two [`Values`](#values) in the following shape:

| Field Name | Type                | Value                                                   |
|:-----------|:--------------------|:--------------------------------------------------------|
| Data       | [`Values`](#values) | The actual values of the property                       |
| Presence   | [`Values`](#values) | An indicator for what Instances have 'filled in' values |

This type is used for properties that are considered optional by Roblox (such as `Model.WorldPivotData`).

The `Data` field is a list of all values for the property. The `Presence` field is a list of booleans that indicates whether a particular Instance has an actual value for the property or not. Both fields will be the same length as one would expect when decoding a property.

In the event that a value in `Data` isn't filled, the default of the particular type will be substituted in so as to not leave any holes. As an example, if a `CFrame` weren't filled, the identity CFrame would be written.

The values in `Presence` indicate which values are filler and which ones are 'real'. If, as an example, the second entry in `Presence` was `00` then the second entry in `Data` would be the default value for the type and when applying properties to Instances, the second one would be skipped.

An `Optional` CFrame with value `CFrame.new(0, 0, 1, 0, -1, 0, 1, 0, 0, 0, 0, 1)` followed by no value looks like this: `10 0a 02 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 7f 00 00 00 00 00 00 00 02 01 00`.

Note that the valueless `Optional` is written as the identity CFrame with its corresponding boolean `00` codifying its valuelessness.

## Data Storage Notes

### Integer Transformations
Some integers may be subject to a transformation to make them more compressable.

To transform an integer: if `x` greater than or equal to zero, transform it with `2 * x`. Otherwise, use `2 * |x| - 1`. In most compilers this is equivalent to `(x << 1) ^ (x >> 31)` for 32-bit integers. For 64-bit integers, the same format is used but with `63` instead of `31`.

To untransform one: if `x` is divisible by 2, untransform it with `x / 2`. Otherwise, use `-(x + 1) / 2`. This is equivalent to `(x >> 1) ^ -(x & 1)`.

Untransforming with bitwise operators requires casting to an unsigned integer in some cases because `x >> 1` will result in a negative number if `x` is negative.

### Byte Interleaving
When stored as arrays, some data types have their bytes interleaved to help with compression. Cases where byte interleaving is present are explicitly noted.

When the bytes of an array are interleaved, they're stored with the first bytes all in sequence, then the second bytes, then the third, and so on. As an example, the sequence `A0 A1 B0 B1 C0 C1` is stored as `A0 B0 C0 A1 B1 C1`.

Viewed another way, it means that the bytes are effectively stored in 'columns' rather than 'rows'. If an array of four 32-bit integers were viewed as a 4x4 matrix, for example, it would normally look like this:

|       | Column 1 | Column 2 | Column 3 | Column 4 |
|:-----:|:--------:|:--------:|:--------:|:--------:|
| Row 1 | `A0`     | `A1`     | `A2`     | `A3`     |
| Row 2 | `B0`     | `B1`     | `B2`     | `B3`     |
| Row 3 | `C0`     | `C1`     | `C2`     | `C3`     |
| Row 4 | `D0`     | `D1`     | `D2`     | `D3`     |

When interleaved, the same array would instead look like this:

|       | Column 1 | Column 2 | Column 3 | Column 4 |
|:-----:|:--------:|:--------:|:--------:|:--------:|
| Row 1 | `A0`     | `B0`     | `C0`     | `D0`     |
| Row 2 | `A1`     | `B1`     | `C1`     | `D1`     |
| Row 3 | `A2`     | `B2`     | `C2`     | `D2`     |
| Row 4 | `A3`     | `B3`     | `C3`     | `D3`     |

### Roblox Float Format
Some data types do not follow the [IEEE-754 standard](https://en.wikipedia.org/wiki/Single-precision_floating-point_format) format for 32-bit floating point numbers. Instead, they use a proprietary format where the sign bit is after the mantissa.

| Format   | Bit Layout                            |
|:---------|:--------------------------------------|
| Standard | `seeeeeee emmmmmmm mmmmmmmm mmmmmmmm` |
| Roblox   | `eeeeeeee mmmmmmmm mmmmmmmm mmmmmmms` |

Where `s` is the sign bit, `e` is an exponent bit, and `m` is a mantissa bit.

As a practical example, below is a comparison of how `-0.15625` is stored:

| Format   | Binary View                           | Byte View     |
|:---------|:--------------------------------------|:--------------|
| Standard | `10111110 00100000 00000000 00000000` | `be 20 00 00` |
| Roblox   | `01111100 01000000 00000000 00000001` | `7c 40 00 01` |
