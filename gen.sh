export OPENAPI_GENERATOR_VERSION=5.0.0
openapi-generator-cli validate -i tft_db_service_api.yaml
openapi-generator-cli generate -i tft_db_service_api.yaml -g rust-server -o db_service_api_rs -c config.yaml | tee gen.log
