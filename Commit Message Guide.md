# Commit Message Guide

Write commit messages so someone can understand the change without reading the diff.

## Format

```text
[area]: [verb] [what changed]
```

For example:

```text
lexer: Implement identifier lexing
parser: Add expression parsing
ast: Refactor node structure
docs: Update language specification
tests: Add lexer error cases
```

The `[area]` prefix is optional when it isn't useful.

## Use a clear verb

Prefer:

- `Add` — introduce something new
- `Implement` — add functionality
- `Fix` — correct broken behavior
- `Refactor` — restructure without changing behavior
- `Remove` — delete something
- `Update` — modify existing behavior or documentation
- `Document` — add documentation
- `Test` — add or modify tests

Use the **imperative form**:

```text
Implement identifier lexing
```

rather than:

```text
Implemented identifier lexing
```

## Keep it focused

One commit should represent one **logical change**.

Prefer:

```text
lexer: Implement token iteration
lexer: Improve error handling
```

over:

```text
lexer: Implement token iteration and improve errors and reorganize modules
```

## Quick check

Before committing, ask:

> **"Would I understand what this commit did six months from now?"**

If yes, commit it.