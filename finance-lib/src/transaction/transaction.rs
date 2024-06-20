use lifesuite_common::component::{ComponentMeta, ComponentType};
use uuid::Uuid;

use crate::{account::Account, ledger::Ledger};

pub struct Transaction {
    pub meta: ComponentMeta,
    pub amount: f64,
    pub from: Uuid, // id of the originating account
    pub to: Uuid,   // id of the destination account
    pub ledger: Uuid,
}

impl Transaction {
    pub fn new(amount: f64, to: &Account, from: &Account, ledger: &Ledger) -> Transaction {
        Transaction {
            meta: ComponentMeta::new(ComponentType::Transaction),
            amount,
            to: to.meta.id,
            from: from.meta.id,
            ledger: ledger.meta.id,
        }
    }
    pub fn get_by_id(_id: &Uuid) -> Option<Transaction> {
        todo!();
    }
    pub fn get_current() -> Transaction {
        todo!();
    }
}
