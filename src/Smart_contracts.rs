use ink_lang as ink;

#[ink::contract]
mod greeter {
    #[ink(storage)]
    pub struct Greeter {
        owner: AccountId,
        greeting: String,
    }

    impl Greeter {
        #[ink(constructor)]
        pub fn new(initial_greeting: String) -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                greeting: initial_greeting,
            }
        }

        #[ink(message)]
        pub fn greet(&self) -> String {
            format!("{} says: {}", self.owner, self.greeting)
        }

        #[ink(message)]
        pub fn set_greeting(&mut self, new_greeting: String) {
            let caller = self.env().caller();
            assert_eq!(caller, self.owner, "Only owner can set the greeting");
            self.greeting = new_greeting;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_greet() {
            let mut greeter = Greeter::new(String::from("Hello"));
            assert_eq!(greeter.greet(), "Alice says: Hello");

            // Change the greeting
            greeter.set_greeting(String::from("Hi there"));
            assert_eq!(greeter.greet(), "Alice says: Hi there");
        }
    }
}

fn main() {
    // Deploy the contract and interact with it here
    println!("Smart contract executed");
}