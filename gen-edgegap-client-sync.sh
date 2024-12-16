#!/bin/bash -e
# re-generate the edgegap client from the api definition
wget https://api.edgegap.com/swagger.json -O swagger.json
docker run \
	--rm -v $PWD:/local openapitools/openapi-generator-cli generate \
	-i /local/swagger.json -g rust -o /local/edgegap --additional-properties=supportAsync=false,library=reqwest,packageName=edgegap
rm swagger.json
