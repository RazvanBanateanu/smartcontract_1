#![no_std]

multiversx_sc::imports!();

pub const MAX_PERCENTAGE: u64 = 10_000;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]
pub trait Mycounter {
    #[view(getPercentage)]
    #[storage_mapper("percentage")]
    fn percentage(&self) -> SingleValueMapper<u32>;

    #[view(getAddr)]
    #[storage_mapper("client_address")]
    fn client_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getFees)]
    #[storage_mapper("fees")]
    fn fees(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, percentage: u32, client_address: ManagedAddress, fees:BigUint) {
        self.percentage().set(percentage);
        self.client_address().set(client_address);
        self.fees().set(fees);
    }

    #[payable("EGLD")]
    #[endpoint]
    fn transfer(&self) {
        let payment_amount = self.call_value().egld_value();
        require!(payment_amount > 0, "Must pay more than 0");

        let percentage = self.percentage().get();
        let client_address = self.client_address().get();


        let new_payment = &payment_amount * percentage/MAX_PERCENTAGE;

        self.fees().update(|fees| *fees += payment_amount - &new_payment);

    
        self.send().direct_egld(&client_address, &new_payment);
    }


    #[endpoint]
    fn claim_fees(&self) {

        let caller = self.blockchain().get_caller();
        require!(
            caller == self.blockchain().get_owner_address(),
            "only owner can claim successful funding"
        );

        let current_nonce = self.blockchain().get_block_nonce();
        let sc_balance = self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), current_nonce);
        // let sc_balance = self.fees().get();
        self.send().direct_egld(&caller, &sc_balance);
    }

}
