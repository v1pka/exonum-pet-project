Simple blockchain app
====

# To build config from scratch
* generate-template config.toml
* generate-config config.toml pub_config.toml sec_config.toml --peer-addr 127.0.0.1:6333
* finalize sec_config.toml final_config.toml -p pub_config.toml
# To start application:
run -d db -c final_config.toml --public-api-address 127.0.0.1:8000

# To test api services

* Get contractor
http://127.0.0.1:8000/api/services/dc_service/contractors

* Get contractor by hash
http://127.0.0.1:8000/api/services/dc_service/contractors/addfdf

* Not working - post on http://127.0.0.1:8000/api/services/dc_service/contractors:
```
{
    "id": "114b1d12-a6ea-47dc-b641-0e10ae7c86ab",
    "node_id": "114b1d12-a6ea-47dc-b641-0e10ae7c86ab",
    "status": 0,
    "serialized_contractor": "2103"
}
```

