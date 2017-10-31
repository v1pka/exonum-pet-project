use {DC_SERVICE_ID, TX_CREATE_CONTRACTOR_ID};
use model::model::{Contractor};

message! {
     struct TxCreateContractor {
        const TYPE = DC_SERVICE_ID;
        const ID = TX_CREATE_CONTRACTOR_ID;
        const SIZE = 25;
        field contractor:                  Contractor  [00 => 25]
    }
}