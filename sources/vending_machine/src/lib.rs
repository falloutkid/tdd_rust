//! A simple vending machine implementation.

use anyhow::{Result, anyhow};

use std::collections::HashMap;

/// Represents a vending machine that dispenses drinks and handles money deposits.
pub struct VendingMachine {
    deposits: HashMap<Money, u32>,
    buttons: Buttons,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum ButtonLight {
    On,
    Off,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub struct Buttons {
    coke: ButtonLight,
    oolong_tea: ButtonLight,
    redbull: ButtonLight,
}

impl Buttons {
    pub fn new() -> Self {
        Self {
            coke: ButtonLight::Off,
            oolong_tea: ButtonLight::Off,
            redbull: ButtonLight::Off,
        }
    }
}

impl Default for Buttons {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Drink {
    Coke,
    OolongTea,
    Redbull,
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
impl Default for VendingMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl VendingMachine {
    /// Creates a new `VendingMachine` instance with no initial deposits.
    pub fn new() -> Self {
        Self {
            deposits: HashMap::new(),
            buttons: Buttons::new(),
        }
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
            self.update_button_status();
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
    fn get_cola(&mut self) -> Result<String> {
        let is_have_one_hundred = if let Some(one_hundred) = self.deposits.get(&Money::OneHundred) {
            *one_hundred > 0
        } else {
            false
        };
        if !is_have_one_hundred {
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
    fn get_oolong_tea(&mut self) -> Result<String> {
        let is_have_one_hundred = if let Some(one_hundred) = self.deposits.get(&Money::OneHundred) {
            *one_hundred > 0
        } else {
            false
        };
        if !is_have_one_hundred {
            Err(anyhow!("Could not get Oolong Tea"))
        } else {
            self.deposits
                .entry(Money::OneHundred)
                .and_modify(|v| *v -= 1);
            Ok("Oolong Tea".to_string())
        }
    }

    /// Attempts to get a bottle of Redbull from the vending machine.
    /// Requires at least two `Money::OneHundred` deposit.
    ///
    /// # Returns
    ///
    /// Returns `Ok("Redbull")` if Redbull is successfully dispensed, or an `Err` with an `anyhow` error
    /// if there is insufficient money.
    fn get_redbull(&mut self) -> Result<String> {
        let is_have_one_hundred = if let Some(one_hundred) = self.deposits.get(&Money::OneHundred) {
            *one_hundred >= 2
        } else {
            false
        };
        if !is_have_one_hundred {
            Err(anyhow!("Could not get Redbull"))
        } else {
            self.deposits
                .entry(Money::OneHundred)
                .and_modify(|v| *v -= 2);
            Ok("Redbull".to_string())
        }
    }

    pub fn press_button(&mut self, drink: Drink) -> Result<String> {
        let result = match drink {
            Drink::Coke => self.get_cola(),
            Drink::OolongTea => self.get_oolong_tea(),
            Drink::Redbull => self.get_redbull(),
        };
        self.update_button_status();
        result
    }

    pub fn get_button_status(&self) -> Buttons {
        self.buttons
    }


    fn update_button_status(&mut self) {
        let status = if let Some(one_hundred) = self.deposits.get(&Money::OneHundred) {
            Buttons {
                coke: if *one_hundred > 0 {
                    ButtonLight::On
                } else {
                    ButtonLight::Off
                },
                oolong_tea: if *one_hundred > 0 {
                    ButtonLight::On
                } else {
                    ButtonLight::Off
                },
                redbull: if *one_hundred >= 2 {
                    ButtonLight::On
                } else {
                    ButtonLight::Off
                },
            }
        } else {
            Buttons::new()
        };
        self.buttons = status;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let dispensed_item = vm.press_button(Drink::Coke);
        assert!(dispensed_item.is_ok());
        let message = dispensed_item.unwrap();
        assert_eq!(message, "Coke".to_string());
    }

    /// Test that cola cannot be gotten if there is no money deposited.
    #[test]
    fn should_not_dispense_cola_when_insufficient_funds() {
        let mut vm = VendingMachine::new();
        let dispensed_item = vm.press_button(Drink::Coke);
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
        let dispensed_item = vm.press_button(Drink::OolongTea);
        assert!(dispensed_item.is_ok());
        let message = dispensed_item.unwrap();
        assert_eq!(message, "Oolong Tea".to_string());
    }

    /// Test that getting redbull, oolong tea and cola updates the deposits correctly.
    /// This also tests that if there is no money left, an error is returned.
    #[test]
    fn should_dispense_multiple_items_and_track_funds_correctly() {
        let mut vm = VendingMachine::new();
        vm.deposit(Money::OneHundred).unwrap();
        vm.deposit(Money::OneHundred).unwrap();
        vm.deposit(Money::OneHundred).unwrap();
        vm.deposit(Money::OneHundred).unwrap();

        let dispensed_item = vm.press_button(Drink::OolongTea);
        assert!(dispensed_item.is_ok());
        let message = dispensed_item.unwrap();
        assert_eq!(message, "Oolong Tea".to_string());

        let dispensed_item = vm.press_button(Drink::Coke);
        assert!(dispensed_item.is_ok());
        let message = dispensed_item.unwrap();
        assert_eq!(message, "Coke".to_string());

        let dispensed_item = vm.press_button(Drink::Redbull);
        assert!(dispensed_item.is_ok());
        let message = dispensed_item.unwrap();
        assert_eq!(message, "Redbull".to_string());

        let dispensed_item = vm.press_button(Drink::Redbull);
        assert!(dispensed_item.is_err());
        assert_eq!(
            dispensed_item.err().unwrap().to_string(),
            "Could not get Redbull".to_string()
        );

        let dispensed_item = vm.press_button(Drink::OolongTea);
        assert!(dispensed_item.is_err());
        assert_eq!(
            dispensed_item.err().unwrap().to_string(),
            "Could not get Oolong Tea".to_string()
        );
    }

    /// Test that getting Redbull updates the deposits and dispenses Redbull.
    #[test]
    fn should_dispense_redbull_when_sufficient_funds() {
        let mut vm = VendingMachine::new();
        vm.deposit(Money::OneHundred).unwrap();
        vm.deposit(Money::OneHundred).unwrap();

        let dispensed_item = vm.press_button(Drink::Redbull);
        assert!(dispensed_item.is_ok());
        let message = dispensed_item.unwrap();
        assert_eq!(message, "Redbull".to_string());
    }

    /// Test button status after initialization.
    #[test]
    fn should_all_off_when_initialized() {
        let vm = VendingMachine::new();
        let expected = Buttons {
            coke: ButtonLight::Off,
            oolong_tea: ButtonLight::Off,
            redbull: ButtonLight::Off,
        };
        assert_eq!(vm.get_button_status(), expected);
    }

    /// Test button status after add coins.
    #[test]
    fn should_update_button_status_when_depositing() {
        let mut vm = VendingMachine::new();
        vm.deposit(Money::OneHundred).unwrap();
        let expected = Buttons {
            coke: ButtonLight::On,
            oolong_tea: ButtonLight::On,
            redbull: ButtonLight::Off,
        };
        assert_eq!(vm.get_button_status(), expected);

        vm.deposit(Money::OneHundred).unwrap();
        let expected = Buttons {
            coke: ButtonLight::On,
            oolong_tea: ButtonLight::On,
            redbull: ButtonLight::On,
        };
        assert_eq!(vm.get_button_status(), expected);

        let _ = vm.press_button(Drink::Coke);
        let expected = Buttons {
            coke: ButtonLight::On,
            oolong_tea: ButtonLight::On,
            redbull: ButtonLight::Off,
        };
        assert_eq!(vm.get_button_status(), expected);

        let _ = vm.press_button(Drink::OolongTea);
        let expected = Buttons {
            coke: ButtonLight::Off,
            oolong_tea: ButtonLight::Off,
            redbull: ButtonLight::Off,
        };
        assert_eq!(vm.get_button_status(), expected);

        vm.deposit(Money::OneHundred).unwrap();
        vm.deposit(Money::OneHundred).unwrap();
        let _ = vm.press_button(Drink::Redbull);
        let expected = Buttons {
            coke: ButtonLight::Off,
            oolong_tea: ButtonLight::Off,
            redbull: ButtonLight::Off,
        };
        assert_eq!(vm.get_button_status(), expected);
    }
}
