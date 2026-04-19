use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use chrono::{Datelike, NaiveDate, Local};
use std::fs;

const FILE_PATH: &str = "expense.json";

#[derive(Parser)]
#[command(name = "expense-tracker")]
#[command(about = "Simple CLI expense tracker")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(long)]
        description: String,

        #[arg(long)]
        amount: f64,
    },
    Update {
        #[arg(long)]
        id: u32,

        #[arg(long)]
        description: Option<String>,

        #[arg(long)]
        amount: Option<f64>,
    },
    Delete {
        #[arg(long)]
        id: u32,
    },
    List,
    Summary {
        #[arg(long, value_parser = clap::value_parser!(u8).range(1..=12))]
        month: Option<u8>,
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Expense {
    id: u32,
    description: String,
    amount: f64,
    date: NaiveDate,
}

impl Expense {
    fn new(id: u32, description: String, amount: f64, date: NaiveDate) -> Result<Self, String> {
        if description.trim().is_empty() {
            return Err("Description cannot be empty".into());
        }

        if !amount.is_finite() {
            return Err("Amount must be a valid number".into());
        }

        if amount < 0.0 {
            return Err("Amount cannot be negative".into());
        }

        Ok(Self {
            id,
            description,
            amount,
            date,
        })
    }
}

struct ExpenseTracker {
    expenses: Vec<Expense>,
    next_id: u32,
}

impl ExpenseTracker {
    // At the time of list initialization
    fn new() -> Self {
        Self {
            expenses: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, description: String, amount:f64) -> Result<u32, String> {
        let expense = Expense::new(self.next_id, description, amount, Local::now().date_naive())?;
        let id = expense.id;

        self.expenses.push(expense);
        self.next_id += 1;

        self.save()?;

        Ok(id)
    }

    fn update(&mut self, id: u32, description: Option<String>, amount: Option<f64>) -> Result<u32, String> {
        if description.is_none() && amount.is_none() {
            return Err("At least one field must be provided".into());
        }

        let expense = self.expenses.iter_mut().find(|e| e.id == id);

        match expense {
            Some(e) => {
                if let Some(desc) = description {
                    if desc.trim().is_empty() {
                        return Err("Description cannot be empty".into());
                    }
                    e.description = desc;
                }

                if let Some(amt) = amount {
                    if !amt.is_finite() || amt < 0.0 {
                        return Err("Invalid amount".into());
                    }
                    e.amount = amt;
                }

                self.save()?;

                Ok(id)
            }
            None => Err(format!("Expense with id {} not found", id)),
        }
    }

    fn delete(&mut self, id: u32) -> Result<u32, String> {
        if let Some(pos) = self.expenses.iter().position(|e| e.id == id) {
            self.expenses.remove(pos);
            self.save()?;
            Ok(id)
        } else {
            Err(format!("Expense with id {} not found", id))
        }
    }

    fn list(&self) {
        if self.expenses.is_empty() {
            println!("No expenses found.");
            return;
        }

        println!("# ID  Date       Description  Amount");

        for e in &self.expenses {
            println!(
                "# {:<3} {:<10} {:<12} ${}",
                e.id,
                e.date,
                e.description,
                e.amount
            );
        }
    }

    fn summary(&self, month: Option<u8>) {
        if self.expenses.is_empty() {
            println!("# Total expenses: $0");
            return;
        }

        let filtered = self.expenses.iter().filter(|e| {
            if let Some(m) = month {
                e.date.month() == m as u32
            } else {
                true
            }
        });

        let total: f64 = filtered.map(|e| e.amount).sum();

        match month {
            Some(m) => {
                let month_name = match m {
                    1 => "January",
                    2 => "February",
                    3 => "March",
                    4 => "April",
                    5 => "May",
                    6 => "June",
                    7 => "July",
                    8 => "August",
                    9 => "September",
                    10 => "October",
                    11 => "November",
                    12 => "December",
                    _ => "Unknown",
                };

                println!(
                    "# Total expenses for {}: ${:.2}",
                    month_name,
                    total
                );
            }
            None => {
                println!("# Total expenses: ${:.2}", total);
            }
        }
    }

    fn load() -> Self {
        match fs::read_to_string(FILE_PATH) {
            Ok(content) => {
                match serde_json::from_str::<Vec<Expense>>(&content) {
                    Ok(expenses) => {
                        let next_id = expenses.iter()
                            .map(|e| e.id)
                            .max()
                            .unwrap_or(0) + 1;

                        Self { expenses, next_id }
                    }
                    Err(err) => {
                        eprintln!("Warning: Failed to parse data file: {}", err);
                        eprintln!("Starting with empty expense list.");
                        Self::new()
                    }
                }
            }
            Err(_) => {
                // File doesn't exist yet → normal case
                Self::new()
            }
        }
    }

    fn save(&self) -> Result<(), String> {
        // Convert expenses to pretty JSON
        let json = serde_json::to_string_pretty(&self.expenses)
            .map_err(|e| format!("Failed to serialize expenses: {}", e))?;

        // Write to a temporary file first (prevents corruption)
        let temp_path = format!("{}.tmp", FILE_PATH);

        fs::write(&temp_path, json)
            .map_err(|e| format!("Failed to write temp file: {}", e))?;

        // Replace the original file with the temp file
        fs::rename(&temp_path, FILE_PATH)
            .map_err(|e| format!("Failed to replace data file: {}", e))?;

        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();

    let mut tracker = ExpenseTracker::load();

    match cli.command {
        Commands::Add { description, amount } => {
            match tracker.add(description, amount) {
                Ok(id) => println!("Expense added successfully (ID: {})", id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Commands::Update { id, description, amount } => {
            match tracker.update(id, description, amount) {
                Ok(id) => println!("Expense updated successfully (ID: {})", id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Commands::Delete { id } => {
            match tracker.delete(id) {
                Ok(_) => println!("Expense deleted successfully (ID: {})", id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        Commands::List => {
            tracker.list();
        }

        Commands::Summary { month } => {
            tracker.summary(month);
        }
    }
}

