//! A simple vending machine implementation.

use anyhow::{Result, anyhow};

use std::collections::HashMap;

/// Represents a vending machine that dispenses drinks and handles money deposits.
pub struct VendingMachine {
    deposits: HashMap<Money, u32>,
}

/// Represents the different denominations of money that can be used in the vending machine.
#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Money {
    One,
    Five,
    Ten,
    Fifty,
    OneHundred,
    FiveHundred,
    OneThousand,
    TenThousand,
}

impl VendingMachine {
    /// Creates a new `VendingMachine` instance with no initial deposits.
    pub fn new() -> Self {
        Self {
            deposits: HashMap::new(),
        }
    }

    /// Simulates pressing a button on the vending machine.
    /// Currently, it always dispenses "Coke".
    pub fn press_button(&self) -> String {
        "Coke".to_string()
    }

    /// Deposits a `Money` denomination into the vending machine.
    /// Only `Money::OneHundred` can be deposited successfully.
    ///
    /// # Arguments
    ///
    /// * `money` - The `Money` denomination to deposit.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the deposit is successful, or an `Err` with an `anyhow` error
    /// if the money cannot be deposited (e.g., not `Money::OneHundred`).
    pub fn deposit(&mut self, money: Money) -> Result<()> {
        if money != Money::OneHundred {
            Err(anyhow!("Could not deposit"))
        } else {
            self.deposits
                .entry(money)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            Ok(())
        }
    }

    /// Attempts to get a can of Cola from the vending machine.
    /// Requires at least one `Money::OneHundred` deposit.
    ///
    /// # Returns
    ///
    /// Returns `Ok("Coke")` if a Cola is successfully dispensed, or an `Err` with an `anyhow` error
    /// if there is insufficient money.
    pub fn get_cola(&mut self) -> Result<String> {
        let is_have_one_hundred = if self.deposits.contains_key(&Money::OneHundred) {
            let one_hundred = self.deposits.get(&Money::OneHundred).unwrap();
            *one_hundred > 0
        } else {
            false
        };
        if is_have_one_hundred == false {
            Err(anyhow!("Could not get cola"))
        } else {
            self.deposits
                .entry(Money::OneHundred)
                .and_modify(|v| *v -= 1);
            Ok("Coke".to_string())
        }
    }

    /// Attempts to get a bottle of Oolong Tea from the vending machine.
    /// Requires at least one `Money::OneHundred` deposit.
    ///
    /// # Returns
    ///
    /// Returns `Ok("Oolong Tea")` if Oolong Tea is successfully dispensed, or an `Err` with an `anyhow` error
    /// if there is insufficient money.
    pub fn get_oolong_tea(&mut self) -> Result<String> {
        let is_have_one_hundred = if self.deposits.contains_key(&Money::OneHundred) {
            let one_hundred = self.deposits.get(&Money::OneHundred).unwrap();
            *one_hundred > 0
        } else {
            false
        };
        if is_have_one_hundred == false {
            Err(anyhow!("Could not get Oolong Tea"))
        } else {
            self.deposits
                .entry(Money::OneHundred)
                .and_modify(|v| *v -= 1);
            Ok("Oolong Tea".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that pressing the button dispenses Coke.
    #[test]
    fn should_dispense_coke_when_button_pressed() {
        let vm = VendingMachine::new();
        let dispensed_item = vm.press_button();
        assert_eq!(dispensed_item, "Coke");
    }

    /// Test that depositing money updates the deposits.
    /// Only Money::OneHundred should be successfully deposited.
    #[test]
    fn should_accept_one_hundred_and_reject_other_denominations_when_depositing_money() {
        let mut vm = VendingMachine::new();
        assert!(vm.deposit(Money::One).is_err());
        assert!(vm.deposit(Money::OneHundred).is_ok());
    }

    /// Test that getting cola updates the deposits and dispenses Coke.
    #[test]
    fn should_dispense_cola_when_sufficient_funds() {
        let mut vm = VendingMachine::new();
        vm.deposit(Money::OneHundred).unwrap();
        let dispensed_item = vm.get_cola();
        assert!(dispensed_item.is_ok());
        let message = dispensed_item.unwrap();
        assert_eq!(message, "Coke".to_string());
    }

    /// Test that cola cannot be gotten if there is no money deposited.
    #[test]
    fn should_not_dispense_cola_when_insufficient_funds() {
        let mut vm = VendingMachine::new();
        let dispensed_item = vm.get_cola();
        assert!(dispensed_item.is_err());
        assert_eq!(
            dispensed_item.err().unwrap().to_string(),
            "Could not get cola".to_string()
        );
    }

    /// Test that getting oolong tea updates the deposits and dispenses Oolong Tea.
    #[test]
    fn should_dispense_oolong_tea_when_sufficient_funds() {
        let mut vm = VendingMachine::new();
        vm.deposit(Money::OneHundred).unwrap();
        let dispensed_item = vm.get_oolong_tea();
        assert!(dispensed_item.is_ok());
        let message = dispensed_item.unwrap();
        assert_eq!(message, "Oolong Tea".to_string());
    }

    /// Test that getting oolong tea and cola updates the deposits correctly.
    /// This also tests that if there is no money left, an error is returned.
    #[test]
    fn should_dispense_multiple_items_and_track_funds_correctly() {
        let mut vm = VendingMachine::new();
        vm.deposit(Money::OneHundred).unwrap();
        vm.deposit(Money::OneHundred).unwrap();

        let dispensed_item = vm.get_oolong_tea();
        assert!(dispensed_item.is_ok());
        let message = dispensed_item.unwrap();
        assert_eq!(message, "Oolong Tea".to_string());

        let dispensed_item = vm.get_cola();
        assert!(dispensed_item.is_ok());
        let message = dispensed_item.unwrap();
        assert_eq!(message, "Coke".to_string());

        let dispensed_item = vm.get_oolong_tea();
        assert!(dispensed_item.is_err());
        assert_eq!(
            dispensed_item.err().unwrap().to_string(),
            "Could not get Oolong Tea".to_string()
        );
    }
}
