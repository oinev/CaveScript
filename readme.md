# CaveScript

*A domain-specific language for writing Deep Rock Galactic Custom Difficulty 2 configurations.*

CaveScript is an experimental programming language designed to simplify the creation of **Custom Difficulty 2** configurations for **Deep Rock Galactic**. Rather than writing verbose configuration files directly, CaveScript provides a cleaner, more expressive syntax that compiles into valid CD2 configuration files.

The language is currently under active development. Until version **1.0**, the language specification and compiler behavior should be considered unstable.

---

## Motivation

Custom Difficulty 2 configurations often contain a significant amount of repetitive boilerplate. As projects grow, they become increasingly difficult to maintain and reuse.

CaveScript aims to address this by providing:

* A concise, readable syntax
* Strong typing and custom data structures
* Functions and methods for reusable logic
* A compiler that targets the CD2 configuration format directly

The language is intentionally focused on a single purpose rather than being a general-purpose programming language.

---

## Current Status

| Component         | Status            |
| ----------------- | ----------------- |
| Lexer             | Complete          |
| Parser            | In development    |
| Semantic analysis | In development    |
| Code generation   | Planned           |
| Standard library  | Planned           |
| CD2 integration   | Early development |
| Documentation     | Ongoing           |

The language reference is updated alongside development and may occasionally document features that have not yet been implemented.

---

## Implemented Features

* Variables (`let` and `var`)
* Primitive types
* Arrays
* Functions
* Type aliases
* Structs
* Enums
* Methods (`impl`)

Planned features include control flow, modules, and additional language conveniences where they improve the experience of writing CD2 configurations.

---

## Example

```cavescript
type Line struct {
    length: int
    width: int
}

impl Line {

    fn new(size: int) to Line {
        return {
            length = size
            width = size / 2
        }
    }

}

var line = Line::new(10)

line.addSize(5)

echo("{}", line.length)
```

---

## Project Goals

CaveScript is designed around a small set of principles:

* Keep configuration readable.
* Reduce unnecessary repetition.
* Catch mistakes before generating CD2 files.
* Remain focused on Deep Rock Galactic modding.

Features are only added if they make writing CD2 configurations simpler or more maintainable.

---

## Documentation

The language reference is maintained in the `docs/` directory and evolves alongside the compiler.

As the project matures, the documentation will expand to cover:

* Language syntax
* Standard library
* CD2-specific types
* Compiler behavior
* Complete examples

---

## License

CaveScript is licensed under the **GNU General Public License v2.0 (GPL-2.0)**.

See the `LICENSE` file for the full license text.
