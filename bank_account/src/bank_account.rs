#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount >= 0.00 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount >= 0.00 && self.balance >= amount {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn apply_interest(&mut self, rate: f64) {
        if self.balance > 0.00 && rate >= 0.00 {
            self.balance *= 1.00 + rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let new_account = BankAccount::new(15.00);

        assert_eq!(new_account.balance(), 15.00);
    }

    #[test]
    fn test_deposit() {
        let mut new_account = BankAccount::new(10.00);

        new_account.deposit(5.00);
        assert_eq!(new_account.balance(), 15.00);
        new_account.deposit(-5.00);
        assert_eq!(new_account.balance(), 15.00);
    }

    #[test]
    fn test_withdraw() {
        let mut new_account = BankAccount::new(10.00);

        new_account.withdraw(5.00);
        assert_eq!(new_account.balance(), 5.00);
        new_account.withdraw(-5.00);
        assert_eq!(new_account.balance(), 5.00);
        new_account.withdraw(-10.00);
        assert_eq!(new_account.balance(), 5.00);
    }

    // Add more tests here
    #[test]
    fn test_apply_interest_rate() {
        let mut new_account = BankAccount::new(10.00);

        new_account.apply_interest(0.1);
        assert_eq!(new_account.balance(), 11.00);
        new_account.apply_interest(-1.0);
        assert_eq!(new_account.balance(), 11.00);
        new_account.withdraw(-11.00);
        new_account.apply_interest(1.00);
        assert_eq!(new_account.balance(), 0.00);
    }
}