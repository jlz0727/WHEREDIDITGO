#![no_std]

use soroban_sdk::{contractimpl, contracttype, symbol_short, Address, Env, Symbol};

// ------------------- Data Keys -------------------

#[derive(Clone, Debug, Eq, PartialEq, contracttype)]
pub enum DataKey {
    Voucher(Symbol),
    Merchant(Address),
}

// ------------------- Voucher Struct -------------------

#[derive(Clone, Debug, Eq, PartialEq, contracttype)]
pub struct Voucher {
    pub id: Symbol,
    pub amount: i128,
    pub redeemed: bool,
}

// ------------------- Merchant Struct -------------------

#[derive(Clone, Debug, Eq, PartialEq, contracttype)]
pub struct Merchant {
    pub name: Symbol,
    pub total_redeemed: i128,
}

// ------------------- Events -------------------

#[derive(Clone, Debug, Eq, PartialEq, contracttype)]
pub struct VoucherIssued {
    pub id: Symbol,
    pub ngo: Address,
    pub recipient: Address,
    pub merchant: Address,
    pub amount: i128,
    pub valid_until: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, contracttype)]
pub struct VoucherRedeemed {
    pub id: Symbol,
    pub merchant: Address,
    pub amount: i128,
}

// ------------------- Contract Implementation -------------------

pub struct VoucherContract;

#[contractimpl]
impl VoucherContract {
    // Issue a voucher
    pub fn issue(
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
            amount,
            redeemed: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Voucher(id.clone()), &voucher);

        env.events().publish(
            (symbol_short!("VIssued"),),
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
            .persistent()
            .get(&DataKey::Voucher(id.clone()))
            .expect("Voucher not found");

        if voucher.redeemed {
            panic!("Voucher already redeemed");
        }

        voucher.redeemed = true;
        env.storage()
            .persistent()
            .set(&DataKey::Voucher(id.clone()), &voucher);

        // Update merchant data
        let mut merchant_info: Merchant = env
            .storage()
            .persistent()
            .get(&DataKey::Merchant(merchant.clone()))
            .unwrap_or(Merchant {
                name: symbol_short!("Unknown"),
                total_redeemed: 0,
            });

        merchant_info.total_redeemed += voucher.amount;
        env.storage()
            .persistent()
            .set(&DataKey::Merchant(merchant.clone()), &merchant_info);

        env.events().publish(
            (symbol_short!("VRedeemed"),),
            VoucherRedeemed {
                id,
                merchant,
                amount: voucher.amount,
            },
        );
    }

    // Get voucher info
    pub fn get_voucher(env: Env, id: Symbol) -> Option<Voucher> {
        env.storage().persistent().get(&DataKey::Voucher(id))
    }

    // Get merchant info
    pub fn get_merchant(env: Env, merchant: Address) -> Option<Merchant> {
        env.storage().persistent().get(&DataKey::Merchant(merchant))
    }
}
