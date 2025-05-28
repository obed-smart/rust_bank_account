use rand::Rng;

// #[derive(Debug)]

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
    println!("{}", generate_account())
}
