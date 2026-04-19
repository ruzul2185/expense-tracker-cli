
# Expense Tracker CLI

A simple and efficient command-line expense tracker built with Rust. This tool allows you to manage your daily expenses directly from the terminal with support for adding, updating, deleting, listing, and summarizing expenses.

---

## 🚀 Features

* Add new expenses with description and amount
* Update existing expenses
* Delete expenses by ID
* List all recorded expenses
* View total expense summary (overall or by month)
* Persistent storage using JSON
* Input validation and error handling

---

## 📌 Project Inspiration

This project is based on the **Expense Tracker CLI challenge** from roadmap.sh, which is designed to help developers practice building CLI applications with file handling and basic CRUD operations.

👉 https://roadmap.sh/projects/expense-tracker

The challenge includes features like adding, updating, deleting, listing, and summarizing expenses, making it a great beginner-friendly project for improving logic and CLI skills ([roadmap.sh][1]).

---

## 📦 Installation

### Prerequisites

* Rust (latest stable version recommended)

Install Rust from: https://www.rust-lang.org/

### Clone the Repository

```bash
git clone https://github.com/ruzul2185/expense-tracker-cli
cd expense-tracker-cli
```

### Build the Project

```bash
cargo build --release
```

The executable will be located at:

```
target/release/expense-tracker
```

---

## 🛠️ Usage

### Add an Expense

```bash
expense-tracker add --description "Lunch" --amount 12.50
```

---

### Update an Expense

```bash
expense-tracker update --id 1 --description "Dinner" --amount 20.00
```

You can update either description, amount, or both.

---

### Delete an Expense

```bash
expense-tracker delete --id 1
```

---

### List All Expenses

```bash
expense-tracker list
```

---

### View Summary

#### Total Summary

```bash
expense-tracker summary
```

#### Monthly Summary

```bash
expense-tracker summary --month 4
```

---

## 📁 Data Storage

* Expenses are stored in a local file:

  ```
  expense.json
  ```
* Data is saved safely using a temporary file to prevent corruption.

---

## 🧱 Project Structure

* `main.rs` — CLI logic and command handling
* `Expense` — Data model with validation
* `ExpenseTracker` — Core logic (CRUD + summary)
* JSON file — Persistent storage

---

## ⚠️ Validations

* Description cannot be empty
* Amount must be a valid non-negative number
* ID must exist for update/delete operations

---

## 🔗 Repository

GitHub: https://github.com/ruzul2185/expense-tracker-cli

---

## 🧑‍💻 Author

Developed as a simple Rust CLI project to practice file handling, struct design, and CLI parsing.

---

## 📜 License

This project is open-source and available under the MIT License.

[1]: https://roadmap.sh/projects/expense-tracker?utm_source=chatgpt.com "Expense Tracker"
