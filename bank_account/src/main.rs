mod bank_account;

fn main() {
    let mut account = bank_account::BankAccount::new(20.00);

    println!("Initial Balance: ${:.2}", account.balance());
    account.withdraw(10.00);
    println!("After Withdraw: ${:.2}", account.balance());
    account.deposit(5.00);
    println!("After Deposit: ${:.2}", account.balance());
    account.apply_interest(0.5);
    println!("After Interest: ${:.2}", account.balance());
}
