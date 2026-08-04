# CaveScript Development Roadmap

> *"Make it work, make it nice, then make it complete."*

This roadmap is intended to keep the project focused on one problem at a time. The goal is **not** to perfectly design the language up front, but to iteratively discover the right abstractions while maintaining a working compiler.

---

# Guiding Principles

## Keep Layers Independent

The project consists of three largely independent systems:

```
CaveScript Language
        │
        ▼
 Compiler / Semantic Analysis
        │
        ▼
     CD2 Backend
```

Each layer should know as little as possible about the others.

* The language should not know about JSON.
* The compiler should not know specific CD2 modules.
* The backend should only serialize semantic concepts into CD2.

---

## Design Through Examples

Before implementing a feature:

1. Write several CaveScript examples.
2. Compare different APIs.
3. Choose the most natural one.
4. Only then implement it.

Avoid designing features that have never been used in an example.

---

## Expect Change

The first implementation of almost every subsystem will be replaced.

This is normal.

Refactoring is part of discovering the language.

---

# Phase 1 — Core Language

**Goal:** Build a language that is pleasant to write.

## Lexer

* [ ] Tokens
* [ ] Comments
* [ ] Literals
* [ ] Operators

---

## Parser

* [ ] Expressions
* [ ] Variables
* [ ] Functions
* [ ] Structs
* [ ] Enums
* [ ] Arrays
* [ ] Methods
* [ ] Blocks

---

## AST

* [ ] Stable AST nodes
* [ ] Source spans
* [ ] Visitor utilities

---

## Semantic Analysis

* [ ] Name resolution
* [ ] Type checking
* [ ] Mutability checking
* [ ] Scope resolution
* [ ] Method lookup
* [ ] Function resolution

---

# Phase 2 — Intermediate Representation

**Goal:** Represent program meaning instead of syntax.

## High-Level IR (HIR)

* [ ] Typed expressions
* [ ] Statements
* [ ] Variables
* [ ] Function calls
* [ ] Struct construction
* [ ] Field access

HIR should still resemble CaveScript.

---

## Lowering

* [ ] Constant folding
* [ ] Function inlining
* [ ] Dead code elimination
* [ ] Type lowering

---

## CD2 IR

* [ ] Modules
* [ ] Fields
* [ ] Expressions
* [ ] Mutators
* [ ] Runtime variables

This IR should resemble CD2 without being tied to JSON.

---

# Phase 3 — Backend

**Goal:** Produce valid CD2 output.

## Serializer

* [ ] JSON generation
* [ ] Formatting
* [ ] Validation

---

## Testing

* [ ] Compare generated output with expected CD2
* [ ] Golden file tests
* [ ] Regression tests

---

# Phase 4 — CD2 SDK

**Goal:** Build an ergonomic programming interface.

Do **not** implement every module immediately.

Instead, explore them one at a time.

For every module answer:

* What is the underlying CD2 representation?
* What mental model fits best?
* What CaveScript API feels natural?
* Can this abstraction be reused?

---

## Candidate Modules

* [ ] Description
* [ ] Game Values
* [ ] Caps
* [ ] Enemies
* [ ] Spawn Pools
* [ ] Mission Events
* [ ] Warnings
* [ ] Difficulty Scaling

---

# Phase 5 — Language Features

Only after the compiler is stable.

* [ ] if
* [ ] match
* [ ] while
* [ ] for
* [ ] generics (if needed)
* [ ] traits/interfaces (if needed)
* [ ] slices
* [ ] modules/import system

---

# Phase 6 — Tooling

* [ ] CLI
* [ ] Better diagnostics
* [ ] Formatter
* [ ] LSP
* [ ] Syntax highlighting
* [ ] Documentation generator

---

# Design Notebook

Whenever a major design question appears, write it down before implementing.

```
Question:
    How should Enemies be represented?

Options:
    1.
    2.
    3.

Pros:
    ...

Cons:
    ...

Decision:

Reasoning:
```

Never rely on memory for important design decisions.

---

# Current Priorities

At any moment there should be **one** active task.

Current Focus:

* [ ] Finish parser
* [ ] Finish semantic analysis
* [ ] Design HIR
* [ ] Design CD2 IR
* [ ] Implement serializer
* [ ] Design first CD2 SDK module

Everything else is intentionally postponed.

---

# Long-Term Vision

CaveScript should feel like a real programming language—not a prettier JSON syntax.

Users should think in terms of:

* functions
* types
* objects
* collections
* reusable abstractions

The compiler should translate those concepts into valid CD2 configurations while hiding as much of CD2's complexity as possible.

The measure of success is not exposing every CD2 feature directly, but creating a language that makes Custom Difficulty 2 configuration enjoyable to write and maintain.

**Rock and Stone!**
