# Purpose
This repository was created for learning the Rust language and develop web apps personally.

# Learning Blog

## Day 1
- Install and configure the env of Rust.
- Learn some commands.
- Create first web app

Cargo: Package installer like a pip.

#### Commands
`cargo init`: initialize directory for the project.

`cargo add`: add dependencies to Cargo.toml, this is different to "cargo install".
This gonna just **add** the line to the Cargo.toml.

`cargo install`: install dependencies **directly**

## Day 2
- Search about Query.
- Create simple system
    - parse users message from query param
    - display them via response

#### breakthrough
Return type of handlers(e.g. root) is **No** needs to be &'static str all the time.**
It can be **String**.

#### Snippets
- `Query<HashMap<String, String>>`
- `Some(message_value)`
- `message_value.to_string()`
- `query.get("message")`
    - get value with key from HashMap
