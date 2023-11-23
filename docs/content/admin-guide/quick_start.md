# Quick Start

## Prerequesits
To run the quick start example please ensure to have a working DAPS.

### Private Key
You will need the private key in the following formats:
* .jks
* .der

### Environment
```.env
VERSION=v1.0.0-alpha.5
SERVICE_ID_LOG=1
SHARED_SECRET=changethis
KEY_PASSWORD=password
DAPS_URL=
DAPS_JWKS_URL=
API_KEY=changethis
```

## docker-compose.yml
```sh
docker compose up
```

```docker-compoye.yml
version: "3.8"

services:
    ch-app:
        image: ghcr.io/truzzt/ids-basecamp-clearing/ch-app:$VERSION 
        environment:
            CH_APP_PROCESS_DATABASE_URL: mongodb://mongodb:27017 
            CH_APP_KEYRING_DATABASE_URL: mongodb://mongodb:27017
            CH_APP_DOCUMENT_DATABASE_URL: mongodb://mongodb:27017
            CH_APP_CLEAR_DB: false
            CH_APP_LOG_LEVEL: INFO
            SERVICE_ID_LOG: $SERVICE_ID
            SHARED_SECRET: $SHARED_SECRET
        volumes:
            - ./YOUR_PRIVATE_KEY.der:/app/keys/private_key.der:ro

    ch-edc:
        image: ghcr.io/truzzt/ids-basecamp-clearing/ch-edc:$VERSION
        environment:
            WEB_HTTP_PORT=11001
            WEB_HTTP_PATH=/api
            WEB_HTTP_DATA_PORT=11002
            WEB_HTTP_DATA_PATH=/api/v1/data
            WEB_HTTP_IDS_PORT=11003
            WEB_HTTP_IDS_PATH=/api/v1/ids
            EDC_IDS_ID=urn:connector:example-connector
            EDC_IDS_TITLE='truzzt Test EDC Connector'
            EDC_IDS_DESCRIPTION='Minimally configured Open Source EDC built by truzzt.'
            EDC_IDS_ENDPOINT=http://ch-edc:11003/api/v1/ids
            IDS_WEBHOOK_ADDRESS=http://ch-edc:11003
            EDC_IDS_CURATOR=https://truzzt.com
            EDC_IDS_MAINTAINER=https://truzzt.com
            EDC_CONNECTOR_NAME=truzzt-example-connector
            EDC_HOSTNAME=ch-edc
            EDC_API_AUTH_KEY=$API_KEY
            EDC_WEB_REST_CORS_ENABLED='true'
            EDC_WEB_REST_CORS_HEADERS='origin,content-type,accept,authorization,x-api-key'
            EDC_WEB_REST_CORS_ORIGINS='*'
            EDC_VAULT=/resources/vault/edc/vault.properties
            EDC_OAUTH_TOKEN_URL=$DAPS_URL
            EDC_OAUTH_PROVIDER_JWKS_URL=$DAPS_JWKS_URL
            EDC_OAUTH_CLIENT_ID=$CLIENT_ID
            EDC_KEYSTORE=/resources/vault/edc/keystore.jks
            EDC_KEYSTORE_PASSWORD=$KEY_PASSWORD
            EDC_OAUTH_CERTIFICATE_ALIAS=1
            EDC_OAUTH_PRIVATE_KEY_ALIAS=1
            TRUZZT_CLEARINGHOUSE_JWT_AUDIENCE=$SERVICE_ID
            TRUZZT_CLEARINGHOUSE_JWT_ISSUER=ch-edc
            TRUZZT_CLEARINGHOUSE_JWT_SIGN_SECRET=$SHARED_SECRET 
            TRUZZT_CLEARINGHOUSE_JWT_EXPIRES_AT=30
            TRUZZT_CLEARINGHOUSE_APP_BASE_URL=ch-edc:8080
        volumes:
            - ./YOUR_PRIVATE_KEY.jks:/resources/vault/edc/keystore.jks

    mongodb:
        image: mongo
```

