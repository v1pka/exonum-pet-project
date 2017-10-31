extern crate exonum;
extern crate exonum_configuration;
extern crate dcontract;

use exonum::helpers::fabric::NodeBuilder;

use exonum_configuration::ConfigurationService;
use dcontract::{DigitalContractService};



fn main() {
    exonum::helpers::init_logger().unwrap();

    println!("Starting node");

    NodeBuilder::new()
        .with_service::<ConfigurationService>()
        .with_service::<DigitalContractService>()
        .run();


}
