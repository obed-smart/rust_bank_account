use std::io::{self, Write};

use rand::Rng;

#[derive(Debug)]
struct Account {
    first_name: String,
    second_name: String,
    age: u32,
    balance: f32,
    account_number: String,
}

fn generate_account() -> String {
    let mut rng = rand::thread_rng();

    (0..10).map(|_| rng.gen_range(0..10).to_string()).collect()
}
fn custom_input(display_text: &str) -> String {
    println!("{display_text}");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input.trim().to_string()
}

impl Account {
    // this is a method on the struct instance
    fn deposit(&mut self, amout: f32) {
        println!("Enter amount");
        self.balance += &amout;
        println!("deposit successfull ✅");
    }

    fn check_balance(self) {
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
                } else if number < 1 {
                    println!("you can not go below 1");
                    continue;
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
        1 => println!("deposit"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        5 => println!("cancel"),
        _ =>  println!("erro"),
    }

    // let mut new_account = Account::create_account(first_name, second_name, age, balance, account_number)
}
