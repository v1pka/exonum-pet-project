Simple blockchain app
====

Main goal - to put contractor data to the blockchain.

# To build config from scratch (commands for executable dcontract)
* generate-template config.toml
* generate-config config.toml pub_config.toml sec_config.toml --peer-addr 127.0.0.1:6333
* finalize sec_config.toml final_config.toml -p pub_config.toml
# To start application (command for executable dcontract):
run -d db -c final_config.toml --public-api-address 127.0.0.1:8000

# To test api services

* Put contractor:
curl -H "Content-Type: applicationjson" -X PUT -d @contractor.json http://127.0.0.1:8000/api/services/dc_service/contractors -v --noproxy '*'

* Get contractor
http://127.0.0.1:8000/api/services/dc_service/contractors

* Get contractor by hash
http://127.0.0.1:8000/api/services/dc_service/contractors/11as1d12-a6ea-47dc-b641-0e20ae7c86ab

# To debug
Set this variables before running an executable:
export RUST_LOG=debug
export RUST_BACKTRACE=1

# Full command to run application from target/debug dir (make sure that you've created db dir)
./dcontract run -d db -c ../../final_config.toml --public-api-address 127.0.0.1:8000

