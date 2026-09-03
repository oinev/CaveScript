# CaveScript Reference

> **CaveScript** is a domain-specific language (DSL) designed to simplify writing
> **Deep Rock Galactic – Custom Difficulty 2** configurations.
>
> It replaces verbose configuration syntax with a cleaner, more expressive language
> while compiling directly to the CD2 format.

---

## Table of Contents

- [Comments](#comments)
- [Primitive Types](#primitive-types)
- [Variables](#variables)
- [Functions](#functions)
- [Arrays](#arrays)
- [Defined Types](#defined-types)
  - [Cells](#cells)
  - [Structs](#structs)
  - [Enums](#enums)
- [Methods](#methods)
- [Control Flow](#control-flow)
- [Deep Rock Galactic Integration](#deep-rock-galactic-integration)

---

# Comments

Single-line and multi-line comments are both supported.

```cavescript
// Single-line comment

/*
    Multi-line
    comment
*/
```

---

# Primitive Types

CaveScript provides several built-in primitive types.

| Type | Description |
|------|-------------|
| `bool` | Boolean value (`true` or `false`) |
| `char` | Single Unicode character |
| `string` | UTF-8 string |
| `int` | Integer value |
| `float` | Floating-point value |

> **Note**
>
> Although `int` exists as a language type, it is compiled into CD2's numeric
> representation, which is internally stored as floating-point values.

---

# Variables

Variables are declared using either `let` or `var`.

- `let` creates an immutable binding.
- `var` creates a mutable binding.
- Types may be omitted and will be inferred automatically.

```cavescript
let truth: bool = true

var lie = false

lie = true

let letter: char = 'C'

let message: string = "This is a string!"

var number: int = 1234

var decimal: float = 12.34
```

---

# Functions

Functions are declared with the `fn` keyword.

The return type is specified using the `to` keyword.

```cavescript
fn add(a: int, b: int) to int {
    return a + b
}
```

---

# Arrays

Arrays are growable collections that are indexed using integer values.

> **Note**
>
> Arrays are **zero-indexed**, meaning the first element is at index `0`, the
> second at index `1`, and so on.

```cavescript
var numbers: [int] = [10, 20, 30, 40]
```

Accessing elements:

```cavescript
numbers[0] // 10
numbers[1] // 20
numbers[2] // 30
numbers[3] // 40
```

Elements can be modified if the array is mutable.

```cavescript
numbers[1] = 11
```

Mutable arrays may also grow.

```cavescript
numbers.push(50)
```

Immutable arrays cannot be modified.

```cavescript
let immutable = [1, 2, 3]

immutable.push(4) // Error
immutable[0] = 10 // Error
```

### Design Notes

> Arrays are intentionally growable because many CD2 collections are dynamic.
>
> Planned features:
>
> - [ ] `.pop()`
> - [ ] `.insert()`
> - [ ] Slices

---

# Defined Types

## Cells

The `type` keyword defines a new cell type.

Unlike a traditional type alias, a cell type is a completely distinct type,
even if it shares the same underlying representation.

```cavescript
type PlayerId int
type WeaponId int

var player: PlayerId = 1
var weapon: WeaponId = 1

player = weapon // Error
```

Named types may be created from any existing type.

```cavescript
type PlayerName string
type Position [float]
type Inventory [Item]
type Laser Line
```

---

## Structs

Structs group related values together into a single type.

```cavescript
type Position struct {
    x: int
    y: int
}
```

Fields may also be declared inline.

```cavescript
type Line struct {
    length: int,
    width: int
}
```

### Creating Structs

Structs may be initialized using field assignments.

```cavescript
var laser: Line = {
    length = 1
    width = 1
}
```

or in a compact form.

```cavescript
var laser: Line = {
    length = width = 1
}
```

Fields are accessed using the dot operator.

```cavescript
echo("{}", laser.length)
```

---

## Enums

Enums represent one value selected from a fixed set.

```cavescript
type State enum {
    dead
    alive
}
```

---

# Methods

Methods are declared inside an `impl` block.

They allow behavior to be attached directly to custom types.

```cavescript
impl Line {

    fn new(size: int) to Line {

        let line: Line = {
            length = size
            width = size / 2
        }

        return line
    }

    fn addSize(self, addedSize: int) {

        self.length += addedSize
        self.width += addedSize / 2
    }

}
```

Methods may then be called using the dot operator.

```cavescript
var line = Line::new(10)

line.addSize(5)
```

---

# Control Flow

> 🚧 **Work in Progress**
>
> Control flow constructs such as `if`, `match`, `for`, and `while` are currently
> under development.

---

# Deep Rock Galactic Integration

> 🚧 **Work in Progress**
>
> CaveScript is designed specifically for **Custom Difficulty 2**.
>
> Future versions of this documentation will include:
>
> - Enemy definitions
> - Spawn pools
> - Mission events
> - Difficulty scaling
> - Hazard configuration
> - Direct mappings to CD2 configuration files

---

# Language Philosophy

CaveScript aims to be:

- **Readable** — configuration should read like code.
- **Concise** — eliminate repetitive CD2 boilerplate.
- **Safe** — catch common mistakes before compilation.
- **Expressive** — provide higher-level abstractions while compiling into valid CD2 configurations.

> **Rock and Stone!**