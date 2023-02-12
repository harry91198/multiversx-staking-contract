#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const REWARD_RATE: u64 = 100_000;
pub const DECIMAL_PREC: u64 = 1_000_000_000_000_000_000;

#[multiversx_sc::contract]
pub trait StakingContract {
    #[init]
    fn init(&self) {
        self.updated_at().set(self.blockchain().get_block_timestamp());
    }

    #[payable("EGLD")]
    #[endpoint]
    fn fundrewards(&self) {
        let payment_amount = self.call_value().egld_value();
        require!(payment_amount > 0, "Must pay more than 0");

        self.contract_reward_balance().update(|current_amount| *current_amount += payment_amount);
    }

    #[payable("EGLD")]
    #[endpoint]
    fn stake(&self) {
        let payment_amount = self.call_value().egld_value();
        let staking_amount = self.call_value().egld_value();
        require!(payment_amount > 0, "Must pay more than 0");

        let caller = self.blockchain().get_caller();
        self.update_reward(&caller);
        self.staking_position(&caller)
            .update(|current_amount| *current_amount += payment_amount);
        self.staked_addresses().insert(caller);
        self.total_supply()
            .update(|current_amount| *current_amount += staking_amount);

    }

    #[endpoint]
    fn unstake(&self, opt_unstake_amount: OptionalValue<BigUint>) {
        let caller = self.blockchain().get_caller();
        self.update_reward(&caller);
        let stake_mapper = self.staking_position(&caller);
        let unstake_amount = match opt_unstake_amount {
            OptionalValue::Some(amt) => amt,
            OptionalValue::None => stake_mapper.get(),
        };
    
        let remaining_stake = stake_mapper.update(|staked_amount| {
            require!(
                unstake_amount > 0 && unstake_amount <= *staked_amount,
                "Invalid unstake amount"
            );
            *staked_amount -= &unstake_amount;
    
            staked_amount.clone()
        });
        if remaining_stake == 0 {
            self.staked_addresses().swap_remove(&caller);
        }
        self.total_supply().update(|current_amount| *current_amount -= &unstake_amount);

    
        self.send().direct_egld(&caller, &unstake_amount);
    }

    #[endpoint]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();
        self.update_reward(&caller);

        let reward_amount = self.rewards(&caller).get();
        let self_contract_reward_balance = self.contract_reward_balance().get();
        require!(&self_contract_reward_balance >= &reward_amount, "Reward fund not sufficient in contract, wait for refill");
        if &reward_amount > &0 {
            self.rewards(&caller).set(BigUint::zero());
            self.contract_reward_balance().update(|current_amount| *current_amount -= &reward_amount);
            self.send().direct_egld(&caller, &reward_amount);
        }
    }

    #[view(getStakedAddresses)]
    #[storage_mapper("stakedAddresses")]
    fn staked_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getStakingPosition)]
    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getContractRewardBalance)]
    #[storage_mapper("contractRewardBalance")]
    fn contract_reward_balance(&self) -> SingleValueMapper<BigUint>;

    #[view(getUpdatedAt)]
    #[storage_mapper("updatedAt")]
    fn updated_at(&self) -> SingleValueMapper<u64>;

    #[view(getRewardStored)]
    #[storage_mapper("rewardStored")]
    fn reward_stored(&self) -> SingleValueMapper<BigUint>;

    #[view(getUserRewardPaid)]
    #[storage_mapper("userRewardPaid")]
    fn user_reward_paid(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;


    #[view(getRewards)]
    #[storage_mapper("rewards")]
    fn rewards(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getTotalSupply)]
    #[storage_mapper("totalSupply")]
    fn total_supply(&self) -> SingleValueMapper<BigUint>;

    fn update_reward(
        &self,
        user: &ManagedAddress
    ) {
        let reward_stored_now = self.reward();
        self.reward_stored().set(reward_stored_now);
        let current_block_timestamp = self.blockchain().get_block_timestamp();
        self.updated_at().set(current_block_timestamp);

  
        self.rewards(&user)
            .set(self.earned(&user));
        self.user_reward_paid(&user)
            .set(self.reward_stored().get());
    }

    #[view(reward)]
    fn reward(&self) -> BigUint {
        let total_supply = self.total_supply().get();
        if total_supply == 0 {
            return self.reward_stored().get();
        }
        let self_reward_stored = self.reward_stored().get();
        let current_block_timestamp = self.blockchain().get_block_timestamp();
        let self_updated_at = self.updated_at().get();
        let total_reward = BigUint::from(REWARD_RATE) * BigUint::from(current_block_timestamp - self_updated_at) * (DECIMAL_PREC);

        return
        self_reward_stored + total_reward/total_supply;// total_reward/total_supply;
    }

    #[view(earned)]
    fn earned(
        &self,
        user: &ManagedAddress
    ) -> BigUint {
        let self_reward = self.reward();
        let self_user_reward_paid = self.user_reward_paid(user).get();
        let self_staking_position = self.staking_position(user).get();
        return
            ((self_staking_position * (self_reward - self_user_reward_paid)) / DECIMAL_PREC) + self.rewards(user).get();
    }

    
}