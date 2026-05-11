# Interactive Bill Manager

Interactive Bill Manager is a simple Rust command-line application for tracking bills in a terminal-friendly workflow. 

## Overview

The app lets you create, view, edit, and remove bill entries by name and amount. It is intentionally lightweight, but it shows the core patterns you would expect in a real CLI utility:

- clear data modeling with structs
- encapsulated business logic in a manager type
- reusable input helpers
- simple terminal UI with a looped menu

## Features

- Add a bill with a name and amount
- View all saved bills
- Edit an existing bill amount
- Remove a bill by name
- Handle invalid numeric input gracefully
- Keep bill data in memory using `HashMap` for fast lookups

## Tech Stack

- Rust 2024 edition
- Standard library only
- `std::collections::HashMap` for storage
- `std::io` for terminal input and output

## Project Structure

```text
interactive-bill-manager/
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src/
    └── main.rs
```

## Getting Started

### Prerequisites

- Rust installed via [rustup](https://rustup.rs/)
- Cargo available in your terminal

### Clone the repository

```bash
git clone https://github.com/freddyjaoko/interactive-bill-manager.git
cd interactive-bill-manager
```

### Run the application

```bash
cargo run
```

## How to Use

When the app starts, you will see a menu with the following actions:

1. Add a new bill
2. View all bills
3. Remove a bill
4. Edit a bill
5. Exit the program

### Example Flow

```text
--- Bill Manager ---
1. Add, 2. View, 3. Remove, 4. Edit, 5. Exit
Action: 1
Enter bill name:
Internet
Enter amount:
75.50
Bill added!
```

```text
--- Bill Manager ---
1. Add, 2. View, 3. Remove, 4. Edit, 5. Exit
Action: 2
Bill: Internet | Amount: $75.5
```

## What This Project Demonstrates

This project is small, but it shows several useful software engineering habits:

- Separating data, logic, and input handling into focused functions
- Using a `HashMap` to make lookup, update, and deletion straightforward
- Writing defensive input parsing for a better terminal user experience
- Building a complete feature loop from setup to interaction to exit

## Possible Improvements

If I expand this project, the next steps would be:

- persist bills to a file so they survive restarts
- add categories such as rent, utilities, or subscriptions
- calculate total monthly spending
- support sorting and search
- add automated tests for bill operations and input parsing

## Portfolio Note

This project is designed to show practical Rust fundamentals in a real CLI workflow. It is a good example of how I organize simple application logic, keep the code readable, and build interactive tools that are easy to extend.

## License

No license has been added yet. If you want to publish or share this project more broadly, add a license that matches your intended use.