#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol};

#[contracttype]
pub enum DataKey {
    Voucher(Symbol),
    Merchant(Address),
}

#[contracttype]
pub struct Voucher {
    pub id: Symbol,
    pub ngo: Address,
    pub recipient: Address,
    pub merchant: Address,
    pub amount: i128,
    pub redeemed: bool,
    pub valid_until: u64,
}

#[contracttype]
pub struct Merchant {
    pub name: String,
    pub location: String,
    pub is_active: bool,
}

#[event]
pub struct VoucherIssued {
    pub id: Symbol,
    pub ngo: Address,
    pub recipient: Address,
    pub merchant: Address,
    pub amount: i128,
    pub valid_until: u64,
}

#[event]
pub struct VoucherRedeemed {
    pub id: Symbol,
    pub merchant: Address,
    pub amount: i128,
}

#[contract]
pub struct WhereDidItGo;

#[contractimpl]
impl WhereDidItGo {
    pub fn issue_voucher(
        env: Env,
        id: Symbol,
        ngo: Address,
        recipient: Address,
        merchant: Address,
        amount: i128,
        valid_until: u64,
    ) {
        let voucher = Voucher {
            id: id.clone(),
            ngo,
            recipient,
            merchant: merchant.clone(),
            amount,
            redeemed: false,
            valid_until,
        };

        env.storage().set_contract_data(&DataKey::Voucher(id.clone()), &voucher);

        env.events().publish(
            (symbol_short!("VoucherIssued"),),
            VoucherIssued {
                id,
                ngo,
                recipient,
                merchant,
                amount,
                valid_until,
            },
        );
    }

    pub fn redeem(env: Env, id: Symbol, merchant: Address) {
        let mut voucher: Voucher = env
            .storage()
            .get_contract_data(&DataKey::Voucher(id.clone()))
            .expect("Voucher not found");

        if voucher.redeemed {
            panic!("Voucher already redeemed");
        }
        if merchant != voucher.merchant {
            panic!("Unauthorized merchant");
        }

        voucher.redeemed = true;
        env.storage().set_contract_data(&DataKey::Voucher(id.clone()), &voucher);

        env.events().publish(
            (symbol_short!("VoucherRedeemed"),),
            VoucherRedeemed {
                id,
                merchant,
                amount: voucher.amount,
            },
        );
    }

    pub fn register_merchant(env: Env, merchant: Address, name: String, location: String) {
        let merchant_info = Merchant {
            name,
            location,
            is_active: true,
        };
        env.storage().set_contract_data(&DataKey::Merchant(merchant), &merchant_info);
    }

    pub fn deregister_merchant(env: Env, merchant: Address) {
        let mut merchant_info: Merchant = env
            .storage()
            .get_contract_data(&DataKey::Merchant(merchant))
            .expect("Merchant not found");
        merchant_info.is_active = false;
        env.storage().set_contract_data(&DataKey::Merchant(merchant), &merchant_info);
    }

    pub fn is_merchant_active(env: Env, merchant: Address) -> bool {
        match env.storage().get_contract_data::<_, Merchant>(&DataKey::Merchant(merchant)) {
            Some(merchant_info) => merchant_info.is_active,
            None => false,
        }
    }

    pub fn get_voucher(env: Env, id: Symbol) -> Voucher {
        env.storage()
            .get_contract_data(&DataKey::Voucher(id))
            .expect("Voucher not found")
    }
}
