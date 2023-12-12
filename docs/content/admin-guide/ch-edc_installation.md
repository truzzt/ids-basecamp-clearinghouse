# Installation

## Clearinghouse-edc
This module contains the Clearing House Extension that works with the Eclipse Dataspace Connector
allowing logging operations.

### Configurations
It is required to configure those parameters:

| Parameter name                         | Description                                  | Default value          |
|----------------------------------------|----------------------------------------------|------------------------|
| `truzzt.clearinghouse.jwt.audience`    | Defines the intended recipients of the token | 1                      |
| `truzzt.clearinghouse.jwt.issuer`      | Person or entity offering the token          | 1                      |
| `truzzt.clearinghouse.jwt.sign.secret` | Secret key to encode the token               | 123                    |
| `truzzt.clearinghouse.jwt.expires.at`  | Time to token Expiration (in Seconds)        | 30                     |
| `truzzt.clearinghouse.app.base.url`    | Base URL from the clearing house app         | http://localhost:8000  |

### Build
To build the project run the command below:

    ./gradlew build


### Running
Local execution:

    java -Dedc.fs.config=launchers/connector-local/resources/config.properties -Dedc.keystore=launchers/connector-local/resources/keystore.jks -Dedc.keystore.password=password -Dedc.vault=launchers/connector-local/resources/vault.properties -jar launchers/connector-local/build/libs/clearing-house-edc.jar
