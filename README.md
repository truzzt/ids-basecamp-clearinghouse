# IDS Clearing House
The IDS Clearing House Service is a prototype implementation of the Clearing House component of the [Industrial Data Space](https://github.com/International-Data-Spaces-Association/IDS-G). The Clearing House provides an API to store and retrieve data. Data in the Clearing House is stored encrypted and practically immutable.

## Trusted Connector
The Clearing House Service API is based on the Trusted Connector [Trusted Connector](https://github.com/industrial-data-space/trusted-connector), i.e. it is run as a Data App in the trusted connector. To achieve this, the service consists of two parts:
1. Clearing House App
2. Clearing House Processors

The Clearing House App is a REST API written in RUST that is run in a container in the trusted connector. The Clearing House Processors contain the routes and the CAMEL processors that handle IDS messages that are supported by the Clearing House Service API.

## IDS Clearing House Core
The Clearing House Service API depends on two micro services from the [Clearing House Core](https://github.com/Fraunhofer-AISEC/ids-clearing-house-core):
1. Document API
2. Keyring API

The Document API is responsible for storing the data, while the Keyring API provides cryptographic support for encryption and decryption of the stored data. Please refer to the documentation [here](https://github.com/Fraunhofer-AISEC/ids-clearing-house-core/README.md) how to set up those services.

## Clearing House Service API
The Clearing House Service API is defined by the [IDS-G](https://github.com/International-Data-Spaces-Association/IDS-G/tree/feature/paris/core) and represents the top level service. 

### Clearing House Processors Configuration
The Clearing House Processors are written in Java for use in the Camel Component of the Trusted Connector. The process of setting up a Trusted Connector is described [here](https://industrial-data-space.github.io/trusted-connector-documentation/docs/getting_started/). To configure the Trusted Connector for the Clearing House Service API it needs access to the following files inside the docker container:
- `clearing-house-processors-1.1-SNAPSHOT.jar`: The Clearing House Processors need to be placed in the `/root/jars` folder of the Trusted Connector. The jar file needs to be build from the Clearing House Processors using `gradle`.
- `clearing-house-routes.xml`: The camel routes required by the Clearing House need to be placed in the `/root/deploy` folder of the Trusted Connector.

Other than that the Trusted Connector requires a e.g. a truststore and a keystore with appropriate key material. Please refer to the [Documentation](https://industrial-data-space.github.io/trusted-connector-documentation/) of the Trusted Connector for more information.

### Clearing House App Configuration
The Clearing House App is configured using the configuration file `Rocket.toml`, which must specify a set of configuration options, such as the correct URLs of the database and other service apis:
- `daps_api_url`: Specifies the URL of the DAPS Service. Required to validate DAPS token
- `keyring_api_url`: Specifies the URL of the Keyring API
- `document_api_url`: Specifies the URL of the Document API
- `database_url`: Specifies the URL of the database to store process information. Currently only mongodb is supported so URL is supposed to be `mongodb://<host>:<port>`
- `infomodel_version`: Specifies which Version of the [InfoModel](https://github.com/International-Data-Spaces-Association/InformationModel) is used by the Clearing House. Currently: `4.0.0`
- `connector_name`: Needed for IDS Messages as specified by the [InfoModel](https://github.com/International-Data-Spaces-Association/InformationModel)
- `server_agent`: Needed for IDS Messages as specified by the [InfoModel](https://github.com/International-Data-Spaces-Association/InformationModel)
- `clear_db`: `true` or `false` indicates if the database should be cleared when starting the Service API or not. If `true` a restart will wipe the database! Starting the Service API on a clean database will initialize the database.
- `signing_key`: Location of the private key (DER format) used for signing the Receipts. Clearing House uses PS512 algorithm for signing.

More information on general configuration options in a `Rocket.toml` file can be found [here](https://rocket.rs/v0.5-rc/guide/configuration/#rockettoml). An example configuration is located [here](clearing-house-api/Rocket.toml).

#### Logging
When starting the Clearing House Service API it also needs the following environment variables set:
- `API_LOG_LEVEL`: Allowed log levels are: `Off`, `Error`, `Warn`, `Info`, `Debug`, `Trace`

#### Signing Key
The Clearing House API sends a signed receipt as response to a logging request. The key can be created using openssl:

`openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:4096 -outform der -out private_key.der`

Please note that the Clearing House requires the key to be in DER format.

#### DAPS
The Clearing House needs to be able to validate the certificate used by the DAPS. If the DAPS uses a self-signed certificate the certificate needs to be added in two places:
1. `/server/certs`: The Clearing House App will load certificates in this folder and use them for validation. The certificate needs to be in DER format.
2. `/usr/local/share/ca-certificates`: The Clearing House App relies on openssl for parts of the validation and openssl will not trust a self-signed certificate unless it was added in this folder and `update-ca-certificates` was called. Once this is done the container might need to be restarted.

If you are using the prebuild docker containers and use `daps.aisec.fraunhofer.de` as the DAPS, only Step 1 is required.

### Data Immutability
There are multiple ways in which the Clearing House enforces Data Immutability:
- Using the Clearing House Service API there is no way to update an already existing log entry in the database
- Log entries in the database include a hash value of the previous log entry, chaining together all log entries. Any change to a previous log entry would require rehashing all following log entries.
- The connector logging information in the Clearing House receives a signed receipt from the Clearing House that includes among other things a timestamp and the current chain hash. A single valid receipt in possession of any connector is enough to detect any change to data up to the time indicated in the receipt.

## Docker Containers
Dockerfiles are located [here](docker/). There are two types of dockerfiles:
1. Simple builds (e.g. [dockerfile](docker/clearing-house-api.Dockerfile)) that require you to build the Clearing House APIs yourself using [Rust](https://www.rust-lang.org)
2. Multistage builds (e.g. [dockerfile](docker/clearingh-house-api-multistage.Dockerfile)) that have a stage for building the rust code

To build the containers check out the repository and in the main directory execute

`docker build -f docker/<dockerfile> . -t <image-name>`

Please read the Clearing House App Configuration section, before using `docker run` oder `docker-compose`. Containers build with the provided dockerfiles need three volumes:
1. The configuration file `Rocket.toml`is expected at `/server/Rocket.toml`
2. The folder containing the signing key is expected at `/server/keys`
3. The folder containing the daps certificate is expected at `/server/certs`

