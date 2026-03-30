#[cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::Address as _, Address, Env,
    token::StellarAssetClient,
};

fn setup() -> (Env, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let staker = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token_id = env
        .register_stellar_asset_contract_v2(token_admin.clone())
        .address();
    let token_admin_client = StellarAssetClient::new(&env, &token_id);
    token_admin_client.mint(&staker, &10_000i128);

    let contract_id = env.register_contract(None, StakingContract);
    let client = StakingContractClient::new(&env, &contract_id);
    client.initialize(&admin, &token_id);

    (env, contract_id, staker)
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StakingContract);
    let client = StakingContractClient::new(&env, &contract_id);

    assert_eq!(client.hello(), 101112);
}

#[test]
fn stake_fails_when_paused() {
    let (env, contract_id, staker) = setup();
    let client = StakingContractClient::new(&env, &contract_id);

    client.pause();
    assert!(client.is_paused());

    let result = client.try_stake(&staker, &500i128);
    assert_eq!(result, Err(Ok(StakingError::Paused)));
}

#[test]
fn unstake_fails_when_paused() {
    let (env, contract_id, staker) = setup();
    let client = StakingContractClient::new(&env, &contract_id);

    // Stake successfully while unpaused, then pause.
    client.stake(&staker, &1_000i128);
    assert_eq!(client.staked_balance(&staker), 1_000i128);

    client.pause();
    assert!(client.is_paused());

    let result = client.try_unstake(&staker, &500i128);
    assert_eq!(result, Err(Ok(StakingError::Paused)));

    // Balance unchanged.
    assert_eq!(client.staked_balance(&staker), 1_000i128);
}

#[test]
fn unpause_restores_functionality() {
    let (env, contract_id, staker) = setup();
    let client = StakingContractClient::new(&env, &contract_id);

    // Pause and verify stake is blocked.
    client.pause();
    assert!(client.is_paused());
    assert_eq!(
        client.try_stake(&staker, &500i128),
        Err(Ok(StakingError::Paused))
    );

    // Unpause and verify stake succeeds.
    client.unpause();
    assert!(!client.is_paused());

    let shares = client.stake(&staker, &500i128);
    assert_eq!(shares, 500i128);
    assert_eq!(client.staked_balance(&staker), 500i128);

    // Unstake also works after unpause.
    let returned = client.unstake(&staker, &500i128);
    assert_eq!(returned, 500i128);
    assert_eq!(client.staked_balance(&staker), 0i128);
}
