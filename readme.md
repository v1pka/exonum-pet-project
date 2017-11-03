Simple blockchain app
====

Main goal - to put contractor data to the blockchain.

# How to buid, run and test
## Commands for executable **dcontract**
### To build config from scratch
* generate-template config.toml
* generate-config config.toml pub_config.toml sec_config.toml --peer-addr 127.0.0.1:6333
* finalize sec_config.toml final_config.toml -p pub_config.toml
### Starting an application:
run -d db -c final_config.toml --public-api-address 127.0.0.1:8000

## Testing api services

* Put contractor:
curl -H "Content-Type: applicationjson" -X PUT -d @contractor.json http://127.0.0.1:8000/api/services/dc_service/contractors -v --noproxy '*'

* Get contractor
http://127.0.0.1:8000/api/services/dc_service/contractors

* Get contractor by hash
http://127.0.0.1:8000/api/services/dc_service/contractors/11as1d12-a6ea-47dc-b641-0e20ae7c86ab

## Enabling debug log
Set this variables before running an executable:
export RUST_LOG=debug
export RUST_BACKTRACE=1

## Full command to run application from target/debug dir (make sure that you've created db dir)
./dcontract run -d db -c ../../final_config.toml --public-api-address 127.0.0.1:8000

# How to live with RUST
* Required environment (specially for Visual Studio code):
rustup default nightly
rustup update nightly
rustup component add rls
rustup component add rust-analysis
rustup component add rust-sr
* To test Exonum see this article:
https://exonum.com/doc/advanced/service-testing/
