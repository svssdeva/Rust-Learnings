use std::collections::HashMap;
use std::io;

use menu::{add_bill, remove_bill, update_bill, view_bills};

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}
pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }
    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }
    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }
    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}
fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        };
    }
}

mod menu {
    use crate::{get_bill_amount, get_input, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill Name:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Bill Amount:");
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill added !");
    }

    pub fn remove_bill(bills: &mut Bills) {
        let all_bills = &bills.get_all();
        if all_bills.len() == 0 {
            println!("No bills found, please add a bill");
            return;
        }
        for bill in all_bills {
            println!("{:?}", bill);
        }
        println!("Enter bill name to remove: ");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        if bills.remove(&name) {
            println!("Bill removed");
        } else {
            println!("Bill not found");
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        let all_bills = &bills.get_all();
        if all_bills.len() == 0 {
            println!("No bills found, please add a bill");
            return;
        }
        for bill in all_bills {
            println!("{:?}", bill);
        }
        println!("Enter bill name to update: ");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        println!("Enter amount to update:");
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        if bills.update(&name, amount) {
            println!("Bill updated");
        } else {
            println!("Bill not found");
        }
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::UpdateBill),
            _ => None,
        }
    }
    fn show() {
        println!("");
        println!("==Bill Manager==");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Update Bill");
        println!("");
        println!("Enter Selection: ");
    }
}

fn run_program() -> Option<()> {
    let mut bills = Bills::new();

    loop {
        MainMenu::show();
        let input = get_input()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => add_bill(&mut bills),
            Some(MainMenu::ViewBill) => view_bills(&bills),
            Some(MainMenu::RemoveBill) => remove_bill(&mut bills),
            Some(MainMenu::UpdateBill) => update_bill(&mut bills),
            None => break,
        }
    }
    None
}

fn main() {
    run_program();
}
