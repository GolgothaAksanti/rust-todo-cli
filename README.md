# ✅ CLI To-Do App in Rust

A simple and effective command-line To-Do application built in Rust. Tasks are stored in a local text file (`todo.txt`), making it lightweight and portable. The app allows users to add, list, and remove tasks via an intuitive terminal menu.

---

## ✨ Features

- 📝 Add tasks from the command line
- 📋 View all your tasks
- ❌ Remove tasks by number
- 💾 Tasks are saved in `todo.txt`
- 🚀 Lightweight with zero external dependencies

---

## 📦 Getting Started

### 🔧 Prerequisites

Make sure Rust is installed. If not:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Then clone and run the app:

```bash
git clone https://github.com/GolgothaAksanti/rust-todo-cli.git
cd rust-todo-cli
cargo run

```

## 🧪 Example Usage

```bash
--- To-Do Menu ---
1. List tasks
2. Add task
3. Remove task
4. Quit
Choose an option: 2
Enter a new task: Buy groceries
Task added.

--- To-Do Menu ---
1. List tasks
2. Add task
3. Remove task
4. Quit
Choose an option: 1
1. Buy groceries


```

## 📁 File Structure

```bash
cli-todo/
├── src/
│   └── main.rs      # CLI app logic
├── todo.txt         # Local task storage
├── Cargo.toml       # Rust project config

```

## 💡 How It Works

- Tasks are read from and written to a local `todo.txt` file.

- All interactions happen through the terminal via a numbered menu.

- Uses Rust's standard library: `std::fs`, `std::io`, and `std::path`.

## 📚 What You'll Learn

- File reading and writing in Rust

- Handling user input with `stdin`

- Using `Result`, `match`, and error propagation with `?`

- Terminal interaction and control flow

## 🧰 Future Improvements

- Mark tasks as completed

- Edit existing tasks

- Add due dates or categories

- Support search and filtering

- Use JSON or TOML for structured storage

## 🙌 Acknowledgments

Built as a beginner-friendly project to practice file I/O, error handling, and terminal interfaces in Rust 🦀.
