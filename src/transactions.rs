use serde_json;
use serde_json::Value;

use exonum::storage::Fork;
use exonum::blockchain::Transaction;

use exonum::crypto::hash;

use model::{DigitalContractSchema, TxCreateContractor};

impl Transaction for TxCreateContractor {
    fn verify(&self) -> bool {
        true
    }

    fn execute(&self, fork: &mut Fork) {
        let mut schema = DigitalContractSchema::new(fork);
        let content = self.contractor();
        let hash = hash(content.id().as_bytes());
        schema.contents_mut().put(&hash, content.clone());
    }

    fn info(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}