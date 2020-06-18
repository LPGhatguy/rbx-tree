# Roblox Binary Model Format, Version 0
This document is based on:
- [*ROBLOX File Format* by Gregory Comer](http://www.classy-studios.com/Downloads/RobloxFileSpec.pdf)
- [LibRbxl by Gregory Comer](https://github.com/GregoryComer/LibRbxl)
- [rbxfile by Anaminus](https://github.com/RobloxAPI/rbxfile)
- [Roblox-File-Format by CloneTrooper1019](https://github.com/CloneTrooper1019/Roblox-File-Format)
- Observing `rbxm` and `rbxl` output from Roblox Studio

## Contents
- [File Structure](#file-structure)
- [File Header](#file-header)
- [Chunks](#chunks)
	- [`META` Chunk](#meta-chunk)
	- [`INST` Chunk](#inst-chunk)
	- [`PROP` Chunk](#prop-chunk)
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
	- [Axis](#axis)
	- [BrickColor](#brickcolor)
	- [Color3](#color3)
	- [Vector2](#vector2)
	- [Vector3](#vector3)
	- [CFrame](#cframe)
	- [Referent](#referent)
	- [Vector3int16](#vector3int16)
	- [NumberSequence](#numbersequence)
	- [ColorSequence](#colorsequence)
	- [NumberRange](#numberrange)
	- [Rect2D](#rect2d)
	- [PhysicalProperties](#physicalproperties)
	- [Color3uint8](#color3uint8)
	- [Int64](#int64)
- [Data Storage Notes](#data-storage-notes)
	- [Interleaved Array](#interleaved-array)

## File Structure
1. File Header
2. Chunks
	1. Zero or one `META` chunks
	2. Zero or more `INST` chunk
	3. Zero or more `PROP` chunks
	4. One `PRNT` chunk
	5. One `END` chunk

## File Header
Every file starts with a 16 byte header.

1. Magic number: 8 bytes, always `<roblox!`
2. Signature: 6 bytes, always `89 ff 0d 0a 1a 0a`
3. Version: u16, always `00 00`

## Chunks
Every chunk starts with a 16 byte header followed by the chunk's data.

1. Chunk name: 4 bytes, like `META` or `INST`
2. Compressed length: u32
3. Uncompressed length: u32
4. Reserved bytes: u32, always `0`

If **chunk name** is less than four bytes, the remainder is filled with zeros.

If **compressed length** is zero, **chunk data** contains **uncompressed length** bytes of data for the chunk.

If **compressed length** is nonzero, **chunk data** contains an LZ4 compressed block. It is **compressed length** bytes long and will expand to **uncompressed length** bytes when decompressed.

When the **chunk data** is compressed, it is done so using the [LZ4](https://github.com/lz4/lz4) compression algorithm.

When documentation for individual chunks uses the term "chunk data", it refers to **chunk data** after it has been decompressed, if necessary.

### `META` Chunk

| `META` Chunk Data |
| ----------------- |
| Number of entries (`u32`) |
| Metadata Entries (fills rest of chunk) |

| Metadata Entry |
| ----- |
| Key ([String](#string)) |
| Value ([String](#string)) |

The Metadata chunk (`META`) is a map of strings to strings. It represents metadata about the model, such as whether it was authored with `ExplicitAutoJoints` enabled.

There should be zero or one `META` chunks.

Observed metadata entries and their values:

- `ExplicitAutoJoints`: `true` or `false`

### `INST` Chunk
| `INST` Chunk Data |
| ----------------- |
| Type ID (`u32`) |
| Type Name ([String](#string)) |
| Object Format (`u8`) |
| Number Instances (`u32`) |
| Instance Referents ([Referent](#referent) Array) |
| Service Markers (`u8` * number instances) |

The Instance chunk (`INST`) defines a type of instance, how many of them there are in this file, and what referent IDs they have.

There should be one `INST` chunk for each type of instance defined.

There are two forms of the `INST` chunk determined by the **object format** field:

- `0`: regular
- `1`: service

If the object format is **regular**, the service markers section will not be present.

If the object format is **service**, the service markers section contains `1` repeated for the number of instances of that type in the file. If this field is not set, Roblox may create duplicate copies of services, like in [rojo-rbx/rbx-dom#11](https://github.com/rojo-rbx/rbx-dom/issues/11).

**Type ID** must be unique and ideally sorted monotonically among all `INST` chunks. It's used later in the file to refer to this type.

**Type Name** should match the `ClassName` specified on an instance in Roblox.

The length of the **Instance Referents** array must match the **Number of Instances** field.

### `PROP` Chunk
| `PROP` Chunk Data |
| ----------------- |
| Type ID (`u32`) |
| Property Name ([String](#string)) |
| Data Type (`u8`) |
| Values (array of data) |

The property chunk (`PROP`) defines a single property for a single instance type.

There should be one `PROP` chunk per property per instance type.

Because of the shape of this chunk, every instance of a given type must have the same properties specified with the same times. Put another way, if any instance in the file defines a property, all other instances of the same type must also specify that property!

**Type ID** defines the instance that this property applies to as defined in a preceding `INST` chunk.

**Property Name** defines the serializable name of the property. Note that this is not necessarily the same as the name reflected to Lua, which is sometimes referred to as the _canonical name_.

**Data Type** corresponds to a value from [Data Types](#data-types).

**Values** contains an array of values of **Data Type** whose length is the same as the number of instances with the type ID **Type ID**.

### `PRNT` Chunk
| `PRNT` Chunk Data |
| ----------------- |
| Version (`u8`, zero) |
| Number of Objects (`u32`) |
| Object Array ([Referent](#referent) Array) |
| Parent Array ([Referent](#referent) Array) |

The parent chunk (`PRNT`) defines the hierarchy relationship between every instance in the file.

There should be exactly one `PRNT` chunk.

**Version** field should currently always be zero.

**Number of Objects** should be the same as the number of instances in the file header chunk, since each object should have a parent.

**Object Array** and **Parent Array** should both have length equal to **Number of Objects**. The parent of the ID at position *N* in the **Object Array** is a child of the ID at position *N* in the **Parent Array**.

A null parent referent (`-1`) indicates that the object is a root instance. In a place, that means the object is a child of `DataModel`. In a model, that means the object should be placed directly under the object the model is being inserted into.

### `END` Chunk
| `END` Chunk Data |
| ---------------- |
| Magic Value `</roblox>` |

The ending chunk (`END`) signifies the end of the file.

The `END` chunk should not be compressed, since it's used as a rough form of file validation when uploading places to the website.

## Data Types

### String
**Type ID `0x01`**

| String |
| ------ |
| String length in bytes (u32) |
| Data |

String data is UTF-8 encoded.

### Bool
**Type ID `0x02`**

### Int32
**Type ID `0x03`**

**Untransformed integers**, generally in header data, are little-endian and two's complement. Integers are untransformed unless denoted otherwise.

**Transformed integers**, normally used in property data, are big-endian and are transformed and untransformed via:

```rust
fn transform_i32(value: i32) -> i32 {
	if value >= 0 {
		value * 2
	} else {
		2 * -value - 1
	}
}

fn untransform_i32(value: i32) -> i32 {
	if value % 2 == 0 {
		value / 2
	} else {
		-(value +1 1) / 2
	}
}
```

Integers can also be transformed via bitwise ops to avoid branches:

```rust
fn transform_i32(value: i32) -> i32 {
	(value << 1) ^ (value >> 31)
}

fn untransform_i32(value: i32) -> i32 {
	((value as u32) >> 1) as i32 ^ -(value & 1)
}
```

### Float32
**Type ID 0x04**

### Float64
**Type ID 0x05**

### UDim
**Type ID 0x06**

### UDim2
**Type ID 0x07**

### Ray
**Type ID 0x08**

### Faces
**Type ID 0x09**

### Axis
**Type ID 0x0A**

### BrickColor
**Type ID 0x0B**

### Color3
**Type ID 0x0C**

### Vector2
**Type ID 0x0D**

### Vector3
**Type ID 0x0E**

### CFrame
**Type ID 0x10**

### Enum
**Type ID 0x12**

### Referent
**Type ID 0x13**

Referents are stored as transformed 32-bit signed integers. A value of `-1` (untransformed) indicates a null referent.

When reading an [Interleaved Array](#interleaved-array) of referents, they should be read accumulatively. In other words, the value of each referent id should be itself, plus its previous value.

Without accumulation, referents read from a file may look like this. This is **incorrect**:

- 1619
- 1
- 4
- 2
- 3
- 5

The **correct** interpretation of this data, with accumulation, is:

- 1619
- 1620
- 1624
- 1626
- 1629
- 1634

### Vector3int16
**Type ID 0x14**

### NumberSequence
**Type ID 0x15**

### ColorSequence
**Type ID 0x16**

### NumberRange
**Type ID 0x17**

### Rect2D
**Type ID 0x18**

### PhysicalProperties
**Type ID 0x19**

### Color3uint8
**Type ID 0x1A**

### Int64
**Type ID 0x1B**

## Data Storage Notes

### Interleaved Array
Arrays of many types in property data have their bytes interleaved.

For example, an array of 4 bit integers normally represented as:

|||||||||||||
|--|--|--|--|--|--|--|--|--|--|--|--|
|**A0**|**A1**|**A2**|**A3**|B0|B1|B2|B3|C0|C1|C2|C3|

Would become, after interleaving:

|||||||||||||
|--|--|--|--|--|--|--|--|--|--|--|--|
|**A0**|B0|C0|**A1**|B1|C1|**A2**|B2|C2|**A3**|B3|C3|

Note that arrays of integers are generally subject to both interleaving and integer transformation.