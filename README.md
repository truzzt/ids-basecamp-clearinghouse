# IDS Clearing House
The IDS Clearing House Service is a prototype implementation of the Clearing House component of the [Industrial Data Space](https://github.com/International-Data-Spaces-Association/IDS-G). The Clearing House provides an API to store and retrieve data. Data in the Clearing House is stored encrypted and practically immutable. There are multiple ways in which the Clearing House enforces Data Immutability:
- Using the Clearing House Service API there is no way to update an already existing log entry in the database
- Log entries in the database include a hash value of the previous log entry, chaining together all log entries. Any change to a previous log entry would require rehashing all following log entries.
- The connector logging information in the Clearing House receives a signed receipt from the Clearing House that includes among other things a timestamp and the current chain hash. A single valid receipt in possession of any connector is enough to detect any change to data up to the time indicated in the receipt.

The IDS Clearing House Service consists of two parts:

1. [Clearing House App](clearing-house-app)
2. [Clearing House Processors](clearing-house-processors)

The Clearing House Service API is defined by the [IDS-G](https://github.com/International-Data-Spaces-Association/IDS-G/tree/main) and represents the top level service.

## Requirements
- [OpenSSL](https://www.openssl.org)
- [MongoDB](https://www.mongodb.com)
- ([Docker](https://www.docker.com))

Additionally, the Clearing House App depends on on two micro services from the [Clearing House Core](https://github.com/Fraunhofer-AISEC/ids-clearing-house-core):
1. Document API
2. Keyring API

The Document API is responsible for storing the data, while the Keyring API provides cryptographic support for encryption and decryption of the stored data. Please refer to the documentation [here](https://github.com/Fraunhofer-AISEC/ids-clearing-house-core) how to set up those services.

The Clearing House Service API requires a Trusted Connector [Trusted Connector](https://github.com/industrial-data-space/trusted-connector) for deployment. The process of setting up a Trusted Connector is described [here](https://industrial-data-space.github.io/trusted-connector-documentation/docs/getting_started/). Using a docker image of the Trusted Connector should be sufficient for most deployments:

`docker pull fraunhoferaisec/trusted-connector-core:6.0.0`

## Configuration

### Clearing House App
The Clearing House App is configured using the configuration file [`Rocket.toml`](clearing-house-app/clearing-house-api/Rocket.toml), which must specify a set of configuration options, such as the correct URLs of the database and other service apis:
- `daps_api_url`: Specifies the URL of the DAPS Service. Required to validate DAPS token
- `keyring_api_url`: Specifies the URL of the Keyring API
- `document_api_url`: Specifies the URL of the Document API
- `database_url`: Specifies the URL of the database to store process information. Currently only mongodb is supported so URL is supposed to be `mongodb://<host>:<port>`
- `infomodel_version`: Specifies which Version of the [InfoModel](https://github.com/International-Data-Spaces-Association/InformationModel) is used by the Clearing House. Currently: `4.0.0`
- `connector_name`: Needed for IDS Messages as specified by the [InfoModel](https://github.com/International-Data-Spaces-Association/InformationModel)
- `server_agent`: Needed for IDS Messages as specified by the [InfoModel](https://github.com/International-Data-Spaces-Association/InformationModel)
- `clear_db`: `true` or `false` indicates if the database should be cleared when starting the Service API or not. If `true` a restart will wipe the database! Starting the Service API on a clean database will initialize the database.
- `signing_key`: Location of the private key (DER format) used for signing the Receipts. Clearing House uses PS512 algorithm for signing.

More information on general configuration options in a `Rocket.toml` file can be found [here](https://rocket.rs/v0.5-rc/guide/configuration/#rockettoml).

#### Logging
When starting the Clearing House Service API it also needs the following environment variables set:
- `API_LOG_LEVEL`: Allowed log levels are: `Off`, `Error`, `Warn`, `Info`, `Debug`, `Trace`

#### Signing Key
The Clearing House API sends a signed receipt as response to a logging request. The key can be created using openssl:

`openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:4096 -outform der -out private_key.der`

Please note that the Clearing House requires the key to be in DER format. It must be available to the Clearing House App under the path configured in `Rocket.toml`, e.g. `/server/keys/private_key.der`.

#### DAPS
The Clearing House needs to be able to validate the certificate used by the DAPS. If the DAPS uses a self-signed certificate the certificate needs to be added in two places:
1. `/server/certs`: The Clearing House App will load certificates in this folder and use them for validation. The certificate needs to be in DER format.
2. `/usr/local/share/ca-certificates`: The Clearing House App relies on openssl for parts of the validation and openssl will not trust a self-signed certificate unless it was added in this folder and `update-ca-certificates` was called. Once this is done the container might need to be restarted.

If you are using [these dockerfiles](docker/) and use `daps.aisec.fraunhofer.de` as the DAPS, you only need to follow Step 1. The certificate needed for Step 1 can be found [here](clearing-house-app/clearing-house-api/certs).

#### Example Configuration (docker-compose)
```
clearing-house-api:
    container_name: "clearing-house-api"
    depends_on:
        - document-api
        - keyring-api
        - clearing-house-mongo
    environment:
        # Allowed levels: Off, Error, Warn, Info, Debug, Trace
        - API_LOG_LEVEL=Debug
    ports:
        - "8000:8000"
    volumes:
        - ./data/Rocket.toml:/server/Rocket.toml
        - ./data/keys:/server/keys
        - ./data/certs:/server/certs
```

### Trusted Connector
The Clearing House Processors are written in Java for use in the Camel Component of the Trusted Connector. To configure the Trusted Connector for the Clearing House Service API it needs access to the following files inside the docker container (e.g. mounted as a volume):
- `clearing-house-processors.jar`: The Clearing House Processors need to be placed in the `/root/jars` folder of the Trusted Connector. The jar file needs to be [build](#building-from-source) from the Clearing House Processors using `gradle`.
- [`clearing-house-routes.xml`](clearing-house-processors/src/routes/clearing-house-routes.xml): The camel routes required by the Clearing House need to be placed in the `/root/deploy` folder of the Trusted Connector.

Besides those files that are specific for the configuration of the Clearing House Service API, the Trusted Connector requires other files for its configuration, e.g. a truststore and a keystore with appropriate key material. Please refer to the [Documentation](https://industrial-data-space.github.io/trusted-connector-documentation/) of the Trusted Connector for more information or check the [Examples](https://github.com/industrial-data-space/trusted-connector/tree/master/examples).

#### Environment Variables
The Clearing House Processors can override some standard configuration settings of the Trusted Connector using environment variables. If these variables are not set, the Clearing House Processors will use the standard values provided by the Trusted Connector:
- `TC_DAPS_URL`: The token url of the DAPS used by the Clearing House. The Trusted Connector uses `https://daps.aisec.fraunhofer.de/v2/token` as the default DAPS token url.
- `TC_KEYSTORE_PW`: The password of the key store mounted in the Trusted Connector. Defaults to `password`.
- `TC_TRUSTSTORE_PW`: The password of the trust store mounted in the Trusted Connector. Defaults to `password`.

#### Example Configuration (docker-compose)
```
tc-core:
    container_name: "tc-core"
    image: fraunhoferaisec/trusted-connector-core:5.0.2
    tty: true
    stdin_open: true
    volumes:
        - /var/run/docker.sock:/var/run/docker.sock
        - ./data/trusted-connector/allow-all-flows.pl:/root/deploy/allow-all-flows.pl
        - ./data/trusted-connector/ch-ids.p12:/root/etc/keystore.p12
        - ./data/trusted-connector/truststore.p12:/root/etc/truststore.p12
        - ./data/trusted-connector/clearing-house-processors-0.7.2.jar:/root/jars/clearing-house-processors.jar
        - ./data/trusted-connector/routes/clearing-house-routes.xml:/root/deploy/clearing-house-routes.xml
    environment:
        TC_DAPS_URL: https://<my-daps-url>
    ports:
        - "8443:8443"
        - "9999:9999"
```


## Docker Containers
The Clearing House App can be build using Dockerfiles that are located [here](docker/). There are two types of dockerfiles:
1. Simple builds (e.g. [dockerfile](docker/clearing-house-api.Dockerfile)) that require you to build the Clearing House APIs yourself using [Rust](https://www.rust-lang.org)
2. Multistage builds (e.g. [dockerfile](docker/clearing-house-api-multistage.Dockerfile)) that have a stage for building the rust code

To build the containers check out the repository and in the main directory execute

`docker build -f docker/<dockerfile> . -t <image-name>`

Please read the Clearing House App Configuration section, before using `docker run` oder `docker-compose`. Containers build with the provided dockerfiles need three volumes:
1. The configuration file `Rocket.toml`is expected at `/server/Rocket.toml`
2. The folder containing the signing key needs to match the path configured for the signing key in `Rocket.toml`, e.g. `/sever/keys`
3. The folder containing the daps certificate is expected at `/server/certs`

The Clearing House Processors are not run as docker containers. The Clearing House Processors are needed to configure the Trusted Connector.

## Building from Source
### Clearing House App
The Clearing House App is written in [Rust](https://www.rust-lang.org) and can be build using

```
cd clearing-house-app
cargo build --release
```

The build requires OpenSSL to be installed.

### Clearing House Processors
The Clearing House Processors are written in Java and require Java 11 and can be build using gradle:

```
cd clearing-house-processors
./gradlew build
```
