use anyhow::{Result, anyhow};

use std::collections::HashMap;

pub struct VendingMachine {
    deposits: HashMap<Money, u32>,
}

#[derive(Eq, Hash, PartialEq)]
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
    pub fn new() -> Self {
        Self {
            deposits: HashMap::new(),
        }
    }

    pub fn press_button(&self) -> String {
        "Coke".to_string()
    }

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
    use std::f32::consts::E;

    use super::*;

    #[test]
    fn pressing_button_dispenses_coke() {
        let vm = VendingMachine::new();
        let dispensed_item = vm.press_button();
        assert_eq!(dispensed_item, "Coke");
    }

    #[test]
    fn depositing_money_updates_deposits() {
        let mut vm = VendingMachine::new();
        assert!(vm.deposit(Money::One).is_err());
        assert!(vm.deposit(Money::OneHundred).is_ok());
    }

    #[test]
    fn getting_cola_updates_deposits() {
        let mut vm = VendingMachine::new();
        vm.deposit(Money::OneHundred).unwrap();
        let dispensed_item = vm.get_cola();
        assert!(dispensed_item.is_ok());
        let message = dispensed_item.unwrap();
        assert_eq!(message, "Coke".to_string());
    }

    #[test]
    fn cannot_getting_cola_updates_deposits() {
        let mut vm = VendingMachine::new();
        let dispensed_item = vm.get_cola();
        assert!(dispensed_item.is_err());
        assert_eq!(
            dispensed_item.err().unwrap().to_string(),
            "Could not get cola".to_string()
        );
    }

    #[test]
    fn getting_oolong_tea_updates_deposits() {
        let mut vm = VendingMachine::new();
        vm.deposit(Money::OneHundred).unwrap();
        let dispensed_item = vm.get_oolong_tea();
        assert!(dispensed_item.is_ok());
        let message = dispensed_item.unwrap();
        assert_eq!(message, "Oolong Tea".to_string());
    }

    #[test]
    fn getting_oolong_tea_and_colaupdates_deposits() {
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
