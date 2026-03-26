#[cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_set_token_success_when_zero_staked() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, ArenaContract);
    let client = ArenaContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let initial_token = Address::generate(&env);
    let new_token = Address::generate(&env);

    client.initialize(&admin, &initial_token);
    assert_eq!(client.get_token(), initial_token);

    // Should succeed when total_staked is 0
    client.set_token(&new_token);
    assert_eq!(client.get_token(), new_token);
}

#[test]
#[should_panic(expected = "cannot change token while funds are staked")]
fn test_set_token_fails_when_funds_staked() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, ArenaContract);
    let client = ArenaContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token = Address::generate(&env);

    client.initialize(&admin, &token);
    
    // Stake some funds
    client.mock_stake(&100);
    assert_eq!(client.get_total_staked(), 100);

    // This should panic because total_staked != 0
    client.set_token(&Address::generate(&env));
}
