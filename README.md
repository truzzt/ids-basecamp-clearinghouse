# IDS Clearing House
The IDS Clearing House Service is a prototype implementation of the Clearing House component of the [Industrial Data Space](https://github.com/International-Data-Spaces-Association/IDS-G).

## Trusted Connector
The Clearing House Service is based on the Trusted Connector [Trusted Connector](https://github.com/industrial-data-space/trusted-connector), i.e. it is run as a Data App in the trusted connector. To achieve this, the service consists of two parts:
1. Clearing House App
2. Clearing House Processors
The Clearing House App is a REST API written in RUST that is supposed to run in a container in the trusted connector. The Clearing House Processors contain the routes and the CAMEL processors that handle IDS messages that are supported by the [Clearing House API](https://github.com/International-Data-Spaces-Association/IDS-G-pre/blob/clearinghouse/Components/ClearingHouse/README.md#example-http-multipart-request).

## IDS Clearing House Core
The prototype implementation of the Clearing House service uses a microservice architecture that consists of three services:
1. Clearing House Service API
2. Document API
3. Keyring API

### Clearing House Service API
The Clearing House Service API is defined by the [IDS-G](https://github.com/International-Data-Spaces-Association/IDS-G/tree/feature/paris/core) and represents the top level service.

### Document API
The Document API is responsible for storing and retrieving persistent data. It uses the Keyring API to get the symmetric keys for encryption and decryption of data at rest.

### Keyring API
The Keyring API is responsible for the creation and retrieval of symmetric keys for encryption and decryption of data.

### Configuration
All APIs are configured using the configuration file `Rocket.toml` which must specify the correct URLs of the respective database and the other service apis on which a service depends.
