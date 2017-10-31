
use exonum::crypto::Hash;
use exonum::storage::{Snapshot, Fork};
use exonum::blockchain::{gen_prefix};
use exonum::storage::ProofMapIndex;

use model::{Contractor};

use {DC_SERVICE_ID};



pub struct DigitalContractSchema<T> {
    view: T,
}

impl<T> DigitalContractSchema<T>
    where
        T: AsRef<Snapshot>,
{
    pub fn new(snapshot: T) -> DigitalContractSchema<T> {
        DigitalContractSchema { view: snapshot }
    }

    pub fn contents(&self) -> ProofMapIndex<&T, Hash, Contractor> {
        let prefix = gen_prefix(DC_SERVICE_ID, 0, &());
        ProofMapIndex::new(prefix, &self.view)
    }

    pub fn state_hash(&self) -> Vec<Hash> {
        vec![self.contents().root_hash()]
    }
}

impl<'a> DigitalContractSchema<&'a mut Fork> {
    pub fn contents_mut(&mut self) -> ProofMapIndex<&mut Fork, Hash, Contractor> {
        let prefix = gen_prefix(DC_SERVICE_ID, 0, &());
        ProofMapIndex::new(prefix, &mut self.view)
    }
}
