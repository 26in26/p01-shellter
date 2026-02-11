# P01 — Shellter

> 26 in 26 · Weeks 01–02 · systems

<p align="center"><img width="450" height="300" alt="worship rust" src="assets/shellter.jpeg" /></p>

## Goal

- Develop a Rust REPL that implements shell behavior with built-in commands.
- Explore the distinctions between shells and terminals, and define the scope and responsibilities of a shell implementation.
- Get proficient with Rust and it's tools

## Scope

**In scope**

- Read user input
- Parse commands + arguments
- Dispatch:
  - Built-in commands (internal implementation)
  - External commands (`std::process::Command`)
- Maintain working directory

**Out of scope**

- Redirection (`>`, `<`)
- Environment variable expansion
- Scripting
- Signals

## Timeline

- **Week 1:** Design, research, POC
- **Week 2:** Implementation, docs

## Status

- [x] Design
- [x] POC
- [x] Core implementation
- [x] Documentation

## 🛠 Tech Stack

- Language: rust
- Constraints: No use of external packeges

## 🚀 Running the Project

```bash
make run
```
