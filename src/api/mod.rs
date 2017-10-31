use iron::prelude::*;
use router::{Router, Params};
use bodyparser;

use exonum::crypto::{hash, Hash, HexValue, Signature};
use exonum::blockchain::Blockchain;
use exonum::node::TransactionSend;
use exonum::api::{Api, ApiError};


use model::*;

#[derive(Clone)]
pub struct PublicApi<T: TransactionSend + Clone> {
    channel: T,
    blockchain: Blockchain,
}

impl<T> PublicApi<T>
where
    T: TransactionSend + Clone,
{
    pub fn new(blockchain: Blockchain, channel: T) -> PublicApi<T> {
        PublicApi {
            blockchain: blockchain,
            channel: channel,
        }
    }

    fn put_contractor(&self, contractor: Contractor) -> Result<TxCreateContractor, ApiError> {
        {
            let id = contractor.id();
            let hash = hash (id);
            let snapshot = self.blockchain.snapshot();

            if DigitalContractSchema::new(&snapshot)
                .contents()
                .get(&hash)
                .is_some()
            {
                return Err(ApiError::FileExists(hash));
            }
        }
        // Create transaction
        let tx = TxCreateContractor::new_with_signature(contractor, &Signature::zero());
        self.channel.send(Box::new(tx.clone()))?;
        Ok(tx)
    }

    fn get_contractor(&self, hash: &Hash) -> Result<Contractor, ApiError> {
        let view = self.blockchain.snapshot();
        DigitalContractSchema::new(&view)
            .contents()
            .get(&hash)
            .ok_or_else(|| ApiError::FileNotFound(*hash))
    }

    fn get_contractors(&self) -> Result<Vec<(Hash, Contractor)>, ApiError> {
        let view = self.blockchain.snapshot();

        let mut result = Vec::new();
        let exonum_schema = DigitalContractSchema::new(&view);
        let contents = exonum_schema.contents();
        result = contents.iter().collect();
        info!("================== get_contents ============== {:?}", result);
        Ok(result.clone())
    }
}

fn parse_hex(map: &Params, id: &str) -> Result<Hash, ApiError> {
    match map.find(id) {
        Some(hex_str) => {
            let hash = Hash::from_hex(hex_str).map_err(|e| {
                let msg = format!(
                    "An error during parsing of the `{}` id occurred: {}",
                    hex_str,
                    e
                );
                ApiError::IncorrectRequest(msg.into())
            })?;
            Ok(hash)
        }
        None => {
            let msg = format!("The `{}` hash is not specified.", id);
            Err(ApiError::IncorrectRequest(msg.into()))
        }
    }
}

impl<T> Api for PublicApi<T>
where
    T: TransactionSend + Clone + 'static,
{
    fn wire(&self, router: &mut Router) {
        // Receive a message by POST and play it back.
        let api = self.clone();
        let put_contractor = move |req: &mut Request| -> IronResult<Response> {
            match req.get::<bodyparser::Struct<Contractor>>() {
                Ok(Some(content)) => {
                    let tx = api.put_contractor(content)?;
                    api.ok_response(&json!(tx))
                }
                Ok(None) => Err(ApiError::IncorrectRequest("Empty request body".into()))?,
                Err(e) => Err(ApiError::IncorrectRequest(Box::new(e)))?,
            }
        };

        let api = self.clone();
        let get_contractor = move |req: &mut Request| -> IronResult<Response> {
            let map = req.extensions.get::<Router>().unwrap();

            let hash = parse_hex(&map, "hash")?;
            let content = api.get_contractor(&hash)?;

            api.ok_response(&json!(content))
        };

        let api = self.clone();
        let get_contractors = move |_: &mut Request| -> IronResult<Response> {
            let contents = api.get_contractors()?;

            api.ok_response(&json!(contents))
        };

        // Contents
        router.put("/contractors",put_contractor, "put_contractor");
        router.get("/contractors/:hash", get_contractor, "get_contractor");
        router.get("/contractors", get_contractors, "get_contractors");
    }
}
