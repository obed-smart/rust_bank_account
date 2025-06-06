use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::fs;

const FILE_PATH: &str = "data.json";

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    first_name: String,
    second_name: String,
    age: u32,
    balance: f32,
    account_number: String,
}

/// read the data from data.json file
fn load_data() -> Vec<Account> {
    let account_data: Vec<Account> = fs::read_to_string(FILE_PATH)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default();
    if account_data.is_empty() {
        println!("No account found");
    }
    account_data
}

/// save the data vec on the data.json file

fn save_data(account: &Vec<Account>) {
    let data = serde_json::to_string_pretty(&account).expect("failed to serialize the data");

    fs::write(FILE_PATH, data).expect("failed to write data to data.json file");
}

fn add_account(account: Account) {
    let mut accounts = load_data();
    accounts.push(account);

    save_data(&accounts);
}

/// generate an account to the new user
fn generate_account() -> String {
    let mut rng = rand::thread_rng();

    (0..10).map(|_| rng.gen_range(0..10).to_string()).collect()
}

/// custom input field for all input with the message
fn custom_input(display_text: &str) -> String {
    println!("{display_text}");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input.trim().to_string()
}

/// get each user account buy name
// fn get_account_by_account_number(account_number: String) -> Option<Account> {
//     let accounts = load_data();

//     accounts
//         .into_iter()
//         .find(|account| account.account_number == account_number)
// }

impl Account {
    // this is a method on the struct instance

    fn deposit(&mut self, amount: f32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f32) {
        self.balance -= amount;
    }

    fn check_balance(&mut self) {
        println!(" your account balance is: {}", self.balance)
    }

    fn create_account(
        first_name: String,
        second_name: String,
        age: u32,
        balance: f32,
        account_number: String,
    ) -> Self {
        Self {
            first_name,
            second_name,
            age,
            balance,
            account_number,
        }
    }
}



fn update_or_return_balance(nums: u32) {
    let mut accounts = load_data();

    let account_number = custom_input("Enter you account number");

    match accounts
        .iter_mut()
        .find(|account| account.account_number == account_number.trim())
    {
        Some(acc) => {
            if nums != 4 {
                let amount = custom_input("Énter your amount");

                let amount: f32 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input! please enter a number on the age");
                        return;
                    }
                };
                if nums == 2 {
                    acc.deposit(amount);

                    println!("Deposit successful ✅. New balance: {}", acc.balance);
                } else if nums == 3 {
                    acc.withdraw(amount);

                    println!("Withdrawal successful ✅. New balance: {}", acc.balance);
                }
            }

            if nums == 4 {
                acc.check_balance();
            }
        }

        None => println!("No account found ❌"),
    }

    save_data(&accounts);
}

fn check_name(first_name: &str, second_name: &str) -> String {
    format!("{} {}", first_name, second_name)
}

fn main() {
    let action_btn = vec![
        "create account",
        "Deposit",
        "Withdraw",
        "Check balance",
        "cancle",
    ];

    for (index, value) in action_btn.iter().enumerate() {
        println!("{}: {}", index + 1, value);
    }
    let selected_input: usize = 'get_input: loop {
        let input = custom_input("Choose one of the following...");

        match input.trim().parse() {
            Ok(number) => {
                if number <= action_btn.len() {
                    println!("{number}");
                    break 'get_input number;
                } else {
                    println!(
                        "Input number is out of rang! please enter number in rang of 1 to {}",
                        action_btn.len() - 1
                    );
                    continue;
                }
            }

            Err(_) => {
                println!("invalid input! please select a valid input");

                continue;
            }
        };
    };

    match selected_input {
        1 => {
            let accounts = load_data();
            let first_name = custom_input("Enter your first name below.");

            let second_name = custom_input("Enter your second name below");

            let age = custom_input("Enter your age below");

            let age: u32 = match age.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("invalid input! please enter a number on the age");

                    return;
                }
            };

            let user_account = accounts.iter().find(|acc| {
                check_name(&acc.first_name, &acc.second_name)
                    == check_name(&first_name, &second_name)
            });

            match user_account {
                Some(account) => {
                    println!(
                        "A user with this name {} already exist",
                        check_name(&account.first_name, &account.second_name)
                    );

                    return;
                }
                None => {
                    let new_account = Account::create_account(
                        first_name,
                        second_name,
                        age,
                        0.0,
                        generate_account(),
                    );
                    // let check_account = &new_account;

                    add_account(new_account);
                    println!("New account created ✅");
                }
            }
        }
        2 => update_or_return_balance(2),

        3 => update_or_return_balance(3),

        4 => update_or_return_balance(4),
        5 => return,
        _ => println!("error"),
    }

    // let mut
}
