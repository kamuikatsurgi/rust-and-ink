#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod toss_caller {

    use toss::TossRef;

    #[ink(storage)]
    pub struct TossCaller {
        toss: TossRef,
    }

    impl TossCaller {
        #[ink(constructor)]
        pub fn new(toss_contract_code_hash: Hash) -> Self {
            let toss = TossRef::new(true)
                .code_hash(toss_contract_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            Self { toss }
        }

        #[ink(message)]
        pub fn toss_and_get(&mut self) -> u32 {
            self.toss.toss();
            self.toss.get_count()
        }
    }
}
