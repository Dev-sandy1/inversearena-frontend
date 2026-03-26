#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    Token,
    TotalStaked,
}

#[contract]
pub struct ArenaContract;

#[contractimpl]
impl ArenaContract {
    /// Initialize the contract with an admin and the initial token address.
    pub fn initialize(env: Env, admin: Address, token: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::TotalStaked, &0i128);
    }

    /// Update the reward/stake token address.
    /// SECURITY: Can only be updated if there are strictly zero funds staked in the protocol.
    pub fn set_token(env: Env, new_token: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let total_staked: i128 = env.storage().instance().get(&DataKey::TotalStaked).unwrap_or(0);
        
        // Gate set_token behind a total_staked == 0 check.
        if total_staked != 0 {
            panic!("cannot change token while funds are staked");
        }

        env.storage().instance().set(&DataKey::Token, &new_token);
    }

    pub fn get_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Token).unwrap()
    }

    pub fn get_total_staked(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalStaked).unwrap_or(0)
    }

    /// Mock function to simulate staking for testing the security gate.
    pub fn mock_stake(env: Env, amount: i128) {
        let total_staked: i128 = env.storage().instance().get(&DataKey::TotalStaked).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalStaked, &(total_staked + amount));
    }
}

#[cfg(test)]
mod test;
