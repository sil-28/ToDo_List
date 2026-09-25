# To-Do List (Rust CLI)

A simple command-line to-do list application written in Rust, built as a learning project.

## Features

- Add, complete, and remove tasks
- View the full list with completion status
- Load tasks from a file at startup (optional, user-chosen filename)
- Automatically save changes to a file so tasks persist between runs
- Input validation: invalid menu choices or indices don't crash the program

## How it works

Tasks are stored as plain text, one per line, in the format:

```
[x] Buy milk
[ ] Call dentist
```

`[x]` marks a completed task, `[ ]` marks a pending one.

## Requirements

- [Rust and Cargo](https://www.rust-lang.org/tools/install)

## Running the project

Clone the repository and run it with Cargo:

```bash
git clone https://github.com/sil-28/ToDo_List.git
cd ToDo_List
cargo run
```

On startup, you'll be asked whether you want to load an existing file. Answer `Y` and provide a filename (without extension) to load a previously saved list, or `N` to start with an empty list saved to `tasks.txt` by default.

## Menu options

```
1. Add task
2. Complete task
3. Remove task
4. Show list
5. Exit
```

## What I learned

This project was built while learning Rust from scratch, focusing on:

- Ownership and borrowing (`&`, `&mut`)
- Structs and enums
- Pattern matching (`match`)
- `Option` and `Result` for error handling
- Reading from and writing to files (`std::fs`, `std::io`)

## Possible improvements

- Load the filename via command-line arguments instead of an interactive prompt
- Switch file storage to JSON using `serde`
- Add due dates or priority levels to tasks