#![no_std]

use soroban_sdk::{testutils::Env, Address, Symbol, String};
use super::WhereDidItGo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path() {
        let env = Env::default();
        let contract_id = env.register_contract(None, WhereDidItGo);

        let ngo = Address::random(&env);
        let recipient = Address::random(&env);
        let merchant = Address::random(&env);
        let id = Symbol::short("V1");

        // Issue voucher
        env.invoke_contract(
            &contract_id,
            "issue_voucher",
            (&id, &ngo, &recipient, &merchant, &100i128, &1000u64),
        );

        // Redeem voucher
        env.invoke_contract(
            &contract_id,
            "redeem",
            (&id, &merchant),
        );

        // Check voucher is marked redeemed
        let voucher: super::Voucher = env
            .invoke_contract(&contract_id, "get_voucher", (&id,))
            .unwrap();
        assert!(voucher.redeemed);
    }

    #[test]
    fn test_edge_case_duplicate_redeem() {
        let env = Env::default();
        let contract_id = env.register_contract(None, WhereDidItGo);

        let ngo = Address::random(&env);
        let recipient = Address::random(&env);
        let merchant = Address::random(&env);
        let id = Symbol::short("V2");

        env.invoke_contract(
            &contract_id,
            "issue_voucher",
            (&id, &ngo, &recipient, &merchant, &50i128, &1000u64),
        );

        // Redeem first time
        env.invoke_contract(&contract_id, "redeem", (&id, &merchant));

        // Redeem second time should panic
        let result = std::panic::catch_unwind(|| {
            env.invoke_contract(&contract_id, "redeem", (&id, &merchant))
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_state_verification() {
        let env = Env::default();
        let contract_id = env.register_contract(None, WhereDidItGo);

        let ngo = Address::random(&env);
        let recipient = Address::random(&env);
        let merchant = Address::random(&env);
        let id = Symbol::short("V3");

        env.invoke_contract(
            &contract_id,
            "issue_voucher",
            (&id, &ngo, &recipient, &merchant, &200i128, &1000u64),
        );

        // Check storage state
        let voucher: super::Voucher = env
            .invoke_contract(&contract_id, "get_voucher", (&id,))
            .unwrap();
        assert_eq!(voucher.amount, 200i128);
        assert!(!voucher.redeemed);
    }
}
