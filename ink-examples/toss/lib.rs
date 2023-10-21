#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::toss::TossRef;

#[ink::contract]
mod toss {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Toss {
        value: bool,
        count: Mapping<AccountId, u32>,
    }

    impl Toss {
        #[ink(constructor)]
        pub fn new() -> Self {
            let count = Mapping::default();
            Self { value: false, count}
        }

        #[ink(message)]
        pub fn toss(&mut self) {
            self.value = !self.value;
            let caller = self.env().caller();
            let _count = self.count.get(caller).unwrap_or(0);
            self.count.insert(caller, &(_count + 1));
        }
        
        #[ink(message)]
        pub fn get_count(&self) -> u32 {
            let caller = self.env().caller();
            let _count = self.count.get(caller).unwrap_or(0);
            return _count;
        }
        
    }
}
