use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn temp_dir() -> TempDir {
    TempDir::new().unwrap()
}

#[test]
fn test_add_and_list_expenses() {
    let dir = temp_dir();

    // Add Lunch
    Command::cargo_bin("expense-tracker-cli")
        .unwrap()
        .env("EXPENSE_TRACKER_DATA", dir.path())
        .args(["add", "--description", "Lunch", "--amount", "20"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Expense added successfully"));

    // Add Dinner
    Command::cargo_bin("expense-tracker-cli")
        .unwrap()
        .env("EXPENSE_TRACKER_DATA", dir.path())
        .args(["add", "--description", "Dinner", "--amount", "10"])
        .assert()
        .success();

    // List
    Command::cargo_bin("expense-tracker-cli")
        .unwrap()
        .env("EXPENSE_TRACKER_DATA", dir.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Lunch"))
        .stdout(predicate::str::contains("Dinner"))
        .stdout(predicate::str::contains("$20"))
        .stdout(predicate::str::contains("$10"));
}

#[test]
fn test_summary_total() {
    let dir = temp_dir();

    Command::cargo_bin("expense-tracker-cli")
        .unwrap()
        .env("EXPENSE_TRACKER_DATA", dir.path())
        .args(["add", "--description", "Lunch", "--amount", "20"])
        .assert()
        .success();

    Command::cargo_bin("expense-tracker-cli")
        .unwrap()
        .env("EXPENSE_TRACKER_DATA", dir.path())
        .args(["add", "--description", "Dinner", "--amount", "10"])
        .assert()
        .success();

    Command::cargo_bin("expense-tracker-cli")
        .unwrap()
        .env("EXPENSE_TRACKER_DATA", dir.path())
        .arg("summary")
        .assert()
        .success()
        .stdout(predicate::str::contains("Total expenses: $30"));
}

#[test]
fn test_delete_expense() {
    let dir = temp_dir();

    // Add two expenses
    Command::cargo_bin("expense-tracker-cli")
        .unwrap()
        .env("EXPENSE_TRACKER_DATA", dir.path())
        .args(["add", "--description", "Lunch", "--amount", "20"])
        .assert()
        .success();

    Command::cargo_bin("expense-tracker-cli")
        .unwrap()
        .env("EXPENSE_TRACKER_DATA", dir.path())
        .args(["add", "--description", "Dinner", "--amount", "10"])
        .assert()
        .success();

    // Delete ID 2
    Command::cargo_bin("expense-tracker-cli")
        .unwrap()
        .env("EXPENSE_TRACKER_DATA", dir.path())
        .args(["delete", "--id", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Expense deleted successfully"));

    // Check summary
    Command::cargo_bin("expense-tracker-cli")
        .unwrap()
        .env("EXPENSE_TRACKER_DATA", dir.path())
        .arg("summary")
        .assert()
        .success()
        .stdout(predicate::str::contains("Total expenses: $20"));
}

#[test]
fn test_summary_by_month() {
    let dir = temp_dir();

    Command::cargo_bin("expense-tracker-cli")
        .unwrap()
        .env("EXPENSE_TRACKER_DATA", dir.path())
        .args(["add", "--description", "Lunch", "--amount", "20"])
        .assert()
        .success();

    Command::cargo_bin("expense-tracker-cli")
        .unwrap()
        .env("EXPENSE_TRACKER_DATA", dir.path())
        .arg("summary")
        .arg("--month")
        .arg("8")
        .assert()
        .success()
        .stdout(predicate::str::contains("Total expenses for August: $20"));
}
