#![no_std] // REQUIRED for Soroban WASM

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, String, contractevent};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Voucher(Symbol),
    Merchant(Address),
}

#[derive(Clone)]
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

#[derive(Clone)]
#[contracttype]
pub struct Merchant {
    pub name: String,
    pub location: String,
    pub is_active: bool,
}

#[contractevent]
pub struct VoucherIssued {
    pub id: Symbol,
    pub ngo: Address,
    pub recipient: Address,
    pub merchant: Address,
    pub amount: i128,
    pub valid_until: u64,
}

#[contractevent]
pub struct VoucherRedeemed {
    pub id: Symbol,
    pub merchant: Address,
    pub amount: i128,
}

#[contract]
pub struct WhereDidItGo;

#[contractimpl]
impl WhereDidItGo {
    // Issue a new voucher
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
        env.storage().set(&DataKey::Voucher(id.clone()), &voucher);
        env.events().publish(
            (Symbol::short("VoucherIssued"),),
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

    // Redeem a voucher
    pub fn redeem(env: Env, id: Symbol, merchant: Address) {
        let mut voucher: Voucher = env
            .storage()
            .get(&DataKey::Voucher(id.clone()))
            .expect("Voucher not found");

        if voucher.redeemed {
            panic!("Voucher already redeemed");
        }
        if merchant != voucher.merchant {
            panic!("Unauthorized merchant");
        }

        voucher.redeemed = true;
        env.storage().set(&DataKey::Voucher(id.clone()), &voucher);

        env.events().publish(
            (Symbol::short("VoucherRedeemed"),),
            VoucherRedeemed {
                id,
                merchant,
                amount: voucher.amount,
            },
        );
    }

    // Register a merchant
    pub fn register_merchant(env: Env, merchant: Address, name: String, location: String) {
        let merchant_info = Merchant {
            name,
            location,
            is_active: true,
        };
        env.storage().set(&DataKey::Merchant(merchant), &merchant_info);
    }

    // Deregister a merchant
    pub fn deregister_merchant(env: Env, merchant: Address) {
        let mut merchant_info: Merchant = env
            .storage()
            .get(&DataKey::Merchant(merchant))
            .unwrap_or_else(|| panic!("Merchant not found"));
        merchant_info.is_active = false;
        env.storage().set(&DataKey::Merchant(merchant), &merchant_info);
    }

    // Check if a merchant is active
    pub fn is_merchant_active(env: Env, merchant: Address) -> bool {
        match env.storage().get::<_, Merchant>(&DataKey::Merchant(merchant)) {
            Some(merchant_info) => merchant_info.is_active,
            None => false,
        }
    }

    // Get voucher details
    pub fn get_voucher(env: Env, id: Symbol) -> Voucher {
        env.storage()
            .get(&DataKey::Voucher(id))
            .expect("Voucher not found")
    }
}
