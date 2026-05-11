use std::collections::HashMap; // Used to store bills by name
use std::io::{self, Write};    // Used for terminal input and output

// --- Data Setup ---

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
}

struct BillManager {
    // A HashMap stores bills like a dictionary: Key = Name, Value = Bill Data
    bills: HashMap<String, Bill>,
}

// --- Bill Logic ---

impl BillManager {
    // Initialize an empty manager
    fn new() -> Self {
        Self {
            bills: HashMap::new(),
        }
    }

    // Add or update a bill in the list
    fn add(&mut self, bill: Bill) {
        self.bills.insert(bill.name.clone(), bill);
    }

    // Print all bills stored in the list
    fn view_all(&self) {
        if self.bills.is_empty() {
            println!("No bills found.");
            return;
        }
        for bill in self.bills.values() {
            println!("Bill: {} | Amount: ${}", bill.name, bill.amount);
        }
    }

    // Remove a bill by its name. Returns true if it was found and deleted.
    fn remove(&mut self, name: &str) -> bool {
        self.bills.remove(name).is_some()
    }

    // Find an existing bill and change its amount
    fn edit(&mut self, name: &str, amount: f64) -> bool {
        if let Some(bill) = self.bills.get_mut(name) {
            bill.amount = amount;
            return true;
        }
        false
    }
}

// --- Getting User Input ---

// Helper function to read a line from the terminal
fn get_input() -> Option<String> {
    let mut buffer = String::new();
    if io::stdin().read_line(&mut buffer).is_err() {
        return None;
    }
    let input = buffer.trim().to_string();
    if input.is_empty() {
        None // Returns None if the user just hits Enter
    } else {
        Some(input)
    }
}

// Helper function to get a valid number from the user
fn get_amount() -> Option<f64> {
    loop {
        println!("Enter amount:");
        let input = get_input()?; // If input is empty, go back
        match input.parse::<f64>() {
            Ok(num) => return Some(num),
            Err(_) => println!("Invalid number, please try again."),
        }
    }
}

// --- Menu Functions ---

fn add_menu(manager: &mut BillManager) {
    println!("Enter bill name:");
    let name = match get_input() {
        Some(n) => n,
        None => return, // Go back to main menu
    };

    if let Some(amount) = get_amount() {
        manager.add(Bill { name, amount });
        println!("Bill added!");
    }
}

fn remove_menu(manager: &mut BillManager) {
    println!("Enter name to remove:");
    if let Some(name) = get_input() {
        if manager.remove(&name) {
            println!("Bill removed!");
        } else {
            println!("Bill not found.");
        }
    }
}

fn edit_menu(manager: &mut BillManager) {
    println!("Enter name to edit:");
    let name = match get_input() {
        Some(n) => n,
        None => return,
    };

    if let Some(amount) = get_amount() {
        if manager.edit(&name, amount) {
            println!("Bill updated!");
        } else {
            println!("Bill not found.");
        }
    }
}

// --- Main Program ---

fn main() {
    let mut manager = BillManager::new();

    loop {
        println!("\n--- Bill Manager ---");
        println!("1. Add, 2. View, 3. Remove, 4. Edit, 5. Exit");
        print!("Action: ");
        let _ = io::stdout().flush(); // Make sure the text appears immediately

        match get_input().as_deref() {
            Some("1") => add_menu(&mut manager),
            Some("2") => manager.view_all(),
            Some("3") => remove_menu(&mut manager),
            Some("4") => edit_menu(&mut manager),
            Some("5") => break,
            _ => println!("Invalid choice, try again."),
        }
    }
    println!("Goodbye!");
}